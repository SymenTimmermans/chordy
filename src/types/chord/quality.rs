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
    /// Detects the quality of a chord based on its intervals.
    /// 
    /// Uses simple boolean logic to determine chord quality by checking for the presence
    /// of specific third and fifth intervals. This method is more straightforward than
    /// counting intervals and handles the common chord quality cases effectively.
    pub fn detect<T>(chord: &T) -> Option<Self> 
    where 
        T: HasIntervals 
    {
        Self::from_intervals(chord.intervals())
    }
    
    /// Determine chord quality from a slice of intervals.
    /// 
    /// This is the core logic used by both the generic `detect` method and the 
    /// chord analyzer. It prioritizes clarity and correctness over complex counting.
    pub fn from_intervals(intervals: &[Interval]) -> Option<Self> {
        let has_major_third = intervals.contains(&Interval::MAJOR_THIRD);
        let has_minor_third = intervals.contains(&Interval::MINOR_THIRD);
        let has_perfect_fifth = intervals.contains(&Interval::PERFECT_FIFTH);
        let has_diminished_fifth = intervals.contains(&Interval::DIMINISHED_FIFTH);
        let has_augmented_fifth = intervals.contains(&Interval::AUGMENTED_FIFTH);
        
        match (has_major_third, has_minor_third, has_perfect_fifth, has_diminished_fifth, has_augmented_fifth) {
            // Augmented: Major third + Augmented fifth (no perfect fifth)
            (true, false, false, false, true) => Some(ChordQuality::Augmented),
            // Diminished: Minor third + Diminished fifth (no perfect fifth)
            (false, true, false, true, false) => Some(ChordQuality::Diminished),
            // Minor: Minor third (regardless of fifth, as long as it's not diminished-only)
            (false, true, _, _, _) => Some(ChordQuality::Minor),
            // Major: Major third (regardless of fifth, as long as it's not augmented-only)
            (true, false, _, _, _) => Some(ChordQuality::Major),
            // No clear third: return None (ambiguous)
            _ => None,
        }
    }
}
