use super::NoteName;

/// The mode of a key (Major, Minor, etc.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    Major(NoteName),
    Minor(NoteName),
}

/// Represents a key signature with sharps or flats
impl Key {
    // Number of sharps (positive) or flats (negative)
    pub fn accidentals(&self) -> i8 {
        match self {
            Key::Major(note) | Key::Minor(note) => note.fifths(),
        }
    }
}
