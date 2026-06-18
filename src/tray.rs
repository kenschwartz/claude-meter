use crate::providers::claude::UsageResponse;
use crate::ui::colors::ColorRef;
use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::Graphics::Gdi::{
    CreateCompatibleDC, CreateDIBSection, CreateFontW, DeleteDC, DeleteObject, DrawTextW,
    SelectObject, SetBkMode, SetTextColor, BITMAPINFO, BITMAPINFOHEADER, BI_RGB,
    CLIP_DEFAULT_PRECIS, DEFAULT_CHARSET, DEFAULT_PITCH, DIB_RGB_COLORS, DT_CENTER, DT_SINGLELINE,
    DT_VCENTER, FF_DONTCARE, FW_BOLD, OUT_DEFAULT_PRECIS, PROOF_QUALITY, TRANSPARENT,
};
use windows::Win32::UI::Shell::{
    Shell_NotifyIconW, NIF_ICON, NIF_INFO, NIF_MESSAGE, NIF_TIP, NIM_ADD, NIM_DELETE, NIM_MODIFY,
    NOTIFYICONDATAW,
};
use windows::Win32::UI::WindowsAndMessaging::{
    CreateIconIndirect, DestroyIcon, LoadIconW, LoadImageW, HICON, ICONINFO, IDI_APPLICATION,
    IMAGE_ICON, LR_DEFAULTSIZE, LR_SHARED, WM_USER,
};

// Tray callback message
pub const WM_TRAY_ICON: u32 = WM_USER + 1;
pub const TRAY_ID: u32 = 1;

// Context menu command IDs
pub const IDM_REFRESH: u32 = 1001;
pub const IDM_OPEN_DASHBOARD: u32 = 1002;
pub const IDM_OPEN_CLAUDE: u32 = 1003;
pub const IDM_OPEN_CHATGPT: u32 = 1004;
pub const IDM_SETTINGS: u32 = 1005;
pub const IDM_AUTOSTART: u32 = 1006;
pub const IDM_ABOUT: u32 = 1007;
pub const IDM_EXPORT_CSV: u32 = 1009;
pub const IDM_EXIT: u32 = 1008;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TrayIconColor {
    Green,
    Yellow,
    Red,
    Gray,
}

impl TrayIconColor {
    pub fn from_utilization(max_util: Option<f64>) -> Self {
        match max_util {
            None => Self::Gray,
            Some(u) if u >= 80.0 => Self::Red,
            Some(u) if u >= 50.0 => Self::Yellow,
            Some(_) => Self::Green,
        }
    }

    fn to_colorref(self) -> ColorRef {
        use crate::ui::colors::rgb;
        match self {
            Self::Green => rgb(64, 160, 43),   // #40a02b
            Self::Yellow => rgb(223, 142, 29), // #df8e1d
            Self::Red => rgb(210, 15, 57),     // #d20f39
            Self::Gray => rgb(128, 128, 128),
        }
    }

    /// Text color for the tray icon number (GDI COLORREF format: 0x00BBGGRR)
    fn text_colorref(self) -> u32 {
        match self {
            Self::Yellow => 0x00000000, // black text on yellow
            Self::Green | Self::Red | Self::Gray => 0x00FFFFFF, // white text on others
        }
    }
}

pub struct TrayIcon {
    hwnd: HWND,
    current_color: TrayIconColor,
    icon_green: HICON,
    icon_yellow: HICON,
    icon_red: HICON,
    icon_gray: HICON,
    dynamic_icon: Option<HICON>,
}

// Resource IDs for embedded tray icons (must match build.rs)
const ICON_GREEN_ID: u16 = 101;
const ICON_YELLOW_ID: u16 = 102;
const ICON_RED_ID: u16 = 103;
const ICON_GRAY_ID: u16 = 104;

fn load_icon_from_resource(resource_id: u16) -> Result<HICON, String> {
    unsafe {
        let hinstance = windows::Win32::System::LibraryLoader::GetModuleHandleW(None)
            .map_err(|e| format!("GetModuleHandleW: {e}"))?;
        let handle = LoadImageW(
            hinstance,
            windows::core::PCWSTR(resource_id as usize as *const u16),
            IMAGE_ICON,
            16,
            16,
            LR_DEFAULTSIZE | LR_SHARED,
        )
        .map_err(|e| format!("Failed to load icon resource {resource_id}: {e}"))?;
        Ok(HICON(handle.0))
    }
}

/// Create a 16x16 icon with text rendered on a colored background.
fn create_text_icon(text: &str, font_size: i32, color: ColorRef, text_color: u32) -> Option<HICON> {
    const SIZE: i32 = 16;

    unsafe {
        let dc = CreateCompatibleDC(None);
        if dc.is_invalid() {
            return None;
        }

        // Extract RGB from COLORREF (0x00BBGGRR)
        let cr = color.0;
        let r = (cr & 0xFF) as u8;
        let g = ((cr >> 8) & 0xFF) as u8;
        let b = ((cr >> 16) & 0xFF) as u8;

        // Create 32-bit DIB section for the color bitmap
        let bmi = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: SIZE,
                biHeight: -SIZE, // top-down
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB.0,
                ..Default::default()
            },
            ..Default::default()
        };

        let mut bits: *mut std::ffi::c_void = std::ptr::null_mut();
        let hbm_color = CreateDIBSection(dc, &bmi, DIB_RGB_COLORS, &mut bits, None, 0);
        if hbm_color.is_err() {
            let _ = DeleteDC(dc);
            return None;
        }
        let hbm_color = hbm_color.unwrap();

        let old_bm = SelectObject(dc, hbm_color);

        // Fill with the color (BGRA format, fully opaque)
        let pixels = std::slice::from_raw_parts_mut(bits as *mut u32, (SIZE * SIZE) as usize);
        let bg_pixel = 0xFF000000 | (r as u32) << 16 | (g as u32) << 8 | b as u32;
        for px in pixels.iter_mut() {
            *px = bg_pixel;
        }

        // Draw text
        let text_wide: Vec<u16> = text.encode_utf16().chain(std::iter::once(0)).collect();

        let font_name: Vec<u16> = "Segoe UI"
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect();
        let mut face_name = [0u16; 32];
        let copy_len = font_name.len().min(31);
        face_name[..copy_len].copy_from_slice(&font_name[..copy_len]);

        let font = CreateFontW(
            -font_size,
            0,
            0,
            0,
            FW_BOLD.0 as i32,
            0,
            0,
            0,
            DEFAULT_CHARSET.0 as u32,
            OUT_DEFAULT_PRECIS.0 as u32,
            CLIP_DEFAULT_PRECIS.0 as u32,
            PROOF_QUALITY.0 as u32,
            (DEFAULT_PITCH.0 | FF_DONTCARE.0) as u32,
            windows::core::PCWSTR(face_name.as_ptr()),
        );

        let old_font = SelectObject(dc, font);
        let _ = SetBkMode(dc, TRANSPARENT);
        SetTextColor(dc, windows::Win32::Foundation::COLORREF(text_color));

        let mut rc = RECT {
            left: 0,
            top: 0,
            right: SIZE,
            bottom: SIZE,
        };
        DrawTextW(
            dc,
            &mut text_wide[..text_wide.len() - 1].to_vec(),
            &mut rc,
            DT_CENTER | DT_VCENTER | DT_SINGLELINE,
        );

        // Make text pixels fully opaque (DrawTextW doesn't set alpha)
        let pixels = std::slice::from_raw_parts_mut(bits as *mut u32, (SIZE * SIZE) as usize);
        for px in pixels.iter_mut() {
            if *px != bg_pixel {
                *px |= 0xFF000000; // set alpha to 255
            }
        }

        SelectObject(dc, old_font);
        SelectObject(dc, old_bm);
        let _ = DeleteObject(font);

        // Create mask bitmap (all zeros = fully opaque)
        let mask_bmi = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: SIZE,
                biHeight: -SIZE,
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB.0,
                ..Default::default()
            },
            ..Default::default()
        };
        let mut mask_bits: *mut std::ffi::c_void = std::ptr::null_mut();
        let hbm_mask = CreateDIBSection(dc, &mask_bmi, DIB_RGB_COLORS, &mut mask_bits, None, 0);
        if hbm_mask.is_err() {
            let _ = DeleteObject(hbm_color);
            let _ = DeleteDC(dc);
            return None;
        }
        let hbm_mask = hbm_mask.unwrap();

        // mask_bits is already zeroed (all opaque)

        let icon_info = ICONINFO {
            fIcon: true.into(),
            xHotspot: 0,
            yHotspot: 0,
            hbmMask: hbm_mask,
            hbmColor: hbm_color,
        };

        let icon = CreateIconIndirect(&icon_info).ok();

        let _ = DeleteObject(hbm_color);
        let _ = DeleteObject(hbm_mask);
        let _ = DeleteDC(dc);

        icon
    }
}

/// Create a 16x16 icon with a circular progress ring.
/// Ring fills clockwise from 12 o'clock based on percentage.
fn create_ring_icon(pct: f64, color: ColorRef) -> Option<HICON> {
    const SIZE: i32 = 16;

    unsafe {
        let dc = CreateCompatibleDC(None);
        if dc.is_invalid() {
            return None;
        }

        let bmi = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: SIZE,
                biHeight: -SIZE,
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB.0,
                ..Default::default()
            },
            ..Default::default()
        };

        let mut bits: *mut std::ffi::c_void = std::ptr::null_mut();
        let hbm_color = CreateDIBSection(dc, &bmi, DIB_RGB_COLORS, &mut bits, None, 0);
        if hbm_color.is_err() {
            let _ = DeleteDC(dc);
            return None;
        }
        let hbm_color = hbm_color.unwrap();

        let pixels = std::slice::from_raw_parts_mut(bits as *mut u32, (SIZE * SIZE) as usize);

        // Transparent background
        let bg_pixel: u32 = 0x00000000;

        // Ring color from ColorRef (0x00BBGGRR)
        let cr = color.0;
        let ring_r = cr & 0xFF;
        let ring_g = (cr >> 8) & 0xFF;
        let ring_b = (cr >> 16) & 0xFF;
        let ring_pixel = 0xFF000000 | (ring_r << 16) | (ring_g << 8) | ring_b;

        // Track color (semi-transparent gray)
        let track_pixel = 0x80505050;

        let cx = 7.5_f64;
        let cy = 7.5_f64;
        let outer_r = 7.0_f64;
        let inner_r = 4.0_f64;
        let fill_angle = (pct / 100.0) * 360.0;

        for row in 0..SIZE {
            for col in 0..SIZE {
                let dx = col as f64 - cx;
                let dy = row as f64 - cy;
                let dist = (dx * dx + dy * dy).sqrt();

                if dist >= inner_r && dist <= outer_r {
                    // In the ring area — check angle
                    // atan2: 0 = right, PI/2 = down; we want 0 = top, clockwise
                    let mut angle = (dx.atan2(-dy)).to_degrees();
                    if angle < 0.0 {
                        angle += 360.0;
                    }

                    if angle <= fill_angle {
                        pixels[(row * SIZE + col) as usize] = ring_pixel;
                    } else {
                        pixels[(row * SIZE + col) as usize] = track_pixel;
                    }
                } else {
                    pixels[(row * SIZE + col) as usize] = bg_pixel;
                }
            }
        }

        // Create mask bitmap
        let mask_bmi = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: SIZE,
                biHeight: -SIZE,
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB.0,
                ..Default::default()
            },
            ..Default::default()
        };
        let mut mask_bits: *mut std::ffi::c_void = std::ptr::null_mut();
        let hbm_mask = CreateDIBSection(dc, &mask_bmi, DIB_RGB_COLORS, &mut mask_bits, None, 0);
        if hbm_mask.is_err() {
            let _ = DeleteObject(hbm_color);
            let _ = DeleteDC(dc);
            return None;
        }
        let hbm_mask = hbm_mask.unwrap();

        let icon_info = ICONINFO {
            fIcon: true.into(),
            xHotspot: 0,
            yHotspot: 0,
            hbmMask: hbm_mask,
            hbmColor: hbm_color,
        };

        let icon = CreateIconIndirect(&icon_info).ok();

        let _ = DeleteObject(hbm_color);
        let _ = DeleteObject(hbm_mask);
        let _ = DeleteDC(dc);

        icon
    }
}

/// Create a 16x16 icon with a vertical progress bar that fills upward.
fn create_bar_icon(pct: f64, color: ColorRef) -> Option<HICON> {
    const SIZE: i32 = 16;

    unsafe {
        let dc = CreateCompatibleDC(None);
        if dc.is_invalid() {
            return None;
        }

        let bmi = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: SIZE,
                biHeight: -SIZE,
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB.0,
                ..Default::default()
            },
            ..Default::default()
        };

        let mut bits: *mut std::ffi::c_void = std::ptr::null_mut();
        let hbm_color = CreateDIBSection(dc, &bmi, DIB_RGB_COLORS, &mut bits, None, 0);
        if hbm_color.is_err() {
            let _ = DeleteDC(dc);
            return None;
        }
        let hbm_color = hbm_color.unwrap();

        let pixels = std::slice::from_raw_parts_mut(bits as *mut u32, (SIZE * SIZE) as usize);

        // Transparent background
        let bg_pixel: u32 = 0x00000000;

        // Bar color from ColorRef (0x00BBGGRR)
        let cr = color.0;
        let bar_r = cr & 0xFF;
        let bar_g = (cr >> 8) & 0xFF;
        let bar_b = (cr >> 16) & 0xFF;
        let bar_pixel = 0xFF000000 | (bar_r << 16) | (bar_g << 8) | bar_b;

        // Track color (semi-transparent gray)
        let track_pixel = 0x80505050;

        // Bar area: x=2..14 (12px wide), y=1..15 (14px tall)
        let bar_left = 2;
        let bar_right = 14;
        let bar_top = 1;
        let bar_bottom = 15;
        let bar_height = (bar_bottom - bar_top) as f64;
        let fill_height = ((pct / 100.0) * bar_height).round() as i32;

        for row in 0..SIZE {
            for col in 0..SIZE {
                let idx = (row * SIZE + col) as usize;
                if col >= bar_left && col < bar_right && row >= bar_top && row < bar_bottom {
                    // Inside bar area
                    let row_from_bottom = bar_bottom - 1 - row;
                    if row_from_bottom < fill_height {
                        pixels[idx] = bar_pixel;
                    } else {
                        pixels[idx] = track_pixel;
                    }
                } else {
                    pixels[idx] = bg_pixel;
                }
            }
        }

        // Create mask bitmap
        let mask_bmi = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: SIZE,
                biHeight: -SIZE,
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB.0,
                ..Default::default()
            },
            ..Default::default()
        };
        let mut mask_bits: *mut std::ffi::c_void = std::ptr::null_mut();
        let hbm_mask = CreateDIBSection(dc, &mask_bmi, DIB_RGB_COLORS, &mut mask_bits, None, 0);
        if hbm_mask.is_err() {
            let _ = DeleteObject(hbm_color);
            let _ = DeleteDC(dc);
            return None;
        }
        let hbm_mask = hbm_mask.unwrap();

        let icon_info = ICONINFO {
            fIcon: true.into(),
            xHotspot: 0,
            yHotspot: 0,
            hbmMask: hbm_mask,
            hbmColor: hbm_color,
        };

        let icon = CreateIconIndirect(&icon_info).ok();

        let _ = DeleteObject(hbm_color);
        let _ = DeleteObject(hbm_mask);
        let _ = DeleteDC(dc);

        icon
    }
}

/// Create a 16x16 icon with a pie chart showing metrics as proportional sectors.
fn create_pie_icon(metrics: &[(f64, u32)]) -> Option<HICON> {
    const SIZE: i32 = 16;

    unsafe {
        let dc = CreateCompatibleDC(None);
        if dc.is_invalid() {
            return None;
        }

        let bmi = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: SIZE,
                biHeight: -SIZE,
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB.0,
                ..Default::default()
            },
            ..Default::default()
        };

        let mut bits: *mut std::ffi::c_void = std::ptr::null_mut();
        let hbm_color = CreateDIBSection(dc, &bmi, DIB_RGB_COLORS, &mut bits, None, 0);
        if hbm_color.is_err() {
            let _ = DeleteDC(dc);
            return None;
        }
        let hbm_color = hbm_color.unwrap();

        let pixels = std::slice::from_raw_parts_mut(bits as *mut u32, (SIZE * SIZE) as usize);

        // Transparent background
        let bg_pixel: u32 = 0x00000000;

        // Track color (semi-transparent gray)
        let track_pixel = 0x80505050;

        let cx = 7.5_f64;
        let cy = 7.5_f64;
        let radius = 6.5_f64;

        let total: f64 = metrics.iter().map(|(u, _)| u).sum::<f64>().max(0.001);

        for row in 0..SIZE {
            for col in 0..SIZE {
                let dx = col as f64 - cx;
                let dy = row as f64 - cy;
                let dist = (dx * dx + dy * dy).sqrt();
                let idx = (row * SIZE + col) as usize;

                if dist <= radius {
                    let mut angle = dx.atan2(-dy).to_degrees();
                    if angle < 0.0 {
                        angle += 360.0;
                    }

                    let mut sector_start = 0.0_f64;
                    let mut chosen_pixel = track_pixel;
                    for (util, color_rgb) in metrics {
                        let sector_angle = (util / total) * 360.0;
                        if angle >= sector_start && angle < sector_start + sector_angle {
                            // color_rgb is 0x00RRGGBB
                            let r = (color_rgb >> 16) & 0xFF;
                            let g = (color_rgb >> 8) & 0xFF;
                            let b = color_rgb & 0xFF;
                            chosen_pixel = 0xFF000000 | (r << 16) | (g << 8) | b;
                            break;
                        }
                        sector_start += sector_angle;
                    }

                    pixels[idx] = chosen_pixel;
                } else {
                    pixels[idx] = bg_pixel;
                }
            }
        }

        // Create mask bitmap
        let mask_bmi = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: SIZE,
                biHeight: -SIZE,
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB.0,
                ..Default::default()
            },
            ..Default::default()
        };
        let mut mask_bits: *mut std::ffi::c_void = std::ptr::null_mut();
        let hbm_mask = CreateDIBSection(dc, &mask_bmi, DIB_RGB_COLORS, &mut mask_bits, None, 0);
        if hbm_mask.is_err() {
            let _ = DeleteObject(hbm_color);
            let _ = DeleteDC(dc);
            return None;
        }
        let hbm_mask = hbm_mask.unwrap();

        let icon_info = ICONINFO {
            fIcon: true.into(),
            xHotspot: 0,
            yHotspot: 0,
            hbmMask: hbm_mask,
            hbmColor: hbm_color,
        };

        let icon = CreateIconIndirect(&icon_info).ok();

        let _ = DeleteObject(hbm_color);
        let _ = DeleteObject(hbm_mask);
        let _ = DeleteDC(dc);

        icon
    }
}

/// Pie chart color palette (Catppuccin-inspired)
const PIE_PALETTE: [u32; 5] = [
    0x40a02b, // green
    0x89b4fa, // blue
    0xdf8e1d, // amber
    0xcba6f7, // lavender
    0xea76cb, // pink
];

impl TrayIcon {
    pub fn new(hwnd: HWND) -> Result<Self, String> {
        let fallback = unsafe { LoadIconW(None, IDI_APPLICATION).map_err(|e| e.to_string())? };

        let icon_green = load_icon_from_resource(ICON_GREEN_ID).unwrap_or(fallback);
        let icon_yellow = load_icon_from_resource(ICON_YELLOW_ID).unwrap_or(fallback);
        let icon_red = load_icon_from_resource(ICON_RED_ID).unwrap_or(fallback);
        let icon_gray = load_icon_from_resource(ICON_GRAY_ID).unwrap_or(fallback);

        let tray = Self {
            hwnd,
            current_color: TrayIconColor::Gray,
            icon_green,
            icon_yellow,
            icon_red,
            icon_gray,
            dynamic_icon: None,
        };

        tray.add_to_tray()?;
        Ok(tray)
    }

    fn add_to_tray(&self) -> Result<(), String> {
        let mut nid = self.make_nid();
        nid.uFlags = NIF_ICON | NIF_MESSAGE | NIF_TIP;
        nid.hIcon = self.icon_gray;
        nid.uCallbackMessage = WM_TRAY_ICON;
        let tip = "ZaiMeter";
        let tip_wide: Vec<u16> = tip.encode_utf16().chain(std::iter::once(0)).collect();
        let copy_len = tip_wide.len().min(127);
        nid.szTip[..copy_len].copy_from_slice(&tip_wide[..copy_len]);

        unsafe {
            Shell_NotifyIconW(NIM_ADD, &nid)
                .ok()
                .map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    pub fn update(&mut self, usage: &Option<UsageResponse>, tooltip: &str, icon_style: &str) {
        let max_util = usage.as_ref().and_then(|u| u.max_utilization());

        // If active session (five_hour with resets_at), show its %; otherwise show "..."
        let session_util = usage
            .as_ref()
            .and_then(|u| u.five_hour.as_ref())
            .filter(|m| m.resets_at.is_some())
            .map(|m| m.utilization);

        // Icon color matches the displayed value: session % when active, max otherwise
        let displayed_util = session_util.or(max_util);
        let color = TrayIconColor::from_utilization(displayed_util);

        let icon = if max_util.is_some() {
            let color_ref = color.to_colorref();
            let text_cr = color.text_colorref();
            let pct = session_util.unwrap_or(max_util.unwrap_or(0.0));
            let dyn_icon = match icon_style {
                "ring" => create_ring_icon(pct, color_ref),
                "bar" => create_bar_icon(pct, color_ref),
                "pie" => {
                    if let Some(u) = usage.as_ref() {
                        let pie_metrics: Vec<(f64, u32)> = u
                            .all_metrics()
                            .iter()
                            .enumerate()
                            .filter(|(_, (_, m))| m.utilization > 0.0)
                            .map(|(i, (_, m))| (m.utilization, PIE_PALETTE[i % PIE_PALETTE.len()]))
                            .collect();
                        if pie_metrics.is_empty() {
                            create_ring_icon(0.0, color_ref)
                        } else {
                            create_pie_icon(&pie_metrics)
                        }
                    } else {
                        None
                    }
                }
                _ => {
                    // "number" style (default)
                    if let Some(session_pct) = session_util {
                        let value = session_pct.round() as u32;
                        let text = if value >= 100 {
                            "!!".to_string()
                        } else {
                            format!("{}", value)
                        };
                        let font_size = if value >= 10 { 9 } else { 11 };
                        create_text_icon(&text, font_size, color_ref, text_cr)
                    } else {
                        create_text_icon("...", 8, color_ref, text_cr)
                    }
                }
            };
            if let Some(dyn_icon) = dyn_icon {
                // Destroy previous dynamic icon
                if let Some(old) = self.dynamic_icon.take() {
                    unsafe {
                        let _ = DestroyIcon(old);
                    }
                }
                self.dynamic_icon = Some(dyn_icon);
                dyn_icon
            } else {
                self.fallback_icon(color)
            }
        } else {
            self.fallback_icon(color)
        };

        let mut nid = self.make_nid();
        nid.uFlags = NIF_ICON | NIF_TIP;
        nid.hIcon = icon;

        // Truncate tooltip to 127 chars (Win32 limit is 128 with null)
        let truncated: String = tooltip.chars().take(127).collect();
        let tip_wide: Vec<u16> = truncated.encode_utf16().chain(std::iter::once(0)).collect();
        let copy_len = tip_wide.len().min(127);
        nid.szTip[..copy_len].copy_from_slice(&tip_wide[..copy_len]);

        unsafe {
            let _ = Shell_NotifyIconW(NIM_MODIFY, &nid);
        }
        self.current_color = color;
    }

    fn fallback_icon(&self, color: TrayIconColor) -> HICON {
        match color {
            TrayIconColor::Green => self.icon_green,
            TrayIconColor::Yellow => self.icon_yellow,
            TrayIconColor::Red => self.icon_red,
            TrayIconColor::Gray => self.icon_gray,
        }
    }

    /// Show a balloon notification from the tray icon.
    pub fn show_balloon(&self, title: &str, body: &str) {
        let mut nid = self.make_nid();
        nid.uFlags = NIF_INFO;

        let title_wide: Vec<u16> = title.encode_utf16().chain(std::iter::once(0)).collect();
        let copy_len = title_wide.len().min(63);
        nid.szInfoTitle[..copy_len].copy_from_slice(&title_wide[..copy_len]);

        let body_wide: Vec<u16> = body.encode_utf16().chain(std::iter::once(0)).collect();
        let copy_len = body_wide.len().min(255);
        nid.szInfo[..copy_len].copy_from_slice(&body_wide[..copy_len]);

        unsafe {
            let _ = Shell_NotifyIconW(NIM_MODIFY, &nid);
        }
    }

    fn make_nid(&self) -> NOTIFYICONDATAW {
        NOTIFYICONDATAW {
            cbSize: std::mem::size_of::<NOTIFYICONDATAW>() as u32,
            hWnd: self.hwnd,
            uID: TRAY_ID,
            ..Default::default()
        }
    }
}

impl Drop for TrayIcon {
    fn drop(&mut self) {
        let nid = self.make_nid();
        unsafe {
            let _ = Shell_NotifyIconW(NIM_DELETE, &nid);
            if let Some(icon) = self.dynamic_icon.take() {
                let _ = DestroyIcon(icon);
            }
        }
    }
}

/// Classify an error string into a short tooltip-friendly label.
fn error_tooltip_label(err: &str) -> &'static str {
    let tag = err
        .strip_prefix('[')
        .and_then(|s| s.split_once(']'))
        .map(|(t, _)| t);
    match tag {
        Some("token_expired") => "\u{26a0} Token expired",
        Some("network_error") => "\u{26a0} Connection error",
        Some("rate_limited") => "\u{26a0} Rate limited",
        Some("server_error") => "\u{26a0} Server error",
        Some("api_error") => "\u{26a0} API error",
        _ => {
            if err.contains("credentials not found") || err.contains("accessToken") {
                "\u{26a0} Not logged in"
            } else {
                "\u{26a0} Error"
            }
        }
    }
}

/// Build full tooltip text without truncation (used by widget tooltip).
pub fn build_tooltip_full(
    usage: &Option<UsageResponse>,
    show_chatgpt: bool,
    last_error: &Option<String>,
) -> String {
    use crate::i18n::format_duration;
    use crate::providers::claude::format_metric_name;

    let header = match (&usage, last_error) {
        (Some(_), Some(_)) => "ZaiMeter \u{26a0}".to_string(),
        _ => "ZaiMeter".to_string(),
    };
    let mut lines = vec![header];

    match usage {
        None => {
            if let Some(err) = last_error {
                lines.push(error_tooltip_label(err).to_string());
            } else {
                lines.push("No data".to_string());
            }
        }
        Some(u) => {
            lines.push(format!("Claude ({})", u.detected_plan()));
            for (key, metric) in u.all_metrics() {
                let name = format_metric_name(&key);
                let reset_str = metric
                    .resets_at
                    .as_deref()
                    .and_then(crate::i18n::seconds_until)
                    .map(|s| format!(" | {}", format_duration(s)))
                    .unwrap_or_default();
                lines.push(String::new()); // empty line between metrics
                lines.push(format!("{}: {:.0}%{}", name, metric.utilization, reset_str));
            }
        }
    }

    if show_chatgpt {
        lines.push("ChatGPT: click to open usage".to_string());
    }

    lines.join("\n")
}

/// Build the tray tooltip string (truncated to 127 chars for Win32 szTip limit).
pub fn build_tooltip(
    usage: &Option<UsageResponse>,
    _show_chatgpt: bool,
    last_error: &Option<String>,
) -> String {
    use crate::i18n::format_duration;
    use crate::providers::claude::format_metric_name;

    let mut lines: Vec<String> = Vec::new();

    match usage {
        None => {
            lines.push("ZaiMeter".to_string());
            if let Some(err) = last_error {
                lines.push(error_tooltip_label(err).to_string());
            } else {
                lines.push("No data".to_string());
            }
        }
        Some(u) => {
            // Compact plan name: "Max 5X" → "5x", "Max 20X" → "20x", "Max" → "Max", "Pro" → "Pro"
            let plan = u.detected_plan();
            let compact_plan = if let Some(rest) = plan.strip_prefix("Max ") {
                rest.to_lowercase()
            } else {
                plan
            };
            let header = if last_error.is_some() {
                format!("Claude ({}) \u{26a0}", compact_plan)
            } else {
                format!("Claude ({})", compact_plan)
            };
            lines.push(header);

            let all = u.all_metrics();
            let extra_metrics: Vec<_> = all
                .iter()
                .filter(|(k, _)| u.extra.contains_key(k))
                .collect();

            for (key, metric) in &all {
                if u.extra.contains_key(key) {
                    continue;
                }
                let name = format_metric_name(key);
                let reset_str = metric
                    .resets_at
                    .as_deref()
                    .and_then(crate::i18n::seconds_until)
                    .map(|s| format!(" | {}", format_duration(s)))
                    .unwrap_or_default();
                lines.push(String::new());
                lines.push(format!("{}: {:.0}%{}", name, metric.utilization, reset_str));
            }

            // Extra metrics: show inline if 1, summary if more
            if extra_metrics.len() == 1 {
                let (key, metric) = extra_metrics[0];
                let name = format_metric_name(key);
                let reset_str = metric
                    .resets_at
                    .as_deref()
                    .and_then(crate::i18n::seconds_until)
                    .map(|s| format!(" | {}", format_duration(s)))
                    .unwrap_or_default();
                lines.push(String::new());
                lines.push(format!("{}: {:.0}%{}", name, metric.utilization, reset_str));
            } else if extra_metrics.len() > 1 {
                lines.push(String::new());
                lines.push(format!("+{} extra", extra_metrics.len()));
            }
        }
    }

    let result = lines.join("\n");
    result.chars().take(127).collect()
}
