use crate::{Color, Radius, ShadowLayer, ThemePack};

/// Visual role for resolving surface theme tokens.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SurfaceRole {
    /// App or page background.
    Background,
    /// Regular component surface.
    Surface,
    /// Raised panel, popover, or interactive container.
    Raised,
}

/// Resolved theme tokens used by surface-like components.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SurfaceStyleTokens {
    /// Surface fill color.
    pub background: Color,
    /// Default foreground color.
    pub foreground: Color,
    /// Surface border color.
    pub border: Color,
    /// Surface corner radius.
    pub radius: Radius,
    /// Surface shadow.
    pub shadow: Option<ShadowLayer>,
}

impl SurfaceStyleTokens {
    /// Resolves surface style tokens from the theme baseline.
    #[must_use]
    pub fn from_theme(theme: &ThemePack, role: SurfaceRole) -> Self {
        let palette = &theme.palette;
        let (background, radius, shadow) = match role {
            SurfaceRole::Background => (palette.background, theme.shape.panel_radius, None),
            SurfaceRole::Surface => (palette.surface, theme.shape.panel_radius, None),
            SurfaceRole::Raised => (
                palette.surface_raised,
                theme.shape.panel_radius,
                Some(theme.elevation.raised),
            ),
        };

        Self {
            background,
            foreground: palette.text,
            border: palette.border,
            radius,
            shadow,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{SurfaceRole, SurfaceStyleTokens};
    use crate::ThemePack;

    #[test]
    fn background_surface_has_no_shadow() {
        let theme = ThemePack::adwaita();
        let style = SurfaceStyleTokens::from_theme(&theme, SurfaceRole::Background);

        assert_eq!(style.background, theme.palette.background);
        assert_eq!(style.foreground, theme.palette.text);
        assert_eq!(style.shadow, None);
    }

    #[test]
    fn raised_surface_uses_theme_elevation() {
        let theme = ThemePack::adwaita();
        let style = SurfaceStyleTokens::from_theme(&theme, SurfaceRole::Raised);

        assert_eq!(style.background, theme.palette.surface_raised);
        assert_eq!(style.radius, theme.shape.panel_radius);
        assert_eq!(style.shadow, Some(theme.elevation.raised));
    }
}
