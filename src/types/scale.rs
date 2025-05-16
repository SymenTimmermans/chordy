use super::NoteName;

/// A scale with a tonic and mode
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scale {
    tonic: NoteName,
    mode: ScaleType,
}

/// A scale is a sequence of notes that defines a musical key.
/// ```
/// use chordy::{NoteName, Letter, Accidental, Scale, ScaleType};
///
/// // Create a C major scale
/// let c = NoteName::new(Letter::C, Accidental::Natural);
/// let c_major = Scale::new(c, ScaleType::Major);
///
/// // Get the notes in the scale
/// let notes = c_major.notes();
/// // [C, D, E, F, G, A, B]
/// assert_eq!(notes, vec![
///     NoteName::new(Letter::C, Accidental::Natural),
///     NoteName::new(Letter::D, Accidental::Natural),
///     NoteName::new(Letter::E, Accidental::Natural),
///     NoteName::new(Letter::F, Accidental::Natural),
///     NoteName::new(Letter::G, Accidental::Natural),
///     NoteName::new(Letter::A, Accidental::Natural),
///     NoteName::new(Letter::B, Accidental::Natural),
/// ]);
/// ```
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
