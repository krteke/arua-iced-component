//! Shared motion configuration used by animated components.

mod preferences;
mod tokens;

pub use aura_anim_core::timing::{Duration, Easing, Timing};
pub use preferences::{MotionPreferences, MotionPreferencesController};
pub use tokens::{MotionSpeed, MotionTokens, MotionTransition};
