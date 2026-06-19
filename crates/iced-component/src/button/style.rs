use crate::{Color, Radius, ShadowLayer, ThemePack};

/// Visual role for resolving button theme tokens.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ButtonVariant {
    /// Neutral action using the regular surface palette.
    Standard,
    /// Primary action using the accent palette.
    Primary,
}

/// Resolved theme tokens used by button rendering.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ButtonStyleTokens {
    /// Button background color.
    pub background: Color,
    /// Button text or icon color.
    pub foreground: Color,
    /// Button border color.
    pub border: Color,
    /// Focus ring color.
    pub focus_ring: Color,
    /// Button corner radius.
    pub radius: Radius,
    /// Raised-state shadow.
    pub shadow: ShadowLayer,
}

impl ButtonStyleTokens {
    /// Resolves button style tokens from the theme baseline.
    #[must_use]
    pub fn from_theme(theme: &ThemePack, variant: ButtonVariant) -> Self {
        let palette = &theme.palette;
        let (background, foreground, border) = match variant {
            ButtonVariant::Standard => (palette.surface_raised, palette.text, palette.border),
            ButtonVariant::Primary => (palette.accent, palette.accent_text, palette.accent),
        };

        Self {
            background,
            foreground,
            border,
            focus_ring: palette.focus_ring,
            radius: theme.shape.control_radius,
            shadow: theme.elevation.raised,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ButtonStyleTokens, ButtonVariant};
    use crate::ThemePack;

    #[test]
    fn standard_button_uses_neutral_surface_tokens() {
        let theme = ThemePack::adwaita();
        let style = ButtonStyleTokens::from_theme(&theme, ButtonVariant::Standard);

        assert_eq!(style.background, theme.palette.surface_raised);
        assert_eq!(style.foreground, theme.palette.text);
        assert_eq!(style.border, theme.palette.border);
    }

    #[test]
    fn primary_button_uses_accent_tokens() {
        let theme = ThemePack::adwaita();
        let style = ButtonStyleTokens::from_theme(&theme, ButtonVariant::Primary);

        assert_eq!(style.background, theme.palette.accent);
        assert_eq!(style.foreground, theme.palette.accent_text);
        assert_eq!(style.border, theme.palette.accent);
    }

    #[test]
    fn button_shape_and_elevation_come_from_theme() {
        let theme = ThemePack::adwaita();
        let style = ButtonStyleTokens::from_theme(&theme, ButtonVariant::Standard);

        assert_eq!(style.radius, theme.shape.control_radius);
        assert_eq!(style.shadow, theme.elevation.raised);
        assert!(style.shadow.color().alpha() <= 48);
    }
}
