//! Shared motion configuration used by animated components.

mod preferences;
mod spring;
mod tokens;
mod transition;

pub use aura_anim_core::SpringConfig;
pub use aura_anim_core::timing::{Delay, Direction, Duration, Easing, IterationCount, Timing};
pub use preferences::{MotionPreferences, MotionPreferencesController};
pub use spring::{MotionSpring, MotionSpringTokens};
pub use tokens::{MotionSpeed, MotionTokens};
pub use transition::MotionTransition;
