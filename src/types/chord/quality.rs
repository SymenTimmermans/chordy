//! Chord quality detection based on intervals.
use crate::{traits::HasIntervals, Interval};

/// The ChordQuality enum represents the quality of a chord based on the type of third and fifth
/// interval in the chord.
///
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    /// Detects the quality of a type based on its intervals.
    pub fn detect<T>(c: &T) -> Option<Self> where T: HasIntervals {
        let intervals = c.intervals();
        
        // Count occurrences of each type of third and fifth
        let mut major_thirds = 0;
        let mut minor_thirds = 0;
        let mut perfect_fifths = 0;
        let mut diminished_fifths = 0;
        let mut augmented_fifths = 0;

        for interval in intervals {
            match *interval {
                Interval::MAJOR_THIRD => major_thirds += 1,
                Interval::MINOR_THIRD => minor_thirds += 1,
                Interval::PERFECT_FIFTH => perfect_fifths += 1,
                Interval::DIMINISHED_FIFTH => diminished_fifths += 1,
                Interval::AUGMENTED_FIFTH => augmented_fifths += 1,
                _ => continue,
            }
        }

        // Determine the most prevalent third and fifth types
        let third_type = match (major_thirds, minor_thirds) {
            (maj, min) if maj > min => Some(true),  // Major third
            (maj, min) if min > maj => Some(false), // Minor third
            (0, 0) => None,                         // No third
            _ => None,                              // Equal number - ambiguous
        };

        let fifth_type = match (perfect_fifths, diminished_fifths, augmented_fifths) {
            (perf, dim, aug) if perf > dim && perf > aug => Some(0),  // Perfect
            (perf, dim, aug) if dim > perf && dim > aug => Some(-1),  // Diminished
            (perf, dim, aug) if aug > perf && aug > dim => Some(1),   // Augmented
            (0, 0, 0) => None,                                       // No fifth
            _ => None,                                                // Ambiguous
        };

        // Determine quality based on third and fifth types
        match (third_type, fifth_type) {
            (Some(true), Some(0)) => Some(ChordQuality::Major),
            (Some(false), Some(0)) => Some(ChordQuality::Minor),
            (Some(false), Some(-1)) => Some(ChordQuality::Diminished),
            (Some(true), Some(1)) => Some(ChordQuality::Augmented),
            // Classify diads as major/minor
            (Some(false), None) => Some(ChordQuality::Minor),
            (Some(true), None) => Some(ChordQuality::Major),
            // Any other combination is ambiguous
            _ => None, 
        }
    }
}
