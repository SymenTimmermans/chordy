//! ScaleDefinition represents a musical scale with its name, intervals, and optional properties.
use crate::{Interval, ScaleBitmask};

/// ScaleDefinition represents a musical scale with its name, intervals, and optional properties.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScaleDefinition {
    /// A human-readable name for the scale, e.g., "Major", "Minor", "Dorian".
    pub name: &'static str,
    /// Array of intervals that define the scale, e.g., [m2, M3, A4].
    pub intervals: &'static [Interval],
    /// For modes, the degree offset indicates the starting degree in the parent scale.
    pub degree_offset: Option<u8>,
    /// Optional name of the parent scale for modes, e.g., "Major" for Dorian.
    pub mode_of: Option<&'static str>,
    /// Scale bitmask representing the presence of pitch classes in the scale.
    pub bitmask: ScaleBitmask,
}
