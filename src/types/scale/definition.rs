use crate::{Interval, ScaleBitmask};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScaleDefinition {
    pub name: &'static str,
    pub intervals: &'static [Interval], // m2, M3, A4, etc.
    pub degree_offset: Option<u8>,
    pub mode_of: Option<&'static str>,  // name of the parent scale
    pub bitmask: ScaleBitmask, // optional computed field, not canonical
}
