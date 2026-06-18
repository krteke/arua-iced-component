//! Shared motion preferences used by animated components.

use std::cell::Cell;
use std::rc::Rc;

/// Read-only motion preferences passed to components.
///
/// Clones share the same backing state, so applications can update preferences
/// once and every component can observe the latest value during `view`.
#[derive(Clone, Debug)]
pub struct MotionPreferences {
    reduce_motion: Rc<Cell<bool>>,
}

/// Application-side controller for shared motion preferences.
#[derive(Clone, Debug)]
pub struct MotionPreferencesController {
    reduce_motion: Rc<Cell<bool>>,
}

impl MotionPreferences {
    /// Creates shared motion preferences and the controller that can mutate them.
    #[must_use]
    pub fn new(reduce_motion: bool) -> (Self, MotionPreferencesController) {
        let reduce_motion = Rc::new(Cell::new(reduce_motion));

        (
            Self {
                reduce_motion: Rc::clone(&reduce_motion),
            },
            MotionPreferencesController { reduce_motion },
        )
    }

    /// Returns whether components should avoid non-essential animation.
    #[must_use]
    pub fn reduce_motion(&self) -> bool {
        self.reduce_motion.get()
    }

    /// Returns `true` when two preference readers observe the same state.
    #[must_use]
    pub fn is_shared_with(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.reduce_motion, &other.reduce_motion)
    }
}

impl Default for MotionPreferences {
    fn default() -> Self {
        Self::new(false).0
    }
}

impl MotionPreferencesController {
    /// Creates a read-only preference handle for components.
    #[must_use]
    pub fn preferences(&self) -> MotionPreferences {
        MotionPreferences {
            reduce_motion: Rc::clone(&self.reduce_motion),
        }
    }

    /// Updates whether components should avoid non-essential animation.
    pub fn set_reduce_motion(&self, reduce_motion: bool) {
        self.reduce_motion.set(reduce_motion);
    }

    /// Returns whether reduced motion is currently enabled.
    #[must_use]
    pub fn reduce_motion(&self) -> bool {
        self.reduce_motion.get()
    }
}

#[cfg(test)]
mod tests {
    use super::MotionPreferences;

    #[test]
    fn defaults_to_full_motion() {
        let preferences = MotionPreferences::default();

        assert!(!preferences.reduce_motion());
    }

    #[test]
    fn controller_updates_all_readers() {
        let (preferences, controller) = MotionPreferences::new(false);
        let component_preferences = controller.preferences();

        controller.set_reduce_motion(true);

        assert!(preferences.reduce_motion());
        assert!(component_preferences.reduce_motion());
        assert!(controller.reduce_motion());
    }

    #[test]
    fn cloned_readers_share_state() {
        let (preferences, controller) = MotionPreferences::new(false);
        let cloned = preferences.clone();

        controller.set_reduce_motion(true);

        assert!(preferences.is_shared_with(&cloned));
        assert!(cloned.reduce_motion());
    }
}
