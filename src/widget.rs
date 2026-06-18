//! Mini floating widget — always-on-top small window showing current usage %.

use windows::core::PCWSTR;
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, RECT, WPARAM};
use windows::Win32::Graphics::Gdi::{
    BeginPaint, CreateFontW, CreateSolidBrush, DeleteObject, EndPaint, FillRect, SelectObject,
    SetBkMode, SetTextColor, TextOutW, HBRUSH, HGDIOBJ, PAINTSTRUCT, TRANSPARENT,
};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, GetClientRect, LoadCursorW, RegisterClassExW, SendMessageW,
    CS_HREDRAW, CS_VREDRAW, IDC_ARROW, WINDOW_STYLE, WM_LBUTTONUP, WM_NCHITTEST, WM_PAINT,
    WNDCLASSEXW, WS_EX_LAYERED, WS_EX_TOOLWINDOW, WS_EX_TOPMOST, WS_POPUP,
};

use crate::ui::colors::rgb;

pub const WIDGET_CLASS: &str = "ZaiMeterWidget";
const WIDGET_W: i32 = 52;
const WIDGET_H: i32 = 28;

// Win32 tooltip message constants (from commctrl.h)
const TTM_ADDTOOLW: u32 = 0x0432;
const TTM_TRACKACTIVATE: u32 = 0x0411;
const TTM_TRACKPOSITION: u32 = 0x0412;
const TTM_UPDATETIPTEXTW: u32 = 0x0439;
const TTM_SETMAXTIPWIDTH: u32 = 0x0418;
const TTF_TRACK: u32 = 0x0020;
const TTF_ABSOLUTE: u32 = 0x0080;
const TTF_IDISHWND: u32 = 0x0001;

// Non-client mouse messages
const WM_NCMOUSEMOVE_MSG: u32 = 0x00A0;
const WM_NCMOUSELEAVE_MSG: u32 = 0x02A2;

/// TTTOOLINFOW layout matching Win32 API (64-bit).
#[repr(C)]
struct ToolInfoW {
    cb_size: u32,
    u_flags: u32,
    hwnd: HWND,
    u_id: usize,
    rect: RECT,
    hinst: isize,
    lpsz_text: *mut u16,
    l_param: isize,
    lp_reserved: *mut core::ffi::c_void,
}

/// TRACKMOUSEEVENT for requesting WM_NCMOUSELEAVE.
#[repr(C)]
struct TrackMouseEventS {
    cb_size: u32,
    dw_flags: u32,
    hwnd_track: HWND,
    dw_hover_time: u32,
}

extern "system" {
    fn TrackMouseEvent(lp: *mut TrackMouseEventS) -> i32;
}

// Tooltip state
static mut WIDGET_TOOLTIP_HWND: Option<HWND> = None;
static mut TOOLTIP_ACTIVE: bool = false;

/// Create a ToolInfoW struct for the given widget HWND.
unsafe fn make_tool_info(widget_hwnd: HWND) -> ToolInfoW {
    ToolInfoW {
        cb_size: std::mem::size_of::<ToolInfoW>() as u32,
        u_flags: TTF_IDISHWND | TTF_TRACK | TTF_ABSOLUTE,
        hwnd: widget_hwnd,
        u_id: widget_hwnd.0 as usize,
        rect: RECT::default(),
        hinst: 0,
        lpsz_text: std::ptr::null_mut(),
        l_param: 0,
        lp_reserved: std::ptr::null_mut(),
    }
}

/// Register the widget window class.
pub unsafe fn register_widget_class() {
    let hinstance = GetModuleHandleW(None).unwrap_or_default();
    let class_name: Vec<u16> = WIDGET_CLASS
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();

    let wc = WNDCLASSEXW {
        cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
        style: CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(widget_wnd_proc),
        hInstance: hinstance.into(),
        hCursor: LoadCursorW(None, IDC_ARROW).unwrap_or_default(),
        hbrBackground: HBRUSH(std::ptr::null_mut()),
        lpszClassName: PCWSTR(class_name.as_ptr()),
        ..Default::default()
    };
    RegisterClassExW(&wc);
}

/// Create the widget window (hidden by default).
pub unsafe fn create_widget_window() -> Option<HWND> {
    let hinstance = GetModuleHandleW(None).unwrap_or_default();
    let class_name: Vec<u16> = WIDGET_CLASS
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();
    let title: Vec<u16> = "ZaiMeter Widget"
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();

    let hwnd = CreateWindowExW(
        WS_EX_TOOLWINDOW | WS_EX_TOPMOST | WS_EX_LAYERED,
        PCWSTR(class_name.as_ptr()),
        PCWSTR(title.as_ptr()),
        WS_POPUP,
        100,
        100,
        WIDGET_W,
        WIDGET_H,
        None,
        None,
        hinstance,
        None,
    )
    .ok()?;

    // Set 230/255 opacity
    let _ = windows::Win32::UI::WindowsAndMessaging::SetLayeredWindowAttributes(
        hwnd,
        windows::Win32::Foundation::COLORREF(0),
        230,
        windows::Win32::UI::WindowsAndMessaging::LWA_ALPHA,
    );

    // Round corners on Win11
    let corner_pref: u32 = 2;
    let _ = windows::Win32::Graphics::Dwm::DwmSetWindowAttribute(
        hwnd,
        windows::Win32::Graphics::Dwm::DWMWINDOWATTRIBUTE(33),
        &corner_pref as *const u32 as *const _,
        std::mem::size_of::<u32>() as u32,
    );

    // Create tooltip control
    let tooltip_class: Vec<u16> = "tooltips_class32"
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();

    if let Ok(tooltip) = CreateWindowExW(
        WS_EX_TOPMOST,
        PCWSTR(tooltip_class.as_ptr()),
        PCWSTR::null(),
        WINDOW_STYLE(WS_POPUP.0 | 0x01 | 0x02), // TTS_ALWAYSTIP | TTS_NOPREFIX
        0,
        0,
        0,
        0,
        hwnd,
        None,
        hinstance,
        None,
    ) {
        // Enable multiline tooltips
        SendMessageW(tooltip, TTM_SETMAXTIPWIDTH, WPARAM(0), LPARAM(300));

        // Register widget as a tracked tool
        let mut empty_text: Vec<u16> = vec![0];
        let mut tool = make_tool_info(hwnd);
        tool.lpsz_text = empty_text.as_mut_ptr();
        SendMessageW(
            tooltip,
            TTM_ADDTOOLW,
            WPARAM(0),
            LPARAM(&tool as *const _ as isize),
        );

        WIDGET_TOOLTIP_HWND = Some(tooltip);
    }

    Some(hwnd)
}

unsafe extern "system" fn widget_wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_NCHITTEST => {
            // Make entire widget draggable
            LRESULT(2) // HTCAPTION
        }
        WM_LBUTTONUP => {
            // Click opens main popup
            if let Some(state) = crate::APP_STATE.as_ref() {
                let _ = windows::Win32::UI::WindowsAndMessaging::PostMessageW(
                    state.main_hwnd,
                    crate::tray::WM_TRAY_ICON,
                    WPARAM(0),
                    LPARAM(WM_LBUTTONUP as isize),
                );
            }
            LRESULT(0)
        }
        WM_PAINT => {
            let mut ps = PAINTSTRUCT::default();
            let _hdc = BeginPaint(hwnd, &mut ps);
            let hdc = ps.hdc;

            let mut rect = RECT::default();
            let _ = GetClientRect(hwnd, &mut rect);

            // Show 5-hour session utilization (not max across all metrics)
            let (text, bg_color) = if let Some(state) = crate::APP_STATE.as_ref() {
                let five_hour_util = state
                    .usage
                    .as_ref()
                    .and_then(|u| u.five_hour.as_ref())
                    .map(|m| m.utilization);
                match five_hour_util {
                    Some(u) if u >= 80.0 => (format!("{}%", u.round() as u32), rgb(210, 15, 57)),
                    Some(u) if u >= 50.0 => (format!("{}%", u.round() as u32), rgb(223, 142, 29)),
                    Some(u) => (format!("{}%", u.round() as u32), rgb(64, 160, 43)),
                    None => ("\u{2014}".to_string(), rgb(128, 128, 128)),
                }
            } else {
                ("\u{2014}".to_string(), rgb(128, 128, 128))
            };

            // Fill background
            let bg_brush = CreateSolidBrush(bg_color);
            FillRect(hdc, &rect, bg_brush);
            let _ = DeleteObject(HGDIOBJ(bg_brush.0));

            // Draw text
            let font = CreateFontW(
                16,
                0,
                0,
                0,
                700,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                PCWSTR(
                    "Segoe UI"
                        .encode_utf16()
                        .chain(std::iter::once(0))
                        .collect::<Vec<u16>>()
                        .as_ptr(),
                ),
            );
            let old_font = SelectObject(hdc, HGDIOBJ(font.0 as *mut _));
            SetBkMode(hdc, TRANSPARENT);
            SetTextColor(hdc, windows::Win32::Foundation::COLORREF(0x00FFFFFF)); // white

            let text_wide: Vec<u16> = text.encode_utf16().collect();
            // Center text
            let cx = (rect.right - rect.left) / 2;
            let cy = (rect.bottom - rect.top) / 2;

            // Simple centering: approximate char width
            let text_w = text_wide.len() as i32 * 7;
            let x = cx - text_w / 2;
            let y = cy - 8;
            let _ = TextOutW(hdc, x, y, &text_wide);

            let _ = SelectObject(hdc, old_font);
            let _ = DeleteObject(HGDIOBJ(font.0 as *mut _));

            let _ = EndPaint(hwnd, &ps);
            LRESULT(0)
        }
        WM_NCMOUSEMOVE_MSG => {
            // Show tooltip with full usage info on hover
            if let Some(tooltip) = WIDGET_TOOLTIP_HWND {
                // Build tooltip text from current usage state
                let text = if let Some(state) = crate::APP_STATE.as_ref() {
                    crate::tray::build_tooltip_full(
                        &state.usage,
                        state.config_mgr.config.show_chatgpt_section,
                        &state.last_error,
                    )
                } else {
                    "ZaiMeter".to_string()
                };

                let mut text_wide: Vec<u16> =
                    text.encode_utf16().chain(std::iter::once(0)).collect();
                let mut tool = make_tool_info(hwnd);
                tool.lpsz_text = text_wide.as_mut_ptr();

                // Update tooltip text
                SendMessageW(
                    tooltip,
                    TTM_UPDATETIPTEXTW,
                    WPARAM(0),
                    LPARAM(&tool as *const _ as isize),
                );

                // Position tooltip below cursor
                let x = (lparam.0 & 0xFFFF) as i16 as i32;
                let y = ((lparam.0 >> 16) & 0xFFFF) as i16 as i32;
                let pos = (((y + 20) as u16 as u32) << 16) | (x as u16 as u32);
                SendMessageW(tooltip, TTM_TRACKPOSITION, WPARAM(0), LPARAM(pos as isize));

                // Activate tooltip if not already active
                if !TOOLTIP_ACTIVE {
                    SendMessageW(
                        tooltip,
                        TTM_TRACKACTIVATE,
                        WPARAM(1),
                        LPARAM(&tool as *const _ as isize),
                    );
                    TOOLTIP_ACTIVE = true;

                    // Request WM_NCMOUSELEAVE notification
                    let mut tme = TrackMouseEventS {
                        cb_size: std::mem::size_of::<TrackMouseEventS>() as u32,
                        dw_flags: 0x02 | 0x10, // TME_LEAVE | TME_NONCLIENT
                        hwnd_track: hwnd,
                        dw_hover_time: 0,
                    };
                    TrackMouseEvent(&mut tme);
                }
            }
            LRESULT(0)
        }
        WM_NCMOUSELEAVE_MSG => {
            // Hide tooltip when mouse leaves widget
            if let Some(tooltip) = WIDGET_TOOLTIP_HWND {
                if TOOLTIP_ACTIVE {
                    let tool = make_tool_info(hwnd);
                    SendMessageW(
                        tooltip,
                        TTM_TRACKACTIVATE,
                        WPARAM(0),
                        LPARAM(&tool as *const _ as isize),
                    );
                    TOOLTIP_ACTIVE = false;
                }
            }
            LRESULT(0)
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

/// Update the widget display (trigger repaint).
pub unsafe fn invalidate_widget(hwnd: HWND) {
    let _ = windows::Win32::Graphics::Gdi::InvalidateRect(hwnd, None, true);
}
