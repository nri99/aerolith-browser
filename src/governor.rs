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

#[link(name = "kernel32")]
unsafe extern "system" {
    fn GlobalMemoryStatusEx(lp_buffer: *mut MEMORYSTATUSEX) -> i32;
    fn CreateMemoryResourceNotification(notification_type: u32) -> *mut std::ffi::c_void;
    fn QueryMemoryResourceNotification(resource_notification_handle: *mut std::ffi::c_void, resource_state: *mut i32) -> i32;
    fn CloseHandle(h_object: *mut std::ffi::c_void) -> i32;
}

const LOW_MEMORY_RESOURCE_NOTIFICATION: u32 = 0;

pub struct MemoryGovernor {
    low_mem_handle: *mut std::ffi::c_void,
}

// MemoryGovernor can be safely held in the main thread event loop
unsafe impl Send for MemoryGovernor {}
unsafe impl Sync for MemoryGovernor {}

impl MemoryGovernor {
    pub fn new() -> Self {
        let handle = unsafe { CreateMemoryResourceNotification(LOW_MEMORY_RESOURCE_NOTIFICATION) };
        Self {
            low_mem_handle: handle,
        }
    }

    /// Queries current system physical memory status
    pub fn query_memory_status() -> Option<MEMORYSTATUSEX> {
        let mut status = MEMORYSTATUSEX::default();
        let ret = unsafe { GlobalMemoryStatusEx(&mut status) };
        if ret != 0 {
            Some(status)
        } else {
            None
        }
    }

    /// Checks if Windows is experiencing memory pressure (e.g. Memory Load > 80% or Kernel LowMemory Notification)
    pub fn is_memory_pressure_high(&self) -> bool {
        // 1. Check Win32 Kernel Resource Notification
        if !self.low_mem_handle.is_null() {
            let mut state: i32 = 0;
            let query_ret = unsafe { QueryMemoryResourceNotification(self.low_mem_handle, &mut state) };
            if query_ret != 0 && state != 0 {
                return true;
            }
        }

        // 2. Check GlobalMemoryStatusEx threshold (< 1.5 GB physical available or load >= 80%)
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
        if !self.low_mem_handle.is_null() {
            unsafe {
                CloseHandle(self.low_mem_handle);
            }
            self.low_mem_handle = std::ptr::null_mut();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_status_query() {
        let status = MemoryGovernor::query_memory_status();
        assert!(status.is_some(), "GlobalMemoryStatusEx must succeed on Windows");
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
