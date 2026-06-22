use aura_anim_core::timing::{Delay, Direction, IterationCount};
use iced::animation::Easing;

use crate::motion::MotionSpeed;

/// Unresolved transition intent.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MotionTransition {
    /// Duration bucket to use.
    pub speed: MotionSpeed,
    /// Easing curve to use.
    pub easing: Easing,
    /// Start delay before the active animation interval.
    pub delay: Delay,
    /// Playback direction for repeated animations.
    pub direction: Direction,
    /// Number of active iterations.
    pub iterations: IterationCount,
    /// Whether reduced-motion preferences should skip the transition.
    pub follow_reduce_motion: bool,
}

impl MotionTransition {
    /// Creates a transition intent from a speed bucket and easing curve.
    #[must_use]
    pub const fn new(speed: MotionSpeed, easing: Easing) -> Self {
        Self {
            speed,
            easing,
            delay: Delay::ZERO,
            direction: Direction::Normal,
            iterations: IterationCount::ONCE,
            follow_reduce_motion: true,
        }
    }

    /// Default transition for ordinary component state changes.
    #[must_use]
    pub const fn standard() -> Self {
        Self::new(MotionSpeed::Normal, Easing::EaseOut)
    }

    /// Adds a start delay to the transition.
    #[must_use]
    pub const fn with_delay(mut self, delay: Delay) -> Self {
        self.delay = delay;
        self
    }

    /// Sets playback direction for repeated transitions.
    #[must_use]
    pub const fn with_direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    /// Sets the transition iteration count.
    #[must_use]
    pub fn with_iterations(mut self, iterations: impl Into<IterationCount>) -> Self {
        self.iterations = iterations.into();
        self
    }

    /// Sets whether reduced-motion preferences should skip this transition.
    #[must_use]
    pub const fn follow_reduce_motion(mut self, follow: bool) -> Self {
        self.follow_reduce_motion = follow;
        self
    }
}
