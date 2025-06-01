//! Chord quality detection based on intervals.
use crate::traits::HasIntervals;

/// The ChordQuality enum represents the quality of a chord based on the type of third and fifth
/// interval in the chord.
///
///
pub enum ChordQuality {
    /// Major: Major third and perfect fifth
    Major,
    /// Minor: Minor third and perfect fifth
    Minor,
    /// Diminished: Minor third and diminished fifth
    Diminished,
    /// Augmented: Major third and augmented fifth
    Augmented,
}

impl ChordQuality {
    pub fn detect<T>(c: T) -> Option<Self> where T: HasIntervals {
        todo!();
    }
}
