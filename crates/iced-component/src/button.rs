//! Button style and state primitives.

mod animated;
mod flags;
#[cfg(feature = "iced")]
mod iced;
mod icon;
mod motion;
mod style;

pub use animated::{AnimatedButton, AnimatedButtonSnapshot, ButtonEvent, ButtonInteraction};
#[cfg(feature = "iced")]
pub use iced::{AnimatedButtonView, ButtonContent, button_style};
pub use icon::{AnimatedIconButton, IconButtonSize, IconSource};
pub use motion::ButtonMotion;
pub use style::{
    ButtonResolvedStyle, ButtonRole, ButtonShape, ButtonStyleState, ButtonTreatment, ButtonVariant,
};
