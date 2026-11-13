use crate::types::LOGICAL_CHROME_HEIGHT;
use tao::dpi::{PhysicalPosition, PhysicalSize};
use wry::Rect;

#[inline]
pub fn physical_chrome_height(scale_factor: f64) -> u32 {
    (LOGICAL_CHROME_HEIGHT * scale_factor).round() as u32
}

pub fn chrome_expanded_height(mode: &str, scale_factor: f64, window_height: u32) -> u32 {
    match mode {
        "full" => window_height,
        "dropdown" => {
            let extra = (300.0 * scale_factor).round() as u32;
            (physical_chrome_height(scale_factor) + extra).min(window_height)
        }
        _ => physical_chrome_height(scale_factor),
    }
}

pub fn content_rect(size: PhysicalSize<u32>, scale_factor: f64) -> Rect {
    let chrome_h = physical_chrome_height(scale_factor);
    let h = if size.height > chrome_h {
        size.height - chrome_h
    } else {
        100
    };
    Rect {
        position: PhysicalPosition::new(0, chrome_h as i32).into(),
        size: PhysicalSize::new(size.width, h).into(),
    }
}

pub fn reassert_chrome_bounds(
    chrome_wv: &wry::WebView,
    mode: &str,
    scale_factor: f64,
    window_size: PhysicalSize<u32>,
) {
    let chrome_h = chrome_expanded_height(mode, scale_factor, window_size.height);
    let _ = chrome_wv.set_bounds(Rect {
        position: PhysicalPosition::new(0, 0).into(),
        size: PhysicalSize::new(window_size.width, chrome_h).into(),
    });
    let _ = chrome_wv.set_visible(true);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_physical_chrome_height_scaling() {
        assert_eq!(physical_chrome_height(1.0), 84);
        assert_eq!(physical_chrome_height(1.25), 105);
        assert_eq!(physical_chrome_height(1.5), 126);
        assert_eq!(physical_chrome_height(2.0), 168);
    }

    #[test]
    fn test_content_rect_calculation() {
        let size = PhysicalSize::new(1280, 853);
        let rect = content_rect(size, 1.0);
        let pos: (f64, f64) = rect.position.to_logical::<f64>(1.0).into();
        let sz: (f64, f64) = rect.size.to_logical::<f64>(1.0).into();
        assert_eq!(pos.0 as i32, 0);
        assert_eq!(pos.1 as i32, 84);
        assert_eq!(sz.0 as u32, 1280);
        assert_eq!(sz.1 as u32, 853 - 84);

        // Clamping check when window height is smaller than chrome height
        let tiny_size = PhysicalSize::new(500, 50);
        let tiny_rect = content_rect(tiny_size, 1.0);
        let tiny_sz: (f64, f64) = tiny_rect.size.to_logical::<f64>(1.0).into();
        assert_eq!(tiny_sz.1 as u32, 100);
    }

    #[test]
    fn test_chrome_expanded_height_modes() {
        let win_h = 800;
        assert_eq!(chrome_expanded_height("none", 1.0, win_h), 84);
        assert_eq!(chrome_expanded_height("dropdown", 1.0, win_h), 84 + 300);
        assert_eq!(chrome_expanded_height("full", 1.0, win_h), win_h);

        // Surface Go 2 150% scale (scale_factor = 1.5)
        let base_15x = physical_chrome_height(1.5); // 126
        let extra_15x = (300.0 * 1.5 as f64).round() as u32; // 450
        assert_eq!(chrome_expanded_height("none", 1.5, win_h), base_15x);
        assert_eq!(chrome_expanded_height("dropdown", 1.5, win_h), base_15x + extra_15x);
        assert_eq!(chrome_expanded_height("full", 1.5, win_h), win_h);

        // Check clamping if dropdown extra exceeds window height
        let tiny_win_h = 200;
        assert_eq!(chrome_expanded_height("dropdown", 1.0, tiny_win_h), tiny_win_h);
    }

    #[test]
    fn test_tab_bounds_never_obscure_chrome() {
        let scale_factors = [1.0, 1.25, 1.5, 2.0];
        let window_sizes = [(1280, 853), (1920, 1080), (1920, 1280), (800, 600)];

        for &sf in &scale_factors {
            let chrome_h = physical_chrome_height(sf);
            assert!(chrome_h >= 84);

            for &(w, h) in &window_sizes {
                let win_size = PhysicalSize::new(w, h);
                let c_rect = content_rect(win_size, sf);

                let pos: (f64, f64) = c_rect.position.to_logical::<f64>(1.0).into();
                assert_eq!(pos.0 as i32, 0);
                assert_eq!(pos.1 as u32, chrome_h, "Content rect y must begin at physical chrome height");

                let size: (f64, f64) = c_rect.size.to_logical::<f64>(1.0).into();
                let content_top = pos.1 as u32;
                let content_bottom = content_top + size.1 as u32;
                assert!(content_top >= chrome_h);
                assert!(content_bottom >= content_top);
            }
        }
    }
}
