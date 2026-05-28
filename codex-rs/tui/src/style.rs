use crate::color::blend;
use crate::color::is_light;
use crate::terminal_palette::StdoutColorLevel;
use crate::terminal_palette::best_color;
use crate::terminal_palette::default_bg;
use crate::terminal_palette::default_fg;
use crate::terminal_palette::rgb_color;
use crate::terminal_palette::stdout_color_level;
use codex_config::types::TuiColor;
use codex_config::types::TuiColors;
use ratatui::style::Color;
use ratatui::style::Style;
use ratatui::style::Stylize;
use std::sync::OnceLock;
use std::sync::RwLock;

const LIGHT_BG_ACCENT_RGB: (u8, u8, u8) = (0, 95, 135);
// Decorative table rules should remain visible without competing with cell content.
const TABLE_SEPARATOR_FG_ALPHA: f32 = 0.20;
static TUI_COLORS: OnceLock<RwLock<TuiColors>> = OnceLock::new();

pub(crate) fn set_tui_colors(colors: TuiColors) {
    let lock = TUI_COLORS.get_or_init(|| RwLock::new(TuiColors::default()));
    match lock.write() {
        Ok(mut guard) => *guard = colors,
        Err(err) => {
            tracing::warn!("failed to update TUI semantic colors: {err}");
        }
    }
}

pub fn user_message_style() -> Style {
    user_message_style_for_colors(default_bg(), &current_tui_colors())
}

pub fn proposed_plan_style() -> Style {
    proposed_plan_style_for_colors(default_bg(), &current_tui_colors())
}

pub(crate) fn composer_style() -> Style {
    composer_style_for_colors(default_bg(), &current_tui_colors())
}

pub(crate) fn composer_text_style() -> Style {
    foreground_style_for_colors(&current_tui_colors())
}

pub(crate) fn primary_text_style() -> Style {
    foreground_style_for_colors(&current_tui_colors())
}

pub(crate) fn muted_style() -> Style {
    muted_style_for_colors(&current_tui_colors())
}

pub(crate) fn metadata_style() -> Style {
    metadata_style_for_colors(&current_tui_colors())
}

pub(crate) fn border_style() -> Style {
    border_style_for_colors(&current_tui_colors())
}

pub(crate) fn separator_style() -> Style {
    separator_style_for_colors(&current_tui_colors())
}

pub(crate) fn status_style() -> Option<Style> {
    status_style_for_colors(&current_tui_colors())
}

pub(crate) fn success_style() -> Style {
    success_style_for_colors(&current_tui_colors())
}

pub(crate) fn error_style() -> Style {
    error_style_for_colors(&current_tui_colors())
}

pub(crate) fn selection_style() -> Style {
    selection_style_for_colors(default_bg(), &current_tui_colors())
}

/// Returns a low-contrast rule style for separators within markdown tables.
pub(crate) fn table_separator_style() -> Style {
    table_separator_style_for(default_fg(), default_bg(), stdout_color_level())
}

/// Returns the shared accent style for active or selected TUI controls.
pub(crate) fn accent_style() -> Style {
    accent_style_for_colors(default_bg(), &current_tui_colors())
}

/// Returns the style for a user-authored message using the provided terminal background.
pub fn user_message_style_for(terminal_bg: Option<(u8, u8, u8)>) -> Style {
    user_message_style_for_colors(terminal_bg, &TuiColors::default())
}

pub fn proposed_plan_style_for(terminal_bg: Option<(u8, u8, u8)>) -> Style {
    proposed_plan_style_for_colors(terminal_bg, &TuiColors::default())
}

/// Returns the shared accent style for the provided terminal background.
pub(crate) fn accent_style_for(terminal_bg: Option<(u8, u8, u8)>) -> Style {
    accent_style_for_colors(terminal_bg, &TuiColors::default())
}

fn current_tui_colors() -> TuiColors {
    TUI_COLORS
        .get_or_init(|| RwLock::new(TuiColors::default()))
        .read()
        .map(|guard| guard.clone())
        .unwrap_or_default()
}

fn foreground_style_for_colors(colors: &TuiColors) -> Style {
    colors
        .foreground
        .map_or_else(Style::default, |color| Style::default().fg(tui_color(color)))
}

fn muted_style_for_colors(colors: &TuiColors) -> Style {
    colors
        .muted
        .map_or_else(|| Style::default().dim(), |color| Style::default().fg(tui_color(color)))
}

fn metadata_style_for_colors(colors: &TuiColors) -> Style {
    colors
        .muted
        .map_or_else(Style::default, |color| Style::default().fg(tui_color(color)))
}

fn border_style_for_colors(colors: &TuiColors) -> Style {
    colors.border.map_or_else(
        || muted_style_for_colors(colors),
        |color| Style::default().fg(tui_color(color)),
    )
}

fn separator_style_for_colors(colors: &TuiColors) -> Style {
    colors.separator.map_or_else(
        || muted_style_for_colors(colors),
        |color| Style::default().fg(tui_color(color)),
    )
}

fn status_style_for_colors(colors: &TuiColors) -> Option<Style> {
    colors
        .status
        .map(|color| Style::default().fg(tui_color(color)))
}

fn success_style_for_colors(colors: &TuiColors) -> Style {
    colors
        .success
        .map_or_else(|| Style::default().green(), |color| Style::default().fg(tui_color(color)))
}

fn error_style_for_colors(colors: &TuiColors) -> Style {
    colors
        .error
        .map_or_else(|| Style::default().red(), |color| Style::default().fg(tui_color(color)))
}

fn selection_style_for_colors(terminal_bg: Option<(u8, u8, u8)>, colors: &TuiColors) -> Style {
    let mut style = accent_style_for_colors(terminal_bg, colors);
    if let Some(fg) = colors.selection_fg {
        style = style.fg(tui_color(fg));
    }
    if let Some(bg) = colors.selection_bg {
        style = style.bg(tui_color(bg));
    }
    style
}

fn user_message_style_for_colors(
    terminal_bg: Option<(u8, u8, u8)>,
    colors: &TuiColors,
) -> Style {
    surface_style_for_colors(
        foreground_style_for_colors(colors),
        colors.user_message_bg,
        terminal_bg.map(user_message_bg),
    )
}

fn composer_style_for_colors(terminal_bg: Option<(u8, u8, u8)>, colors: &TuiColors) -> Style {
    surface_style_for_colors(
        foreground_style_for_colors(colors),
        colors.composer_bg.or(colors.user_message_bg),
        terminal_bg.map(user_message_bg),
    )
}

fn proposed_plan_style_for_colors(
    terminal_bg: Option<(u8, u8, u8)>,
    colors: &TuiColors,
) -> Style {
    surface_style_for_colors(
        foreground_style_for_colors(colors),
        colors.proposed_plan_bg,
        terminal_bg.map(proposed_plan_bg),
    )
}

fn surface_style_for_colors(
    mut style: Style,
    configured_bg: Option<TuiColor>,
    fallback_bg: Option<Color>,
) -> Style {
    let bg = configured_bg.map(tui_color).or(fallback_bg);
    if let Some(bg) = bg {
        style = style.bg(bg);
    }
    style
}

fn accent_style_for_colors(terminal_bg: Option<(u8, u8, u8)>, colors: &TuiColors) -> Style {
    if let Some(color) = colors.accent {
        return Style::default().fg(tui_color(color)).bold();
    }

    if terminal_bg.is_some_and(is_light) {
        Style::default().fg(best_color(LIGHT_BG_ACCENT_RGB)).bold()
    } else {
        Style::default().fg(Color::Cyan).bold()
    }
}

fn tui_color(color: TuiColor) -> Color {
    best_color(color.rgb())
}

fn table_separator_style_for(
    terminal_fg: Option<(u8, u8, u8)>,
    terminal_bg: Option<(u8, u8, u8)>,
    color_level: StdoutColorLevel,
) -> Style {
    let (Some(fg), Some(bg)) = (terminal_fg, terminal_bg) else {
        return Style::default().dim();
    };
    let separator_rgb = blend(fg, bg, TABLE_SEPARATOR_FG_ALPHA);
    match color_level {
        StdoutColorLevel::TrueColor => Style::default().fg(rgb_color(separator_rgb)),
        StdoutColorLevel::Ansi256 => Style::default().fg(best_color(separator_rgb)),
        StdoutColorLevel::Ansi16 | StdoutColorLevel::Unknown => Style::default().dim(),
    }
}

#[allow(clippy::disallowed_methods)]
pub fn user_message_bg(terminal_bg: (u8, u8, u8)) -> Color {
    let (top, alpha) = if is_light(terminal_bg) {
        ((0, 0, 0), 0.04)
    } else {
        ((255, 255, 255), 0.12)
    };
    best_color(blend(top, terminal_bg, alpha))
}

#[allow(clippy::disallowed_methods)]
pub fn proposed_plan_bg(terminal_bg: (u8, u8, u8)) -> Color {
    user_message_bg(terminal_bg)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use ratatui::style::Modifier;

    #[test]
    fn accent_style_uses_darker_cyan_on_light_backgrounds() {
        let style = accent_style_for(Some((255, 255, 255)));

        assert_eq!(style.fg, Some(best_color(LIGHT_BG_ACCENT_RGB)));
        assert!(style.add_modifier.contains(Modifier::BOLD));
    }

    #[test]
    fn accent_style_uses_cyan_on_dark_or_unknown_backgrounds() {
        let expected = Style::default().fg(Color::Cyan).bold();

        assert_eq!(accent_style_for(Some((0, 0, 0))), expected);
        assert_eq!(accent_style_for(/*terminal_bg*/ None), expected);
    }

    #[test]
    fn semantic_text_styles_fall_back_to_existing_dim_behavior() {
        let colors = TuiColors::default();

        assert_eq!(muted_style_for_colors(&colors), Style::default().dim());
        assert_eq!(metadata_style_for_colors(&colors), Style::default());
        assert_eq!(border_style_for_colors(&colors), Style::default().dim());
        assert_eq!(separator_style_for_colors(&colors), Style::default().dim());
        assert_eq!(status_style_for_colors(&colors), None);
    }

    #[test]
    fn semantic_text_styles_use_configured_rgb_values() {
        let colors = TuiColors {
            muted: Some(TuiColor::new(128, 128, 128)),
            border: Some(TuiColor::new(48, 54, 61)),
            separator: Some(TuiColor::new(64, 64, 64)),
            status: Some(TuiColor::new(121, 192, 255)),
            success: Some(TuiColor::new(63, 185, 80)),
            error: Some(TuiColor::new(248, 81, 73)),
            ..Default::default()
        };

        assert_eq!(
            muted_style_for_colors(&colors),
            Style::default().fg(best_color((128, 128, 128)))
        );
        assert_eq!(
            metadata_style_for_colors(&colors),
            Style::default().fg(best_color((128, 128, 128)))
        );
        assert_eq!(
            border_style_for_colors(&colors),
            Style::default().fg(best_color((48, 54, 61)))
        );
        assert_eq!(
            separator_style_for_colors(&colors),
            Style::default().fg(best_color((64, 64, 64)))
        );
        assert_eq!(
            status_style_for_colors(&colors),
            Some(Style::default().fg(best_color((121, 192, 255))))
        );
        assert_eq!(
            success_style_for_colors(&colors),
            Style::default().fg(best_color((63, 185, 80)))
        );
        assert_eq!(
            error_style_for_colors(&colors),
            Style::default().fg(best_color((248, 81, 73)))
        );
    }

    #[test]
    fn semantic_surface_styles_can_override_foreground_and_background() {
        let colors = TuiColors {
            foreground: Some(TuiColor::new(230, 237, 243)),
            user_message_bg: Some(TuiColor::new(31, 31, 31)),
            composer_bg: Some(TuiColor::new(32, 32, 32)),
            proposed_plan_bg: Some(TuiColor::new(34, 34, 34)),
            ..Default::default()
        };

        assert_eq!(
            user_message_style_for_colors(Some((0, 0, 0)), &colors),
            Style::default()
                .fg(best_color((230, 237, 243)))
                .bg(best_color((31, 31, 31)))
        );
        assert_eq!(
            composer_style_for_colors(Some((0, 0, 0)), &colors),
            Style::default()
                .fg(best_color((230, 237, 243)))
                .bg(best_color((32, 32, 32)))
        );
        assert_eq!(
            proposed_plan_style_for_colors(Some((0, 0, 0)), &colors),
            Style::default()
                .fg(best_color((230, 237, 243)))
                .bg(best_color((34, 34, 34)))
        );
    }

    #[test]
    fn composer_background_falls_back_to_user_message_background() {
        let colors = TuiColors {
            user_message_bg: Some(TuiColor::new(31, 31, 31)),
            ..Default::default()
        };

        assert_eq!(
            composer_style_for_colors(Some((0, 0, 0)), &colors),
            Style::default().bg(best_color((31, 31, 31)))
        );
    }

    #[test]
    fn selection_style_overrides_accent_with_configured_selection_colors() {
        let colors = TuiColors {
            accent: Some(TuiColor::new(88, 166, 255)),
            selection_fg: Some(TuiColor::new(255, 255, 255)),
            selection_bg: Some(TuiColor::new(38, 79, 120)),
            ..Default::default()
        };

        assert_eq!(
            selection_style_for_colors(Some((0, 0, 0)), &colors),
            Style::default()
                .fg(best_color((255, 255, 255)))
                .bg(best_color((38, 79, 120)))
                .bold()
        );
    }

    #[test]
    fn table_separator_blends_toward_dark_background() {
        let style = table_separator_style_for(
            Some((255, 255, 255)),
            Some((0, 0, 0)),
            StdoutColorLevel::TrueColor,
        );

        assert_eq!(style.fg, Some(rgb_color((51, 51, 51))));
    }

    #[test]
    fn table_separator_blends_toward_light_background() {
        let style = table_separator_style_for(
            Some((0, 0, 0)),
            Some((255, 255, 255)),
            StdoutColorLevel::TrueColor,
        );

        assert_eq!(style.fg, Some(rgb_color((204, 204, 204))));
    }

    #[test]
    fn table_separator_dims_when_palette_aware_color_is_unavailable() {
        let expected = Style::default().dim();

        assert_eq!(
            table_separator_style_for(
                Some((255, 255, 255)),
                Some((0, 0, 0)),
                StdoutColorLevel::Ansi16,
            ),
            expected
        );
        assert_eq!(
            table_separator_style_for(
                /*terminal_fg*/ None,
                Some((0, 0, 0)),
                StdoutColorLevel::TrueColor,
            ),
            expected
        );
    }
}
