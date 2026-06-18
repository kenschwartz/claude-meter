#![allow(static_mut_refs)]
#![allow(clippy::too_many_arguments)]

use crate::config::ConfigManager;
use crate::db::Database;
use crate::i18n::{format_duration, I18n};
use crate::notifications::NotificationTracker;
use crate::providers::claude::{ClaudeClient, UsageResponse};
use crate::theme::{resolve_theme, ThemeMode};
use crate::tray::{
    build_tooltip, TrayIcon, IDM_ABOUT, IDM_AUTOSTART, IDM_EXIT, IDM_EXPORT_CSV, IDM_OPEN_CHATGPT,
    IDM_OPEN_CLAUDE, IDM_OPEN_DASHBOARD, IDM_REFRESH, IDM_SETTINGS, WM_TRAY_ICON,
};
use crate::ui::colors::colorref_to_d2d;
use crate::ui::render::{draw_settings_panel, D2DResources, HoveredElement, PopupRenderer};
use crate::{autostart, credentials, i18n, providers, updater, widget};
use chrono::{Local, Timelike};
use windows::core::PCWSTR;
use windows::Win32::Foundation::{GetLastError, HWND, LPARAM, LRESULT, POINT, RECT, WPARAM};
use windows::Win32::Graphics::Dwm::DwmSetWindowAttribute;
use windows::Win32::Graphics::Gdi::ClientToScreen;
use windows::Win32::Graphics::Gdi::{BeginPaint, EndPaint, PAINTSTRUCT};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::System::Threading::CreateMutexW;
use windows::Win32::UI::Controls::WM_MOUSELEAVE;
use windows::Win32::UI::Input::KeyboardAndMouse::{TrackMouseEvent, TME_LEAVE, TRACKMOUSEEVENT};
use windows::Win32::UI::WindowsAndMessaging::LWA_ALPHA;
use windows::Win32::UI::WindowsAndMessaging::{
    AppendMenuW, CreatePopupMenu, CreateWindowExW, DefWindowProcW, DestroyMenu, DispatchMessageW,
    GetCursorPos, GetWindowLongW, LoadCursorW, LoadIconW, PeekMessageW, PostMessageW,
    PostQuitMessage, RegisterClassExW, SetCursor, SetForegroundWindow, SetLayeredWindowAttributes,
    SetWindowLongW, ShowWindow, TrackPopupMenu, TranslateMessage, CS_DROPSHADOW, CS_HREDRAW,
    CS_VREDRAW, GWL_EXSTYLE, HMENU, IDC_ARROW, IDC_HAND, IDI_APPLICATION, MF_CHECKED, MF_SEPARATOR,
    MF_STRING, MF_UNCHECKED, MSG, PM_REMOVE, SW_HIDE, SW_SHOWNOACTIVATE, TPM_BOTTOMALIGN,
    TPM_LEFTALIGN, TPM_RETURNCMD, WM_COMMAND, WM_DESTROY, WM_KEYDOWN, WM_KILLFOCUS, WM_LBUTTONUP,
    WM_MOUSEMOVE, WM_PAINT, WM_RBUTTONUP, WM_SETCURSOR, WM_TIMER, WNDCLASSEXW, WS_EX_LAYERED,
    WS_EX_TOOLWINDOW, WS_EX_TOPMOST, WS_POPUP,
};

// Language popup menu IDs
const IDM_LANG_AUTO: u32 = 5000;
const IDM_LANG_BASE: u32 = 5001;

// Theme popup menu IDs
const IDM_THEME_AUTO: u32 = 5100;
const IDM_THEME_DARK: u32 = 5101;
const IDM_THEME_LIGHT: u32 = 5102;

const WINDOW_CLASS: &str = "ClaudeMeterMain";
const POPUP_CLASS: &str = "ClaudeMeterPopup";
const TIMER_POLL: usize = 1;
const TIMER_ANIM: usize = 2;
const TIMER_BLINK: usize = 3;
const TIMER_FADE: usize = 4;
const TIMER_SLIDE: usize = 5;
const TIMER_CONFIG: usize = 6;
const TIMER_WAKE_RETRY: usize = 7;
const ANIM_INTERVAL_MS: u32 = 16; // ~60fps
const CONFIG_CHECK_INTERVAL_MS: u32 = 5_000; // 5 seconds
const BLINK_INTERVAL_MS: u32 = 500;
const FADE_INTERVAL_MS: u32 = 16;
const IDLE_TIMEOUT_MS: u32 = 5 * 60 * 1000; // 5 minutes
const WM_POLL_RESULT: u32 = 0x0400 + 20; // WM_USER + 20
const WM_UPDATE_AVAILABLE: u32 = 0x0400 + 21; // WM_USER + 21
const WM_DB_RESULT: u32 = 0x0400 + 22; // WM_USER + 22
const WM_CREDENTIAL_CHANGED: u32 = 0x0400 + 23; // WM_USER + 23
const WM_NETWORK_CHANGED: u32 = 0x0400 + 24; // WM_USER + 24

/// Shared application state accessible from the window proc.
pub(crate) struct AppState {
    pub(crate) config_mgr: ConfigManager,
    i18n: I18n,
    tray: Option<TrayIcon>,
    pub(crate) usage: Option<UsageResponse>,
    last_updated: String,
    pub(crate) last_error: Option<String>,
    pub(crate) main_hwnd: HWND,
    popup_hwnd: HWND,
    popup_visible: bool,
    popup_in_settings: bool,
    // Hit-test rectangles (in popup client coordinates)
    settings_rect: RECT,
    close_rect: RECT,
    refresh_rect: RECT,
    copy_rect: RECT,
    install_rect: RECT,
    chatgpt_link_rect: RECT,
    status_link_rect: RECT,
    plan_link_rect: RECT,
    back_rect: RECT,
    setting_rects: [RECT; 13],
    notification_tracker: NotificationTracker,
    exe_dir: std::path::PathBuf,
    chart_data: Vec<f64>,
    chart_data_7d: Vec<f64>,
    chart_data_30d: Vec<f64>,
    chart_mode: u8, // 0=24h, 1=7d, 2=30d
    chart_reset_lines: Vec<f64>,
    // Chart hit-testing
    chart_rect: RECT,
    chart_bar_count: usize,
    chart_toggle_rect: RECT,
    // Animation state for progress bars
    anim_targets: Vec<f64>,
    anim_current: Vec<f64>,
    anim_active: bool,
    // Fade-in animation
    fade_alpha: u8,
    // Tray icon blink on critical usage
    blink_active: bool,
    blink_visible: bool,
    // Retry backoff
    consecutive_failures: u32,
    // Guard against concurrent polls
    poll_in_progress: bool,
    // Queue one user-requested refresh if a poll is already running.
    pending_force_poll: bool,
    // Last poll timestamp for auto-refresh
    last_poll_time: Option<std::time::Instant>,
    // Direct2D resources
    d2d: Option<D2DResources>,
    hovered_element: HoveredElement,
    mouse_tracking: bool,
    // Token expiry warning (avoid repeated notifications)
    token_expiry_warned: bool,
    // Mini-widget window
    widget_hwnd: Option<HWND>,
    // Rate of change per metric (%/hour)
    rate_of_change: std::collections::HashMap<String, f64>,
    // Slide animation state for settings ↔ dashboard transition
    slide_anim_offset: f32,
    slide_anim_target: f32,
    slide_anim_active: bool,
    // Cached resolved theme (updated by TIMER_CONFIG, not every paint)
    cached_theme: crate::theme::ResolvedTheme,
    // Wake retry progressive schedule index
    wake_retry_index: usize,
    // URL of latest pending update — opened when user clicks the update balloon
    pending_update_url: Option<String>,
}

// Safety: AppState is accessed only from the main thread via raw pointer.
unsafe impl Send for AppState {}
unsafe impl Sync for AppState {}

pub(crate) static mut APP_STATE: Option<AppState> = None;

pub fn run() {
    env_logger::init();

    // Single-instance check
    if !ensure_single_instance() {
        log::info!("Another instance is already running.");
        return;
    }

    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.to_path_buf()))
        .unwrap_or_default();

    let config_mgr = ConfigManager::new(&exe_dir);
    let i18n = I18n::from_config(&config_mgr.config.language);

    // Try to open DB early to verify it's accessible, log warning if not.
    if let Err(e) = Database::open(&exe_dir) {
        log::warn!("Database open check failed: {e}. History will not be saved.");
    }

    // Sync autostart registry entry with current exe path on every launch.
    // This handles the case where the user moved the exe to a different folder
    // (e.g., after downloading a new release from GitHub).
    if config_mgr.config.autostart {
        let exe_path = exe_dir
            .join("claudemeter.exe")
            .to_string_lossy()
            .to_string();
        if let Err(e) = autostart::set_autostart(true, &exe_path) {
            log::warn!("Failed to sync autostart registry entry: {e}");
        }
    }

    unsafe { run_message_loop(exe_dir, config_mgr, i18n) };
}

unsafe fn run_message_loop(exe_dir: std::path::PathBuf, config_mgr: ConfigManager, i18n: I18n) {
    let hinstance = GetModuleHandleW(None).unwrap();

    // Register window classes
    register_main_class(hinstance);
    register_popup_class(hinstance);

    // Create hidden message window
    let main_class_w = wide(WINDOW_CLASS);
    let main_title_w = wide("ClaudeMeter");
    let popup_class_w = wide(POPUP_CLASS);
    let popup_title_w = wide("ClaudeMeter Dashboard");

    let main_hwnd = CreateWindowExW(
        windows::Win32::UI::WindowsAndMessaging::WINDOW_EX_STYLE(0),
        PCWSTR(main_class_w.as_ptr()),
        PCWSTR(main_title_w.as_ptr()),
        windows::Win32::UI::WindowsAndMessaging::WINDOW_STYLE(0),
        0,
        0,
        0,
        0,
        None,
        None,
        hinstance,
        None,
    )
    .unwrap();

    // Create popup window (hidden initially)
    let popup_hwnd = CreateWindowExW(
        WS_EX_TOOLWINDOW | WS_EX_TOPMOST,
        PCWSTR(popup_class_w.as_ptr()),
        PCWSTR(popup_title_w.as_ptr()),
        WS_POPUP,
        0,
        0,
        crate::ui::render::POPUP_WIDTH,
        400,
        None,
        None,
        hinstance,
        None,
    )
    .unwrap();

    // Apply DWM attributes (rounded corners + dark mode)
    apply_dwm_rounded_corners(popup_hwnd);
    apply_acrylic_backdrop(popup_hwnd);

    // Initialize D2D resources
    let d2d = match D2DResources::new() {
        Ok(d) => Some(d),
        Err(e) => {
            log::error!("Failed to init Direct2D: {e}");
            None
        }
    };

    // Resolve initial theme
    let initial_theme = resolve_theme(ThemeMode::from_str(&config_mgr.config.theme));

    // Initialize app state
    APP_STATE = Some(AppState {
        config_mgr,
        i18n,
        tray: None,
        usage: None,
        last_updated: String::new(),
        last_error: None,
        main_hwnd,
        popup_hwnd,
        popup_visible: false,
        popup_in_settings: false,
        settings_rect: RECT::default(),
        close_rect: RECT::default(),
        refresh_rect: RECT::default(),
        copy_rect: RECT::default(),
        install_rect: RECT::default(),
        chatgpt_link_rect: RECT::default(),
        status_link_rect: RECT::default(),
        plan_link_rect: RECT::default(),
        back_rect: RECT::default(),
        setting_rects: [RECT::default(); 13],
        notification_tracker: NotificationTracker::new(),
        exe_dir,
        chart_data: Vec::new(),
        chart_data_7d: Vec::new(),
        chart_data_30d: Vec::new(),
        chart_mode: 0,
        chart_reset_lines: Vec::new(),
        chart_rect: RECT::default(),
        chart_bar_count: 0,
        chart_toggle_rect: RECT::default(),
        anim_targets: Vec::new(),
        anim_current: Vec::new(),
        anim_active: false,
        fade_alpha: 255,
        blink_active: false,
        blink_visible: true,
        consecutive_failures: 0,
        poll_in_progress: false,
        pending_force_poll: false,
        last_poll_time: None,
        d2d,
        hovered_element: HoveredElement::None,
        mouse_tracking: false,
        token_expiry_warned: false,
        widget_hwnd: None,
        rate_of_change: std::collections::HashMap::new(),
        slide_anim_offset: 0.0,
        slide_anim_target: 0.0,
        slide_anim_active: false,
        cached_theme: initial_theme,
        wake_retry_index: 0,
        pending_update_url: None,
    });

    // Create tray icon
    if let Some(state) = APP_STATE.as_mut() {
        match TrayIcon::new(main_hwnd) {
            Ok(tray) => state.tray = Some(tray),
            Err(e) => log::error!("Failed to create tray icon: {e}"),
        }

        // Startup notification (balloon tip from tray icon)
        if state.config_mgr.config.notifications.enabled
            && state.config_mgr.config.show_startup_notification
        {
            if let Some(tray) = &state.tray {
                tray.show_balloon(
                    "ClaudeMeter",
                    state
                        .i18n
                        .t("Running in system tray. Click the icon for details."),
                );
            }
        }
    }

    // Register and create mini-widget window
    widget::register_widget_class();
    if let Some(state) = APP_STATE.as_mut() {
        if let Some(w) = widget::create_widget_window() {
            state.widget_hwnd = Some(w);
            if state.config_mgr.config.show_widget {
                let _ = ShowWindow(w, SW_SHOWNOACTIVATE);
            }
        }
    }

    // Auto-update check (background thread)
    if APP_STATE
        .as_ref()
        .is_some_and(|s| s.config_mgr.config.check_updates)
    {
        let hwnd_raw = main_hwnd.0 as usize;
        std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            if let Some((tag, url)) = rt.block_on(updater::check_for_update()) {
                // Post update notification back to main thread
                let data = Box::new((tag, url));
                let hwnd = HWND(hwnd_raw as *mut _);
                let _ = PostMessageW(
                    hwnd,
                    WM_UPDATE_AVAILABLE,
                    WPARAM(Box::into_raw(data) as usize),
                    LPARAM(0),
                );
            }
        });
    }

    // Credential file watcher: monitors ~/.claude/ for changes
    {
        let hwnd_raw = main_hwnd.0 as usize;
        std::thread::spawn(move || {
            credential_file_watcher(hwnd_raw);
        });
    }

    // Network connectivity change monitor
    {
        let hwnd_raw = main_hwnd.0 as usize;
        std::thread::spawn(move || {
            network_change_monitor(hwnd_raw);
        });
    }

    // Load cached data from DB so we have something to show before first poll
    if let Some(state) = APP_STATE.as_mut() {
        if let Ok(db) = Database::open(&state.exe_dir) {
            if let Ok(latest) = db.query_latest() {
                if !latest.is_empty() {
                    use providers::claude::UsageMetric;
                    let mut resp = providers::claude::UsageResponse {
                        five_hour: None,
                        seven_day: None,
                        seven_day_sonnet: None,
                        seven_day_opus: None,
                        seven_day_oauth_apps: None,
                        extra: std::collections::HashMap::new(),
                        subscription_type: None,
                        rate_limit_tier: None,
                    };
                    for (key, util, resets) in &latest {
                        let metric = UsageMetric {
                            utilization: *util,
                            resets_at: resets.clone(),
                        };
                        match key.as_str() {
                            "five_hour" => resp.five_hour = Some(metric),
                            "seven_day" => resp.seven_day = Some(metric),
                            "seven_day_sonnet" => resp.seven_day_sonnet = Some(metric),
                            "seven_day_opus" => resp.seven_day_opus = Some(metric),
                            "seven_day_oauth_apps" => resp.seven_day_oauth_apps = Some(metric),
                            other => {
                                resp.extra.insert(other.to_string(), metric);
                            }
                        }
                    }
                    // Load credentials info for plan detection
                    if let Ok(cred) = credentials::read_claude_token() {
                        resp.subscription_type = cred.subscription_type;
                        resp.rate_limit_tier = cred.rate_limit_tier;
                    }
                    state.usage = Some(resp);
                    state.last_updated = "(cached)".to_string();
                }
            }
            if let Ok(slots) = db.query_24h_chart("claude") {
                state.chart_data = slots;
            }
            if let Ok(slots) = db.query_7d_chart() {
                state.chart_data_7d = slots;
            }
            if let Ok(slots) = db.query_30d_chart() {
                state.chart_data_30d = slots;
            }
        }
    }

    // Initial poll (async via tokio)
    trigger_poll(main_hwnd);

    // Set up polling timer with random interval (90-180 seconds)
    let interval = crate::config::Config::random_polling_interval() as u32 * 1000;
    windows::Win32::UI::WindowsAndMessaging::SetTimer(main_hwnd, TIMER_POLL, interval, None);

    // Config/theme refresh timer (every 5 seconds)
    windows::Win32::UI::WindowsAndMessaging::SetTimer(
        main_hwnd,
        TIMER_CONFIG,
        CONFIG_CHECK_INTERVAL_MS,
        None,
    );

    // Message loop
    let mut msg = MSG::default();
    loop {
        let has_msg = PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE).as_bool();
        if has_msg {
            if msg.message == windows::Win32::UI::WindowsAndMessaging::WM_QUIT {
                break;
            }
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        } else {
            // Yield CPU when idle
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }
}

unsafe fn register_main_class(hinstance: windows::Win32::Foundation::HMODULE) {
    let class_name = wide(WINDOW_CLASS);
    let wc = WNDCLASSEXW {
        cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
        style: CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(main_wnd_proc),
        hInstance: hinstance.into(),
        hIcon: LoadIconW(None, IDI_APPLICATION).unwrap_or_default(),
        lpszClassName: PCWSTR(class_name.as_ptr()),
        ..Default::default()
    };
    RegisterClassExW(&wc);
}

unsafe fn register_popup_class(hinstance: windows::Win32::Foundation::HMODULE) {
    let class_name = wide(POPUP_CLASS);
    let wc = WNDCLASSEXW {
        cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
        style: CS_HREDRAW | CS_VREDRAW | CS_DROPSHADOW,
        lpfnWndProc: Some(popup_wnd_proc),
        hInstance: hinstance.into(),
        hIcon: LoadIconW(None, IDI_APPLICATION).unwrap_or_default(),
        hCursor: LoadCursorW(None, IDC_ARROW).unwrap_or_default(),
        lpszClassName: PCWSTR(class_name.as_ptr()),
        hbrBackground: windows::Win32::Graphics::Gdi::HBRUSH(6usize as *mut _),
        ..Default::default()
    };
    RegisterClassExW(&wc);
}

unsafe extern "system" fn main_wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_TRAY_ICON => {
            // NIN_BALLOONUSERCLICK = WM_USER + 5
            const NIN_BALLOONUSERCLICK: u32 = 0x0400 + 5;
            let event = (lparam.0 & 0xFFFF) as u32;
            match event {
                WM_LBUTTONUP => {
                    toggle_popup(hwnd);
                }
                WM_RBUTTONUP => {
                    show_context_menu(hwnd);
                }
                NIN_BALLOONUSERCLICK => {
                    // User clicked the balloon body — open update URL if one is pending.
                    if let Some(state) = APP_STATE.as_mut() {
                        if let Some(url) = state.pending_update_url.take() {
                            let _ = open::that(&url);
                        }
                    }
                }
                _ => {}
            }
            LRESULT(0)
        }
        WM_COMMAND => {
            let cmd = (wparam.0 & 0xFFFF) as u32;
            handle_menu_command(hwnd, cmd);
            LRESULT(0)
        }
        WM_TIMER => {
            if wparam.0 == TIMER_POLL {
                // Skip polling when user is idle (screen locked, AFK)
                if !is_user_idle(IDLE_TIMEOUT_MS) {
                    trigger_poll(hwnd);
                }
            } else if wparam.0 == TIMER_BLINK {
                // Blink tray icon when critical usage
                if let Some(state) = APP_STATE.as_mut() {
                    state.blink_visible = !state.blink_visible;
                    if let Some(tray) = &mut state.tray {
                        let style = &state.config_mgr.config.tray_icon_style.clone();
                        if state.blink_visible {
                            let tooltip = build_tooltip(
                                &state.usage,
                                state.config_mgr.config.show_chatgpt_section,
                                &state.last_error,
                            );
                            tray.update(&state.usage, &tooltip, style);
                        } else {
                            tray.update(&None, "ClaudeMeter", style);
                        }
                    }
                }
            } else if wparam.0 == TIMER_WAKE_RETRY {
                // Progressive retry after sleep/wake: 2s, 5s, 15s, 30s
                const WAKE_RETRY_DELAYS: [u32; 4] = [2000, 5000, 15000, 30000];
                trigger_poll(hwnd);
                if let Some(state) = APP_STATE.as_mut() {
                    state.wake_retry_index += 1;
                    if state.wake_retry_index < WAKE_RETRY_DELAYS.len() {
                        let delay = WAKE_RETRY_DELAYS[state.wake_retry_index];
                        let _ = windows::Win32::UI::WindowsAndMessaging::KillTimer(
                            hwnd,
                            TIMER_WAKE_RETRY,
                        );
                        windows::Win32::UI::WindowsAndMessaging::SetTimer(
                            hwnd,
                            TIMER_WAKE_RETRY,
                            delay,
                            None,
                        );
                    } else {
                        let _ = windows::Win32::UI::WindowsAndMessaging::KillTimer(
                            hwnd,
                            TIMER_WAKE_RETRY,
                        );
                    }
                }
            } else if wparam.0 == TIMER_CONFIG {
                // Periodically check config file changes and refresh theme
                if let Some(state) = APP_STATE.as_mut() {
                    state.config_mgr.reload_if_changed();
                    let theme_mode = ThemeMode::from_str(&state.config_mgr.config.theme);
                    let resolved = resolve_theme(theme_mode);
                    if resolved != state.cached_theme {
                        state.cached_theme = resolved;
                        // Repaint popup if visible
                        if state.popup_visible {
                            let _ = windows::Win32::Graphics::Gdi::InvalidateRect(
                                state.popup_hwnd,
                                None,
                                true,
                            );
                        }
                        // Refresh widget
                        if let Some(w) = state.widget_hwnd {
                            widget::invalidate_widget(w);
                        }
                    }
                    // Update i18n if language changed
                    let new_i18n = I18n::from_config(&state.config_mgr.config.language);
                    state.i18n = new_i18n;
                }
            }
            LRESULT(0)
        }
        WM_POLL_RESULT => {
            // Poll result received (usage data posted back to main thread)
            // wparam = pointer to Box<PollResult>
            let result_ptr = wparam.0 as *mut PollResult;
            if !result_ptr.is_null() {
                let result = *Box::from_raw(result_ptr);
                on_poll_result(hwnd, result);
            }
            LRESULT(0)
        }
        WM_DB_RESULT => {
            // DB query results received from background thread
            let result_ptr = wparam.0 as *mut DbResult;
            if !result_ptr.is_null() {
                let db_result = *Box::from_raw(result_ptr);
                if let Some(state) = APP_STATE.as_mut() {
                    state.chart_data = db_result.chart_data;
                    state.chart_data_7d = db_result.chart_data_7d;
                    state.chart_data_30d = db_result.chart_data_30d;
                    state.rate_of_change = db_result.rate_of_change;

                    // Repaint popup if visible to show updated charts
                    if state.popup_visible {
                        let _ = windows::Win32::Graphics::Gdi::InvalidateRect(
                            state.popup_hwnd,
                            None,
                            true,
                        );
                    }
                }
            }
            LRESULT(0)
        }
        WM_UPDATE_AVAILABLE => {
            // Auto-update notification from background thread
            let data_ptr = wparam.0 as *mut (String, String);
            if !data_ptr.is_null() {
                let (tag, url) = *Box::from_raw(data_ptr);
                if let Some(state) = APP_STATE.as_mut() {
                    state.pending_update_url = Some(url.clone());
                    if let Some(tray) = &state.tray {
                        let title = state.i18n.t("Update available");
                        let body = format!(
                            "{} {}",
                            tag,
                            state.i18n.t("is available. Click to download.")
                        );
                        tray.show_balloon(title, &body);
                    }
                }
                log::info!("Update available: {} — {}", tag, url);
            }
            LRESULT(0)
        }
        // WM_POWERBROADCAST: resume from sleep/hibernate
        0x0218 => {
            // PBT_APMRESUMEAUTOMATIC = 0x12
            if wparam.0 == 0x12 {
                log::info!("System resumed from sleep, starting progressive retry");
                if let Some(state) = APP_STATE.as_mut() {
                    state.consecutive_failures = 0;
                    state.wake_retry_index = 0;
                }
                // Progressive retry: 2s, 5s, 15s, 30s — gives network stack time to come up
                let _ = windows::Win32::UI::WindowsAndMessaging::KillTimer(hwnd, TIMER_WAKE_RETRY);
                windows::Win32::UI::WindowsAndMessaging::SetTimer(
                    hwnd,
                    TIMER_WAKE_RETRY,
                    2000,
                    None,
                );
            }
            LRESULT(1)
        }
        WM_CREDENTIAL_CHANGED => {
            // Credential file changed — trigger poll if not too recent
            log::info!("Credential file changed, triggering re-poll");
            if let Some(state) = APP_STATE.as_mut() {
                let should_poll = state
                    .last_poll_time
                    .map_or(true, |t| t.elapsed().as_secs() > 5);
                if should_poll {
                    state.consecutive_failures = 0;
                    state.token_expiry_warned = false;
                    trigger_poll(hwnd);
                }
            }
            LRESULT(0)
        }
        WM_NETWORK_CHANGED => {
            // Network interface changed — trigger poll if we had failures
            log::info!("Network change detected");
            if let Some(state) = APP_STATE.as_mut() {
                let should_poll = state.consecutive_failures > 0
                    || state
                        .last_poll_time
                        .map_or(true, |t| t.elapsed().as_secs() > 10);
                if should_poll {
                    state.consecutive_failures = 0;
                    trigger_poll(hwnd);
                }
            }
            LRESULT(0)
        }
        WM_DESTROY => {
            PostQuitMessage(0);
            LRESULT(0)
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

unsafe extern "system" fn popup_wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_PAINT => {
            let mut ps = PAINTSTRUCT::default();
            let _hdc = BeginPaint(hwnd, &mut ps);

            let mut rect = RECT::default();
            let _ = windows::Win32::UI::WindowsAndMessaging::GetClientRect(hwnd, &mut rect);

            if let Some(state) = APP_STATE.as_mut() {
                let resolved = state.cached_theme;
                let colors = crate::ui::colors::ThemeColors::for_theme(resolved)
                    .with_overrides(&state.config_mgr.config.custom_colors);

                // Apply DWM dark mode based on theme
                apply_dwm_dark_mode(hwnd, matches!(resolved, crate::theme::ResolvedTheme::Dark));

                if let Some(d2d) = state.d2d.as_mut() {
                    if d2d.ensure_render_target(hwnd).is_ok() {
                        // Clone the COM render target (cheap AddRef) to avoid
                        // borrow conflict with d2d being passed mutably to draw fns
                        if let Some(rt) = d2d.render_target.clone() {
                            rt.BeginDraw();
                            let mut bg = colorref_to_d2d(colors.background);
                            bg.a = 0.88; // Semi-transparent for Acrylic backdrop
                            rt.Clear(Some(&bg as *const _));

                            let renderer = PopupRenderer::new(hwnd);

                            if state.slide_anim_active {
                                // During slide animation, draw both panels with X offset
                                let offset = state.slide_anim_offset;
                                let popup_w = crate::ui::render::POPUP_WIDTH as f32;

                                // Draw outgoing panel (opposite of current state)
                                let outgoing_dx = if state.popup_in_settings {
                                    offset - popup_w
                                } else {
                                    offset + popup_w
                                };
                                let outgoing_transform = windows::Foundation::Numerics::Matrix3x2 {
                                    M11: 1.0,
                                    M12: 0.0,
                                    M21: 0.0,
                                    M22: 1.0,
                                    M31: outgoing_dx,
                                    M32: 0.0,
                                };
                                rt.SetTransform(&outgoing_transform);
                                if state.popup_in_settings {
                                    // Outgoing is dashboard
                                    renderer.draw(
                                        d2d,
                                        &rect,
                                        &state.usage,
                                        &state.last_updated,
                                        state.config_mgr.config.show_chatgpt_section,
                                        state.config_mgr.config.compact_mode,
                                        &colors,
                                        &state.i18n,
                                        match state.chart_mode {
                                            1 => &state.chart_data_7d,
                                            2 => &state.chart_data_30d,
                                            _ => &state.chart_data,
                                        },
                                        &state.chart_reset_lines,
                                        state.chart_mode,
                                        &state.last_error,
                                        &state.hovered_element,
                                        &state.anim_current,
                                        &state.config_mgr.config.dashboard_layout,
                                        &state.rate_of_change,
                                        state.config_mgr.config.hide_extra_usage,
                                        &mut state.settings_rect,
                                        &mut state.close_rect,
                                        &mut state.refresh_rect,
                                        &mut state.copy_rect,
                                        &mut state.install_rect,
                                        &mut state.chatgpt_link_rect,
                                        &mut state.status_link_rect,
                                        &mut state.plan_link_rect,
                                        &mut state.chart_rect,
                                        &mut state.chart_bar_count,
                                        &mut state.chart_toggle_rect,
                                    );
                                } else {
                                    // Outgoing is settings
                                    draw_settings_panel(
                                        d2d,
                                        &rect,
                                        &colors,
                                        &state.i18n,
                                        &state.config_mgr.config,
                                        &mut state.back_rect,
                                        &mut state.close_rect,
                                        &mut state.setting_rects,
                                        &state.hovered_element,
                                    );
                                }

                                // Draw incoming panel (current state)
                                let incoming_transform = windows::Foundation::Numerics::Matrix3x2 {
                                    M11: 1.0,
                                    M12: 0.0,
                                    M21: 0.0,
                                    M22: 1.0,
                                    M31: offset,
                                    M32: 0.0,
                                };
                                rt.SetTransform(&incoming_transform);
                                if state.popup_in_settings {
                                    draw_settings_panel(
                                        d2d,
                                        &rect,
                                        &colors,
                                        &state.i18n,
                                        &state.config_mgr.config,
                                        &mut state.back_rect,
                                        &mut state.close_rect,
                                        &mut state.setting_rects,
                                        &state.hovered_element,
                                    );
                                } else {
                                    renderer.draw(
                                        d2d,
                                        &rect,
                                        &state.usage,
                                        &state.last_updated,
                                        state.config_mgr.config.show_chatgpt_section,
                                        state.config_mgr.config.compact_mode,
                                        &colors,
                                        &state.i18n,
                                        match state.chart_mode {
                                            1 => &state.chart_data_7d,
                                            2 => &state.chart_data_30d,
                                            _ => &state.chart_data,
                                        },
                                        &state.chart_reset_lines,
                                        state.chart_mode,
                                        &state.last_error,
                                        &state.hovered_element,
                                        &state.anim_current,
                                        &state.config_mgr.config.dashboard_layout,
                                        &state.rate_of_change,
                                        state.config_mgr.config.hide_extra_usage,
                                        &mut state.settings_rect,
                                        &mut state.close_rect,
                                        &mut state.refresh_rect,
                                        &mut state.copy_rect,
                                        &mut state.install_rect,
                                        &mut state.chatgpt_link_rect,
                                        &mut state.status_link_rect,
                                        &mut state.plan_link_rect,
                                        &mut state.chart_rect,
                                        &mut state.chart_bar_count,
                                        &mut state.chart_toggle_rect,
                                    );
                                }

                                // Reset transform
                                let identity = windows::Foundation::Numerics::Matrix3x2 {
                                    M11: 1.0,
                                    M12: 0.0,
                                    M21: 0.0,
                                    M22: 1.0,
                                    M31: 0.0,
                                    M32: 0.0,
                                };
                                rt.SetTransform(&identity);
                            } else if state.popup_in_settings {
                                draw_settings_panel(
                                    d2d,
                                    &rect,
                                    &colors,
                                    &state.i18n,
                                    &state.config_mgr.config,
                                    &mut state.back_rect,
                                    &mut state.close_rect,
                                    &mut state.setting_rects,
                                    &state.hovered_element,
                                );
                            } else {
                                renderer.draw(
                                    d2d,
                                    &rect,
                                    &state.usage,
                                    &state.last_updated,
                                    state.config_mgr.config.show_chatgpt_section,
                                    state.config_mgr.config.compact_mode,
                                    &colors,
                                    &state.i18n,
                                    match state.chart_mode {
                                        1 => &state.chart_data_7d,
                                        2 => &state.chart_data_30d,
                                        _ => &state.chart_data,
                                    },
                                    &state.chart_reset_lines,
                                    state.chart_mode,
                                    &state.last_error,
                                    &state.hovered_element,
                                    &state.anim_current,
                                    &state.config_mgr.config.dashboard_layout,
                                    &state.rate_of_change,
                                    state.config_mgr.config.hide_extra_usage,
                                    &mut state.settings_rect,
                                    &mut state.close_rect,
                                    &mut state.refresh_rect,
                                    &mut state.copy_rect,
                                    &mut state.install_rect,
                                    &mut state.chatgpt_link_rect,
                                    &mut state.status_link_rect,
                                    &mut state.plan_link_rect,
                                    &mut state.chart_rect,
                                    &mut state.chart_bar_count,
                                    &mut state.chart_toggle_rect,
                                );
                            }

                            if rt.EndDraw(None, None).is_err() {
                                d2d.discard_render_target();
                            }
                        }
                    }
                }
            }

            let _ = EndPaint(hwnd, &ps);
            LRESULT(0)
        }
        WM_TIMER => {
            if wparam.0 == TIMER_ANIM {
                if let Some(state) = APP_STATE.as_mut() {
                    if state.anim_active {
                        let mut all_done = true;
                        for (cur, &tgt) in
                            state.anim_current.iter_mut().zip(state.anim_targets.iter())
                        {
                            let diff = tgt - *cur;
                            if diff.abs() < 0.3 && *cur >= 0.0 {
                                *cur = tgt;
                            } else {
                                // Ease-out: fast start, smooth deceleration
                                let ease = if *cur < 0.0 {
                                    0.35 // quick catch-up from negative (cascade delay)
                                } else {
                                    let t = (*cur / tgt.max(0.01)).clamp(0.0, 1.0);
                                    0.12 + 0.10 * (1.0 - t)
                                };
                                *cur += diff * ease;
                                all_done = false;
                            }
                        }
                        let _ = windows::Win32::Graphics::Gdi::InvalidateRect(hwnd, None, false);
                        if all_done {
                            state.anim_active = false;
                            let _ = windows::Win32::UI::WindowsAndMessaging::KillTimer(
                                hwnd, TIMER_ANIM,
                            );
                        }
                    }
                }
            } else if wparam.0 == TIMER_SLIDE {
                if let Some(state) = APP_STATE.as_mut() {
                    if state.slide_anim_active {
                        let diff = state.slide_anim_target - state.slide_anim_offset;
                        if diff.abs() < 0.5 {
                            state.slide_anim_offset = state.slide_anim_target;
                            state.slide_anim_active = false;
                            let _ = windows::Win32::UI::WindowsAndMessaging::KillTimer(
                                hwnd,
                                TIMER_SLIDE,
                            );
                        } else {
                            // Ease-out-cubic: snappy start, smooth finish
                            state.slide_anim_offset += diff * 0.22;
                        }
                        let _ = windows::Win32::Graphics::Gdi::InvalidateRect(hwnd, None, false);
                    }
                }
            } else if wparam.0 == TIMER_FADE {
                if let Some(state) = APP_STATE.as_mut() {
                    // Ease-in: accelerate fade (larger steps as alpha grows)
                    let step = 25 + (state.fade_alpha as u16 / 10);
                    let new_alpha = (state.fade_alpha as u16 + step).min(255) as u8;
                    state.fade_alpha = new_alpha;
                    let _ = SetLayeredWindowAttributes(
                        hwnd,
                        windows::Win32::Foundation::COLORREF(0),
                        new_alpha,
                        LWA_ALPHA,
                    );
                    if new_alpha == 255 {
                        // Fade complete — remove WS_EX_LAYERED for normal rendering
                        let ex = GetWindowLongW(hwnd, GWL_EXSTYLE);
                        SetWindowLongW(hwnd, GWL_EXSTYLE, ex & !(WS_EX_LAYERED.0 as i32));
                        let _ =
                            windows::Win32::UI::WindowsAndMessaging::KillTimer(hwnd, TIMER_FADE);
                    }
                }
            }
            LRESULT(0)
        }
        WM_MOUSEMOVE => {
            let x = (lparam.0 & 0xFFFF) as i16 as i32;
            let y = ((lparam.0 >> 16) & 0xFFFF) as i16 as i32;
            let pt = POINT { x, y };

            if let Some(state) = APP_STATE.as_mut() {
                if !state.mouse_tracking {
                    let mut tme = TRACKMOUSEEVENT {
                        cbSize: std::mem::size_of::<TRACKMOUSEEVENT>() as u32,
                        dwFlags: TME_LEAVE,
                        hwndTrack: hwnd,
                        dwHoverTime: 0,
                    };
                    let _ = TrackMouseEvent(&mut tme);
                    state.mouse_tracking = true;
                }

                let new_hover = if state.popup_in_settings {
                    if crate::popup::point_in_rect(pt, state.back_rect) {
                        HoveredElement::BackButton
                    } else if crate::popup::point_in_rect(pt, state.close_rect) {
                        HoveredElement::CloseButton
                    } else {
                        let mut found = HoveredElement::None;
                        for (i, rect) in state.setting_rects.iter().enumerate() {
                            if crate::popup::point_in_rect(pt, *rect) {
                                found = HoveredElement::SettingRow(i);
                                break;
                            }
                        }
                        found
                    }
                } else if crate::popup::point_in_rect(pt, state.settings_rect) {
                    HoveredElement::SettingsButton
                } else if crate::popup::point_in_rect(pt, state.close_rect) {
                    HoveredElement::CloseButton
                } else if crate::popup::point_in_rect(pt, state.refresh_rect) {
                    HoveredElement::RefreshButton
                } else if crate::popup::point_in_rect(pt, state.copy_rect) {
                    HoveredElement::CopyButton
                } else if crate::popup::point_in_rect(pt, state.install_rect) {
                    HoveredElement::InstallButton
                } else if crate::popup::point_in_rect(pt, state.chatgpt_link_rect) {
                    HoveredElement::ChatGptLink
                } else if crate::popup::point_in_rect(pt, state.status_link_rect) {
                    HoveredElement::StatusLink
                } else if crate::popup::point_in_rect(pt, state.plan_link_rect) {
                    HoveredElement::PlanLink
                } else if crate::popup::point_in_rect(pt, state.chart_toggle_rect) {
                    HoveredElement::ChartToggle
                } else if crate::popup::point_in_rect(pt, state.chart_rect)
                    && state.chart_bar_count > 0
                {
                    let chart_w = state.chart_rect.right - state.chart_rect.left;
                    let rel_x = pt.x - state.chart_rect.left;
                    let bar_idx = (rel_x as usize * state.chart_bar_count / chart_w as usize)
                        .min(state.chart_bar_count - 1);
                    HoveredElement::ChartBar(bar_idx)
                } else {
                    HoveredElement::None
                };

                if new_hover != state.hovered_element {
                    state.hovered_element = new_hover;
                    let _ = windows::Win32::Graphics::Gdi::InvalidateRect(hwnd, None, false);
                }
            }
            LRESULT(0)
        }
        WM_MOUSELEAVE => {
            if let Some(state) = APP_STATE.as_mut() {
                state.mouse_tracking = false;
                if state.hovered_element != HoveredElement::None {
                    state.hovered_element = HoveredElement::None;
                    let _ = windows::Win32::Graphics::Gdi::InvalidateRect(hwnd, None, false);
                }
            }
            LRESULT(0)
        }
        WM_SETCURSOR => {
            if let Some(state) = APP_STATE.as_ref() {
                if !matches!(
                    state.hovered_element,
                    HoveredElement::None | HoveredElement::ChartBar(_)
                ) {
                    let hand = LoadCursorW(None, IDC_HAND).unwrap_or_default();
                    SetCursor(hand);
                    return LRESULT(1);
                }
            }
            DefWindowProcW(hwnd, msg, wparam, lparam)
        }
        WM_LBUTTONUP => {
            let x = (lparam.0 & 0xFFFF) as i32;
            let y = ((lparam.0 >> 16) & 0xFFFF) as i32;
            let pt = POINT { x, y };

            if let Some(state) = APP_STATE.as_mut() {
                if crate::popup::point_in_rect(pt, state.close_rect) {
                    hide_popup(state);
                } else if state.popup_in_settings
                    && crate::popup::point_in_rect(pt, state.back_rect)
                {
                    // Slide back to dashboard
                    state.popup_in_settings = false;
                    let renderer = PopupRenderer::new(hwnd);
                    let h = renderer.calculate_height(
                        &state.usage,
                        state.config_mgr.config.show_chatgpt_section,
                        state.config_mgr.config.compact_mode,
                        &state.config_mgr.config.dashboard_layout,
                        state.config_mgr.config.hide_extra_usage,
                    );
                    resize_popup(hwnd, h);
                    state.slide_anim_offset = -(crate::ui::render::POPUP_WIDTH as f32);
                    state.slide_anim_target = 0.0;
                    state.slide_anim_active = true;
                    let _ = windows::Win32::UI::WindowsAndMessaging::SetTimer(
                        hwnd,
                        TIMER_SLIDE,
                        ANIM_INTERVAL_MS,
                        None,
                    );
                    let _ = windows::Win32::Graphics::Gdi::InvalidateRect(hwnd, None, true);
                } else if state.popup_in_settings
                    && crate::popup::point_in_rect(pt, state.setting_rects[0])
                {
                    // Theme: show popup menu
                    show_theme_popup(hwnd, pt, state);
                } else if state.popup_in_settings
                    && crate::popup::point_in_rect(pt, state.setting_rects[1])
                {
                    // Language: show popup menu with all languages
                    show_language_popup(hwnd, pt, state);
                } else if state.popup_in_settings
                    && crate::popup::point_in_rect(pt, state.setting_rects[2])
                {
                    // Compact mode: toggle
                    state.config_mgr.config.compact_mode = !state.config_mgr.config.compact_mode;
                    state.config_mgr.save();
                    let _ = windows::Win32::Graphics::Gdi::InvalidateRect(hwnd, None, true);
                } else if state.popup_in_settings
                    && crate::popup::point_in_rect(pt, state.setting_rects[3])
                {
                    // Show ChatGPT section: toggle
                    state.config_mgr.config.show_chatgpt_section =
                        !state.config_mgr.config.show_chatgpt_section;
                    state.config_mgr.save();
                    let _ = windows::Win32::Graphics::Gdi::InvalidateRect(hwnd, None, true);
                } else if state.popup_in_settings
                    && crate::popup::point_in_rect(pt, state.setting_rects[4])
                {
                    // Autostart: toggle + apply
                    state.config_mgr.config.autostart = !state.config_mgr.config.autostart;
                    let exe_path = state
                        .exe_dir
                        .join("claudemeter.exe")
                        .to_string_lossy()
                        .to_string();
                    if let Err(e) =
                        autostart::set_autostart(state.config_mgr.config.autostart, &exe_path)
                    {
                        log::warn!("Failed to set autostart: {e}");
                    }
                    state.config_mgr.save();
                    let _ = windows::Win32::Graphics::Gdi::InvalidateRect(hwnd, None, true);
                } else if state.popup_in_settings
                    && crate::popup::point_in_rect(pt, state.setting_rects[5])
                {
                    // Show widget: toggle
                    state.config_mgr.config.show_widget = !state.config_mgr.config.show_widget;
                    state.config_mgr.save();
                    // Toggle widget visibility
                    if state.config_mgr.config.show_widget {
                        if let Some(w) = state.widget_hwnd {
                            let _ = ShowWindow(w, SW_SHOWNOACTIVATE);
                        }
                    } else if let Some(w) = state.widget_hwnd {
                        let _ = ShowWindow(w, SW_HIDE);
                    }
                    let _ = windows::Win32::Graphics::Gdi::InvalidateRect(hwnd, None, true);
                } else if state.popup_in_settings
                    && crate::popup::point_in_rect(pt, state.setting_rects[6])
                {
                    // Check for updates: toggle
                    state.config_mgr.config.check_updates = !state.config_mgr.config.check_updates;
                    state.config_mgr.save();
                    let _ = windows::Win32::Graphics::Gdi::InvalidateRect(hwnd, None, true);
                } else if state.popup_in_settings
                    && crate::popup::point_in_rect(pt, state.setting_rects[7])
                {
                    // Accessibility patterns: toggle
                    state.config_mgr.config.accessibility_patterns =
                        !state.config_mgr.config.accessibility_patterns;
                    state.config_mgr.save();
                    let _ = windows::Win32::Graphics::Gdi::InvalidateRect(hwnd, None, true);
                } else if state.popup_in_settings
                    && crate::popup::point_in_rect(pt, state.setting_rects[8])
                {
                    // Icon style: cycle number → ring → bar → pie → number
                    let next = match state.config_mgr.config.tray_icon_style.as_str() {
                        "number" => "ring",
                        "ring" => "bar",
                        "bar" => "pie",
                        _ => "number",
                    };
                    state.config_mgr.config.tray_icon_style = next.to_string();
                    state.config_mgr.save();
                    // Immediately update tray icon with new style
                    let tooltip = build_tooltip(
                        &state.usage,
                        state.config_mgr.config.show_chatgpt_section,
                        &state.last_error,
                    );
                    if let Some(tray) = &mut state.tray {
                        tray.update(&state.usage, &tooltip, next);
                    }
                    let _ = windows::Win32::Graphics::Gdi::InvalidateRect(hwnd, None, true);
                } else if state.popup_in_settings
                    && crate::popup::point_in_rect(pt, state.setting_rects[9])
                {
                    // Dashboard layout: cycle minimal → standard → detailed
                    let next = match state.config_mgr.config.dashboard_layout.as_str() {
                        "minimal" => "standard",
                        "standard" => "detailed",
                        _ => "minimal",
                    };
                    state.config_mgr.config.dashboard_layout = next.to_string();
                    state.config_mgr.save();
                    let _ = windows::Win32::Graphics::Gdi::InvalidateRect(hwnd, None, true);
                } else if state.popup_in_settings
                    && crate::popup::point_in_rect(pt, state.setting_rects[10])
                {
                    // Hide Extra Usage: toggle
                    state.config_mgr.config.hide_extra_usage =
                        !state.config_mgr.config.hide_extra_usage;
                    state.config_mgr.save();
                    let _ = windows::Win32::Graphics::Gdi::InvalidateRect(hwnd, None, true);
                } else if state.popup_in_settings
                    && crate::popup::point_in_rect(pt, state.setting_rects[11])
                {
                    // Show startup notification: toggle
                    state.config_mgr.config.show_startup_notification =
                        !state.config_mgr.config.show_startup_notification;
                    state.config_mgr.save();
                    let _ = windows::Win32::Graphics::Gdi::InvalidateRect(hwnd, None, true);
                } else if state.popup_in_settings
                    && crate::popup::point_in_rect(pt, state.setting_rects[12])
                {
                    // Token expiry warning: toggle
                    state.config_mgr.config.token_expiry_warning =
                        !state.config_mgr.config.token_expiry_warning;
                    state.config_mgr.save();
                    let _ = windows::Win32::Graphics::Gdi::InvalidateRect(hwnd, None, true);
                } else if crate::popup::point_in_rect(pt, state.settings_rect) {
                    // Slide to settings
                    state.popup_in_settings = true;
                    let h = settings_panel_height();
                    resize_popup(hwnd, h);
                    state.slide_anim_offset = crate::ui::render::POPUP_WIDTH as f32;
                    state.slide_anim_target = 0.0;
                    state.slide_anim_active = true;
                    let _ = windows::Win32::UI::WindowsAndMessaging::SetTimer(
                        hwnd,
                        TIMER_SLIDE,
                        ANIM_INTERVAL_MS,
                        None,
                    );
                    let _ = windows::Win32::Graphics::Gdi::InvalidateRect(hwnd, None, true);
                } else if crate::popup::point_in_rect(pt, state.chart_toggle_rect) {
                    state.chart_mode = (state.chart_mode + 1) % 3;
                    let _ = windows::Win32::Graphics::Gdi::InvalidateRect(hwnd, None, true);
                } else if crate::popup::point_in_rect(pt, state.copy_rect) {
                    // Copy usage metrics to clipboard
                    if let Some(u) = &state.usage {
                        let text = build_usage_text(u);
                        copy_to_clipboard(&text);
                    }
                } else if crate::popup::point_in_rect(pt, state.refresh_rect) {
                    state.last_updated = "...".to_string();
                    let _ = windows::Win32::Graphics::Gdi::InvalidateRect(hwnd, None, true);
                    trigger_poll_force(state.main_hwnd);
                } else if crate::popup::point_in_rect(pt, state.install_rect) {
                    let url = state.config_mgr.config.claude_install_url.clone();
                    let _ = open::that(&url);
                } else if crate::popup::point_in_rect(pt, state.chatgpt_link_rect) {
                    let url = state.config_mgr.config.chatgpt_usage_url.clone();
                    let _ = open::that(&url);
                } else if crate::popup::point_in_rect(pt, state.status_link_rect) {
                    let _ = open::that("https://status.claude.com/");
                } else if crate::popup::point_in_rect(pt, state.plan_link_rect) {
                    let _ = open::that("https://claude.ai/settings/usage");
                }
            }
            LRESULT(0)
        }
        WM_KEYDOWN => {
            let vk = wparam.0 as u16;
            if vk == 0x1B {
                // VK_ESCAPE — close popup
                if let Some(state) = APP_STATE.as_mut() {
                    hide_popup(state);
                }
            } else if vk == 0x74 {
                // VK_F5 — refresh
                if let Some(state) = APP_STATE.as_mut() {
                    state.last_updated = "...".to_string();
                    let _ = windows::Win32::Graphics::Gdi::InvalidateRect(hwnd, None, false);
                    trigger_poll_force(state.main_hwnd);
                }
            }
            LRESULT(0)
        }
        // Close popup when clicking outside (WM_KILLFOCUS)
        WM_KILLFOCUS => {
            if let Some(state) = APP_STATE.as_mut() {
                if state.popup_visible {
                    hide_popup(state);
                }
            }
            LRESULT(0)
        }
        WM_DESTROY => LRESULT(0),
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

/// Apply DWM rounded corners to popup window (Win11+, silently fails on Win10)
unsafe fn apply_dwm_rounded_corners(hwnd: HWND) {
    // DWMWA_WINDOW_CORNER_PREFERENCE = 33, DWMWCP_ROUND = 2
    let corner_pref: u32 = 2;
    let _ = DwmSetWindowAttribute(
        hwnd,
        windows::Win32::Graphics::Dwm::DWMWINDOWATTRIBUTE(33),
        &corner_pref as *const u32 as *const _,
        std::mem::size_of::<u32>() as u32,
    );
}

/// Apply Acrylic backdrop to popup window (Win11 22H2+, silently fails on older)
unsafe fn apply_acrylic_backdrop(hwnd: HWND) {
    // DWMWA_SYSTEMBACKDROP_TYPE = 38, value 3 = Acrylic (transient window)
    let backdrop_type: u32 = 3;
    let _ = DwmSetWindowAttribute(
        hwnd,
        windows::Win32::Graphics::Dwm::DWMWINDOWATTRIBUTE(38),
        &backdrop_type as *const u32 as *const _,
        std::mem::size_of::<u32>() as u32,
    );

    // Extend DWM frame into entire client area so backdrop shows through
    let margins = windows::Win32::UI::Controls::MARGINS {
        cxLeftWidth: -1,
        cxRightWidth: -1,
        cyTopHeight: -1,
        cyBottomHeight: -1,
    };
    let _ = windows::Win32::Graphics::Dwm::DwmExtendFrameIntoClientArea(hwnd, &margins);
}

/// Apply DWM dark/light mode to popup window (Win11+, silently fails on Win10)
unsafe fn apply_dwm_dark_mode(hwnd: HWND, is_dark: bool) {
    // DWMWA_USE_IMMERSIVE_DARK_MODE = 20
    let dark: u32 = if is_dark { 1 } else { 0 };
    let _ = DwmSetWindowAttribute(
        hwnd,
        windows::Win32::Graphics::Dwm::DWMWINDOWATTRIBUTE(20),
        &dark as *const u32 as *const _,
        std::mem::size_of::<u32>() as u32,
    );
}

/// Hide the popup and release D2D resources to reclaim memory.
unsafe fn hide_popup(state: &mut AppState) {
    state.popup_visible = false;
    state.hovered_element = HoveredElement::None;
    state.mouse_tracking = false;
    state.anim_active = false;
    let _ = windows::Win32::UI::WindowsAndMessaging::KillTimer(state.popup_hwnd, TIMER_ANIM);
    let _ = windows::Win32::UI::WindowsAndMessaging::KillTimer(state.popup_hwnd, TIMER_FADE);
    // Remove WS_EX_LAYERED if still set from fade
    let ex = GetWindowLongW(state.popup_hwnd, GWL_EXSTYLE);
    if ex & WS_EX_LAYERED.0 as i32 != 0 {
        SetWindowLongW(
            state.popup_hwnd,
            GWL_EXSTYLE,
            ex & !(WS_EX_LAYERED.0 as i32),
        );
    }
    let _ = windows::Win32::UI::WindowsAndMessaging::ShowWindow(
        state.popup_hwnd,
        windows::Win32::UI::WindowsAndMessaging::SW_HIDE,
    );
    if let Some(d2d) = state.d2d.as_mut() {
        d2d.release();
    }
    trim_working_set();
}

/// Ask Windows to trim the process working set so freed memory is returned to the OS.
fn trim_working_set() {
    extern "system" {
        fn SetProcessWorkingSetSize(
            hProcess: *mut core::ffi::c_void,
            dwMinimumWorkingSetSize: usize,
            dwMaximumWorkingSetSize: usize,
        ) -> i32;
    }
    unsafe {
        let process = windows::Win32::System::Threading::GetCurrentProcess();
        SetProcessWorkingSetSize(process.0, usize::MAX, usize::MAX);
    }
}

unsafe fn toggle_popup(main_hwnd: HWND) {
    if let Some(state) = APP_STATE.as_mut() {
        if state.popup_visible {
            hide_popup(state);
        } else {
            show_popup(main_hwnd);
        }
    }
}

/// Calculate the settings panel height (header + rows + footer).
/// Check if Windows Focus Assist / DND is active.
/// Uses SHQueryUserNotificationState from shell32.
fn is_focus_assist_active() -> bool {
    use windows::Win32::UI::Shell::SHQueryUserNotificationState;
    unsafe {
        match SHQueryUserNotificationState() {
            Ok(state) => {
                // QUNS_ACCEPTS_NOTIFICATIONS = 5; anything else means DND is active
                state.0 != 5
            }
            Err(_) => false,
        }
    }
}

fn settings_panel_height() -> i32 {
    let header_h = 40;
    let row_h = 38;
    let num_rows = 13;
    let legend_h = 10 + 1 + 10 + 18 + (4 * 18) + 10; // gap + sep + gap + title + 4 rows + bottom padding
    let footer_h = 44;
    header_h + 8 + (num_rows * row_h) + legend_h + footer_h
}

unsafe fn resize_popup(popup_hwnd: HWND, h: i32) {
    let mut work_area = RECT::default();
    let _ = windows::Win32::UI::WindowsAndMessaging::SystemParametersInfoW(
        windows::Win32::UI::WindowsAndMessaging::SPI_GETWORKAREA,
        0,
        Some(&mut work_area as *mut RECT as *mut _),
        windows::Win32::UI::WindowsAndMessaging::SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0),
    );
    let popup_w = crate::ui::render::POPUP_WIDTH;
    let x = work_area.right - popup_w - 10;
    let y = work_area.bottom - h - 10;

    let _ = windows::Win32::UI::WindowsAndMessaging::MoveWindow(
        popup_hwnd,
        x.max(0),
        y.max(0),
        popup_w,
        h,
        true,
    );

    // Resize D2D render target
    if let Some(state) = APP_STATE.as_mut() {
        if let Some(d2d) = state.d2d.as_mut() {
            d2d.resize(popup_w as u32, h as u32);
        }
    }
}

unsafe fn show_popup(_main_hwnd: HWND) {
    if let Some(state) = APP_STATE.as_mut() {
        // Calculate height based on current mode
        let h = if state.popup_in_settings {
            settings_panel_height()
        } else {
            let renderer = PopupRenderer::new(state.popup_hwnd);
            renderer.calculate_height(
                &state.usage,
                state.config_mgr.config.show_chatgpt_section,
                state.config_mgr.config.compact_mode,
                &state.config_mgr.config.dashboard_layout,
                state.config_mgr.config.hide_extra_usage,
            )
        };

        resize_popup(state.popup_hwnd, h);

        // Fade-in: add WS_EX_LAYERED and start transparent
        let ex = GetWindowLongW(state.popup_hwnd, GWL_EXSTYLE);
        SetWindowLongW(state.popup_hwnd, GWL_EXSTYLE, ex | WS_EX_LAYERED.0 as i32);
        state.fade_alpha = 0;
        let _ = SetLayeredWindowAttributes(
            state.popup_hwnd,
            windows::Win32::Foundation::COLORREF(0),
            0,
            LWA_ALPHA,
        );

        let _ = windows::Win32::UI::WindowsAndMessaging::SetWindowPos(
            state.popup_hwnd,
            windows::Win32::UI::WindowsAndMessaging::HWND_TOPMOST,
            0,
            0,
            0,
            0,
            windows::Win32::UI::WindowsAndMessaging::SWP_NOMOVE
                | windows::Win32::UI::WindowsAndMessaging::SWP_NOSIZE
                | windows::Win32::UI::WindowsAndMessaging::SWP_SHOWWINDOW,
        );
        let _ = windows::Win32::UI::WindowsAndMessaging::ShowWindow(
            state.popup_hwnd,
            windows::Win32::UI::WindowsAndMessaging::SW_SHOW,
        );
        let _ = SetForegroundWindow(state.popup_hwnd);
        let _ = windows::Win32::Graphics::Gdi::InvalidateRect(state.popup_hwnd, None, true);
        state.popup_visible = true;

        // Start fade-in timer
        windows::Win32::UI::WindowsAndMessaging::SetTimer(
            state.popup_hwnd,
            TIMER_FADE,
            FADE_INTERVAL_MS,
            None,
        );

        // Stop tray icon blink when user opens popup
        if state.blink_active {
            state.blink_active = false;
            state.blink_visible = true;
            let _ =
                windows::Win32::UI::WindowsAndMessaging::KillTimer(state.main_hwnd, TIMER_BLINK);
            // Restore normal icon
            let tooltip = build_tooltip(
                &state.usage,
                state.config_mgr.config.show_chatgpt_section,
                &state.last_error,
            );
            if let Some(tray) = &mut state.tray {
                let style = &state.config_mgr.config.tray_icon_style.clone();
                tray.update(&state.usage, &tooltip, style);
            }
        }

        // Start animation if we have targets (cascading: each metric starts with a delay)
        if !state.anim_targets.is_empty() {
            // Stagger: each metric starts at a negative value proportional to index.
            // The animation loop clamps to 0, so later metrics visually wait before filling.
            state.anim_current = state
                .anim_targets
                .iter()
                .enumerate()
                .map(|(i, _)| -(i as f64 * 8.0))
                .collect();
            state.anim_active = true;
            windows::Win32::UI::WindowsAndMessaging::SetTimer(
                state.popup_hwnd,
                TIMER_ANIM,
                ANIM_INTERVAL_MS,
                None,
            );
        }

        // Auto-refresh if data is stale. Popup opens should prefer fresh API data
        // over DB cache, but still avoid hammering the API on rapid open/close.
        let stale = state
            .last_poll_time
            .map(|t| t.elapsed().as_secs() > 15)
            .unwrap_or(true);
        if stale {
            trigger_poll_force(state.main_hwnd);
        }
    }
}

unsafe fn show_theme_popup(hwnd: HWND, client_pt: POINT, state: &mut AppState) {
    let menu = CreatePopupMenu().unwrap();
    let current = &state.config_mgr.config.theme;

    let themes = [
        (IDM_THEME_AUTO, "auto", "Auto"),
        (IDM_THEME_DARK, "dark", "Dark"),
        (IDM_THEME_LIGHT, "light", "Light"),
    ];

    for (id, value, i18n_key) in &themes {
        let flag = if current == *value {
            MF_STRING | MF_CHECKED
        } else {
            MF_STRING | MF_UNCHECKED
        };
        let label = wide(state.i18n.t(i18n_key));
        let _ = AppendMenuW(menu, flag, *id as usize, PCWSTR(label.as_ptr()));
    }

    let mut screen_pt = client_pt;
    let _ = ClientToScreen(hwnd, &mut screen_pt);

    let _ = SetForegroundWindow(hwnd);
    let cmd = TrackPopupMenu(
        menu,
        TPM_LEFTALIGN | TPM_RETURNCMD,
        screen_pt.x,
        screen_pt.y,
        0,
        hwnd,
        None,
    );
    let _ = DestroyMenu(menu);

    if cmd.as_bool() {
        let new_theme = match cmd.0 as u32 {
            IDM_THEME_DARK => "dark",
            IDM_THEME_LIGHT => "light",
            _ => "auto",
        };
        state.config_mgr.config.theme = new_theme.to_string();
        state.config_mgr.save();
        let _ = windows::Win32::Graphics::Gdi::InvalidateRect(hwnd, None, true);
    }
}

unsafe fn show_language_popup(hwnd: HWND, client_pt: POINT, state: &mut AppState) {
    let menu = CreatePopupMenu().unwrap();
    let current_lang = &state.config_mgr.config.language;

    // "Auto (detected)" option
    let auto_flag = if current_lang == "auto" {
        MF_STRING | MF_CHECKED
    } else {
        MF_STRING | MF_UNCHECKED
    };
    let auto_label = state.i18n.t("Auto (English)");
    let auto_wide = wide(auto_label);
    let _ = AppendMenuW(
        menu,
        auto_flag,
        IDM_LANG_AUTO as usize,
        PCWSTR(auto_wide.as_ptr()),
    );
    append_menu_sep(menu);

    // All locales sorted alphabetically by display name
    for (i, locale) in crate::i18n::Locale::all().iter().enumerate() {
        let flag = if current_lang == locale.as_str() {
            MF_STRING | MF_CHECKED
        } else {
            MF_STRING | MF_UNCHECKED
        };
        let label_wide = wide(locale.display_name());
        let _ = AppendMenuW(
            menu,
            flag,
            (IDM_LANG_BASE + i as u32) as usize,
            PCWSTR(label_wide.as_ptr()),
        );
    }

    // Convert client coords to screen coords for TrackPopupMenu
    let mut screen_pt = client_pt;
    let _ = ClientToScreen(hwnd, &mut screen_pt);

    let _ = SetForegroundWindow(hwnd);
    let cmd = TrackPopupMenu(
        menu,
        TPM_LEFTALIGN | TPM_RETURNCMD,
        screen_pt.x,
        screen_pt.y,
        0,
        hwnd,
        None,
    );
    let _ = DestroyMenu(menu);

    if cmd.as_bool() {
        let cmd_id = cmd.0 as u32;
        let new_lang = if cmd_id == IDM_LANG_AUTO {
            "auto".to_string()
        } else {
            let idx = (cmd_id - IDM_LANG_BASE) as usize;
            let locales = crate::i18n::Locale::all();
            if idx < locales.len() {
                locales[idx].as_str().to_string()
            } else {
                return;
            }
        };
        state.config_mgr.config.language = new_lang;
        state.i18n = I18n::from_config(&state.config_mgr.config.language);
        state.config_mgr.save();
        let _ = windows::Win32::Graphics::Gdi::InvalidateRect(hwnd, None, true);
    }
}

unsafe fn show_context_menu(hwnd: HWND) {
    if let Some(state) = APP_STATE.as_ref() {
        let menu = CreatePopupMenu().unwrap();
        let show_chatgpt = state.config_mgr.config.show_chatgpt_section;
        let autostart = state.config_mgr.config.autostart;

        append_menu_str(menu, IDM_REFRESH, state.i18n.t("Refresh Now"));
        append_menu_sep(menu);
        append_menu_str(menu, IDM_OPEN_DASHBOARD, state.i18n.t("Open Dashboard"));
        append_menu_str(menu, IDM_EXPORT_CSV, state.i18n.t("Export History (CSV)"));
        append_menu_sep(menu);
        append_menu_str(menu, IDM_OPEN_CLAUDE, "Open Claude.ai \u{2192}");
        if show_chatgpt {
            append_menu_str(
                menu,
                IDM_OPEN_CHATGPT,
                state.i18n.t("Open ChatGPT Usage \u{2192}"),
            );
        }
        append_menu_sep(menu);
        append_menu_str(menu, IDM_SETTINGS, state.i18n.t("Settings"));
        // Autostart toggle with checkmark
        let autostart_flag = if autostart {
            MF_STRING | MF_CHECKED
        } else {
            MF_STRING | MF_UNCHECKED
        };
        let autostart_text = wide(state.i18n.t("Start with Windows"));
        let _ = AppendMenuW(
            menu,
            autostart_flag,
            IDM_AUTOSTART as usize,
            PCWSTR(autostart_text.as_ptr()),
        );
        append_menu_sep(menu);
        let about_label = format!("ClaudeMeter v{}", env!("CARGO_PKG_VERSION"));
        append_menu_str(menu, IDM_ABOUT, &about_label);
        append_menu_str(menu, IDM_EXIT, state.i18n.t("Exit"));

        let mut pt = POINT::default();
        let _ = GetCursorPos(&mut pt);
        let _ = SetForegroundWindow(hwnd);
        let cmd = TrackPopupMenu(
            menu,
            TPM_LEFTALIGN | TPM_BOTTOMALIGN | TPM_RETURNCMD,
            pt.x,
            pt.y,
            0,
            hwnd,
            None,
        );
        let _ = DestroyMenu(menu);

        if cmd.as_bool() {
            handle_menu_command(hwnd, cmd.0 as u32);
        }
    }
}

unsafe fn append_menu_str(menu: HMENU, id: u32, text: &str) {
    let wide_text = wide(text);
    let _ = AppendMenuW(menu, MF_STRING, id as usize, PCWSTR(wide_text.as_ptr()));
}

unsafe fn append_menu_sep(menu: HMENU) {
    let _ = AppendMenuW(menu, MF_SEPARATOR, 0, PCWSTR::null());
}

unsafe fn handle_menu_command(hwnd: HWND, cmd: u32) {
    match cmd {
        IDM_REFRESH => {
            trigger_poll_force(hwnd);
        }
        IDM_OPEN_DASHBOARD => {
            show_popup(hwnd);
        }
        IDM_OPEN_CLAUDE => {
            let _ = open::that("https://claude.ai/settings/usage");
        }
        IDM_OPEN_CHATGPT => {
            if let Some(state) = APP_STATE.as_ref() {
                let url = state.config_mgr.config.chatgpt_usage_url.clone();
                let _ = open::that(&url);
            }
        }
        IDM_SETTINGS => {
            if let Some(state) = APP_STATE.as_mut() {
                state.popup_in_settings = true;
                show_popup(hwnd);
            }
        }
        IDM_AUTOSTART => {
            if let Some(state) = APP_STATE.as_mut() {
                let new_val = !state.config_mgr.config.autostart;
                state.config_mgr.config.autostart = new_val;
                state.config_mgr.save();
                let exe_path = std::env::current_exe()
                    .ok()
                    .and_then(|p| p.to_str().map(|s| s.to_string()))
                    .unwrap_or_default();
                if let Err(e) = autostart::set_autostart(new_val, &exe_path) {
                    log::warn!("Failed to set autostart: {e}");
                }
            }
        }
        IDM_EXPORT_CSV => {
            if let Some(state) = APP_STATE.as_ref() {
                let csv_path = state.exe_dir.join("claudemeter_history.csv");
                match Database::open(&state.exe_dir) {
                    Ok(db) => match db.export_csv(&csv_path) {
                        Ok(count) => {
                            let msg =
                                format!("{} rows exported to:\n{}", count, csv_path.display());
                            let _ = windows::Win32::UI::WindowsAndMessaging::MessageBoxW(
                                hwnd,
                                windows::core::PCWSTR(wide(&msg).as_ptr()),
                                windows::core::PCWSTR(wide("Export CSV").as_ptr()),
                                windows::Win32::UI::WindowsAndMessaging::MB_ICONINFORMATION
                                    | windows::Win32::UI::WindowsAndMessaging::MB_OK,
                            );
                        }
                        Err(e) => {
                            let msg = format!("Export failed: {e}");
                            let _ = windows::Win32::UI::WindowsAndMessaging::MessageBoxW(
                                hwnd,
                                windows::core::PCWSTR(wide(&msg).as_ptr()),
                                windows::core::PCWSTR(wide("Export CSV").as_ptr()),
                                windows::Win32::UI::WindowsAndMessaging::MB_ICONERROR
                                    | windows::Win32::UI::WindowsAndMessaging::MB_OK,
                            );
                        }
                    },
                    Err(e) => {
                        let msg = format!("Could not open database: {e}");
                        let _ = windows::Win32::UI::WindowsAndMessaging::MessageBoxW(
                            hwnd,
                            windows::core::PCWSTR(wide(&msg).as_ptr()),
                            windows::core::PCWSTR(wide("Export CSV").as_ptr()),
                            windows::Win32::UI::WindowsAndMessaging::MB_ICONERROR
                                | windows::Win32::UI::WindowsAndMessaging::MB_OK,
                        );
                    }
                }
            }
        }
        IDM_ABOUT => {
            // Show simple about message
            let _ = windows::Win32::UI::WindowsAndMessaging::MessageBoxW(
                hwnd,
                windows::core::PCWSTR(wide(&format!("ClaudeMeter v{}\nby klivak\nhttps://github.com/klivak/claudemeter\n\nMIT License", env!("CARGO_PKG_VERSION"))).as_ptr()),
                windows::core::PCWSTR(wide("About ClaudeMeter").as_ptr()),
                windows::Win32::UI::WindowsAndMessaging::MB_ICONINFORMATION | windows::Win32::UI::WindowsAndMessaging::MB_OK,
            );
        }
        IDM_EXIT => {
            // Clean up tray icon
            if let Some(state) = APP_STATE.as_mut() {
                state.tray = None;
            }
            PostQuitMessage(0);
        }
        _ => {}
    }
}

/// Spawn async poll task. Result is posted back to main hwnd via WM_USER+20.
unsafe fn trigger_poll(hwnd: HWND) {
    trigger_poll_internal(hwnd, false);
}

/// Force a user-requested poll. If a poll is already running, queue exactly one
/// follow-up poll so manual refresh cannot be dropped behind stale cached data.
unsafe fn trigger_poll_force(hwnd: HWND) {
    trigger_poll_internal(hwnd, true);
}

unsafe fn trigger_poll_internal(hwnd: HWND, force: bool) {
    // Prevent concurrent polls (avoids 429 rate limiting)
    let web_fallback = if let Some(state) = APP_STATE.as_mut() {
        if state.poll_in_progress {
            if force {
                state.pending_force_poll = true;
            }
            return;
        }
        state.poll_in_progress = true;
        if force {
            state.pending_force_poll = false;
        }
        // Capture web API fallback config for the background thread
        match (
            &state.config_mgr.config.web_api_session_key,
            &state.config_mgr.config.web_api_org_id,
        ) {
            (Some(key), Some(org)) if !key.is_empty() && !org.is_empty() => {
                Some((key.clone(), org.clone()))
            }
            _ => None,
        }
    } else {
        None
    };

    // We run a background thread for async work to avoid blocking the message loop.
    // tokio::spawn would require a runtime, so we use std::thread + tokio block_on.
    let hwnd_val = hwnd.0 as isize;

    std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_io()
            .enable_time()
            .build()
            .unwrap();

        rt.block_on(async move {
            let result = do_poll(web_fallback).await;

            // Post result back to main thread
            let hwnd = HWND(hwnd_val as *mut _);
            let result_ptr = Box::into_raw(Box::new(result)) as isize;

            let _ = PostMessageW(hwnd, WM_POLL_RESULT, WPARAM(result_ptr as usize), LPARAM(0));
        });
    });
}

/// Result of a poll, including optional token expiry info.
struct PollResult {
    usage: Option<UsageResponse>,
    error: Option<String>,
    /// Milliseconds until token expires (None if unknown)
    token_expires_in_ms: Option<u64>,
}

/// Result of background DB operations, posted back to main thread via WM_DB_RESULT.
struct DbResult {
    chart_data: Vec<f64>,
    chart_data_7d: Vec<f64>,
    chart_data_30d: Vec<f64>,
    rate_of_change: std::collections::HashMap<String, f64>,
}

async fn do_poll(web_fallback: Option<(String, String)>) -> PollResult {
    let cred_info = match credentials::read_claude_token() {
        Ok(info) => info,
        Err(e) => {
            log::warn!("Could not read Claude token: {e}");
            // Try web API fallback if credentials not found
            if let Some((session_key, org_id)) = &web_fallback {
                log::info!("Attempting web API fallback");
                if let Ok(client) = ClaudeClient::new() {
                    if let Ok(usage) = client.fetch_usage_web(session_key, org_id).await {
                        return PollResult {
                            usage: Some(usage),
                            error: None,
                            token_expires_in_ms: None,
                        };
                    }
                }
            }
            return PollResult {
                usage: None,
                error: Some(e.to_string()),
                token_expires_in_ms: None,
            };
        }
    };

    // Check token expiry
    let now_ms = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;

    let token_expires_in_ms = cred_info.expires_at.map(|exp| exp.saturating_sub(now_ms));

    // If token is already expired, try web fallback first
    if let Some(0) = token_expires_in_ms {
        if let Some((session_key, org_id)) = &web_fallback {
            log::info!("OAuth token expired, attempting web API fallback");
            if let Ok(client) = ClaudeClient::new() {
                if let Ok(usage) = client.fetch_usage_web(session_key, org_id).await {
                    return PollResult {
                        usage: Some(usage),
                        error: None,
                        token_expires_in_ms: Some(0),
                    };
                }
            }
        }
        return PollResult {
            usage: None,
            error: Some(
                "[token_expired] OAuth token has expired. Run `claude login` to refresh.".into(),
            ),
            token_expires_in_ms: Some(0),
        };
    }

    let client = match ClaudeClient::new() {
        Ok(c) => c,
        Err(e) => {
            log::error!("Failed to create HTTP client: {e}");
            return PollResult {
                usage: None,
                error: Some(e),
                token_expires_in_ms,
            };
        }
    };

    match client.fetch_usage(&cred_info.access_token).await {
        Ok(mut usage) => {
            usage.subscription_type = cred_info.subscription_type;
            usage.rate_limit_tier = cred_info.rate_limit_tier;
            PollResult {
                usage: Some(usage),
                error: None,
                token_expires_in_ms,
            }
        }
        Err(e) => {
            // On OAuth failure, try web API fallback
            if let Some((session_key, org_id)) = &web_fallback {
                log::info!("OAuth API failed, attempting web API fallback: {e}");
                if let Ok(usage) = client.fetch_usage_web(session_key, org_id).await {
                    return PollResult {
                        usage: Some(usage),
                        error: None,
                        token_expires_in_ms,
                    };
                }
            }
            log::warn!("Failed to fetch usage: {e}");
            PollResult {
                usage: None,
                error: Some(e),
                token_expires_in_ms,
            }
        }
    }
}

unsafe fn on_poll_result(hwnd: HWND, result: PollResult) {
    let mut run_pending_force_poll = false;

    if let Some(state) = APP_STATE.as_mut() {
        state.poll_in_progress = false;
        state.last_poll_time = Some(std::time::Instant::now());

        let PollResult {
            usage,
            error,
            token_expires_in_ms,
        } = result;

        // Token expiry warning: notify once when token expires within 1 hour
        if let Some(ms) = token_expires_in_ms {
            if ms > 0
                && ms < 3_600_000
                && !state.token_expiry_warned
                && !is_focus_assist_active()
                && state.config_mgr.config.token_expiry_warning
            {
                state.token_expiry_warned = true;
                let minutes = ms / 60_000;
                if let Some(tray) = &state.tray {
                    tray.show_balloon(
                        "ClaudeMeter",
                        &format!(
                            "OAuth token expires in ~{} min. Run `claude login` to refresh.",
                            minutes
                        ),
                    );
                }
            }
            // Reset warning flag if token was refreshed (>1h remaining)
            if ms >= 3_600_000 {
                state.token_expiry_warned = false;
            }
        }

        if let Some(u) = &usage {
            state.consecutive_failures = 0;
            // Stop wake retry schedule on successful poll
            let _ = windows::Win32::UI::WindowsAndMessaging::KillTimer(hwnd, TIMER_WAKE_RETRY);
            state.last_updated = Local::now().format("%H:%M:%S").to_string();

            // Store to DB and query charts on background thread
            let metrics: Vec<(String, f64, Option<String>)> = u
                .all_metrics()
                .iter()
                .filter(|(key, metric)| !(key == "five_hour" && metric.resets_at.is_none()))
                .map(|(key, metric)| (key.clone(), metric.utilization, metric.resets_at.clone()))
                .collect();
            let exe_dir = state.exe_dir.clone();
            let hwnd_val = hwnd.0 as isize;
            std::thread::spawn(move || {
                if let Ok(db) = Database::open(&exe_dir) {
                    for (key, utilization, resets_at) in &metrics {
                        let _ = db.insert("claude", key, *utilization, resets_at.as_deref());
                    }
                    let chart_data = db.query_24h_chart("claude").unwrap_or_default();
                    let chart_data_7d = db.query_7d_chart().unwrap_or_default();
                    let chart_data_30d = db.query_30d_chart().unwrap_or_default();
                    let rate_of_change = db.query_rate_of_change(60).unwrap_or_default();
                    let result = Box::new(DbResult {
                        chart_data,
                        chart_data_7d,
                        chart_data_30d,
                        rate_of_change,
                    });
                    let result_ptr = Box::into_raw(result) as isize;
                    let hwnd = HWND(hwnd_val as *mut _);
                    let _ =
                        PostMessageW(hwnd, WM_DB_RESULT, WPARAM(result_ptr as usize), LPARAM(0));
                }
            });

            // Calculate 5-hour session reset lines for chart
            state.chart_reset_lines.clear();
            if let Some(fh) = u.five_hour.as_ref() {
                if let Some(secs) = fh.resets_at.as_deref().and_then(crate::i18n::seconds_until) {
                    let hours_until = secs as f64 / 3600.0;
                    let mut hours_ago = 5.0 - hours_until;
                    while hours_ago <= 24.0 {
                        if hours_ago > 0.0 {
                            state.chart_reset_lines.push(hours_ago);
                        }
                        hours_ago += 5.0;
                    }
                }
            }

            // Check notifications (skip during quiet hours)
            let thresholds = state.config_mgr.config.notifications.thresholds.clone();
            let in_quiet = is_in_quiet_hours(&state.config_mgr.config.quiet_hours);
            let focus_assist = is_focus_assist_active();
            if state.config_mgr.config.notifications.enabled && !in_quiet && !focus_assist {
                // Collect all fired notifications, then emit a single aggregated balloon
                let mut fired_alerts: Vec<(String, f64, u8, String)> = Vec::new();

                for (key, metric) in u.all_metrics() {
                    let fired =
                        state
                            .notification_tracker
                            .check(&key, metric.utilization, &thresholds);
                    for threshold in fired {
                        let metric_name = providers::claude::format_metric_name(&key);
                        let reset_duration = metric
                            .resets_at
                            .as_deref()
                            .and_then(i18n::seconds_until)
                            .map(format_duration);
                        let reset_target = metric
                            .resets_at
                            .as_deref()
                            .and_then(i18n::format_reset_target);
                        let reset_info = match (&reset_duration, &reset_target) {
                            (Some(dur), Some(tgt)) => {
                                format!(" ({} {} {})", state.i18n.t("resets in"), dur, tgt)
                            }
                            (Some(dur), None) => {
                                format!(" ({} {})", state.i18n.t("resets in"), dur)
                            }
                            _ => String::new(),
                        };
                        fired_alerts.push((metric_name, metric.utilization, threshold, reset_info));
                    }
                }

                if !fired_alerts.is_empty() {
                    let max_threshold = fired_alerts.iter().map(|a| a.2).max().unwrap_or(0);
                    let is_critical = max_threshold >= 90;

                    let title = if is_critical {
                        format!("ClaudeMeter \u{2014} {}", state.i18n.t("Usage Critical"))
                    } else {
                        format!("ClaudeMeter \u{2014} {}", state.i18n.t("Usage Alert"))
                    };

                    let body = if fired_alerts.len() == 1 {
                        let (ref name, util, thr, ref reset) = fired_alerts[0];
                        format!(
                            "{}: {:.0}% ({} {}%){}",
                            name,
                            util,
                            state.i18n.t("exceeded"),
                            thr,
                            reset
                        )
                    } else {
                        // Aggregated: one line per metric
                        fired_alerts
                            .iter()
                            .map(|(name, util, thr, reset)| {
                                format!(
                                    "{}: {:.0}% ({} {}%){}",
                                    name,
                                    util,
                                    state.i18n.t("exceeded"),
                                    thr,
                                    reset
                                )
                            })
                            .collect::<Vec<_>>()
                            .join("\n")
                    };

                    if let Some(tray) = &state.tray {
                        tray.show_balloon(&title, &body);
                    }

                    // Play notification sound (critical if any threshold >= 90)
                    if state.config_mgr.config.notifications.sound {
                        play_notification_sound(is_critical);
                    }

                    // Start tray icon blink for critical usage
                    if is_critical && !state.blink_active {
                        state.blink_active = true;
                        state.blink_visible = true;
                        windows::Win32::UI::WindowsAndMessaging::SetTimer(
                            state.main_hwnd,
                            TIMER_BLINK,
                            BLINK_INTERVAL_MS,
                            None,
                        );
                    }
                }
            }
        } else {
            // Poll failed — track for backoff
            state.consecutive_failures += 1;

            // Start error blink if there's no cached data to show
            if state.usage.is_none() && !state.blink_active {
                state.blink_active = true;
                state.blink_visible = true;
                windows::Win32::UI::WindowsAndMessaging::SetTimer(
                    state.main_hwnd,
                    TIMER_BLINK,
                    BLINK_INTERVAL_MS,
                    None,
                );
            }
        }

        // Only overwrite usage when poll succeeded; keep previous data on failure
        if usage.is_some() {
            state.usage = usage;
        }
        state.last_error = error;

        // Adjust polling interval: random 90-180s on success, exponential backoff on failures
        let base = crate::config::Config::random_polling_interval() as u32 * 1000;
        let interval = if state.consecutive_failures > 0 {
            let multiplier = 2u32.pow(state.consecutive_failures.min(3));
            (base * multiplier).min(600_000) // cap at 10 minutes
        } else {
            base
        };
        let _ = windows::Win32::UI::WindowsAndMessaging::KillTimer(hwnd, TIMER_POLL);
        windows::Win32::UI::WindowsAndMessaging::SetTimer(hwnd, TIMER_POLL, interval, None);

        // Start progress bar animation
        if let Some(u) = &state.usage {
            let targets: Vec<f64> = u.all_metrics().iter().map(|(_, m)| m.utilization).collect();
            // Initialize current values at 0 if sizes differ (new data shape)
            if state.anim_current.len() != targets.len() {
                state.anim_current = vec![0.0; targets.len()];
            }
            state.anim_targets = targets;
            state.anim_active = true;
            if state.popup_visible {
                windows::Win32::UI::WindowsAndMessaging::SetTimer(
                    state.popup_hwnd,
                    TIMER_ANIM,
                    ANIM_INTERVAL_MS,
                    None,
                );
            }
        }

        // Update tray
        let tooltip = build_tooltip(
            &state.usage,
            state.config_mgr.config.show_chatgpt_section,
            &state.last_error,
        );
        if let Some(tray) = &mut state.tray {
            let style = &state.config_mgr.config.tray_icon_style.clone();
            tray.update(&state.usage, &tooltip, style);
        }

        // Refresh popup if visible (resize + repaint)
        if state.popup_visible && !state.popup_in_settings {
            let renderer = PopupRenderer::new(state.popup_hwnd);
            let h = renderer.calculate_height(
                &state.usage,
                state.config_mgr.config.show_chatgpt_section,
                state.config_mgr.config.compact_mode,
                &state.config_mgr.config.dashboard_layout,
                state.config_mgr.config.hide_extra_usage,
            );
            resize_popup(state.popup_hwnd, h);
            let _ = windows::Win32::Graphics::Gdi::InvalidateRect(state.popup_hwnd, None, true);
        } else if state.popup_visible {
            let _ = windows::Win32::Graphics::Gdi::InvalidateRect(state.popup_hwnd, None, true);
        }

        // Refresh mini-widget
        if let Some(w) = state.widget_hwnd {
            widget::invalidate_widget(w);
        }

        if state.pending_force_poll {
            state.pending_force_poll = false;
            state.last_updated = "...".to_string();
            run_pending_force_poll = true;
        }
    }

    if run_pending_force_poll {
        trigger_poll_force(hwnd);
    }
}

fn ensure_single_instance() -> bool {
    let name = wide("ClaudeMeter-SingleInstance");
    unsafe {
        let mutex = CreateMutexW(None, true, windows::core::PCWSTR(name.as_ptr()));
        match mutex {
            Ok(_) => {
                let last_err = GetLastError();
                // ERROR_ALREADY_EXISTS = 183
                if last_err.0 == 183 {
                    return false;
                }
                true
            }
            Err(_) => true, // Proceed anyway if we can't create mutex
        }
    }
}

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

/// Check if current local time falls within the quiet hours window.
fn is_in_quiet_hours(qh: &crate::config::QuietHoursConfig) -> bool {
    if !qh.enabled {
        return false;
    }
    let parse_hm = |s: &str| -> Option<(u32, u32)> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() == 2 {
            Some((parts[0].parse().ok()?, parts[1].parse().ok()?))
        } else {
            None
        }
    };
    let (sh, sm) = match parse_hm(&qh.start) {
        Some(v) => v,
        None => return false,
    };
    let (eh, em) = match parse_hm(&qh.end) {
        Some(v) => v,
        None => return false,
    };
    let now = chrono::Local::now();
    let now_mins = now.hour() * 60 + now.minute();
    let start_mins = sh * 60 + sm;
    let end_mins = eh * 60 + em;
    if start_mins <= end_mins {
        // Same-day range (e.g., 08:00 → 18:00)
        now_mins >= start_mins && now_mins < end_mins
    } else {
        // Overnight range (e.g., 22:00 → 08:00)
        now_mins >= start_mins || now_mins < end_mins
    }
}

/// Check if the user has been idle for more than `timeout_ms` milliseconds.
fn is_user_idle(timeout_ms: u32) -> bool {
    #[repr(C)]
    #[allow(clippy::upper_case_acronyms)]
    struct LASTINPUTINFO {
        cb_size: u32,
        dw_time: u32,
    }
    extern "system" {
        fn GetLastInputInfo(plii: *mut LASTINPUTINFO) -> i32;
        fn GetTickCount() -> u32;
    }
    unsafe {
        let mut lii = LASTINPUTINFO {
            cb_size: 8,
            dw_time: 0,
        };
        if GetLastInputInfo(&mut lii) != 0 {
            let idle = GetTickCount().wrapping_sub(lii.dw_time);
            idle > timeout_ms
        } else {
            false
        }
    }
}

/// Build a text summary of current usage for clipboard.
fn build_usage_text(usage: &UsageResponse) -> String {
    let mut lines = vec![format!("ClaudeMeter — Claude ({})", usage.detected_plan())];
    for (key, metric) in usage.all_metrics() {
        let name = providers::claude::format_metric_name(&key);
        let reset_str = metric
            .resets_at
            .as_deref()
            .and_then(i18n::seconds_until)
            .map(|s| format!(" (resets in {})", format_duration(s)))
            .unwrap_or_default();
        lines.push(format!("{}: {:.0}%{}", name, metric.utilization, reset_str));
    }
    lines.join("\n")
}

/// Copy text to the Windows clipboard using raw Win32 API.
fn copy_to_clipboard(text: &str) {
    extern "system" {
        fn OpenClipboard(hwnd: *mut core::ffi::c_void) -> i32;
        fn CloseClipboard() -> i32;
        fn EmptyClipboard() -> i32;
        fn SetClipboardData(format: u32, hmem: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
        fn GlobalAlloc(flags: u32, bytes: usize) -> *mut core::ffi::c_void;
        fn GlobalLock(hmem: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
        fn GlobalUnlock(hmem: *mut core::ffi::c_void) -> i32;
    }
    const CF_UNICODETEXT: u32 = 13;
    const GMEM_MOVEABLE: u32 = 0x0002;

    let wide: Vec<u16> = text.encode_utf16().chain(std::iter::once(0)).collect();
    let byte_len = wide.len() * 2;

    unsafe {
        if OpenClipboard(std::ptr::null_mut()) == 0 {
            return;
        }
        EmptyClipboard();

        let hmem = GlobalAlloc(GMEM_MOVEABLE, byte_len);
        if !hmem.is_null() {
            let ptr = GlobalLock(hmem);
            if !ptr.is_null() {
                std::ptr::copy_nonoverlapping(wide.as_ptr() as *const u8, ptr as *mut u8, byte_len);
                GlobalUnlock(hmem);
                SetClipboardData(CF_UNICODETEXT, hmem);
            }
        }
        CloseClipboard();
    }
}

/// Play a system notification sound.
fn play_notification_sound(critical: bool) {
    extern "system" {
        fn MessageBeep(uType: u32) -> i32;
    }
    unsafe {
        if critical {
            MessageBeep(0x10); // MB_ICONHAND — critical/error sound
        } else {
            MessageBeep(0x30); // MB_ICONEXCLAMATION — warning sound
        }
    }
}

/// Background thread that watches ~/.claude/ directory for credential file changes.
/// Posts WM_CREDENTIAL_CHANGED to the main window when changes are detected.
fn credential_file_watcher(hwnd_raw: usize) {
    use windows::Win32::Storage::FileSystem::{
        FindFirstChangeNotificationW, FindNextChangeNotification, FILE_NOTIFY_CHANGE_LAST_WRITE,
    };
    use windows::Win32::System::Threading::WaitForSingleObject;

    let home = std::env::var("USERPROFILE")
        .or_else(|_| std::env::var("HOME"))
        .unwrap_or_default();
    if home.is_empty() {
        log::warn!("Cannot determine home directory for credential watcher");
        return;
    }

    let watch_dir = std::path::Path::new(&home).join(".claude");
    if !watch_dir.exists() {
        log::debug!("~/.claude/ does not exist, credential watcher not started");
        return;
    }

    let dir_wide: Vec<u16> = watch_dir
        .to_string_lossy()
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();

    unsafe {
        let handle = FindFirstChangeNotificationW(
            PCWSTR(dir_wide.as_ptr()),
            false,
            FILE_NOTIFY_CHANGE_LAST_WRITE,
        );

        let handle = match handle {
            Ok(h) => h,
            Err(e) => {
                log::warn!("FindFirstChangeNotificationW failed: {e}");
                return;
            }
        };

        loop {
            // Wait indefinitely for a change
            let result = WaitForSingleObject(handle, u32::MAX);
            if result.0 != 0 {
                // WAIT_FAILED or timeout
                log::warn!("WaitForSingleObject failed in credential watcher");
                break;
            }

            // Debounce: wait a bit for file writes to settle
            std::thread::sleep(std::time::Duration::from_millis(500));

            let hwnd = HWND(hwnd_raw as *mut _);
            let _ = PostMessageW(hwnd, WM_CREDENTIAL_CHANGED, WPARAM(0), LPARAM(0));

            if FindNextChangeNotification(handle).is_err() {
                log::warn!("FindNextChangeNotification failed");
                break;
            }
        }
    }
}

/// Background thread that monitors network interface changes.
/// Posts WM_NETWORK_CHANGED to the main window when connectivity changes.
fn network_change_monitor(hwnd_raw: usize) {
    use windows::Win32::NetworkManagement::IpHelper::NotifyIpInterfaceChange;
    use windows::Win32::Networking::WinSock::AF_UNSPEC;

    // We use a callback that posts a message to the main window.
    // The callback must be extern "system" and take specific parameters.
    unsafe extern "system" fn callback(
        _caller_context: *const std::ffi::c_void,
        _row: *const windows::Win32::NetworkManagement::IpHelper::MIB_IPINTERFACE_ROW,
        _notification_type: windows::Win32::NetworkManagement::IpHelper::MIB_NOTIFICATION_TYPE,
    ) {
        // Extract hwnd from caller_context
        let hwnd_raw = _caller_context as usize;
        let hwnd = HWND(hwnd_raw as *mut _);
        let _ = PostMessageW(hwnd, WM_NETWORK_CHANGED, WPARAM(0), LPARAM(0));
    }

    unsafe {
        let mut handle = std::mem::zeroed();
        let result = NotifyIpInterfaceChange(
            AF_UNSPEC,
            Some(callback),
            Some(hwnd_raw as *const std::ffi::c_void),
            false,
            &mut handle,
        );

        if result.is_err() {
            log::warn!("NotifyIpInterfaceChange failed: {:?}", result);
            return;
        }

        // Keep this thread alive — the callback needs it.
        // The notification handle is automatically cleaned up when the process exits.
        loop {
            std::thread::sleep(std::time::Duration::from_secs(3600));
        }
    }
}
