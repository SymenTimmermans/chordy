use crate::traits::HasRoot;

use super::NoteName;

/// The mode of a key (Major, Minor, etc.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    /// Major key signature
    ///
    /// This does not imply a specific scale, just the key signature.
    Major(NoteName),
    /// Minor key signature
    ///
    /// This does not imply a specific scale, just the key signature.
    Minor(NoteName),
}

/// Represents a key signature with sharps or flats
impl Key {
    /// Number of sharps (positive) or flats (negative)
    pub fn accidentals(&self) -> i8 {
        match self {
            Key::Major(note) | Key::Minor(note) => note.fifths(),
        }
    }
}

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Key::Major(note) => write!(f, "{} Major", note),
            Key::Minor(note) => write!(f, "{} Minor", note),
        }
    }
}

impl HasRoot for Key {
    fn root(&self) -> NoteName {
        match self {
            Key::Major(note) => *note,
            Key::Minor(note) => *note,
        }
    }

    fn root_mut(&mut self) -> &mut NoteName {
        match self {
            Key::Major(note) => note,
            Key::Minor(note) => note,
        }
    }
}

