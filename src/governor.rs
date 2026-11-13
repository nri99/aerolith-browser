#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MEMORYSTATUSEX {
    pub dw_length: u32,
    pub dw_memory_load: u32,
    pub ull_total_phys: u64,
    pub ull_avail_phys: u64,
    pub ull_total_page_file: u64,
    pub ull_avail_page_file: u64,
    pub ull_total_virtual: u64,
    pub ull_avail_virtual: u64,
    pub ull_avail_extended_virtual: u64,
}

impl Default for MEMORYSTATUSEX {
    fn default() -> Self {
        Self {
            dw_length: std::mem::size_of::<MEMORYSTATUSEX>() as u32,
            dw_memory_load: 0,
            ull_total_phys: 0,
            ull_avail_phys: 0,
            ull_total_page_file: 0,
            ull_avail_page_file: 0,
            ull_total_virtual: 0,
            ull_avail_virtual: 0,
            ull_avail_extended_virtual: 0,
        }
    }
}

#[cfg(target_os = "windows")]
#[link(name = "kernel32")]
unsafe extern "system" {
    fn GlobalMemoryStatusEx(lp_buffer: *mut MEMORYSTATUSEX) -> i32;
    fn CreateMemoryResourceNotification(notification_type: u32) -> *mut std::ffi::c_void;
    fn QueryMemoryResourceNotification(resource_notification_handle: *mut std::ffi::c_void, resource_state: *mut i32) -> i32;
    fn CloseHandle(h_object: *mut std::ffi::c_void) -> i32;
}

#[cfg(target_os = "windows")]
const LOW_MEMORY_RESOURCE_NOTIFICATION: u32 = 0;

pub struct MemoryGovernor {
    #[cfg(target_os = "windows")]
    low_mem_handle: *mut std::ffi::c_void,
}

// MemoryGovernor can be safely held in the main thread event loop
unsafe impl Send for MemoryGovernor {}
unsafe impl Sync for MemoryGovernor {}

impl MemoryGovernor {
    pub fn new() -> Self {
        #[cfg(target_os = "windows")]
        {
            let handle = unsafe { CreateMemoryResourceNotification(LOW_MEMORY_RESOURCE_NOTIFICATION) };
            Self {
                low_mem_handle: handle,
            }
        }
        #[cfg(not(target_os = "windows"))]
        {
            Self {}
        }
    }

    /// Queries current system physical memory status across Windows, Linux, and macOS
    pub fn query_memory_status() -> Option<MEMORYSTATUSEX> {
        #[cfg(target_os = "windows")]
        {
            let mut status = MEMORYSTATUSEX::default();
            let ret = unsafe { GlobalMemoryStatusEx(&mut status) };
            if ret != 0 {
                Some(status)
            } else {
                None
            }
        }

        #[cfg(target_os = "linux")]
        {
            let content = std::fs::read_to_string("/proc/meminfo").ok()?;
            let mut total_kb: Option<u64> = None;
            let mut avail_kb: Option<u64> = None;
            for line in content.lines() {
                if line.starts_with("MemTotal:") {
                    total_kb = line.split_whitespace().nth(1).and_then(|s| s.parse().ok());
                } else if line.starts_with("MemAvailable:") {
                    avail_kb = line.split_whitespace().nth(1).and_then(|s| s.parse().ok());
                }
                if total_kb.is_some() && avail_kb.is_some() {
                    break;
                }
            }
            let total = total_kb? * 1024;
            let avail = avail_kb? * 1024;
            let load = if total > 0 {
                (((total.saturating_sub(avail)) as f64 / total as f64) * 100.0) as u32
            } else {
                0
            };
            let mut status = MEMORYSTATUSEX::default();
            status.dw_memory_load = load.min(100);
            status.ull_total_phys = total;
            status.ull_avail_phys = avail;
            Some(status)
        }

        #[cfg(target_os = "macos")]
        {
            let total_bytes = std::process::Command::new("sysctl")
                .arg("-n")
                .arg("hw.memsize")
                .output()
                .ok()
                .and_then(|out| {
                    String::from_utf8_lossy(&out.stdout)
                        .trim()
                        .parse::<u64>()
                        .ok()
                })
                .unwrap_or(8 * 1024 * 1024 * 1024);

            let vm_output = std::process::Command::new("vm_stat").output().ok();
            let mut free_pages: u64 = 0;
            if let Some(out) = vm_output {
                let vm_str = String::from_utf8_lossy(&out.stdout);
                for line in vm_str.lines() {
                    if line.starts_with("Pages free:") || line.starts_with("Pages speculative:") {
                        let num_str: String = line.chars().filter(|c| c.is_ascii_digit()).collect();
                        if let Ok(p) = num_str.parse::<u64>() {
                            free_pages += p;
                        }
                    }
                }
            }
            let avail_bytes = if free_pages > 0 {
                free_pages * 4096
            } else {
                total_bytes / 2
            };
            let load = if total_bytes > 0 {
                (((total_bytes.saturating_sub(avail_bytes)) as f64 / total_bytes as f64) * 100.0) as u32
            } else {
                50
            };
            let mut status = MEMORYSTATUSEX::default();
            status.dw_memory_load = load.min(100);
            status.ull_total_phys = total_bytes;
            status.ull_avail_phys = avail_bytes;
            Some(status)
        }

        #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
        {
            let mut status = MEMORYSTATUSEX::default();
            status.dw_memory_load = 50;
            status.ull_total_phys = 8 * 1024 * 1024 * 1024;
            status.ull_avail_phys = 4 * 1024 * 1024 * 1024;
            Some(status)
        }
    }

    /// Checks if the operating system is experiencing memory pressure (e.g. Memory Load > 80% or low physical RAM)
    pub fn is_memory_pressure_high(&self) -> bool {
        #[cfg(target_os = "windows")]
        {
            if !self.low_mem_handle.is_null() {
                let mut state: i32 = 0;
                let query_ret = unsafe { QueryMemoryResourceNotification(self.low_mem_handle, &mut state) };
                if query_ret != 0 && state != 0 {
                    return true;
                }
            }
        }

        if let Some(status) = Self::query_memory_status() {
            if status.dw_memory_load >= 80 || status.ull_avail_phys < 1_500_000_000 {
                return true;
            }
        }

        false
    }

    #[allow(dead_code)]
    pub fn get_available_mb() -> u64 {
        Self::query_memory_status()
            .map(|s| s.ull_avail_phys / (1024 * 1024))
            .unwrap_or(0)
    }

    #[allow(dead_code)]
    pub fn get_memory_load_pct() -> u32 {
        Self::query_memory_status()
            .map(|s| s.dw_memory_load)
            .unwrap_or(0)
    }
}

impl Drop for MemoryGovernor {
    fn drop(&mut self) {
        #[cfg(target_os = "windows")]
        {
            if !self.low_mem_handle.is_null() {
                unsafe {
                    CloseHandle(self.low_mem_handle);
                }
                self.low_mem_handle = std::ptr::null_mut();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_status_query() {
        let status = MemoryGovernor::query_memory_status();
        assert!(status.is_some(), "Memory status query must succeed across platforms");
        let s = status.unwrap();
        assert!(s.ull_total_phys > 0, "Total physical memory must be > 0");
        assert!(s.dw_memory_load <= 100, "Memory load percentage must be <= 100");
    }

    #[test]
    fn test_memory_governor_lifecycle() {
        let governor = MemoryGovernor::new();
        let _is_high = governor.is_memory_pressure_high();
        let avail_mb = MemoryGovernor::get_available_mb();
        assert!(avail_mb > 0, "Available physical RAM must be > 0");
    }
}
