//! Button style and state primitives.

mod animated;
mod flags;
#[cfg(feature = "iced")]
mod iced;
mod motion;
mod style;

pub use animated::{AnimatedButton, AnimatedButtonSnapshot, ButtonEvent, ButtonInteraction};
#[cfg(feature = "iced")]
pub use iced::{AnimatedButtonView, button_style};
pub use motion::ButtonMotion;
pub use style::{
    ButtonAppearance, ButtonResolvedStyle, ButtonRole, ButtonStyleState, ButtonVariant,
};
