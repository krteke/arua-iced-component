use aura_anim_core::SpringConfig;

/// Named spring presets shared by physics-based component motion.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MotionSpring {
    /// Gentle, low-energy motion for large or ambient changes.
    Soft,
    /// Default spring for ordinary spatial movement.
    Balanced,
    /// Firmer spring for direct manipulation feedback.
    Firm,
    /// Responsive spring for tight control feedback.
    Snappy,
}

/// Spring token values shared by animated components.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MotionSpringTokens {
    /// Gentle spring for large or ambient changes.
    pub soft: SpringConfig,
    /// Default spring for ordinary spatial movement.
    pub balanced: SpringConfig,
    /// Firmer spring for direct manipulation feedback.
    pub firm: SpringConfig,
    /// Responsive spring for tight control feedback.
    pub snappy: SpringConfig,
}

impl Default for MotionSpringTokens {
    fn default() -> Self {
        Self {
            soft: SpringConfig::new(120.0, 20.0),
            balanced: SpringConfig::default(),
            firm: SpringConfig::new(320.0, 28.0),
            snappy: SpringConfig::snappy(),
        }
    }
}
