#![cfg_attr(windows, windows_subsystem = "windows")]
#![allow(static_mut_refs)]
#![allow(clippy::too_many_arguments)]

mod config;
mod credentials;
mod db;
mod providers;

#[cfg(windows)]
mod autostart;
#[cfg(windows)]
mod i18n;
#[cfg(windows)]
mod notifications;
#[cfg(windows)]
mod popup;
#[cfg(windows)]
mod theme;
#[cfg(windows)]
mod tray;
#[cfg(windows)]
mod ui;
#[cfg(windows)]
mod updater;
#[cfg(windows)]
mod widget;
#[cfg(windows)]
mod windows_app;
#[cfg(windows)]
pub(crate) use windows_app::APP_STATE;

#[cfg(target_os = "macos")]
mod macos_app;

#[cfg(windows)]
fn main() {
    windows_app::run();
}

#[cfg(target_os = "macos")]
fn main() {
    macos_app::run();
}

#[cfg(not(any(windows, target_os = "macos")))]
fn main() {
    eprintln!("ZaiMeter currently supports Windows and macOS.");
}
