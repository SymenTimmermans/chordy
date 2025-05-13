use super::{NoteName, Mode};

/// A scale with a tonic and mode
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scale {
    tonic: NoteName,
    mode: ScaleType,
}

impl Scale {
    pub fn new(tonic: NoteName, mode: ScaleType) -> Self {
        Scale { tonic, mode }
    }

    pub fn notes(&self) -> Vec<NoteName> {
        // Generate notes based on tonic and mode
        // This is a placeholder implementation
        vec![self.tonic]
    }
}

/// Types of musical scales
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScaleType {
    Major,
    NaturalMinor,
    HarmonicMinor,
    MelodicMinor,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Locrian,
    // etc.
}
