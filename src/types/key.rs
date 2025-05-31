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
