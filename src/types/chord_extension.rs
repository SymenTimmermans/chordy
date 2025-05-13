use std::fmt;

use crate::error::ParseError;

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
