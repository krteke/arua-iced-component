use aura_anim_core::{MotionError, MotionRuntime};
use std::borrow::Cow;
use std::path::PathBuf;

use super::{
    AnimatedButton, AnimatedButtonSnapshot, ButtonEvent, ButtonInteraction, ButtonVariant,
};
use crate::component::ComponentContext;

/// Icon source rendered inside an [`AnimatedIconButton`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IconSource {
    /// SVG loaded from a filesystem path.
    SvgPath(PathBuf),
    /// SVG loaded from static in-memory bytes.
    SvgBytes(Cow<'static, [u8]>),
    /// Explicit text fallback, useful for tests or icon fonts.
    Text(String),
}

impl IconSource {
    /// Creates an SVG icon source from a filesystem path.
    #[must_use]
    pub fn svg_path(path: impl Into<PathBuf>) -> Self {
        Self::SvgPath(path.into())
    }

    /// Creates an SVG icon source from static in-memory bytes.
    #[must_use]
    pub fn svg_bytes(bytes: impl Into<Cow<'static, [u8]>>) -> Self {
        Self::SvgBytes(bytes.into())
    }

    /// Creates an explicit text fallback icon source.
    #[must_use]
    pub fn text(text: impl Into<String>) -> Self {
        Self::Text(text.into())
    }

    /// Returns fallback text when this source is text-backed.
    #[must_use]
    pub fn text_fallback(&self) -> Option<&str> {
        match self {
            Self::Text(text) => Some(text),
            Self::SvgPath(_) | Self::SvgBytes(_) => None,
        }
    }
}

/// Icon button control size.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IconButtonSize {
    /// Theme default icon button size.
    Default,
    /// Theme compact icon button size.
    Compact,
    /// Explicit fixed square size in pixels.
    Fixed(f32),
}

/// Icon-style animated button backed by [`AnimatedButton`].
#[derive(Debug)]
pub struct AnimatedIconButton {
    button: AnimatedButton,
    icon: IconSource,
    size: IconButtonSize,
}

impl AnimatedIconButton {
    /// Creates a standard SVG icon-style button from static bytes.
    #[must_use]
    pub fn svg_bytes(bytes: impl Into<Cow<'static, [u8]>>) -> Self {
        Self::standard(IconSource::svg_bytes(bytes))
    }

    /// Creates a standard SVG icon-style button from a filesystem path.
    #[must_use]
    pub fn svg_path(path: impl Into<PathBuf>) -> Self {
        Self::standard(IconSource::svg_path(path))
    }

    /// Creates a standard text-fallback icon-style button.
    #[must_use]
    pub fn text(text: impl Into<String>) -> Self {
        Self::standard(IconSource::text(text))
    }

    /// Creates a standard icon-style button.
    #[must_use]
    pub fn new(icon: impl Into<IconSource>) -> Self {
        Self::standard(icon)
    }

    /// Creates a standard icon-style button.
    #[must_use]
    pub fn standard(icon: impl Into<IconSource>) -> Self {
        Self::from_button(icon, AnimatedButton::standard(""))
    }

    /// Creates a suggested-action icon-style button.
    #[must_use]
    pub fn suggested(icon: impl Into<IconSource>) -> Self {
        Self::from_button(icon, AnimatedButton::suggested(""))
    }

    /// Creates a destructive-action icon-style button.
    #[must_use]
    pub fn destructive(icon: impl Into<IconSource>) -> Self {
        Self::from_button(icon, AnimatedButton::destructive(""))
    }

    /// Wraps an existing animated button as an icon-style button.
    #[must_use]
    pub fn from_button(icon: impl Into<IconSource>, button: AnimatedButton) -> Self {
        Self {
            button,
            icon: icon.into(),
            size: IconButtonSize::Default,
        }
    }

    /// Returns this icon button with a different icon source.
    #[must_use]
    pub fn with_icon(mut self, icon: impl Into<IconSource>) -> Self {
        self.icon = icon.into();
        self
    }

    /// Returns this icon button as a standard action.
    #[must_use]
    pub fn as_standard(mut self) -> Self {
        self.button = self.button.as_standard();
        self
    }

    /// Returns this icon button as a suggested action.
    #[must_use]
    pub fn as_suggested(mut self) -> Self {
        self.button = self.button.as_suggested();
        self
    }

    /// Returns this icon button as a destructive action.
    #[must_use]
    pub fn as_destructive(mut self) -> Self {
        self.button = self.button.as_destructive();
        self
    }

    /// Returns this icon button with filled treatment.
    #[must_use]
    pub fn filled(mut self) -> Self {
        self.button = self.button.filled();
        self
    }

    /// Returns this icon button with flat treatment.
    #[must_use]
    pub fn flat(mut self) -> Self {
        self.button = self.button.flat();
        self
    }

    /// Returns this icon button with raised treatment.
    #[must_use]
    pub fn raised(mut self) -> Self {
        self.button = self.button.raised();
        self
    }

    /// Returns this icon button with rounded rectangle shape.
    #[must_use]
    pub fn rounded(mut self) -> Self {
        self.button = self.button.rounded();
        self
    }

    /// Returns this icon button with pill shape.
    #[must_use]
    pub fn pill(mut self) -> Self {
        self.button = self.button.pill();
        self
    }

    /// Returns this icon button with circular shape.
    #[must_use]
    pub fn circular(mut self) -> Self {
        self.button = self.button.circular();
        self
    }

    /// Returns this icon button with compact control size.
    #[must_use]
    pub const fn compact(mut self) -> Self {
        self.size = IconButtonSize::Compact;
        self
    }

    /// Returns this icon button with explicit square size in pixels.
    #[must_use]
    pub const fn size(mut self, size: f32) -> Self {
        self.size = IconButtonSize::Fixed(size);
        self
    }

    /// Returns this icon button with disabled state preconfigured.
    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.button = self.button.disabled(disabled);
        self
    }

    /// Registers the inner button motion handle in the application runtime.
    pub fn register(&mut self, runtime: &mut MotionRuntime, context: &ComponentContext) {
        self.button.register(runtime, context);
    }

    /// Applies an interaction to the inner button.
    pub fn update(
        &mut self,
        interaction: ButtonInteraction,
        runtime: &mut MotionRuntime,
    ) -> Result<bool, MotionError> {
        self.button.update(interaction, runtime)
    }

    /// Applies a button event and returns its application action, if any.
    pub fn update_event<Action>(
        &mut self,
        event: ButtonEvent<Action>,
        runtime: &mut MotionRuntime,
    ) -> Result<Option<Action>, MotionError> {
        self.button.update_event(event, runtime)
    }

    /// Applies a button event and invokes `on_action` when release yields an action.
    pub fn update_event_with<Action>(
        &mut self,
        event: ButtonEvent<Action>,
        runtime: &mut MotionRuntime,
        on_action: impl FnOnce(Action),
    ) -> Result<bool, MotionError> {
        self.button.update_event_with(event, runtime, on_action)
    }

    /// Enables or disables this icon button and updates its motion target.
    pub fn set_disabled(
        &mut self,
        disabled: bool,
        runtime: &mut MotionRuntime,
    ) -> Result<bool, MotionError> {
        self.button.set_disabled(disabled, runtime)
    }

    /// Returns a rendering snapshot of the inner button.
    pub fn snapshot(
        &self,
        runtime: &MotionRuntime,
        context: &ComponentContext,
    ) -> Result<AnimatedButtonSnapshot, MotionError> {
        self.button.snapshot(runtime, context)
    }

    /// Returns this icon button visual variant.
    #[must_use]
    pub const fn variant(&self) -> ButtonVariant {
        self.button.variant()
    }

    /// Returns this icon button source.
    #[must_use]
    pub const fn icon(&self) -> &IconSource {
        &self.icon
    }

    /// Returns this icon button size mode.
    #[must_use]
    pub const fn size_mode(&self) -> IconButtonSize {
        self.size
    }

    /// Returns the inner animated button.
    #[must_use]
    pub const fn as_button(&self) -> &AnimatedButton {
        &self.button
    }
}

#[cfg(feature = "iced")]
impl AnimatedIconButton {
    /// Builds an Iced view for this icon button.
    #[must_use]
    pub fn view<'a, Message>(
        &'a self,
        runtime: &MotionRuntime,
        context: &ComponentContext,
    ) -> super::AnimatedButtonView<'a, Message>
    where
        Message: Clone + 'a,
    {
        use spectrum_theme::iced::IcedColorAdapter;

        let metrics = &context.theme().theme().control.icon_button;
        let size = match self.size {
            IconButtonSize::Default => metrics.size.value(),
            IconButtonSize::Compact => metrics.compact_size.value(),
            IconButtonSize::Fixed(size) => size,
        };
        let icon_size = metrics.icon_size.value();
        let icon_color = self
            .snapshot(runtime, context)
            .expect("button motion handle belongs to the provided runtime")
            .style
            .foreground
            .color();
        let icon: iced::Element<'a, Message> = match &self.icon {
            IconSource::SvgPath(path) => {
                iced::widget::svg(iced::widget::svg::Handle::from_path(path.clone()))
                    .width(iced::Length::Fixed(icon_size))
                    .height(iced::Length::Fixed(icon_size))
                    .style(
                        move |_theme: &iced::Theme, _status| iced::widget::svg::Style {
                            color: Some(icon_color),
                        },
                    )
                    .into()
            }
            IconSource::SvgBytes(bytes) => {
                iced::widget::svg(iced::widget::svg::Handle::from_memory(bytes.clone()))
                    .width(iced::Length::Fixed(icon_size))
                    .height(iced::Length::Fixed(icon_size))
                    .style(
                        move |_theme: &iced::Theme, _status| iced::widget::svg::Style {
                            color: Some(icon_color),
                        },
                    )
                    .into()
            }
            IconSource::Text(text) => iced::widget::text(text.as_str()).size(icon_size).into(),
        };

        self.button
            .view(runtime, context)
            .element(icon)
            .square(size)
    }
}

#[cfg(test)]
mod tests {
    use aura_anim_core::MotionRuntime;

    use crate::{
        button::{
            AnimatedButton, AnimatedIconButton, ButtonEvent, ButtonInteraction, ButtonShape,
            ButtonStyleState, ButtonTreatment, ButtonVariant, IconButtonSize, IconSource,
        },
        component::ComponentContext,
    };

    const TEST_ICON: &[u8] = br#"<svg viewBox="0 0 16 16"><path d="M3 8h10v2H3z"/></svg>"#;

    #[test]
    fn icon_button_wraps_button_without_forcing_shape() {
        let icon = AnimatedIconButton::from_button(
            IconSource::svg_bytes(TEST_ICON),
            AnimatedButton::suggested(""),
        );

        assert_eq!(icon.variant(), ButtonVariant::SUGGESTED);
        assert!(matches!(icon.icon(), IconSource::SvgBytes(_)));
    }

    #[test]
    fn standard_icon_button_defaults_to_filled_rounded() {
        let icon = AnimatedIconButton::svg_bytes(TEST_ICON);

        assert_eq!(icon.variant(), ButtonVariant::STANDARD);
    }

    #[test]
    fn icon_button_exposes_role_treatment_shape_and_size_builders() {
        let icon = AnimatedIconButton::svg_bytes(TEST_ICON)
            .as_destructive()
            .flat()
            .circular()
            .compact();

        assert_eq!(icon.variant().role, crate::button::ButtonRole::Destructive);
        assert_eq!(icon.variant().treatment, ButtonTreatment::Flat);
        assert_eq!(icon.variant().shape, ButtonShape::Circular);
        assert_eq!(icon.size_mode(), IconButtonSize::Compact);
    }

    #[test]
    fn icon_button_delegates_interaction_state() {
        let mut runtime = MotionRuntime::new();
        let context = ComponentContext::current();
        let mut icon = AnimatedIconButton::svg_bytes(TEST_ICON);

        icon.update(ButtonInteraction::HoverEnter, &mut runtime)
            .unwrap();

        let snapshot = icon.snapshot(&runtime, &context).unwrap();
        assert_eq!(snapshot.style_state, ButtonStyleState::Hovered);
    }

    #[test]
    fn icon_button_delegates_press_events() {
        let mut runtime = MotionRuntime::new();
        let mut icon = AnimatedIconButton::svg_bytes(TEST_ICON);

        let action = icon
            .update_event(ButtonEvent::Pressed("open"), &mut runtime)
            .unwrap();

        assert_eq!(action, Some("open"));
    }

    #[test]
    fn icon_button_can_change_source() {
        let icon = AnimatedIconButton::svg_bytes(TEST_ICON).with_icon(IconSource::text("!"));

        assert_eq!(icon.icon().text_fallback(), Some("!"));
    }

    #[test]
    fn text_icon_source_is_explicit_fallback() {
        let icon = AnimatedIconButton::text("!");

        assert_eq!(icon.icon().text_fallback(), Some("!"));
    }

    #[cfg(feature = "iced")]
    #[test]
    fn icon_button_builds_iced_view_from_shared_button_builder() {
        use iced::Element;

        let runtime = MotionRuntime::new();
        let context = ComponentContext::current();
        let icon = AnimatedIconButton::svg_bytes(TEST_ICON);

        let view = icon.view(&runtime, &context).on_press(()).map_event(|_| ());
        let _element: Element<'_, ()> = view.into();
    }
}
