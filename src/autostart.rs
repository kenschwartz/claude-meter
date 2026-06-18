use windows::core::PCWSTR;
use windows::Win32::System::Registry::{
    RegCloseKey, RegDeleteValueW, RegOpenKeyExW, RegQueryValueExW, RegSetValueExW,
    HKEY_CURRENT_USER, KEY_READ, KEY_SET_VALUE, REG_SZ,
};

const RUN_KEY: &str = "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run";
const APP_NAME: &str = "ZaiMeter";

pub fn set_autostart(enabled: bool, exe_path: &str) -> Result<(), String> {
    if enabled {
        enable_autostart(exe_path)
    } else {
        disable_autostart()
    }
}

fn enable_autostart(exe_path: &str) -> Result<(), String> {
    let key_wide: Vec<u16> = RUN_KEY.encode_utf16().chain(std::iter::once(0)).collect();
    let name_wide: Vec<u16> = APP_NAME.encode_utf16().chain(std::iter::once(0)).collect();

    // Build value as UTF-16 bytes (REG_SZ)
    let value = format!("\"{}\"", exe_path);
    let value_wide: Vec<u16> = value.encode_utf16().chain(std::iter::once(0)).collect();
    let value_bytes: &[u8] = unsafe {
        std::slice::from_raw_parts(value_wide.as_ptr() as *const u8, value_wide.len() * 2)
    };

    unsafe {
        let mut hkey = windows::Win32::System::Registry::HKEY::default();
        RegOpenKeyExW(
            HKEY_CURRENT_USER,
            PCWSTR(key_wide.as_ptr()),
            0,
            KEY_SET_VALUE,
            &mut hkey,
        )
        .ok()
        .map_err(|e: windows::core::Error| e.to_string())?;

        let result = RegSetValueExW(
            hkey,
            PCWSTR(name_wide.as_ptr()),
            0,
            REG_SZ,
            Some(value_bytes),
        );

        let _ = RegCloseKey(hkey).ok();
        result
            .ok()
            .map_err(|e: windows::core::Error| e.to_string())?;
    }
    Ok(())
}

fn disable_autostart() -> Result<(), String> {
    let key_wide: Vec<u16> = RUN_KEY.encode_utf16().chain(std::iter::once(0)).collect();
    let name_wide: Vec<u16> = APP_NAME.encode_utf16().chain(std::iter::once(0)).collect();

    unsafe {
        let mut hkey = windows::Win32::System::Registry::HKEY::default();
        let open_result = RegOpenKeyExW(
            HKEY_CURRENT_USER,
            PCWSTR(key_wide.as_ptr()),
            0,
            KEY_SET_VALUE,
            &mut hkey,
        );

        if open_result.is_err() {
            return Ok(());
        }

        let _ = RegDeleteValueW(hkey, PCWSTR(name_wide.as_ptr()));
        let _ = RegCloseKey(hkey).ok();
    }
    Ok(())
}

#[allow(dead_code)]
pub fn is_autostart_enabled() -> bool {
    let key_wide: Vec<u16> = RUN_KEY.encode_utf16().chain(std::iter::once(0)).collect();
    let name_wide: Vec<u16> = APP_NAME.encode_utf16().chain(std::iter::once(0)).collect();

    unsafe {
        let mut hkey = windows::Win32::System::Registry::HKEY::default();
        let open_result = RegOpenKeyExW(
            HKEY_CURRENT_USER,
            PCWSTR(key_wide.as_ptr()),
            0,
            KEY_READ,
            &mut hkey,
        );

        if open_result.is_err() {
            return false;
        }

        let mut data_size = 0u32;
        let exists = RegQueryValueExW(
            hkey,
            PCWSTR(name_wide.as_ptr()),
            None,
            None,
            None,
            Some(&mut data_size),
        )
        .is_ok();

        let _ = RegCloseKey(hkey).ok();
        exists
    }
}
