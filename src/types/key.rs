use super::{Accidental, NoteName};

/// A musical key (combination of tonic and mode)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Key {
    tonic: NoteName,
    mode: Mode,  // Usually just Major or Minor
}

impl Key {
    pub fn new(tonic: NoteName, mode: Mode) -> Self {
        Key { tonic, mode }
    }
}

/// The mode of a key (Major, Minor, etc.)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Major,
    Minor,
    // etc.
}

/// Represents a key signature with sharps or flats
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeySignature {
    // Number of sharps (positive) or flats (negative)
    pub accidentals: i8,
    
    // Maps each letter to its default accidental in this key
    pub letter_map: [Accidental; 7],
}
