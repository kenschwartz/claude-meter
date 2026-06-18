/// Rendering helpers for the popup window using Direct2D + DirectWrite.
///
/// Replaces the legacy GDI rendering with hardware-accelerated Direct2D:
/// - ID2D1HwndRenderTarget for all drawing (auto double-buffered)
/// - IDWriteTextFormat for high-quality ClearType text
/// - Antialiased rounded rectangles, ellipses, lines
///
/// All coordinates are in DIPs (device-independent pixels, 1 DIP = 1/96 inch).
use crate::i18n::{format_duration, format_reset_target, is_system_24h, seconds_until, I18n};
use crate::providers::claude::{format_metric_name, UsageResponse};
use crate::ui::colors::{colorref_to_d2d, lighten_d2d, ColorRef, ThemeColors};
use std::collections::HashMap;
use windows::core::PCWSTR;
use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::Graphics::Direct2D::Common::{
    D2D1_ALPHA_MODE_PREMULTIPLIED, D2D1_COLOR_F, D2D1_GRADIENT_STOP, D2D1_PIXEL_FORMAT,
    D2D_POINT_2F, D2D_RECT_F, D2D_SIZE_U,
};
use windows::Win32::Graphics::Direct2D::{
    D2D1CreateFactory, ID2D1Factory, ID2D1HwndRenderTarget, D2D1_DRAW_TEXT_OPTIONS_NONE,
    D2D1_ELLIPSE, D2D1_EXTEND_MODE_CLAMP, D2D1_FACTORY_TYPE_SINGLE_THREADED, D2D1_GAMMA_2_2,
    D2D1_HWND_RENDER_TARGET_PROPERTIES, D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES,
    D2D1_PRESENT_OPTIONS_NONE, D2D1_RENDER_TARGET_PROPERTIES, D2D1_ROUNDED_RECT,
};
use windows::Win32::Graphics::DirectWrite::{
    DWriteCreateFactory, IDWriteFactory, IDWriteTextFormat, DWRITE_FACTORY_TYPE_SHARED,
    DWRITE_FONT_STRETCH_NORMAL, DWRITE_FONT_STYLE_NORMAL, DWRITE_FONT_WEIGHT_BOLD,
    DWRITE_FONT_WEIGHT_REGULAR, DWRITE_MEASURING_MODE_NATURAL, DWRITE_PARAGRAPH_ALIGNMENT_CENTER,
    DWRITE_PARAGRAPH_ALIGNMENT_NEAR, DWRITE_TEXT_ALIGNMENT_CENTER, DWRITE_TEXT_ALIGNMENT_LEADING,
    DWRITE_TEXT_ALIGNMENT_TRAILING, DWRITE_WORD_WRAPPING_WRAP,
};
use windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_B8G8R8A8_UNORM;
use windows::Win32::Graphics::Gdi::{GetDC, GetDeviceCaps, ReleaseDC, LOGPIXELSX};

pub const POPUP_WIDTH: i32 = 380;
pub const HEADER_HEIGHT: i32 = 40;
pub const PADDING: i32 = 16;
pub const METRIC_LABEL_H: i32 = 22;
pub const PROGRESS_H: i32 = 14;
pub const RESET_LABEL_H: i32 = 18;
pub const SECTION_GAP: i32 = 14;
pub const ITEM_GAP: i32 = 8;
pub const SEPARATOR_H: i32 = 1;
pub const FOOTER_H: i32 = 38;

// --- HoveredElement enum ---

#[derive(Debug, Clone, PartialEq)]
pub enum HoveredElement {
    None,
    SettingsButton,
    CloseButton,
    RefreshButton,
    CopyButton,
    InstallButton,
    ChatGptLink,
    StatusLink,
    PlanLink,
    BackButton,
    SettingRow(usize),
    ChartBar(usize),
    ChartToggle,
}

/// Draw accessibility overlay pattern on a progress bar fill.
/// - Green (<50%): fine dots
/// - Yellow (50-79%): diagonal stripes ///
/// - Red (>=80%): cross-hatch
unsafe fn draw_accessibility_pattern(
    rt: &ID2D1HwndRenderTarget,
    left: f32,
    top: f32,
    right: f32,
    bottom: f32,
    utilization: f64,
) {
    if right - left < 2.0 {
        return;
    }
    let pattern_color = D2D1_COLOR_F {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 0.35,
    };
    let Ok(brush) = rt.CreateSolidColorBrush(&pattern_color as *const _, None) else {
        return;
    };
    // Push clip to keep patterns inside the bar
    rt.PushAxisAlignedClip(
        &D2D_RECT_F {
            left,
            top,
            right,
            bottom,
        },
        windows::Win32::Graphics::Direct2D::D2D1_ANTIALIAS_MODE_PER_PRIMITIVE,
    );
    let h = bottom - top;
    if utilization >= 80.0 {
        // Cross-hatch: two sets of diagonal lines
        let spacing = h * 0.6;
        let span = right - left + h;
        let n = (span / spacing) as i32 + 1;
        for i in 0..n {
            let offset = left - h + i as f32 * spacing;
            rt.DrawLine(
                D2D_POINT_2F {
                    x: offset,
                    y: bottom,
                },
                D2D_POINT_2F {
                    x: offset + h,
                    y: top,
                },
                &brush,
                1.0,
                None,
            );
            rt.DrawLine(
                D2D_POINT_2F { x: offset, y: top },
                D2D_POINT_2F {
                    x: offset + h,
                    y: bottom,
                },
                &brush,
                1.0,
                None,
            );
        }
    } else if utilization >= 50.0 {
        // Diagonal stripes
        let spacing = h * 0.8;
        let span = right - left + h;
        let n = (span / spacing) as i32 + 1;
        for i in 0..n {
            let offset = left - h + i as f32 * spacing;
            rt.DrawLine(
                D2D_POINT_2F {
                    x: offset,
                    y: bottom,
                },
                D2D_POINT_2F {
                    x: offset + h,
                    y: top,
                },
                &brush,
                1.0,
                None,
            );
        }
    } else {
        // Fine dots for green
        let dot_spacing = h * 0.7;
        let mut dx = left + dot_spacing / 2.0;
        let mut row = 0;
        while dx < right {
            let cy = top + h / 2.0 + if row % 2 == 0 { -h * 0.15 } else { h * 0.15 };
            rt.FillEllipse(
                &D2D1_ELLIPSE {
                    point: D2D_POINT_2F { x: dx, y: cy },
                    radiusX: 1.0,
                    radiusY: 1.0,
                },
                &brush,
            );
            dx += dot_spacing;
            row += 1;
        }
    }
    rt.PopAxisAlignedClip();
}

// --- Text format cache key ---

#[derive(Hash, Eq, PartialEq, Clone)]
struct TextFormatKey {
    size_pt: i32,
    bold: bool,
    h_align: u8, // 0=leading, 1=trailing, 2=center
    v_align: u8, // 0=near, 1=center
}

// --- D2DResources ---

pub struct D2DResources {
    pub factory: ID2D1Factory,
    pub dwrite_factory: IDWriteFactory,
    pub render_target: Option<ID2D1HwndRenderTarget>,
    text_formats: HashMap<TextFormatKey, IDWriteTextFormat>,
}

impl D2DResources {
    pub fn new() -> Result<Self, String> {
        unsafe {
            let factory: ID2D1Factory = D2D1CreateFactory(D2D1_FACTORY_TYPE_SINGLE_THREADED, None)
                .map_err(|e| format!("D2D1CreateFactory failed: {e}"))?;

            let dwrite_factory: IDWriteFactory = DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED)
                .map_err(|e| format!("DWriteCreateFactory failed: {e}"))?;

            Ok(Self {
                factory,
                dwrite_factory,
                render_target: None,
                text_formats: HashMap::new(),
            })
        }
    }

    pub fn ensure_render_target(&mut self, hwnd: HWND) -> Result<(), String> {
        if self.render_target.is_some() {
            return Ok(());
        }
        unsafe {
            let mut rc = RECT::default();
            let _ = windows::Win32::UI::WindowsAndMessaging::GetClientRect(hwnd, &mut rc);

            let size = D2D_SIZE_U {
                width: (rc.right - rc.left).max(1) as u32,
                height: (rc.bottom - rc.top).max(1) as u32,
            };

            let pixel_format = D2D1_PIXEL_FORMAT {
                format: DXGI_FORMAT_B8G8R8A8_UNORM,
                alphaMode: D2D1_ALPHA_MODE_PREMULTIPLIED,
            };

            let rt_props = D2D1_RENDER_TARGET_PROPERTIES {
                pixelFormat: pixel_format,
                dpiX: 96.0,
                dpiY: 96.0,
                ..Default::default()
            };

            let hwnd_props = D2D1_HWND_RENDER_TARGET_PROPERTIES {
                hwnd,
                pixelSize: size,
                presentOptions: D2D1_PRESENT_OPTIONS_NONE,
            };

            let rt = self
                .factory
                .CreateHwndRenderTarget(&rt_props, &hwnd_props)
                .map_err(|e| format!("CreateHwndRenderTarget failed: {e}"))?;

            self.render_target = Some(rt);
        }
        Ok(())
    }

    pub fn resize(&mut self, w: u32, h: u32) {
        if let Some(rt) = self.render_target.as_ref() {
            let size = D2D_SIZE_U {
                width: w.max(1),
                height: h.max(1),
            };
            unsafe {
                let _ = rt.Resize(&size);
            }
        }
    }

    pub fn discard_render_target(&mut self) {
        self.render_target = None;
    }

    /// Release all GPU/COM resources to reclaim memory when popup is hidden.
    pub fn release(&mut self) {
        self.render_target = None;
        self.text_formats.clear();
    }

    fn get_text_format(
        &mut self,
        size_pt: i32,
        bold: bool,
        h_align: u8,
        v_align: u8,
    ) -> &IDWriteTextFormat {
        let key = TextFormatKey {
            size_pt,
            bold,
            h_align,
            v_align,
        };
        if !self.text_formats.contains_key(&key) {
            let weight = if bold {
                DWRITE_FONT_WEIGHT_BOLD
            } else {
                DWRITE_FONT_WEIGHT_REGULAR
            };
            // Convert pt to DIPs: 1pt = 96/72 DIPs
            let font_size = size_pt as f32 * 96.0 / 72.0;
            // Segoe UI Variable (Win11) with fallback to Segoe UI (Win10)
            let font_var = wide("Segoe UI Variable");
            let font_fallback = wide("Segoe UI");
            let locale = wide("en-us");
            let format = unsafe {
                self.dwrite_factory
                    .CreateTextFormat(
                        PCWSTR(font_var.as_ptr()),
                        None,
                        weight,
                        DWRITE_FONT_STYLE_NORMAL,
                        DWRITE_FONT_STRETCH_NORMAL,
                        font_size,
                        PCWSTR(locale.as_ptr()),
                    )
                    .or_else(|_| {
                        self.dwrite_factory.CreateTextFormat(
                            PCWSTR(font_fallback.as_ptr()),
                            None,
                            weight,
                            DWRITE_FONT_STYLE_NORMAL,
                            DWRITE_FONT_STRETCH_NORMAL,
                            font_size,
                            PCWSTR(locale.as_ptr()),
                        )
                    })
                    .expect("CreateTextFormat failed")
            };
            // Set alignment
            unsafe {
                let h = match h_align {
                    1 => DWRITE_TEXT_ALIGNMENT_TRAILING,
                    2 => DWRITE_TEXT_ALIGNMENT_CENTER,
                    _ => DWRITE_TEXT_ALIGNMENT_LEADING,
                };
                let v = match v_align {
                    1 => DWRITE_PARAGRAPH_ALIGNMENT_CENTER,
                    _ => DWRITE_PARAGRAPH_ALIGNMENT_NEAR,
                };
                let _ = format.SetTextAlignment(h);
                let _ = format.SetParagraphAlignment(v);
            }
            self.text_formats.insert(key.clone(), format);
        }
        self.text_formats.get(&key).unwrap()
    }

    /// Get a text format for word-wrapping text
    fn get_text_format_wrap(&mut self, size_pt: i32, bold: bool) -> IDWriteTextFormat {
        let weight = if bold {
            DWRITE_FONT_WEIGHT_BOLD
        } else {
            DWRITE_FONT_WEIGHT_REGULAR
        };
        let font_size = size_pt as f32 * 96.0 / 72.0;
        let font_var = wide("Segoe UI Variable");
        let font_fallback = wide("Segoe UI");
        let locale = wide("en-us");
        let format = unsafe {
            self.dwrite_factory
                .CreateTextFormat(
                    PCWSTR(font_var.as_ptr()),
                    None,
                    weight,
                    DWRITE_FONT_STYLE_NORMAL,
                    DWRITE_FONT_STRETCH_NORMAL,
                    font_size,
                    PCWSTR(locale.as_ptr()),
                )
                .or_else(|_| {
                    self.dwrite_factory.CreateTextFormat(
                        PCWSTR(font_fallback.as_ptr()),
                        None,
                        weight,
                        DWRITE_FONT_STYLE_NORMAL,
                        DWRITE_FONT_STRETCH_NORMAL,
                        font_size,
                        PCWSTR(locale.as_ptr()),
                    )
                })
                .expect("CreateTextFormat failed")
        };
        unsafe {
            let _ = format.SetWordWrapping(DWRITE_WORD_WRAPPING_WRAP);
        }
        format
    }
}

// --- PopupRenderer ---

pub struct PopupRenderer {
    pub dpi_scale: f32,
}

impl PopupRenderer {
    pub fn new(hwnd: HWND) -> Self {
        let dpi_scale = unsafe {
            let hdc = GetDC(hwnd);
            let dpi = GetDeviceCaps(hdc, LOGPIXELSX);
            ReleaseDC(hwnd, hdc);
            dpi as f32 / 96.0
        };
        Self { dpi_scale }
    }

    fn scale(&self, px: i32) -> i32 {
        (px as f32 * self.dpi_scale) as i32
    }

    fn sf(&self, px: i32) -> f32 {
        px as f32 * self.dpi_scale
    }

    /// Calculate the total height needed for the popup in full mode.
    /// Must exactly mirror the layout in draw() to prevent clipping.
    pub fn calculate_height(
        &self,
        usage: &Option<UsageResponse>,
        show_chatgpt: bool,
        compact: bool,
        dashboard_layout: &str,
        hide_extra_usage: bool,
    ) -> i32 {
        let extra_hidden = |u: &UsageResponse| -> usize {
            if hide_extra_usage && u.extra.contains_key("extra_usage") {
                1
            } else {
                0
            }
        };

        if compact {
            let metric_count = usage
                .as_ref()
                .map(|u| u.all_metrics().len() - extra_hidden(u))
                .unwrap_or(1)
                .max(1) as i32;
            return self.scale(
                HEADER_HEIGHT
                    + SEPARATOR_H
                    + PADDING
                    + metric_count * (16 + 8 + ITEM_GAP)
                    + PADDING
                    + SEPARATOR_H
                    + FOOTER_H,
            );
        }

        let mut h = 0;
        h += HEADER_HEIGHT;
        h += SEPARATOR_H;
        h += PADDING;

        match usage {
            None => {
                // warn title(28) + desc(70) + btn(28) + gap(12) + status link(24)
                h += 28 + 70 + 28 + 12 + 24;
            }
            Some(u) => {
                match dashboard_layout {
                    "minimal" => {
                        // header(24) + name(24) + gap(8) + percentage(48) + gap(8) + bar(20) + gap(8) + reset(18) + gap
                        h += 24 + 24 + 8 + 48 + 8 + 20 + 8 + 18 + SECTION_GAP;
                    }
                    "detailed" => {
                        h += 24;
                        let metric_count = (u.all_metrics().len() - extra_hidden(u)) as i32;
                        // Extra 28px per metric for sparkline
                        h += metric_count
                            * (METRIC_LABEL_H
                                + 4
                                + PROGRESS_H
                                + 4
                                + RESET_LABEL_H
                                + 28
                                + SECTION_GAP);
                    }
                    _ => {
                        // "standard"
                        h += 24;
                        let metric_count = (u.all_metrics().len() - extra_hidden(u)) as i32;
                        h += metric_count
                            * (METRIC_LABEL_H + 4 + PROGRESS_H + 4 + RESET_LABEL_H + SECTION_GAP);
                    }
                }
            }
        }

        if show_chatgpt {
            h += SEPARATOR_H + 8 + 24 + 55 + 28;
        }

        h += SEPARATOR_H + PADDING;
        h += 22 + 70 + 4 + 14;
        h += PADDING;
        h += SEPARATOR_H + FOOTER_H;

        self.scale(h)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn draw(
        &self,
        d2d: &mut D2DResources,
        rect: &RECT,
        usage: &Option<UsageResponse>,
        last_updated: &str,
        show_chatgpt: bool,
        compact: bool,
        colors: &ThemeColors,
        i18n: &I18n,
        chart_data: &[f64],
        reset_lines: &[f64],
        chart_mode: u8,
        last_error: &Option<String>,
        hovered: &HoveredElement,
        anim_values: &[f64],
        dashboard_layout: &str,
        rate_of_change: &HashMap<String, f64>,
        hide_extra_usage: bool,
        settings_rect: &mut RECT,
        close_rect: &mut RECT,
        refresh_rect: &mut RECT,
        copy_rect: &mut RECT,
        install_rect: &mut RECT,
        chatgpt_link_rect: &mut RECT,
        status_link_rect: &mut RECT,
        plan_link_rect: &mut RECT,
        chart_rect_out: &mut RECT,
        chart_bar_count_out: &mut usize,
        chart_toggle_rect_out: &mut RECT,
    ) {
        let Some(rt) = d2d.render_target.clone() else {
            return;
        };
        let w = (rect.right - rect.left) as f32;

        unsafe {
            let mut y = 0.0f32;

            // Header
            y = self.draw_header(
                &rt,
                d2d,
                w,
                y,
                colors,
                i18n,
                hovered,
                settings_rect,
                close_rect,
            );

            // Separator
            y = self.draw_separator(&rt, w, y, colors);

            y += self.sf(PADDING);

            if compact {
                y = self.draw_compact_metrics(&rt, d2d, w, y, usage, colors, hide_extra_usage);
            } else {
                match usage {
                    None => {
                        y = self.draw_not_detected(
                            &rt,
                            d2d,
                            w,
                            y,
                            colors,
                            i18n,
                            last_error,
                            hovered,
                            install_rect,
                            status_link_rect,
                        );
                    }
                    Some(u) => match dashboard_layout {
                        "minimal" => {
                            y = self.draw_claude_minimal(
                                &rt,
                                d2d,
                                w,
                                y,
                                u,
                                colors,
                                i18n,
                                anim_values,
                                hovered,
                                status_link_rect,
                                plan_link_rect,
                                hide_extra_usage,
                            );
                        }
                        "detailed" => {
                            y = self.draw_claude_detailed(
                                &rt,
                                d2d,
                                w,
                                y,
                                u,
                                colors,
                                i18n,
                                anim_values,
                                rate_of_change,
                                chart_data,
                                hovered,
                                status_link_rect,
                                plan_link_rect,
                                hide_extra_usage,
                            );
                        }
                        _ => {
                            y = self.draw_claude_section(
                                &rt,
                                d2d,
                                w,
                                y,
                                u,
                                colors,
                                i18n,
                                anim_values,
                                rate_of_change,
                                hovered,
                                status_link_rect,
                                plan_link_rect,
                                hide_extra_usage,
                            );
                        }
                    },
                }

                if show_chatgpt {
                    y = self.draw_separator(&rt, w, y, colors);
                    y += self.sf(8);
                    y = self.draw_chatgpt_section(&rt, d2d, w, y, colors, i18n, chatgpt_link_rect);
                }

                // History chart
                y = self.draw_separator(&rt, w, y, colors);
                y += self.sf(PADDING);
                y = self.draw_chart(
                    &rt,
                    d2d,
                    w,
                    y,
                    chart_data,
                    reset_lines,
                    chart_mode,
                    colors,
                    i18n,
                    hovered,
                    chart_rect_out,
                    chart_bar_count_out,
                    chart_toggle_rect_out,
                );
                y += self.sf(PADDING);
            }

            // Footer
            self.draw_separator(&rt, w, y, colors);
            self.draw_footer(
                &rt,
                d2d,
                w,
                y + self.sf(SEPARATOR_H),
                last_updated,
                colors,
                i18n,
                hovered,
                refresh_rect,
                copy_rect,
                status_link_rect,
            );

            // 1px border
            self.draw_border(&rt, w, (rect.bottom - rect.top) as f32, colors);
        }
    }

    /// Draw a gear/cog icon using D2D primitives within the given rect.
    unsafe fn draw_gear_icon(
        &self,
        rt: &ID2D1HwndRenderTarget,
        rect: D2D_RECT_F,
        color: ColorRef,
        bg_color: ColorRef,
    ) {
        let brush = rt
            .CreateSolidColorBrush(&colorref_to_d2d(color) as *const _, None)
            .unwrap();
        let bg_brush = rt
            .CreateSolidColorBrush(&colorref_to_d2d(bg_color) as *const _, None)
            .unwrap();

        let cx = (rect.left + rect.right) / 2.0;
        let cy = (rect.top + rect.bottom) / 2.0;
        let size = ((rect.right - rect.left).min(rect.bottom - rect.top)) * 0.40;

        let body_r = size * 0.58;
        let tooth_reach = size * 1.05;
        let tooth_w = size * 0.40;
        let hole_r = size * 0.22;
        let num_teeth: i32 = 8;

        // 1) Draw teeth as thick lines radiating from center
        for i in 0..num_teeth {
            let angle = (i as f32) * std::f32::consts::TAU / num_teeth as f32;
            rt.DrawLine(
                D2D_POINT_2F { x: cx, y: cy },
                D2D_POINT_2F {
                    x: cx + angle.cos() * tooth_reach,
                    y: cy + angle.sin() * tooth_reach,
                },
                &brush,
                tooth_w,
                None,
            );
        }

        // 2) Draw filled circle body on top (covers inner portions of teeth)
        rt.FillEllipse(
            &D2D1_ELLIPSE {
                point: D2D_POINT_2F { x: cx, y: cy },
                radiusX: body_r,
                radiusY: body_r,
            },
            &brush,
        );

        // 3) Cut out center hole with background color
        rt.FillEllipse(
            &D2D1_ELLIPSE {
                point: D2D_POINT_2F { x: cx, y: cy },
                radiusX: hole_r,
                radiusY: hole_r,
            },
            &bg_brush,
        );
    }

    /// Draw an X close icon using D2D lines
    unsafe fn draw_close_icon(
        &self,
        rt: &ID2D1HwndRenderTarget,
        rect: D2D_RECT_F,
        color: ColorRef,
    ) {
        let brush = rt
            .CreateSolidColorBrush(&colorref_to_d2d(color) as *const _, None)
            .unwrap();
        let cx = (rect.left + rect.right) / 2.0;
        let cy = (rect.top + rect.bottom) / 2.0;
        let half = ((rect.right - rect.left).min(rect.bottom - rect.top)) * 0.18;
        let stroke = 1.8;
        rt.DrawLine(
            D2D_POINT_2F {
                x: cx - half,
                y: cy - half,
            },
            D2D_POINT_2F {
                x: cx + half,
                y: cy + half,
            },
            &brush,
            stroke,
            None,
        );
        rt.DrawLine(
            D2D_POINT_2F {
                x: cx + half,
                y: cy - half,
            },
            D2D_POINT_2F {
                x: cx - half,
                y: cy + half,
            },
            &brush,
            stroke,
            None,
        );
    }

    #[allow(clippy::too_many_arguments)]
    unsafe fn draw_header(
        &self,
        rt: &ID2D1HwndRenderTarget,
        d2d: &mut D2DResources,
        w: f32,
        y: f32,
        colors: &ThemeColors,
        i18n: &I18n,
        hovered: &HoveredElement,
        settings_rect: &mut RECT,
        close_rect: &mut RECT,
    ) -> f32 {
        let h = self.sf(HEADER_HEIGHT);
        let pad = self.sf(PADDING);

        // Header background
        let surface_brush = rt
            .CreateSolidColorBrush(&colorref_to_d2d(colors.surface) as *const _, None)
            .unwrap();
        rt.FillRectangle(
            &D2D_RECT_F {
                left: 0.0,
                top: y,
                right: w,
                bottom: y + h,
            },
            &surface_brush,
        );

        // Title "ZaiMeter"
        let title_text = wide(i18n.t("ZaiMeter"));
        let title_format = d2d.get_text_format(14, true, 0, 1).clone();
        let title_brush = rt
            .CreateSolidColorBrush(&colorref_to_d2d(colors.text_primary) as *const _, None)
            .unwrap();
        rt.DrawText(
            &title_text[..title_text.len() - 1],
            &title_format,
            &D2D_RECT_F {
                left: pad,
                top: y,
                right: w - self.sf(80),
                bottom: y + h,
            },
            &title_brush,
            D2D1_DRAW_TEXT_OPTIONS_NONE,
            DWRITE_MEASURING_MODE_NATURAL,
        );

        // ⚙ button
        let btn_size = self.sf(28);
        let btn_y = y + (h - btn_size) / 2.0;
        let settings_r = D2D_RECT_F {
            left: w - self.sf(64),
            top: btn_y,
            right: w - self.sf(36),
            bottom: btn_y + btn_size,
        };
        *settings_rect = to_win32_rect(&settings_r);

        // Hover highlight for settings button
        if matches!(hovered, HoveredElement::SettingsButton) {
            let hover_brush = rt
                .CreateSolidColorBrush(&colorref_to_d2d(colors.hover) as *const _, None)
                .unwrap();
            rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: settings_r,
                    radiusX: 4.0,
                    radiusY: 4.0,
                },
                &hover_brush,
            );
        }

        let settings_hovered = matches!(hovered, HoveredElement::SettingsButton);
        let gear_bg = if settings_hovered {
            colors.hover
        } else {
            colors.background
        };
        let gear_color = if settings_hovered {
            colors.accent
        } else {
            colors.text_secondary
        };
        self.draw_gear_icon(rt, settings_r, gear_color, gear_bg);

        // × button
        let close_r = D2D_RECT_F {
            left: w - self.sf(32),
            top: btn_y,
            right: w - self.sf(4),
            bottom: btn_y + btn_size,
        };
        *close_rect = to_win32_rect(&close_r);

        // Hover highlight for close button
        if matches!(hovered, HoveredElement::CloseButton) {
            let hover_brush = rt
                .CreateSolidColorBrush(&colorref_to_d2d(colors.hover) as *const _, None)
                .unwrap();
            rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: close_r,
                    radiusX: 4.0,
                    radiusY: 4.0,
                },
                &hover_brush,
            );
        }

        let close_color = if matches!(hovered, HoveredElement::CloseButton) {
            colors.accent
        } else {
            colors.text_secondary
        };
        self.draw_close_icon(rt, close_r, close_color);

        y + h
    }

    unsafe fn draw_separator(
        &self,
        rt: &ID2D1HwndRenderTarget,
        w: f32,
        y: f32,
        colors: &ThemeColors,
    ) -> f32 {
        let pad = self.sf(PADDING);
        let mut sep_color = colorref_to_d2d(colors.separator);
        sep_color.a = 0.4; // Subtle semi-transparent separator
        let brush = rt
            .CreateSolidColorBrush(&sep_color as *const _, None)
            .unwrap();
        rt.DrawLine(
            D2D_POINT_2F { x: pad, y },
            D2D_POINT_2F { x: w - pad, y },
            &brush,
            0.5,
            None,
        );
        y + self.sf(SEPARATOR_H)
    }

    #[allow(clippy::too_many_arguments)]
    unsafe fn draw_claude_section(
        &self,
        rt: &ID2D1HwndRenderTarget,
        d2d: &mut D2DResources,
        w: f32,
        mut y: f32,
        usage: &UsageResponse,
        colors: &ThemeColors,
        i18n: &I18n,
        anim_values: &[f64],
        rate_of_change: &HashMap<String, f64>,
        hovered: &HoveredElement,
        status_link_rect: &mut RECT,
        plan_link_rect: &mut RECT,
        hide_extra_usage: bool,
    ) -> f32 {
        // Section header
        y = self.draw_section_header(
            rt,
            d2d,
            w,
            y,
            usage,
            colors,
            i18n,
            hovered,
            status_link_rect,
            plan_link_rect,
        );

        for (i, (key, metric)) in usage.all_metrics().iter().enumerate() {
            if hide_extra_usage && key == "extra_usage" {
                continue;
            }
            let util = if i < anim_values.len() {
                anim_values[i].max(0.0)
            } else {
                metric.utilization
            };
            let rate = rate_of_change.get(key).copied();
            y = self.draw_metric(
                rt,
                d2d,
                w,
                y,
                key,
                util,
                metric.resets_at.as_deref(),
                colors,
                i18n,
                rate,
            );
            y += self.sf(SECTION_GAP);
        }

        y
    }

    #[allow(clippy::too_many_arguments)]
    unsafe fn draw_metric(
        &self,
        rt: &ID2D1HwndRenderTarget,
        d2d: &mut D2DResources,
        w: f32,
        mut y: f32,
        key: &str,
        utilization: f64,
        resets_at: Option<&str>,
        colors: &ThemeColors,
        i18n: &I18n,
        rate: Option<f64>,
    ) -> f32 {
        let pad = self.sf(PADDING);
        let content_w = w - pad * 2.0;
        let bar_h = self.sf(PROGRESS_H);
        let radius = bar_h / 2.0;

        // Label + percentage + rate arrow on same line
        let metric_name_str = format_metric_name(key);
        let display_name = i18n.t(&metric_name_str);
        let pct_str = format!("{:.0}%", utilization);
        let pct_area_w = self.sf(62);

        // Rate of change arrow
        let (arrow_str, arrow_color) = match rate {
            Some(r) if r > 10.0 => ("\u{2191}", colorref_to_d2d(colors.red)), // ↑
            Some(r) if r > 2.0 => ("\u{2197}", colorref_to_d2d(colors.red)),  // ↗
            Some(r) if r >= -2.0 => ("\u{2192}", colorref_to_d2d(colors.text_secondary)), // →
            Some(r) if r >= -10.0 => ("\u{2198}", colorref_to_d2d(colors.green)), // ↘
            Some(_) => ("\u{2193}", colorref_to_d2d(colors.green)),           // ↓
            None => ("", colorref_to_d2d(colors.text_secondary)),
        };

        // Label (left)
        let label_text = wide(display_name);
        let label_format = d2d.get_text_format(12, false, 0, 1).clone();
        let label_brush = rt
            .CreateSolidColorBrush(&colorref_to_d2d(colors.text_primary) as *const _, None)
            .unwrap();
        rt.DrawText(
            &label_text[..label_text.len() - 1],
            &label_format,
            &D2D_RECT_F {
                left: pad,
                top: y,
                right: w - pad - pct_area_w,
                bottom: y + self.sf(METRIC_LABEL_H),
            },
            &label_brush,
            D2D1_DRAW_TEXT_OPTIONS_NONE,
            DWRITE_MEASURING_MODE_NATURAL,
        );

        // Percentage (right, bold, colored)
        let pct_text = wide(&pct_str);
        let pct_format = d2d.get_text_format(12, true, 1, 1).clone();
        let pct_color = colorref_to_d2d(colors.progress_color(utilization));
        let pct_brush = rt
            .CreateSolidColorBrush(&pct_color as *const _, None)
            .unwrap();
        let pct_right = if arrow_str.is_empty() {
            w - pad
        } else {
            w - pad - self.sf(16)
        };
        rt.DrawText(
            &pct_text[..pct_text.len() - 1],
            &pct_format,
            &D2D_RECT_F {
                left: w - pad - pct_area_w,
                top: y,
                right: pct_right,
                bottom: y + self.sf(METRIC_LABEL_H),
            },
            &pct_brush,
            D2D1_DRAW_TEXT_OPTIONS_NONE,
            DWRITE_MEASURING_MODE_NATURAL,
        );

        // Rate arrow (far right)
        if !arrow_str.is_empty() {
            let arrow_text = wide(arrow_str);
            let arrow_format = d2d.get_text_format(11, false, 1, 1).clone();
            let arrow_brush = rt
                .CreateSolidColorBrush(&arrow_color as *const _, None)
                .unwrap();
            rt.DrawText(
                &arrow_text[..arrow_text.len() - 1],
                &arrow_format,
                &D2D_RECT_F {
                    left: w - pad - self.sf(16),
                    top: y,
                    right: w - pad,
                    bottom: y + self.sf(METRIC_LABEL_H),
                },
                &arrow_brush,
                D2D1_DRAW_TEXT_OPTIONS_NONE,
                DWRITE_MEASURING_MODE_NATURAL,
            );
        }

        y += self.sf(METRIC_LABEL_H + 4);

        // Progress bar background (rounded)
        let bg_brush = rt
            .CreateSolidColorBrush(&colorref_to_d2d(colors.progress_bg) as *const _, None)
            .unwrap();
        rt.FillRoundedRectangle(
            &D2D1_ROUNDED_RECT {
                rect: D2D_RECT_F {
                    left: pad,
                    top: y,
                    right: pad + content_w,
                    bottom: y + bar_h,
                },
                radiusX: radius,
                radiusY: radius,
            },
            &bg_brush,
        );

        // Progress bar fill (rounded, gradient)
        let fill_w = (content_w * utilization as f32 / 100.0)
            .max(0.0)
            .min(content_w);
        if fill_w > 0.5 {
            use crate::ui::colors::{GRADIENT_HIGH, GRADIENT_LOW, GRADIENT_MID};

            let fill_rect = D2D1_ROUNDED_RECT {
                rect: D2D_RECT_F {
                    left: pad,
                    top: y,
                    right: pad + fill_w,
                    bottom: y + bar_h,
                },
                radiusX: radius.min(fill_w / 2.0),
                radiusY: radius,
            };

            if colors.has_overrides {
                // Fallback: original 2-stop gradient for custom color themes
                let fill_color = colorref_to_d2d(colors.progress_color(utilization));
                let light_color = lighten_d2d(&fill_color, 0.35);
                let stops = [
                    D2D1_GRADIENT_STOP {
                        position: 0.0,
                        color: fill_color,
                    },
                    D2D1_GRADIENT_STOP {
                        position: 1.0,
                        color: light_color,
                    },
                ];
                if let Ok(stop_coll) =
                    rt.CreateGradientStopCollection(&stops, D2D1_GAMMA_2_2, D2D1_EXTEND_MODE_CLAMP)
                {
                    let grad_props = D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES {
                        startPoint: D2D_POINT_2F { x: pad, y: 0.0 },
                        endPoint: D2D_POINT_2F {
                            x: pad + fill_w,
                            y: 0.0,
                        },
                    };
                    if let Ok(grad_brush) =
                        rt.CreateLinearGradientBrush(&grad_props, None, &stop_coll)
                    {
                        rt.FillRoundedRectangle(&fill_rect, &grad_brush);
                    }
                }
            } else {
                // Full-spectrum gradient: green → amber → coral across 0-100%
                let stops = [
                    D2D1_GRADIENT_STOP {
                        position: 0.0,
                        color: GRADIENT_LOW,
                    },
                    D2D1_GRADIENT_STOP {
                        position: 0.5,
                        color: GRADIENT_MID,
                    },
                    D2D1_GRADIENT_STOP {
                        position: 1.0,
                        color: GRADIENT_HIGH,
                    },
                ];
                if let Ok(stop_coll) =
                    rt.CreateGradientStopCollection(&stops, D2D1_GAMMA_2_2, D2D1_EXTEND_MODE_CLAMP)
                {
                    // Map gradient across full bar width so color reflects position
                    let grad_props = D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES {
                        startPoint: D2D_POINT_2F { x: pad, y: 0.0 },
                        endPoint: D2D_POINT_2F {
                            x: pad + content_w,
                            y: 0.0,
                        },
                    };
                    if let Ok(grad_brush) =
                        rt.CreateLinearGradientBrush(&grad_props, None, &stop_coll)
                    {
                        rt.FillRoundedRectangle(&fill_rect, &grad_brush);
                    }
                }
            }

            // Accessibility overlay pattern
            if crate::APP_STATE
                .as_ref()
                .is_some_and(|s| s.config_mgr.config.accessibility_patterns)
            {
                draw_accessibility_pattern(rt, pad, y, pad + fill_w, y + bar_h, utilization);
            }
        }

        y += bar_h + self.sf(4);

        // Reset time
        if let Some(reset_str) = resets_at {
            let reset_text = if let Some(secs) = seconds_until(reset_str) {
                if secs > 0 {
                    let duration = format_duration(secs);
                    let target = format_reset_target(reset_str).unwrap_or_default();
                    format!("{} {} {}", i18n.t("resets in"), duration, target)
                } else {
                    "resetting soon".to_string()
                }
            } else {
                String::new()
            };

            if !reset_text.is_empty() {
                let text = wide(&reset_text);
                let format = d2d.get_text_format(10, false, 0, 1).clone();
                let brush = rt
                    .CreateSolidColorBrush(
                        &colorref_to_d2d(colors.text_secondary) as *const _,
                        None,
                    )
                    .unwrap();
                rt.DrawText(
                    &text[..text.len() - 1],
                    &format,
                    &D2D_RECT_F {
                        left: pad,
                        top: y,
                        right: w - pad,
                        bottom: y + self.sf(RESET_LABEL_H),
                    },
                    &brush,
                    D2D1_DRAW_TEXT_OPTIONS_NONE,
                    DWRITE_MEASURING_MODE_NATURAL,
                );
                y += self.sf(RESET_LABEL_H);
            }
        }

        y
    }

    /// Minimal layout: shows only the highest-utilization metric, large and centered.
    #[allow(clippy::too_many_arguments)]
    unsafe fn draw_claude_minimal(
        &self,
        rt: &ID2D1HwndRenderTarget,
        d2d: &mut D2DResources,
        w: f32,
        mut y: f32,
        usage: &UsageResponse,
        colors: &ThemeColors,
        i18n: &I18n,
        anim_values: &[f64],
        hovered: &HoveredElement,
        status_link_rect: &mut RECT,
        plan_link_rect: &mut RECT,
        hide_extra_usage: bool,
    ) -> f32 {
        let pad = self.sf(PADDING);
        let content_w = w - pad * 2.0;

        // Section header (same as standard)
        y = self.draw_section_header(
            rt,
            d2d,
            w,
            y,
            usage,
            colors,
            i18n,
            hovered,
            status_link_rect,
            plan_link_rect,
        );

        // Find highest utilization metric
        let all = usage.all_metrics();
        let metrics: Vec<_> = if hide_extra_usage {
            all.into_iter()
                .filter(|(k, _)| k != "extra_usage")
                .collect()
        } else {
            all
        };
        let fallback_key = String::new();
        let fallback_metric = crate::providers::claude::UsageMetric {
            utilization: 0.0,
            resets_at: None,
        };
        let fallback_pair = (fallback_key, &fallback_metric);
        let (best_idx, (best_key, best_metric)) = metrics
            .iter()
            .enumerate()
            .max_by(|(_, (_, a)), (_, (_, b))| {
                a.utilization
                    .partial_cmp(&b.utilization)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .unwrap_or((0, &fallback_pair));

        let util = if best_idx < anim_values.len() {
            anim_values[best_idx].max(0.0)
        } else {
            best_metric.utilization
        };

        // Metric name (centered, 14pt)
        let metric_name_formatted = format_metric_name(best_key);
        let name = i18n.t(&metric_name_formatted);
        let name_text = wide(name);
        let name_format = d2d.get_text_format(14, true, 2, 1).clone();
        let name_brush = rt
            .CreateSolidColorBrush(&colorref_to_d2d(colors.text_secondary) as *const _, None)
            .unwrap();
        rt.DrawText(
            &name_text[..name_text.len() - 1],
            &name_format,
            &D2D_RECT_F {
                left: pad,
                top: y,
                right: w - pad,
                bottom: y + self.sf(24),
            },
            &name_brush,
            D2D1_DRAW_TEXT_OPTIONS_NONE,
            DWRITE_MEASURING_MODE_NATURAL,
        );
        y += self.sf(24 + 8);

        // Large percentage (centered, 36pt bold)
        let pct_str = format!("{:.0}%", util);
        let pct_text = wide(&pct_str);
        let pct_format = d2d.get_text_format(36, true, 2, 1).clone();
        let pct_color = colorref_to_d2d(colors.progress_color(util));
        let pct_brush = rt
            .CreateSolidColorBrush(&pct_color as *const _, None)
            .unwrap();
        rt.DrawText(
            &pct_text[..pct_text.len() - 1],
            &pct_format,
            &D2D_RECT_F {
                left: pad,
                top: y,
                right: w - pad,
                bottom: y + self.sf(48),
            },
            &pct_brush,
            D2D1_DRAW_TEXT_OPTIONS_NONE,
            DWRITE_MEASURING_MODE_NATURAL,
        );
        y += self.sf(48 + 8);

        // Wide progress bar (20px tall)
        let bar_h = self.sf(20);
        let radius = bar_h / 2.0;

        let bg_brush = rt
            .CreateSolidColorBrush(&colorref_to_d2d(colors.progress_bg) as *const _, None)
            .unwrap();
        rt.FillRoundedRectangle(
            &D2D1_ROUNDED_RECT {
                rect: D2D_RECT_F {
                    left: pad,
                    top: y,
                    right: pad + content_w,
                    bottom: y + bar_h,
                },
                radiusX: radius,
                radiusY: radius,
            },
            &bg_brush,
        );

        let fill_w = (content_w * util as f32 / 100.0).max(0.0).min(content_w);
        if fill_w > 0.5 {
            use crate::ui::colors::{GRADIENT_HIGH, GRADIENT_LOW, GRADIENT_MID};
            let fill_rect = D2D1_ROUNDED_RECT {
                rect: D2D_RECT_F {
                    left: pad,
                    top: y,
                    right: pad + fill_w,
                    bottom: y + bar_h,
                },
                radiusX: radius.min(fill_w / 2.0),
                radiusY: radius,
            };
            let stops = [
                D2D1_GRADIENT_STOP {
                    position: 0.0,
                    color: GRADIENT_LOW,
                },
                D2D1_GRADIENT_STOP {
                    position: 0.5,
                    color: GRADIENT_MID,
                },
                D2D1_GRADIENT_STOP {
                    position: 1.0,
                    color: GRADIENT_HIGH,
                },
            ];
            if let Ok(stop_coll) =
                rt.CreateGradientStopCollection(&stops, D2D1_GAMMA_2_2, D2D1_EXTEND_MODE_CLAMP)
            {
                let grad_props = D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES {
                    startPoint: D2D_POINT_2F { x: pad, y: 0.0 },
                    endPoint: D2D_POINT_2F {
                        x: pad + content_w,
                        y: 0.0,
                    },
                };
                if let Ok(grad_brush) = rt.CreateLinearGradientBrush(&grad_props, None, &stop_coll)
                {
                    rt.FillRoundedRectangle(&fill_rect, &grad_brush);
                }
            }
        }
        y += bar_h + self.sf(8);

        // Reset time
        if let Some(reset_str) = best_metric.resets_at.as_deref() {
            if let Some(secs) = seconds_until(reset_str) {
                if secs > 0 {
                    let duration = format_duration(secs);
                    let target = format_reset_target(reset_str).unwrap_or_default();
                    let reset_text = format!("{} {} {}", i18n.t("resets in"), duration, target);
                    let text = wide(&reset_text);
                    let format = d2d.get_text_format(11, false, 2, 1).clone();
                    let brush = rt
                        .CreateSolidColorBrush(
                            &colorref_to_d2d(colors.text_secondary) as *const _,
                            None,
                        )
                        .unwrap();
                    rt.DrawText(
                        &text[..text.len() - 1],
                        &format,
                        &D2D_RECT_F {
                            left: pad,
                            top: y,
                            right: w - pad,
                            bottom: y + self.sf(18),
                        },
                        &brush,
                        D2D1_DRAW_TEXT_OPTIONS_NONE,
                        DWRITE_MEASURING_MODE_NATURAL,
                    );
                    y += self.sf(18);
                }
            }
        }

        y += self.sf(SECTION_GAP);
        y
    }

    /// Detailed layout: standard metrics + mini sparkline under each metric.
    #[allow(clippy::too_many_arguments)]
    unsafe fn draw_claude_detailed(
        &self,
        rt: &ID2D1HwndRenderTarget,
        d2d: &mut D2DResources,
        w: f32,
        mut y: f32,
        usage: &UsageResponse,
        colors: &ThemeColors,
        i18n: &I18n,
        anim_values: &[f64],
        rate_of_change: &HashMap<String, f64>,
        chart_data: &[f64],
        hovered: &HoveredElement,
        status_link_rect: &mut RECT,
        plan_link_rect: &mut RECT,
        hide_extra_usage: bool,
    ) -> f32 {
        // Section header
        y = self.draw_section_header(
            rt,
            d2d,
            w,
            y,
            usage,
            colors,
            i18n,
            hovered,
            status_link_rect,
            plan_link_rect,
        );

        let pad = self.sf(PADDING);

        for (i, (key, metric)) in usage.all_metrics().iter().enumerate() {
            if hide_extra_usage && key == "extra_usage" {
                continue;
            }
            let util = if i < anim_values.len() {
                anim_values[i].max(0.0)
            } else {
                metric.utilization
            };
            let rate = rate_of_change.get(key).copied();
            y = self.draw_metric(
                rt,
                d2d,
                w,
                y,
                key,
                util,
                metric.resets_at.as_deref(),
                colors,
                i18n,
                rate,
            );

            // Mini sparkline for five_hour metric (we have chart_data for it)
            if key == "five_hour" && !chart_data.is_empty() {
                let spark_w = w - pad * 2.0;
                let spark_h = self.sf(20);
                let max_val = chart_data.iter().cloned().fold(1.0_f64, f64::max);

                // Background
                let spark_bg = rt
                    .CreateSolidColorBrush(&colorref_to_d2d(colors.surface) as *const _, None)
                    .unwrap();
                rt.FillRoundedRectangle(
                    &D2D1_ROUNDED_RECT {
                        rect: D2D_RECT_F {
                            left: pad,
                            top: y,
                            right: pad + spark_w,
                            bottom: y + spark_h,
                        },
                        radiusX: 3.0,
                        radiusY: 3.0,
                    },
                    &spark_bg,
                );

                // Draw sparkline bars
                let bar_w = spark_w / chart_data.len() as f32;
                let gap = 1.0_f32.max(bar_w / 6.0);
                let spark_brush = rt
                    .CreateSolidColorBrush(&colorref_to_d2d(colors.accent) as *const _, None)
                    .unwrap();
                for (j, &val) in chart_data.iter().enumerate() {
                    if val > 0.0 {
                        let bh = (val / max_val) as f32 * (spark_h - 4.0);
                        let bx = pad + j as f32 * bar_w + gap / 2.0;
                        rt.FillRectangle(
                            &D2D_RECT_F {
                                left: bx,
                                top: y + spark_h - 2.0 - bh,
                                right: bx + bar_w - gap,
                                bottom: y + spark_h - 2.0,
                            },
                            &spark_brush,
                        );
                    }
                }
                y += spark_h + self.sf(4);
            }

            y += self.sf(SECTION_GAP);
        }

        y
    }

    /// Draw the section header ("☁ CLAUDE · Plan") shared between layouts.
    #[allow(clippy::too_many_arguments)]
    unsafe fn draw_section_header(
        &self,
        rt: &ID2D1HwndRenderTarget,
        d2d: &mut D2DResources,
        w: f32,
        y: f32,
        usage: &UsageResponse,
        colors: &ThemeColors,
        i18n: &I18n,
        hovered: &HoveredElement,
        status_link_rect: &mut RECT,
        plan_link_rect: &mut RECT,
    ) -> f32 {
        let pad = self.sf(PADDING);
        let detected = usage.detected_plan();
        let plan = i18n.t(&detected);
        let prefix = format!("\u{2601} {} \u{00B7} {} ", i18n.t("CLAUDE"), i18n.t("Plan"),);

        // Draw prefix (non-clickable)
        let prefix_text = wide(&prefix);
        let format = d2d.get_text_format(12, true, 0, 1).clone();
        let brush = rt
            .CreateSolidColorBrush(&colorref_to_d2d(colors.text_secondary) as *const _, None)
            .unwrap();
        let header_rect = D2D_RECT_F {
            left: pad,
            top: y,
            right: w - pad - self.sf(60),
            bottom: y + self.sf(20),
        };
        rt.DrawText(
            &prefix_text[..prefix_text.len() - 1],
            &format,
            &header_rect,
            &brush,
            D2D1_DRAW_TEXT_OPTIONS_NONE,
            DWRITE_MEASURING_MODE_NATURAL,
        );

        // Measure prefix width to position the plan name link
        let dw_factory = &d2d.dwrite_factory;
        let layout = dw_factory
            .CreateTextLayout(&prefix_text[..prefix_text.len() - 1], &format, 999.0, 20.0)
            .ok();
        let prefix_w = layout
            .and_then(|l| {
                let mut metrics =
                    windows::Win32::Graphics::DirectWrite::DWRITE_TEXT_METRICS::default();
                l.GetMetrics(&mut metrics).ok()?;
                // Keep the explicit space after "Plan" when positioning the clickable plan link.
                Some(metrics.widthIncludingTrailingWhitespace)
            })
            .unwrap_or(self.sf(100));

        // Draw plan name as a colored badge (pill shape)
        let is_plan_hovered = matches!(hovered, HoveredElement::PlanLink);
        let plan_text = wide(plan);

        // Measure plan text width (get format first, then measure via factory)
        let plan_format = d2d.get_text_format(10, true, 0, 1).clone();
        let plan_text_w = d2d
            .dwrite_factory
            .CreateTextLayout(&plan_text[..plan_text.len() - 1], &plan_format, 200.0, 20.0)
            .ok()
            .and_then(|l| {
                let mut m = windows::Win32::Graphics::DirectWrite::DWRITE_TEXT_METRICS::default();
                l.GetMetrics(&mut m).ok()?;
                Some(m.widthIncludingTrailingWhitespace)
            })
            .unwrap_or(self.sf(30));

        let badge_h = self.sf(16);
        let badge_pad = self.sf(6);
        let badge_w = plan_text_w + badge_pad * 2.0;
        let badge_y = y + (self.sf(20) - badge_h) / 2.0;
        let badge_rect = D2D_RECT_F {
            left: pad + prefix_w,
            top: badge_y,
            right: pad + prefix_w + badge_w,
            bottom: badge_y + badge_h,
        };

        // Badge background color based on plan
        let detected_lower = detected.to_lowercase();
        let badge_bg = if detected_lower.contains("20x") {
            D2D1_COLOR_F {
                r: 0.85,
                g: 0.65,
                b: 0.13,
                a: if is_plan_hovered { 0.85 } else { 1.0 },
            }
        } else if detected_lower.contains("5x") {
            D2D1_COLOR_F {
                r: 0.58,
                g: 0.34,
                b: 0.80,
                a: if is_plan_hovered { 0.85 } else { 1.0 },
            }
        } else if detected_lower.starts_with("max") {
            D2D1_COLOR_F {
                r: 0.24,
                g: 0.47,
                b: 0.85,
                a: if is_plan_hovered { 0.85 } else { 1.0 },
            }
        } else if detected_lower.starts_with("pro") {
            D2D1_COLOR_F {
                r: 0.30,
                g: 0.69,
                b: 0.31,
                a: if is_plan_hovered { 0.85 } else { 1.0 },
            }
        } else {
            D2D1_COLOR_F {
                r: 0.45,
                g: 0.45,
                b: 0.50,
                a: if is_plan_hovered { 0.85 } else { 1.0 },
            }
        };

        let badge_brush = rt.CreateSolidColorBrush(&badge_bg, None).unwrap();
        let rounded = D2D1_ROUNDED_RECT {
            rect: badge_rect,
            radiusX: self.sf(4),
            radiusY: self.sf(4),
        };
        rt.FillRoundedRectangle(&rounded, &badge_brush);

        // Badge text (white)
        let text_color = D2D1_COLOR_F {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        };
        let text_brush = rt.CreateSolidColorBrush(&text_color, None).unwrap();
        let text_rect = D2D_RECT_F {
            left: badge_rect.left + badge_pad,
            top: badge_rect.top,
            right: badge_rect.right - badge_pad,
            bottom: badge_rect.bottom,
        };
        // Center text vertically in badge (v_align=1 is PARAGRAPH_ALIGNMENT_CENTER)
        let centered_format = d2d.get_text_format(10, true, 0, 1).clone();
        rt.DrawText(
            &plan_text[..plan_text.len() - 1],
            &centered_format,
            &text_rect,
            &text_brush,
            D2D1_DRAW_TEXT_OPTIONS_NONE,
            DWRITE_MEASURING_MODE_NATURAL,
        );

        // Clickable rect covers the whole badge
        let click_rect = D2D_RECT_F {
            left: pad + prefix_w,
            top: y,
            right: pad + prefix_w + badge_w,
            bottom: y + self.sf(20),
        };
        *plan_link_rect = to_win32_rect(&click_rect);

        // "Status ↗" link
        let status_str = format!("{} \u{2197}", i18n.t("Status"));
        let status_text = wide(&status_str);
        let status_format = d2d.get_text_format(10, false, 1, 1).clone();
        let is_status_hovered = matches!(hovered, HoveredElement::StatusLink);
        let status_color = if is_status_hovered {
            lighten_d2d(&colorref_to_d2d(colors.accent), 0.3)
        } else {
            colorref_to_d2d(colors.accent)
        };
        let status_brush = rt
            .CreateSolidColorBrush(&status_color as *const _, None)
            .unwrap();
        let sr = D2D_RECT_F {
            left: w - pad - self.sf(56),
            top: y,
            right: w - pad,
            bottom: y + self.sf(20),
        };
        *status_link_rect = to_win32_rect(&sr);
        rt.DrawText(
            &status_text[..status_text.len() - 1],
            &status_format,
            &sr,
            &status_brush,
            D2D1_DRAW_TEXT_OPTIONS_NONE,
            DWRITE_MEASURING_MODE_NATURAL,
        );

        y + self.sf(24)
    }

    unsafe fn draw_compact_metrics(
        &self,
        rt: &ID2D1HwndRenderTarget,
        d2d: &mut D2DResources,
        w: f32,
        mut y: f32,
        usage: &Option<UsageResponse>,
        colors: &ThemeColors,
        hide_extra_usage: bool,
    ) -> f32 {
        let pad = self.sf(PADDING);
        let content_w = w - pad * 2.0;

        let metrics: Vec<(String, f64)> = match usage {
            Some(u) => u
                .all_metrics()
                .iter()
                .filter(|(k, _)| !hide_extra_usage || k != "extra_usage")
                .map(|(k, m)| (format_metric_name(k), m.utilization))
                .collect(),
            None => vec![("No data".to_string(), 0.0)],
        };

        for (name, utilization) in &metrics {
            // Label
            let label_text = wide(name);
            let label_format = d2d.get_text_format(11, false, 0, 1).clone();
            let label_brush = rt
                .CreateSolidColorBrush(&colorref_to_d2d(colors.text_primary) as *const _, None)
                .unwrap();
            rt.DrawText(
                &label_text[..label_text.len() - 1],
                &label_format,
                &D2D_RECT_F {
                    left: pad,
                    top: y,
                    right: w - pad - self.sf(35),
                    bottom: y + self.sf(16),
                },
                &label_brush,
                D2D1_DRAW_TEXT_OPTIONS_NONE,
                DWRITE_MEASURING_MODE_NATURAL,
            );

            // Percentage (right)
            let pct = format!("{:.0}%", utilization);
            let pct_text = wide(&pct);
            let pct_format = d2d.get_text_format(11, false, 1, 1).clone();
            let pct_color = colorref_to_d2d(colors.progress_color(*utilization));
            let pct_brush = rt
                .CreateSolidColorBrush(&pct_color as *const _, None)
                .unwrap();
            rt.DrawText(
                &pct_text[..pct_text.len() - 1],
                &pct_format,
                &D2D_RECT_F {
                    left: w - pad - self.sf(35),
                    top: y,
                    right: w - pad,
                    bottom: y + self.sf(16),
                },
                &pct_brush,
                D2D1_DRAW_TEXT_OPTIONS_NONE,
                DWRITE_MEASURING_MODE_NATURAL,
            );
            y += self.sf(16);

            // Progress bar (flat)
            let bar_h = self.sf(8);
            let bg_brush = rt
                .CreateSolidColorBrush(&colorref_to_d2d(colors.progress_bg) as *const _, None)
                .unwrap();
            rt.FillRectangle(
                &D2D_RECT_F {
                    left: pad,
                    top: y,
                    right: pad + content_w,
                    bottom: y + bar_h,
                },
                &bg_brush,
            );
            let fill_w = (content_w * *utilization as f32 / 100.0)
                .max(0.0)
                .min(content_w);
            if fill_w > 0.5 {
                let fill_color = colorref_to_d2d(colors.progress_color(*utilization));
                let fill_brush = rt
                    .CreateSolidColorBrush(&fill_color as *const _, None)
                    .unwrap();
                rt.FillRectangle(
                    &D2D_RECT_F {
                        left: pad,
                        top: y,
                        right: pad + fill_w,
                        bottom: y + bar_h,
                    },
                    &fill_brush,
                );
                // Accessibility overlay pattern
                if crate::APP_STATE
                    .as_ref()
                    .is_some_and(|s| s.config_mgr.config.accessibility_patterns)
                {
                    draw_accessibility_pattern(rt, pad, y, pad + fill_w, y + bar_h, *utilization);
                }
            }
            y += bar_h + self.sf(ITEM_GAP);
        }

        y
    }

    #[allow(clippy::too_many_arguments)]
    unsafe fn draw_not_detected(
        &self,
        rt: &ID2D1HwndRenderTarget,
        d2d: &mut D2DResources,
        w: f32,
        mut y: f32,
        colors: &ThemeColors,
        i18n: &I18n,
        last_error: &Option<String>,
        hovered: &HoveredElement,
        install_rect: &mut RECT,
        status_link_rect: &mut RECT,
    ) -> f32 {
        let pad = self.sf(PADDING);

        let err_str = last_error.as_deref().unwrap_or("");
        let error_tag = err_str
            .strip_prefix('[')
            .and_then(|s| s.split_once(']'))
            .map(|(tag, _)| tag);

        let error_detail: String;
        let (title, desc, btn_label): (&str, &str, &str) = match error_tag {
            Some("token_expired") => (
                i18n.t("token_expired"),
                i18n.t("token_expired_desc"),
                i18n.t("Open Claude.ai \u{2192}"),
            ),
            Some("network_error") => {
                error_detail = err_str
                    .strip_prefix("[network_error] ")
                    .unwrap_or(err_str)
                    .to_string();
                (
                    i18n.t("connection_error"),
                    &error_detail,
                    i18n.t("Open Claude.ai \u{2192}"),
                )
            }
            Some("rate_limited") => {
                error_detail = err_str
                    .strip_prefix("[rate_limited] ")
                    .unwrap_or(err_str)
                    .to_string();
                (
                    i18n.t("rate_limited"),
                    &error_detail,
                    i18n.t("Open Claude.ai \u{2192}"),
                )
            }
            Some("server_error") => (
                i18n.t("server_error"),
                i18n.t("server_error_desc"),
                i18n.t("Open Claude.ai \u{2192}"),
            ),
            Some("api_error") => {
                error_detail = err_str
                    .strip_prefix("[api_error] ")
                    .unwrap_or(err_str)
                    .to_string();
                (
                    i18n.t("connection_error"),
                    &error_detail,
                    i18n.t("Open Claude.ai \u{2192}"),
                )
            }
            _ if err_str.contains("credentials not found") => (
                i18n.t("credentials_not_found"),
                i18n.t("run_claude_login_desc"),
                i18n.t("Open Claude.ai \u{2192}"),
            ),
            _ if err_str.contains("accessToken field not found") => (
                i18n.t("credentials_not_found"),
                i18n.t("run_claude_login_desc"),
                i18n.t("Open Claude.ai \u{2192}"),
            ),
            _ => (
                i18n.t("Claude Code not detected"),
                i18n.t("install_claude_desc"),
                i18n.t("Install Claude Code \u{2192}"),
            ),
        };

        // Warning title
        let warn_str = format!("\u{26A0} {}", title);
        let warn_text = wide(&warn_str);
        let warn_format = d2d.get_text_format(12, true, 0, 1).clone();
        let warn_brush = rt
            .CreateSolidColorBrush(&colorref_to_d2d(colors.yellow) as *const _, None)
            .unwrap();
        rt.DrawText(
            &warn_text[..warn_text.len() - 1],
            &warn_format,
            &D2D_RECT_F {
                left: pad,
                top: y,
                right: w - pad,
                bottom: y + self.sf(24),
            },
            &warn_brush,
            D2D1_DRAW_TEXT_OPTIONS_NONE,
            DWRITE_MEASURING_MODE_NATURAL,
        );
        y += self.sf(28);

        // Description (word wrap)
        let desc_text = wide(desc);
        let desc_format = d2d.get_text_format_wrap(11, false);
        let desc_brush = rt
            .CreateSolidColorBrush(&colorref_to_d2d(colors.text_secondary) as *const _, None)
            .unwrap();
        rt.DrawText(
            &desc_text[..desc_text.len() - 1],
            &desc_format,
            &D2D_RECT_F {
                left: pad,
                top: y,
                right: w - pad,
                bottom: y + self.sf(60),
            },
            &desc_brush,
            D2D1_DRAW_TEXT_OPTIONS_NONE,
            DWRITE_MEASURING_MODE_NATURAL,
        );
        y += self.sf(70);

        // Action button
        let btn_h = self.sf(28);
        let btn_rect = D2D_RECT_F {
            left: pad,
            top: y,
            right: w - pad,
            bottom: y + btn_h,
        };
        *install_rect = to_win32_rect(&btn_rect);

        let btn_brush = rt
            .CreateSolidColorBrush(&colorref_to_d2d(colors.accent) as *const _, None)
            .unwrap();
        rt.FillRoundedRectangle(
            &D2D1_ROUNDED_RECT {
                rect: btn_rect,
                radiusX: 4.0,
                radiusY: 4.0,
            },
            &btn_brush,
        );

        let btn_text = wide(btn_label);
        let btn_format = d2d.get_text_format(12, false, 0, 1).clone();
        let white = D2D1_COLOR_F {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        };
        let btn_text_brush = rt.CreateSolidColorBrush(&white as *const _, None).unwrap();
        let text_rect = D2D_RECT_F {
            left: pad + self.sf(8),
            top: y,
            right: w - pad - self.sf(8),
            bottom: y + btn_h,
        };
        rt.DrawText(
            &btn_text[..btn_text.len() - 1],
            &btn_format,
            &text_rect,
            &btn_text_brush,
            D2D1_DRAW_TEXT_OPTIONS_NONE,
            DWRITE_MEASURING_MODE_NATURAL,
        );
        y += btn_h + self.sf(12);

        // "Status ↗" link
        let status_str = format!("{} \u{2197}", i18n.t("Status"));
        let status_text = wide(&status_str);
        let status_format = d2d.get_text_format(10, false, 1, 1).clone();
        let is_status_hovered = matches!(hovered, HoveredElement::StatusLink);
        let status_color = if is_status_hovered {
            lighten_d2d(&colorref_to_d2d(colors.accent), 0.3)
        } else {
            colorref_to_d2d(colors.accent)
        };
        let status_brush = rt
            .CreateSolidColorBrush(&status_color as *const _, None)
            .unwrap();
        let sr = D2D_RECT_F {
            left: pad,
            top: y,
            right: pad + self.sf(80),
            bottom: y + self.sf(20),
        };
        *status_link_rect = to_win32_rect(&sr);
        rt.DrawText(
            &status_text[..status_text.len() - 1],
            &status_format,
            &sr,
            &status_brush,
            D2D1_DRAW_TEXT_OPTIONS_NONE,
            DWRITE_MEASURING_MODE_NATURAL,
        );
        y += self.sf(24);

        y
    }

    unsafe fn draw_chatgpt_section(
        &self,
        rt: &ID2D1HwndRenderTarget,
        d2d: &mut D2DResources,
        w: f32,
        mut y: f32,
        colors: &ThemeColors,
        i18n: &I18n,
        link_rect: &mut RECT,
    ) -> f32 {
        let pad = self.sf(PADDING);

        // Section header
        let header_str = format!("\u{25CE} {}", i18n.t("CHATGPT / CODEX"));
        let header_text = wide(&header_str);
        let header_format = d2d.get_text_format(12, true, 0, 1).clone();
        let header_brush = rt
            .CreateSolidColorBrush(&colorref_to_d2d(colors.text_secondary) as *const _, None)
            .unwrap();
        rt.DrawText(
            &header_text[..header_text.len() - 1],
            &header_format,
            &D2D_RECT_F {
                left: pad,
                top: y,
                right: w - pad,
                bottom: y + self.sf(20),
            },
            &header_brush,
            D2D1_DRAW_TEXT_OPTIONS_NONE,
            DWRITE_MEASURING_MODE_NATURAL,
        );
        y += self.sf(24);

        // Info text (word wrap)
        let info = format!("\u{24D8} {}", i18n.t("openai_no_api"));
        let info_text = wide(&info);
        let info_format = d2d.get_text_format_wrap(11, false);
        let info_brush = rt
            .CreateSolidColorBrush(&colorref_to_d2d(colors.text_secondary) as *const _, None)
            .unwrap();
        rt.DrawText(
            &info_text[..info_text.len() - 1],
            &info_format,
            &D2D_RECT_F {
                left: pad,
                top: y,
                right: w - pad,
                bottom: y + self.sf(50),
            },
            &info_brush,
            D2D1_DRAW_TEXT_OPTIONS_NONE,
            DWRITE_MEASURING_MODE_NATURAL,
        );
        y += self.sf(55);

        // Link
        let link_str = format!("\u{1F4CA} {}", i18n.t("Open ChatGPT Usage \u{2192}"));
        let link_text = wide(&link_str);
        let link_format = d2d.get_text_format(12, false, 0, 1).clone();
        let link_brush = rt
            .CreateSolidColorBrush(&colorref_to_d2d(colors.accent) as *const _, None)
            .unwrap();
        let lr = D2D_RECT_F {
            left: pad,
            top: y,
            right: w - pad,
            bottom: y + self.sf(22),
        };
        *link_rect = to_win32_rect(&lr);
        rt.DrawText(
            &link_text[..link_text.len() - 1],
            &link_format,
            &lr,
            &link_brush,
            D2D1_DRAW_TEXT_OPTIONS_NONE,
            DWRITE_MEASURING_MODE_NATURAL,
        );
        y += self.sf(28);

        y
    }

    #[allow(clippy::too_many_arguments)]
    unsafe fn draw_chart(
        &self,
        rt: &ID2D1HwndRenderTarget,
        d2d: &mut D2DResources,
        w: f32,
        mut y: f32,
        data: &[f64],
        reset_lines: &[f64],
        chart_mode: u8,
        colors: &ThemeColors,
        i18n: &I18n,
        hovered: &HoveredElement,
        chart_rect_out: &mut RECT,
        chart_bar_count_out: &mut usize,
        chart_toggle_rect_out: &mut RECT,
    ) -> f32 {
        let pad = self.sf(PADDING);

        // Header with toggle: "Usage History   24h | 7d | 30d"
        let title = i18n.t("Usage History");
        let title_text = wide(title);
        let title_format = d2d.get_text_format(11, true, 0, 1).clone();
        let title_brush = rt
            .CreateSolidColorBrush(&colorref_to_d2d(colors.text_secondary) as *const _, None)
            .unwrap();
        rt.DrawText(
            &title_text[..title_text.len() - 1],
            &title_format,
            &D2D_RECT_F {
                left: pad,
                top: y,
                right: w - pad,
                bottom: y + self.sf(18),
            },
            &title_brush,
            D2D1_DRAW_TEXT_OPTIONS_NONE,
            DWRITE_MEASURING_MODE_NATURAL,
        );

        // Draw "24h | 7d | 30d" toggle on the right side
        let toggle_h = self.sf(18);
        let toggle_w = self.sf(96);
        let toggle_x = w - pad - toggle_w;
        let toggle_y = y;

        let is_hovered = matches!(hovered, HoveredElement::ChartToggle);

        let active_color = colorref_to_d2d(colors.accent);
        let inactive_color = colorref_to_d2d(colors.text_secondary);

        let fmt_bold = d2d.get_text_format(10, true, 0, 0).clone();
        let fmt_normal = d2d.get_text_format(10, false, 0, 0).clone();

        let segments: &[(&str, u8)] = &[
            ("24h", 0),
            (" | ", 255),
            ("7d", 1),
            (" | ", 255),
            ("30d", 2),
        ];
        let mut sx = toggle_x;
        let seg_widths: &[i32] = &[22, 16, 14, 16, 24];

        for (i, &(label, mode)) in segments.iter().enumerate() {
            let seg_w = self.sf(seg_widths[i]);
            let label_wide = wide(label);

            let (brush, fmt) = if mode == 255 {
                // Separator
                let b = rt
                    .CreateSolidColorBrush(&inactive_color as *const _, None)
                    .unwrap();
                (b, fmt_normal.clone())
            } else if chart_mode == mode {
                let b = rt
                    .CreateSolidColorBrush(&active_color as *const _, None)
                    .unwrap();
                (b, fmt_bold.clone())
            } else if is_hovered {
                let b = rt
                    .CreateSolidColorBrush(&lighten_d2d(&inactive_color, 0.3) as *const _, None)
                    .unwrap();
                (b, fmt_normal.clone())
            } else {
                let b = rt
                    .CreateSolidColorBrush(&inactive_color as *const _, None)
                    .unwrap();
                (b, fmt_normal.clone())
            };

            rt.DrawText(
                &label_wide[..label_wide.len() - 1],
                &fmt,
                &D2D_RECT_F {
                    left: sx,
                    top: toggle_y,
                    right: sx + seg_w,
                    bottom: toggle_y + toggle_h,
                },
                &brush,
                D2D1_DRAW_TEXT_OPTIONS_NONE,
                DWRITE_MEASURING_MODE_NATURAL,
            );
            sx += seg_w;
        }

        *chart_toggle_rect_out = RECT {
            left: toggle_x as i32,
            top: toggle_y as i32,
            right: (toggle_x + toggle_w) as i32,
            bottom: (toggle_y + toggle_h) as i32,
        };

        y += self.sf(22);

        let chart_h = self.sf(70);
        let chart_w = w - pad * 2.0;

        // Output chart area for hit-testing
        *chart_rect_out = RECT {
            left: pad as i32,
            top: y as i32,
            right: (pad + chart_w) as i32,
            bottom: (y + chart_h) as i32,
        };
        *chart_bar_count_out = data.len();

        // Chart background
        let bg_brush = rt
            .CreateSolidColorBrush(&colorref_to_d2d(colors.surface) as *const _, None)
            .unwrap();
        rt.FillRectangle(
            &D2D_RECT_F {
                left: pad,
                top: y,
                right: pad + chart_w,
                bottom: y + chart_h,
            },
            &bg_brush,
        );

        // Grid lines at 25%, 50%, 75%
        let grid_brush = rt
            .CreateSolidColorBrush(&colorref_to_d2d(colors.separator) as *const _, None)
            .unwrap();
        for pct in [25, 50, 75] {
            let gy = y + chart_h - (chart_h * pct as f32 / 100.0);
            rt.DrawLine(
                D2D_POINT_2F { x: pad, y: gy },
                D2D_POINT_2F {
                    x: pad + chart_w,
                    y: gy,
                },
                &grid_brush,
                1.0,
                None,
            );
        }

        if !data.is_empty() {
            let bar_w = (chart_w / data.len() as f32).max(2.0);
            let gap = 1.0f32.max(bar_w / 6.0);
            for (i, &val) in data.iter().enumerate() {
                let bar_h_px = (val / 100.0) * chart_h as f64;
                if bar_h_px > 0.5 {
                    let bar_x = pad + i as f32 * bar_w;
                    let color = if val >= 80.0 {
                        colorref_to_d2d(colors.red)
                    } else if val >= 50.0 {
                        colorref_to_d2d(colors.yellow)
                    } else {
                        colorref_to_d2d(colors.green)
                    };
                    let bar_brush = rt.CreateSolidColorBrush(&color as *const _, None).unwrap();
                    rt.FillRectangle(
                        &D2D_RECT_F {
                            left: bar_x + gap,
                            top: y + chart_h - bar_h_px as f32,
                            right: bar_x + bar_w - gap,
                            bottom: y + chart_h,
                        },
                        &bar_brush,
                    );
                }
            }
        }

        // Reset lines (dashed vertical) — only for 24h view
        if chart_mode == 0 && !reset_lines.is_empty() {
            let reset_brush = rt
                .CreateSolidColorBrush(&colorref_to_d2d(colors.accent) as *const _, None)
                .unwrap();
            let chart_top = y;
            for &hours_ago in reset_lines {
                let rx = pad + chart_w * (1.0 - hours_ago as f32 / 24.0);
                if rx >= pad && rx <= pad + chart_w {
                    let dash = 3.0f32;
                    let mut dy = chart_top;
                    while dy < chart_top + chart_h {
                        let end = (dy + dash).min(chart_top + chart_h);
                        rt.DrawLine(
                            D2D_POINT_2F { x: rx, y: dy },
                            D2D_POINT_2F { x: rx, y: end },
                            &reset_brush,
                            1.0,
                            None,
                        );
                        dy += dash * 2.0;
                    }
                }
            }
        }

        // Hover tooltip
        if let HoveredElement::ChartBar(idx) = hovered {
            let idx = *idx;
            if idx < data.len() {
                let val = data[idx];
                let bar_w = (chart_w / data.len() as f32).max(2.0);
                let bar_cx = pad + idx as f32 * bar_w + bar_w / 2.0;
                let total_hours = match chart_mode {
                    1 => 168.0,
                    2 => 720.0,
                    _ => 24.0,
                };
                let hours_ago = total_hours * (1.0 - (idx as f64 + 0.5) / data.len() as f64);

                let bar_time =
                    chrono::Local::now() - chrono::Duration::seconds((hours_ago * 3600.0) as i64);
                let time_str = if chart_mode >= 1 {
                    bar_time.format("%b %d").to_string()
                } else if is_system_24h() {
                    bar_time.format("%H:%M").to_string()
                } else {
                    bar_time.format("%I:%M %p").to_string()
                };
                let tip_text = format!("{:.0}% | {}", val, time_str);
                let tip_wide = wide(&tip_text);

                let tip_w = self.sf(100);
                let tip_h = self.sf(22);
                let tip_x = (bar_cx - tip_w / 2.0).clamp(pad, pad + chart_w - tip_w);
                let bar_h_px = (val / 100.0) * chart_h as f64;
                let tip_y = (y + chart_h - bar_h_px as f32 - tip_h - self.sf(4)).max(y - tip_h);

                // Tooltip background
                let tip_bg = rt
                    .CreateSolidColorBrush(&colorref_to_d2d(colors.surface) as *const _, None)
                    .unwrap();
                let tip_border_brush = rt
                    .CreateSolidColorBrush(&colorref_to_d2d(colors.border) as *const _, None)
                    .unwrap();
                let tip_rect = D2D1_ROUNDED_RECT {
                    rect: D2D_RECT_F {
                        left: tip_x,
                        top: tip_y,
                        right: tip_x + tip_w,
                        bottom: tip_y + tip_h,
                    },
                    radiusX: 4.0,
                    radiusY: 4.0,
                };
                rt.FillRoundedRectangle(&tip_rect, &tip_bg);
                rt.DrawRoundedRectangle(&tip_rect, &tip_border_brush, 1.0, None);

                // Tooltip text
                let tip_format = d2d.get_text_format(9, true, 2, 1).clone();
                let tip_text_brush = rt
                    .CreateSolidColorBrush(&colorref_to_d2d(colors.text_primary) as *const _, None)
                    .unwrap();
                rt.DrawText(
                    &tip_wide[..tip_wide.len() - 1],
                    &tip_format,
                    &D2D_RECT_F {
                        left: tip_x,
                        top: tip_y,
                        right: tip_x + tip_w,
                        bottom: tip_y + tip_h,
                    },
                    &tip_text_brush,
                    D2D1_DRAW_TEXT_OPTIONS_NONE,
                    DWRITE_MEASURING_MODE_NATURAL,
                );
            }
        }

        y += chart_h + self.sf(4);

        // X-axis labels
        let labels: &[&str] = match chart_mode {
            1 => &["7d", "5d", "4d", "2d", "now"],
            2 => &["30d", "22d", "15d", "7d", "now"],
            _ => &["24h", "18h", "12h", "6h", "now"],
        };
        for (i, label) in labels.iter().enumerate() {
            let lx = pad + (i as f32 * chart_w / 4.0);
            let text = wide(label);
            let format = d2d.get_text_format(9, false, 0, 0).clone();
            let brush = rt
                .CreateSolidColorBrush(&colorref_to_d2d(colors.text_secondary) as *const _, None)
                .unwrap();
            rt.DrawText(
                &text[..text.len() - 1],
                &format,
                &D2D_RECT_F {
                    left: lx - self.sf(14),
                    top: y,
                    right: lx + self.sf(30),
                    bottom: y + self.sf(14),
                },
                &brush,
                D2D1_DRAW_TEXT_OPTIONS_NONE,
                DWRITE_MEASURING_MODE_NATURAL,
            );
        }
        y += self.sf(14);

        y
    }

    #[allow(clippy::too_many_arguments)]
    unsafe fn draw_footer(
        &self,
        rt: &ID2D1HwndRenderTarget,
        d2d: &mut D2DResources,
        w: f32,
        y: f32,
        last_updated: &str,
        colors: &ThemeColors,
        i18n: &I18n,
        hovered: &HoveredElement,
        refresh_rect: &mut RECT,
        copy_rect: &mut RECT,
        _status_link_rect: &mut RECT,
    ) {
        let h = self.sf(FOOTER_H);
        let pad = self.sf(PADDING);

        // Footer background
        let surface_brush = rt
            .CreateSolidColorBrush(&colorref_to_d2d(colors.surface) as *const _, None)
            .unwrap();
        rt.FillRectangle(
            &D2D_RECT_F {
                left: 0.0,
                top: y,
                right: w,
                bottom: y + h,
            },
            &surface_brush,
        );

        // Last updated text
        let updated_text = format!("{} {}", i18n.t("Last updated:"), last_updated);
        let updated_wide = wide(&updated_text);
        let updated_format = d2d.get_text_format(10, false, 0, 1).clone();
        let text_brush = rt
            .CreateSolidColorBrush(&colorref_to_d2d(colors.text_secondary) as *const _, None)
            .unwrap();
        rt.DrawText(
            &updated_wide[..updated_wide.len() - 1],
            &updated_format,
            &D2D_RECT_F {
                left: pad,
                top: y,
                right: w - self.sf(160),
                bottom: y + h,
            },
            &text_brush,
            D2D1_DRAW_TEXT_OPTIONS_NONE,
            DWRITE_MEASURING_MODE_NATURAL,
        );

        // Refresh button
        let btn_w = self.sf(78);
        let btn_h = self.sf(26);
        let btn_rect = D2D_RECT_F {
            left: w - btn_w - pad,
            top: y + (h - btn_h) / 2.0,
            right: w - pad,
            bottom: y + (h - btn_h) / 2.0 + btn_h,
        };
        *refresh_rect = to_win32_rect(&btn_rect);

        // Copy button (left of Refresh)
        let copy_w = self.sf(28);
        let copy_rect_d2d = D2D_RECT_F {
            left: btn_rect.left - copy_w - self.sf(4),
            top: btn_rect.top,
            right: btn_rect.left - self.sf(4),
            bottom: btn_rect.bottom,
        };
        *copy_rect = to_win32_rect(&copy_rect_d2d);

        let copy_hovered = matches!(hovered, HoveredElement::CopyButton);
        if copy_hovered {
            let hover_brush = rt
                .CreateSolidColorBrush(&colorref_to_d2d(colors.hover) as *const _, None)
                .unwrap();
            rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: copy_rect_d2d,
                    radiusX: self.sf(6),
                    radiusY: self.sf(6),
                },
                &hover_brush,
            );
        }
        // Clipboard icon (U+1F4CB or simpler: ⧉ U+29C9)
        self.draw_text_centered(
            rt,
            d2d,
            "\u{2398}",
            copy_rect_d2d,
            if copy_hovered {
                colors.accent
            } else {
                colors.text_secondary
            },
            12,
            false,
        );

        let is_hovered = matches!(hovered, HoveredElement::RefreshButton);

        if is_hovered {
            // Filled accent button when hovered
            let fill_brush = rt
                .CreateSolidColorBrush(&colorref_to_d2d(colors.accent) as *const _, None)
                .unwrap();
            rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: btn_rect,
                    radiusX: self.sf(6),
                    radiusY: self.sf(6),
                },
                &fill_brush,
            );
            // White text on hover
            let white = D2D1_COLOR_F {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            };
            let text_brush = rt.CreateSolidColorBrush(&white as *const _, None).unwrap();
            let refresh_text = wide(i18n.t("Refresh"));
            let text_format = d2d.get_text_format(10, false, 2, 1).clone();
            rt.DrawText(
                &refresh_text[..refresh_text.len() - 1],
                &text_format,
                &btn_rect,
                &text_brush,
                D2D1_DRAW_TEXT_OPTIONS_NONE,
                DWRITE_MEASURING_MODE_NATURAL,
            );
        } else {
            // Outlined button (normal state)
            let border_brush = rt
                .CreateSolidColorBrush(&colorref_to_d2d(colors.accent) as *const _, None)
                .unwrap();
            let bg_brush = rt
                .CreateSolidColorBrush(&colorref_to_d2d(colors.surface) as *const _, None)
                .unwrap();
            rt.FillRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: btn_rect,
                    radiusX: self.sf(6),
                    radiusY: self.sf(6),
                },
                &bg_brush,
            );
            rt.DrawRoundedRectangle(
                &D2D1_ROUNDED_RECT {
                    rect: btn_rect,
                    radiusX: self.sf(6),
                    radiusY: self.sf(6),
                },
                &border_brush,
                1.0,
                None,
            );
            // Accent text
            let accent_brush = rt
                .CreateSolidColorBrush(&colorref_to_d2d(colors.accent) as *const _, None)
                .unwrap();
            let refresh_text = wide(i18n.t("Refresh"));
            let text_format = d2d.get_text_format(10, false, 2, 1).clone();
            rt.DrawText(
                &refresh_text[..refresh_text.len() - 1],
                &text_format,
                &btn_rect,
                &accent_brush,
                D2D1_DRAW_TEXT_OPTIONS_NONE,
                DWRITE_MEASURING_MODE_NATURAL,
            );
        }
    }

    unsafe fn draw_text_centered(
        &self,
        rt: &ID2D1HwndRenderTarget,
        d2d: &mut D2DResources,
        text: &str,
        rect: D2D_RECT_F,
        color: windows::Win32::Foundation::COLORREF,
        size: i32,
        bold: bool,
    ) {
        let format = d2d.get_text_format(size, bold, 2, 1).clone();
        let brush = rt
            .CreateSolidColorBrush(&colorref_to_d2d(color) as *const _, None)
            .unwrap();
        let text_wide = wide(text);
        rt.DrawText(
            &text_wide[..text_wide.len() - 1],
            &format,
            &rect,
            &brush,
            D2D1_DRAW_TEXT_OPTIONS_NONE,
            DWRITE_MEASURING_MODE_NATURAL,
        );
    }

    unsafe fn draw_border(&self, rt: &ID2D1HwndRenderTarget, w: f32, h: f32, colors: &ThemeColors) {
        let brush = rt
            .CreateSolidColorBrush(&colorref_to_d2d(colors.border) as *const _, None)
            .unwrap();
        rt.DrawRectangle(
            &D2D_RECT_F {
                left: 0.5,
                top: 0.5,
                right: w - 0.5,
                bottom: h - 0.5,
            },
            &brush,
            1.0,
            None,
        );
    }
}

/// Settings panel rendering (D2D)
#[allow(clippy::too_many_arguments)]
pub unsafe fn draw_settings_panel(
    d2d: &mut D2DResources,
    rect: &RECT,
    colors: &ThemeColors,
    i18n: &I18n,
    config: &crate::config::Config,
    back_rect: &mut RECT,
    close_rect: &mut RECT,
    setting_rects: &mut [RECT; 13],
    hovered: &HoveredElement,
) {
    let Some(rt) = d2d.render_target.clone() else {
        return;
    };
    let w = (rect.right - rect.left) as f32;
    let pad = 16.0f32;
    let header_h = 40.0f32;
    let row_h = 38.0f32;

    // Header background
    let surf_brush = rt
        .CreateSolidColorBrush(&colorref_to_d2d(colors.surface) as *const _, None)
        .unwrap();
    rt.FillRectangle(
        &D2D_RECT_F {
            left: 0.0,
            top: 0.0,
            right: w,
            bottom: header_h,
        },
        &surf_brush,
    );

    // Header separator
    let sep_brush = rt
        .CreateSolidColorBrush(&colorref_to_d2d(colors.separator) as *const _, None)
        .unwrap();
    rt.DrawLine(
        D2D_POINT_2F {
            x: 0.0,
            y: header_h,
        },
        D2D_POINT_2F { x: w, y: header_h },
        &sep_brush,
        1.0,
        None,
    );

    // Back button: "← Back"
    let back_r = D2D_RECT_F {
        left: pad,
        top: 0.0,
        right: w / 2.0,
        bottom: header_h,
    };
    *back_rect = to_win32_rect(&back_r);

    // Hover highlight for back button
    if matches!(hovered, HoveredElement::BackButton) {
        let hover_brush = rt
            .CreateSolidColorBrush(&colorref_to_d2d(colors.hover) as *const _, None)
            .unwrap();
        rt.FillRoundedRectangle(
            &D2D1_ROUNDED_RECT {
                rect: D2D_RECT_F {
                    left: pad - 4.0,
                    top: 6.0,
                    right: pad + 80.0,
                    bottom: header_h - 6.0,
                },
                radiusX: 4.0,
                radiusY: 4.0,
            },
            &hover_brush,
        );
    }

    let back_text = wide(i18n.t("Back"));
    let back_format = d2d.get_text_format(13, true, 0, 1).clone();
    let accent_brush = rt
        .CreateSolidColorBrush(&colorref_to_d2d(colors.accent) as *const _, None)
        .unwrap();
    rt.DrawText(
        &back_text[..back_text.len() - 1],
        &back_format,
        &back_r,
        &accent_brush,
        D2D1_DRAW_TEXT_OPTIONS_NONE,
        DWRITE_MEASURING_MODE_NATURAL,
    );

    // Title centered
    let title_text = wide(i18n.t("Settings"));
    let title_format = d2d.get_text_format(13, true, 2, 1).clone();
    let title_brush = rt
        .CreateSolidColorBrush(&colorref_to_d2d(colors.text_primary) as *const _, None)
        .unwrap();
    rt.DrawText(
        &title_text[..title_text.len() - 1],
        &title_format,
        &D2D_RECT_F {
            left: 0.0,
            top: 0.0,
            right: w,
            bottom: header_h,
        },
        &title_brush,
        D2D1_DRAW_TEXT_OPTIONS_NONE,
        DWRITE_MEASURING_MODE_NATURAL,
    );

    // Close button ×
    let close_r = D2D_RECT_F {
        left: w - 36.0,
        top: 0.0,
        right: w - 4.0,
        bottom: header_h,
    };
    *close_rect = to_win32_rect(&close_r);

    if matches!(hovered, HoveredElement::CloseButton) {
        let hover_brush = rt
            .CreateSolidColorBrush(&colorref_to_d2d(colors.hover) as *const _, None)
            .unwrap();
        rt.FillRoundedRectangle(
            &D2D1_ROUNDED_RECT {
                rect: D2D_RECT_F {
                    left: w - 36.0,
                    top: 6.0,
                    right: w - 4.0,
                    bottom: header_h - 6.0,
                },
                radiusX: 4.0,
                radiusY: 4.0,
            },
            &hover_brush,
        );
    }

    let close_col = if matches!(hovered, HoveredElement::CloseButton) {
        colors.accent
    } else {
        colors.text_secondary
    };
    draw_close_icon_freestanding(&rt, close_r, close_col);

    let mut y = header_h + 8.0;

    // Settings rows
    // Build language display: "Auto (English)" or "English", "Українська", etc.
    let lang_display = if config.language == "auto" {
        let detected = crate::i18n::Locale::detect_from_windows();
        format!("{} ({})", i18n.t("Auto"), detected.display_name())
    } else {
        crate::i18n::Locale::from_str(&config.language)
            .map(|l| l.display_name().to_string())
            .unwrap_or_else(|| config.language.to_uppercase())
    };

    // (label, Option<text_value>, Option<bool_checked>)
    let rows: Vec<(String, Option<String>, Option<bool>)> = vec![
        (
            i18n.t("Theme").to_string(),
            Some(i18n.t(&capitalize(&config.theme)).to_string()),
            None,
        ),
        (i18n.t("Language").to_string(), Some(lang_display), None),
        (
            i18n.t("Compact mode").to_string(),
            None,
            Some(config.compact_mode),
        ),
        (
            i18n.t("Show ChatGPT section").to_string(),
            None,
            Some(config.show_chatgpt_section),
        ),
        (
            i18n.t("Start with Windows").to_string(),
            None,
            Some(config.autostart),
        ),
        (
            i18n.t("Show widget").to_string(),
            None,
            Some(config.show_widget),
        ),
        (
            i18n.t("Check for updates").to_string(),
            None,
            Some(config.check_updates),
        ),
        (
            i18n.t("Accessibility patterns").to_string(),
            None,
            Some(config.accessibility_patterns),
        ),
        (
            i18n.t("Icon style").to_string(),
            Some(i18n.t(&capitalize(&config.tray_icon_style)).to_string()),
            None,
        ),
        (
            i18n.t("Dashboard layout").to_string(),
            Some(i18n.t(&capitalize(&config.dashboard_layout)).to_string()),
            None,
        ),
        (
            i18n.t("Hide Extra Usage").to_string(),
            None,
            Some(config.hide_extra_usage),
        ),
        (
            i18n.t("Show startup notification").to_string(),
            None,
            Some(config.show_startup_notification),
        ),
        (
            i18n.t("Show login expiry warning").to_string(),
            None,
            Some(config.token_expiry_warning),
        ),
    ];

    for (i, (label, text_val, bool_val)) in rows.iter().enumerate() {
        let is_hovered = matches!(hovered, HoveredElement::SettingRow(idx) if *idx == i);

        // Row background
        let row_color = if is_hovered {
            colorref_to_d2d(colors.hover)
        } else if i % 2 == 1 {
            colorref_to_d2d(colors.surface)
        } else {
            colorref_to_d2d(colors.background)
        };
        let row_brush = rt
            .CreateSolidColorBrush(&row_color as *const _, None)
            .unwrap();
        rt.FillRectangle(
            &D2D_RECT_F {
                left: 0.0,
                top: y,
                right: w,
                bottom: y + row_h,
            },
            &row_brush,
        );

        setting_rects[i] = RECT {
            left: 0,
            top: y as i32,
            right: w as i32,
            bottom: (y + row_h) as i32,
        };

        // Label (left)
        let label_text = wide(label);
        let label_format = d2d.get_text_format(12, false, 0, 1).clone();
        let label_brush = rt
            .CreateSolidColorBrush(&colorref_to_d2d(colors.text_primary) as *const _, None)
            .unwrap();
        rt.DrawText(
            &label_text[..label_text.len() - 1],
            &label_format,
            &D2D_RECT_F {
                left: pad,
                top: y,
                right: w / 2.0 + 40.0,
                bottom: y + row_h,
            },
            &label_brush,
            D2D1_DRAW_TEXT_OPTIONS_NONE,
            DWRITE_MEASURING_MODE_NATURAL,
        );

        // Value (right) — either text or D2D checkbox
        if let Some(checked) = bool_val {
            // D2D checkbox aligned right
            let cb_x = w - pad - 14.0;
            let cb_y = y + (row_h - 14.0) / 2.0;
            draw_checkbox_freestanding(&rt, cb_x, cb_y, *checked, colors);
        } else if let Some(value) = text_val {
            let val_text = wide(value);
            let val_format = d2d.get_text_format(12, false, 1, 1).clone();
            let val_brush = rt
                .CreateSolidColorBrush(&colorref_to_d2d(colors.accent) as *const _, None)
                .unwrap();
            rt.DrawText(
                &val_text[..val_text.len() - 1],
                &val_format,
                &D2D_RECT_F {
                    left: w / 2.0 + 40.0,
                    top: y,
                    right: w - pad,
                    bottom: y + row_h,
                },
                &val_brush,
                D2D1_DRAW_TEXT_OPTIONS_NONE,
                DWRITE_MEASURING_MODE_NATURAL,
            );
        }

        y += row_h;

        // Separator between rows
        if i < rows.len() - 1 {
            let mut row_sep_color = colorref_to_d2d(colors.separator);
            row_sep_color.a = 0.3;
            let row_sep_brush = rt
                .CreateSolidColorBrush(&row_sep_color as *const _, None)
                .unwrap();
            rt.DrawLine(
                D2D_POINT_2F { x: pad, y },
                D2D_POINT_2F { x: w - pad, y },
                &row_sep_brush,
                0.5,
                None,
            );
        }
    }

    // Icon legend section — compact 2-row layout
    y += 10.0;
    let mut sep_line_color = colorref_to_d2d(colors.separator);
    sep_line_color.a = 0.3;
    let sep_line_brush = rt
        .CreateSolidColorBrush(&sep_line_color as *const _, None)
        .unwrap();
    rt.DrawLine(
        D2D_POINT_2F { x: pad, y },
        D2D_POINT_2F { x: w - pad, y },
        &sep_line_brush,
        0.5,
        None,
    );
    y += 10.0;

    // Legend title
    let legend_title = wide(i18n.t("Tray icon colors:"));
    let legend_format = d2d.get_text_format(10, false, 0, 0).clone();
    let legend_brush = rt
        .CreateSolidColorBrush(&colorref_to_d2d(colors.text_secondary) as *const _, None)
        .unwrap();
    rt.DrawText(
        &legend_title[..legend_title.len() - 1],
        &legend_format,
        &D2D_RECT_F {
            left: pad,
            top: y,
            right: w - pad,
            bottom: y + 16.0,
        },
        &legend_brush,
        D2D1_DRAW_TEXT_OPTIONS_NONE,
        DWRITE_MEASURING_MODE_NATURAL,
    );
    y += 18.0;

    // 4 items in single column
    let icon_items: [(windows::Win32::Foundation::COLORREF, &str); 4] = [
        (colors.green, i18n.t("< 50% usage")),
        (colors.yellow, i18n.t("50-79% usage")),
        (colors.red, i18n.t(">= 80% usage")),
        (colors.separator, i18n.t("No data")),
    ];

    for (color, label) in &icon_items {
        let dot_r = 5.0f32;
        let dot_cx = pad + dot_r + 2.0;
        let dot_cy = y + 8.0;
        let dot_brush = rt
            .CreateSolidColorBrush(&colorref_to_d2d(*color) as *const _, None)
            .unwrap();
        rt.FillEllipse(
            &D2D1_ELLIPSE {
                point: D2D_POINT_2F {
                    x: dot_cx,
                    y: dot_cy,
                },
                radiusX: dot_r,
                radiusY: dot_r,
            },
            &dot_brush,
        );

        let lbl_text = wide(label);
        let lbl_format = d2d.get_text_format(10, false, 0, 0).clone();
        let lbl_brush = rt
            .CreateSolidColorBrush(&colorref_to_d2d(colors.text_primary) as *const _, None)
            .unwrap();
        rt.DrawText(
            &lbl_text[..lbl_text.len() - 1],
            &lbl_format,
            &D2D_RECT_F {
                left: pad + dot_r * 2.0 + 8.0,
                top: y,
                right: w - pad,
                bottom: y + 16.0,
            },
            &lbl_brush,
            D2D1_DRAW_TEXT_OPTIONS_NONE,
            DWRITE_MEASURING_MODE_NATURAL,
        );
        y += 18.0;
    }

    // Footer
    let footer_y = (rect.bottom - 44) as f32;
    rt.DrawLine(
        D2D_POINT_2F {
            x: 0.0,
            y: footer_y,
        },
        D2D_POINT_2F { x: w, y: footer_y },
        &sep_brush,
        1.0,
        None,
    );

    let fy = footer_y + 6.0;
    let footer_text1 = wide(&format!(
        "ZaiMeter v{} by klivak",
        env!("CARGO_PKG_VERSION")
    ));
    let footer_format = d2d.get_text_format(10, false, 0, 0).clone();
    let footer_brush = rt
        .CreateSolidColorBrush(&colorref_to_d2d(colors.text_secondary) as *const _, None)
        .unwrap();
    rt.DrawText(
        &footer_text1[..footer_text1.len() - 1],
        &footer_format,
        &D2D_RECT_F {
            left: pad,
            top: fy,
            right: w - pad,
            bottom: fy + 16.0,
        },
        &footer_brush,
        D2D1_DRAW_TEXT_OPTIONS_NONE,
        DWRITE_MEASURING_MODE_NATURAL,
    );

    let footer_text2 = wide("github.com/klivak/zaimeter");
    let footer_link_brush = rt
        .CreateSolidColorBrush(&colorref_to_d2d(colors.accent) as *const _, None)
        .unwrap();
    rt.DrawText(
        &footer_text2[..footer_text2.len() - 1],
        &footer_format,
        &D2D_RECT_F {
            left: pad,
            top: fy + 16.0,
            right: w - pad,
            bottom: fy + 32.0,
        },
        &footer_link_brush,
        D2D1_DRAW_TEXT_OPTIONS_NONE,
        DWRITE_MEASURING_MODE_NATURAL,
    );

    // 1px border
    let border_brush = rt
        .CreateSolidColorBrush(&colorref_to_d2d(colors.border) as *const _, None)
        .unwrap();
    let h = (rect.bottom - rect.top) as f32;
    rt.DrawRectangle(
        &D2D_RECT_F {
            left: 0.5,
            top: 0.5,
            right: w - 0.5,
            bottom: h - 0.5,
        },
        &border_brush,
        1.0,
        None,
    );
}

/// Freestanding close icon (X) using D2D lines — for use in free functions
unsafe fn draw_close_icon_freestanding(
    rt: &ID2D1HwndRenderTarget,
    rect: D2D_RECT_F,
    color: ColorRef,
) {
    let brush = rt
        .CreateSolidColorBrush(&colorref_to_d2d(color) as *const _, None)
        .unwrap();
    let cx = (rect.left + rect.right) / 2.0;
    let cy = (rect.top + rect.bottom) / 2.0;
    let half = ((rect.right - rect.left).min(rect.bottom - rect.top)) * 0.18;
    let stroke = 1.8;
    rt.DrawLine(
        D2D_POINT_2F {
            x: cx - half,
            y: cy - half,
        },
        D2D_POINT_2F {
            x: cx + half,
            y: cy + half,
        },
        &brush,
        stroke,
        None,
    );
    rt.DrawLine(
        D2D_POINT_2F {
            x: cx + half,
            y: cy - half,
        },
        D2D_POINT_2F {
            x: cx - half,
            y: cy + half,
        },
        &brush,
        stroke,
        None,
    );
}

/// Freestanding D2D checkbox — for use in free functions
unsafe fn draw_checkbox_freestanding(
    rt: &ID2D1HwndRenderTarget,
    x: f32,
    y: f32,
    checked: bool,
    colors: &ThemeColors,
) {
    let size = 14.0f32;
    let radius = 3.0f32;
    let r = D2D_RECT_F {
        left: x,
        top: y,
        right: x + size,
        bottom: y + size,
    };

    if checked {
        let fill_brush = rt
            .CreateSolidColorBrush(&colorref_to_d2d(colors.accent) as *const _, None)
            .unwrap();
        rt.FillRoundedRectangle(
            &D2D1_ROUNDED_RECT {
                rect: r,
                radiusX: radius,
                radiusY: radius,
            },
            &fill_brush,
        );
        // White checkmark
        let check_brush = rt
            .CreateSolidColorBrush(
                &D2D1_COLOR_F {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                    a: 1.0,
                } as *const _,
                None,
            )
            .unwrap();
        let stroke = 1.8;
        rt.DrawLine(
            D2D_POINT_2F {
                x: x + size * 0.25,
                y: y + size * 0.52,
            },
            D2D_POINT_2F {
                x: x + size * 0.42,
                y: y + size * 0.70,
            },
            &check_brush,
            stroke,
            None,
        );
        rt.DrawLine(
            D2D_POINT_2F {
                x: x + size * 0.42,
                y: y + size * 0.70,
            },
            D2D_POINT_2F {
                x: x + size * 0.75,
                y: y + size * 0.30,
            },
            &check_brush,
            stroke,
            None,
        );
    } else {
        let mut border_color = colorref_to_d2d(colors.text_secondary);
        border_color.a = 0.5;
        let border_brush = rt
            .CreateSolidColorBrush(&border_color as *const _, None)
            .unwrap();
        rt.DrawRoundedRectangle(
            &D2D1_ROUNDED_RECT {
                rect: r,
                radiusX: radius,
                radiusY: radius,
            },
            &border_brush,
            1.5,
            None,
        );
    }
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

fn to_win32_rect(r: &D2D_RECT_F) -> RECT {
    RECT {
        left: r.left as i32,
        top: r.top as i32,
        right: r.right as i32,
        bottom: r.bottom as i32,
    }
}
