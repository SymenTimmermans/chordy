//! ScaleDefinition represents a musical scale with its name, intervals, and optional properties.
use crate::{Interval, ScaleBitmask};

/// ScaleDefinition represents a musical scale with its name, intervals, and optional properties.
///
/// It is mainly used to define scales in a structured way, allowing for easy reference and
/// manipulation.
/// The ScaleDefinition struct implements equality checks against `Scale`, so you can easily
/// check if a given scale is matches a known scale:
///
/// ```rust
/// use chordy::prelude::*;
///
/// let c_major = Scale::from_definition(note!("C"), scales::IONIAN);
/// let custom_major = Scale::custom(
///     note!("C"),
///     "Custom Major Name",
///     scales::IONIAN.intervals.to_vec(),
///     None,
///     None
/// );
///
/// assert_eq!(c_major, scales::IONIAN); // true - same intervals
/// assert_eq!(custom_major, scales::IONIAN); // true - same intervals
/// ```
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
