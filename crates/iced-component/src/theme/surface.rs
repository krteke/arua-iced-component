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
        let (background, radius, shadow) = match role {
            SurfaceRole::Background => (theme.app.bg, theme.surface.base.radius, None),
            SurfaceRole::Surface => (theme.surface.base.bg, theme.surface.base.radius, None),
            SurfaceRole::Raised => (
                theme.surface.raised.bg,
                theme.surface.raised.radius,
                Some(theme.surface.raised.shadow),
            ),
        };

        Self {
            background,
            foreground: match role {
                SurfaceRole::Background => theme.app.fg,
                SurfaceRole::Surface => theme.surface.base.fg,
                SurfaceRole::Raised => theme.surface.raised.fg,
            },
            border: match role {
                SurfaceRole::Background | SurfaceRole::Surface => theme.surface.base.border,
                SurfaceRole::Raised => theme.surface.raised.border,
            },
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

        assert_eq!(style.background, theme.app.bg);
        assert_eq!(style.foreground, theme.app.fg);
        assert_eq!(style.shadow, None);
    }

    #[test]
    fn raised_surface_uses_theme_elevation() {
        let theme = ThemePack::adwaita();
        let style = SurfaceStyleTokens::from_theme(&theme, SurfaceRole::Raised);

        assert_eq!(style.background, theme.surface.raised.bg);
        assert_eq!(style.radius, theme.surface.raised.radius);
        assert_eq!(style.shadow, Some(theme.surface.raised.shadow));
    }
}
