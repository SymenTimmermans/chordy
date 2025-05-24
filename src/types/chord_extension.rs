

use crate::IntervalDirection;

use super::interval::{Interval, IntervalSize, IntervalQuality};

/// Extensions and alterations that can be added to basic chord triads
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChordExtension {
    /// 7th chords (dominant 7, major 7, etc.)
    Seventh(SeventhType),
    
    /// 9th extension (adds 9th above root)
    Ninth(NinthType),
    
    /// 11th extension (adds 11th above root)
    Eleventh(EleventhType),
    
    /// 13th extension (adds 13th above root)
    Thirteenth(ThirteenthType),
    
    /// Added notes that aren't standard extensions (add2, add4, etc.)
    Add(AddedNote),
    
    /// Suspended notes (sus2, sus4)
    Sus(SuspendedType),
    
    /// Altered fifth (e.g., ♭5, ♯5)
    AlteredFifth(AlteredFifthType),
    
    /// Altered ninth (e.g., ♭9, ♯9)
    AlteredNinth(AlteredNinthType),
    
    /// Omitted notes (e.g., no3, no5)
    Omit(OmittedNote),
}

/// Types of seventh chords
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SeventhType {
    /// Dominant seventh (♭7)
    Dominant,
    
    /// Major seventh (major triad with major 7th)
    Major,
    
    /// Minor seventh (minor triad with minor 7th)
    Minor,
    
    /// Half-diminished seventh (diminished triad with minor 7th)
    HalfDiminished,
    
    /// Diminished seventh (diminished triad with diminished 7th)
    Diminished,
}

/// Types of ninth extensions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NinthType {
    /// Standard ninth (major 9th)
    Natural,
    
    /// Flat ninth (♭9)
    Flat,
    
    /// Sharp ninth (♯9)
    Sharp,
}

/// Types of eleventh extensions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EleventhType {
    /// Standard eleventh (perfect 11th)
    Natural,
    
    /// Sharp eleventh (♯11)
    Sharp,
}

/// Types of thirteenth extensions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThirteenthType {
    /// Standard thirteenth (major 13th)
    Natural,
    
    /// Flat thirteenth (♭13)
    Flat,
}

/// Added notes not part of standard extensions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AddedNote {
    /// Added 2nd/9th without 7th
    Add2,
    
    /// Added 4th/11th without 7th and 9th
    Add4,
    
    /// Added 6th
    Add6,
    
    /// Added ♭6th
    AddFlat6,
}

/// Suspended chord types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SuspendedType {
    /// Suspended 2nd (replaces 3rd with 2nd)
    Sus2,
    
    /// Suspended 4th (replaces 3rd with 4th)
    Sus4,
}

/// Altered fifth variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AlteredFifthType {
    /// Flat fifth (♭5)
    Flat,
    
    /// Sharp fifth (♯5)
    Sharp,
}

/// Altered ninth variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AlteredNinthType {
    /// Flat ninth (♭9)
    Flat,
    
    /// Sharp ninth (♯9)
    Sharp,
}

/// Notes that can be omitted from chords
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OmittedNote {
    /// Omitted 3rd
    No3,
    
    /// Omitted 5th
    No5,
}

impl ChordExtension {
    /// Returns the intervals this extension adds to a chord
    pub fn get_intervals(&self) -> Vec<Interval> {
        use IntervalSize::*;
        use IntervalQuality::*;

        match self {
            ChordExtension::Seventh(seventh_type) => match seventh_type {
                SeventhType::Dominant => vec![Interval::new(Minor, Seventh, IntervalDirection::Ascending)],
                SeventhType::Major => vec![Interval::new(Major, Seventh, IntervalDirection::Ascending)],
                SeventhType::Minor => vec![Interval::new(Minor, Seventh, IntervalDirection::Ascending)],
                SeventhType::HalfDiminished => vec![Interval::new(Minor, Seventh, IntervalDirection::Ascending)],
                SeventhType::Diminished => vec![Interval::new(Diminished(1), Seventh, IntervalDirection::Ascending)],
            },
            ChordExtension::Ninth(ninth_type) => match ninth_type {
                NinthType::Natural => vec![Interval::new(Major, Ninth, IntervalDirection::Ascending)],
                NinthType::Flat => vec![Interval::new(Minor, Ninth, IntervalDirection::Ascending)],
                NinthType::Sharp => vec![Interval::new(Augmented(1), Ninth, IntervalDirection::Ascending)],
            },
            ChordExtension::Eleventh(eleventh_type) => match eleventh_type {
                EleventhType::Natural => vec![Interval::new(Perfect, Eleventh, IntervalDirection::Ascending)],
                EleventhType::Sharp => vec![Interval::new(Augmented(1), Eleventh, IntervalDirection::Ascending)],
            },
            ChordExtension::Thirteenth(thirteenth_type) => match thirteenth_type {
                ThirteenthType::Natural => vec![Interval::new(Major, Thirteenth, IntervalDirection::Ascending)],
                ThirteenthType::Flat => vec![Interval::new(Minor, Thirteenth, IntervalDirection::Ascending)],
            },
            ChordExtension::Add(added_note) => match added_note {
                AddedNote::Add2 => vec![Interval::new(Major, Second, IntervalDirection::Ascending)],
                AddedNote::Add4 => vec![Interval::new(Perfect, Fourth, IntervalDirection::Ascending)],
                AddedNote::Add6 => vec![Interval::new(Major, Sixth, IntervalDirection::Ascending)],
                AddedNote::AddFlat6 => vec![Interval::new(Minor, Sixth, IntervalDirection::Ascending)],
            },
            ChordExtension::Sus(sus_type) => match sus_type {
                SuspendedType::Sus2 => vec![Interval::new(Major, Second, IntervalDirection::Ascending)],
                SuspendedType::Sus4 => vec![Interval::new(Perfect, Fourth, IntervalDirection::Ascending)],
            },
            ChordExtension::AlteredFifth(alt_fifth) => match alt_fifth {
                AlteredFifthType::Flat => vec![Interval::new(Diminished(1), Fifth, IntervalDirection::Ascending)],
                AlteredFifthType::Sharp => vec![Interval::new(Augmented(1), Fifth, IntervalDirection::Ascending)],
            },
            ChordExtension::AlteredNinth(alt_ninth) => match alt_ninth {
                AlteredNinthType::Flat => vec![Interval::new(Minor, Ninth, IntervalDirection::Ascending)],
                AlteredNinthType::Sharp => vec![Interval::new(Augmented(1), Ninth, IntervalDirection::Ascending)],
            },
            ChordExtension::Omit(omit) => match omit {
                OmittedNote::No3 => vec![],
                OmittedNote::No5 => vec![],
            },
        }
    }
}
