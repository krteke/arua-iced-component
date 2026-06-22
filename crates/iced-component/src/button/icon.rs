use aura_anim_core::{MotionError, MotionRuntime};

use super::{
    AnimatedButton, AnimatedButtonSnapshot, ButtonEvent, ButtonInteraction, ButtonVariant,
};
use crate::component::ComponentContext;

/// Icon-style animated button backed by [`AnimatedButton`].
#[derive(Debug)]
pub struct AnimatedIconButton {
    button: AnimatedButton,
}

impl Default for AnimatedIconButton {
    fn default() -> Self {
        Self::new()
    }
}

impl AnimatedIconButton {
    /// Creates a standard circular icon-style button.
    #[must_use]
    pub fn new() -> Self {
        Self::standard()
    }

    /// Creates a standard circular icon-style button.
    #[must_use]
    pub fn standard() -> Self {
        Self::from_button(AnimatedButton::standard(""))
    }

    /// Creates a suggested-action circular icon-style button.
    #[must_use]
    pub fn suggested() -> Self {
        Self::from_button(AnimatedButton::suggested(""))
    }

    /// Creates a destructive-action circular icon-style button.
    #[must_use]
    pub fn destructive() -> Self {
        Self::from_button(AnimatedButton::destructive(""))
    }

    /// Wraps an existing animated button as an icon-style button.
    #[must_use]
    pub fn from_button(button: AnimatedButton) -> Self {
        Self {
            button: button.circular(),
        }
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
        self.button.view(runtime, context).square(34.0)
    }
}

#[cfg(test)]
mod tests {
    use aura_anim_core::MotionRuntime;

    use crate::{
        button::{
            AnimatedButton, AnimatedIconButton, ButtonAppearance, ButtonEvent, ButtonInteraction,
            ButtonStyleState, ButtonVariant,
        },
        component::ComponentContext,
    };

    #[test]
    fn icon_button_wraps_button_as_circular() {
        let icon = AnimatedIconButton::from_button(AnimatedButton::suggested(""));

        assert_eq!(
            icon.variant(),
            ButtonVariant::SUGGESTED.with_appearance(ButtonAppearance::Circular)
        );
    }

    #[test]
    fn icon_button_delegates_interaction_state() {
        let mut runtime = MotionRuntime::new();
        let context = ComponentContext::current();
        let mut icon = AnimatedIconButton::new();

        icon.update(ButtonInteraction::HoverEnter, &mut runtime)
            .unwrap();

        let snapshot = icon.snapshot(&runtime, &context).unwrap();
        assert_eq!(snapshot.style_state, ButtonStyleState::Hovered);
    }

    #[test]
    fn icon_button_delegates_press_events() {
        let mut runtime = MotionRuntime::new();
        let mut icon = AnimatedIconButton::new();

        let action = icon
            .update_event(ButtonEvent::Pressed("open"), &mut runtime)
            .unwrap();

        assert_eq!(action, Some("open"));
    }

    #[cfg(feature = "iced")]
    #[test]
    fn icon_button_builds_iced_view_from_shared_button_builder() {
        use iced::Element;

        let runtime = MotionRuntime::new();
        let context = ComponentContext::current();
        let icon = AnimatedIconButton::new();

        let view = icon
            .view(&runtime, &context)
            .content("i")
            .on_press(())
            .map_event(|_| ());
        let _element: Element<'_, ()> = view.into();
    }
}
