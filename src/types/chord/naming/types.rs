//! Core types for chord naming system
//!
//! This module contains all the enums and structs that represent the intermediate
//! representation of chord names, separating the structure from analysis and rendering logic.

use crate::types::{ChordQuality, Interval, NoteName, RomanNumeral};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Root of a chord - either a concrete note or a roman numeral
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChordRoot {
    /// Concrete note (e.g., C, F#, Bb)
    Note(NoteName),
    /// Roman numeral (e.g., I, ii, V7)
    Roman(RomanNumeral),
}

/// Type of seventh in a chord
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeventhType {
    /// Minor seventh (♭7)
    Minor,
    /// Major seventh (maj7, △7)
    Major,
    /// Diminished seventh (°7)
    Diminished,
    /// Half-diminished seventh (ø7)
    HalfDiminished,
}

/// Extensions beyond the seventh (9, 11, 13)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Extension {
    /// Natural ninth
    Ninth,
    /// Natural eleventh
    Eleventh,
    /// Natural thirteenth
    Thirteenth,
}

/// Altered chord tones
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Alteration {
    /// Flat fifth (♭5)
    FlatFifth,
    /// Sharp fifth (#5)
    SharpFifth,
    /// Flat ninth (♭9)
    FlatNinth,
    /// Sharp ninth (#9)
    SharpNinth,
    /// Sharp eleventh (#11)
    SharpEleventh,
    /// Flat thirteenth (♭13)
    FlatThirteenth,
    /// Sharp thirteenth (#13)
    SharpThirteenth,
}

/// Added tones (non-extension intervals)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AddedTone {
    /// Added second/ninth
    Second,
    /// Added fourth
    Fourth,
    /// Added sixth
    Sixth,
    /// Added ninth (legacy name for Second)
    Ninth,
    /// Added eleventh (compound fourth)
    Eleventh,
}

/// Suspensions (replacements for the third)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Suspension {
    /// Sus2
    Second,
    /// Sus4
    Fourth,
}

/// Omitted chord tones
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Omission {
    /// No third (power chords)
    Third,
    /// No fifth
    Fifth,
    /// No seventh
    Seventh,
    /// No eleventh (common in 13th chords)
    Eleventh,
}

/// Complete intermediate representation of a chord name
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChordName {
    /// The root of the chord (note or roman numeral)
    pub root: ChordRoot,
    /// Basic quality (major, minor, diminished, augmented)
    pub quality: ChordQuality,
    /// Seventh type if present
    pub seventh: Option<SeventhType>,
    /// Extensions (9, 11, 13) in order
    pub extensions: Vec<Extension>,
    /// Alterations to chord tones
    pub alterations: Vec<Alteration>,
    /// Added tones
    pub added_tones: Vec<AddedTone>,
    /// Suspensions
    pub suspensions: Vec<Suspension>,
    /// Omissions
    pub omissions: Vec<Omission>,
    /// Bass note for slash chords
    pub bass_note: Option<ChordRoot>,
}

impl ChordName {
    /// Create a new basic chord name
    pub fn new(root: ChordRoot, quality: ChordQuality) -> Self {
        Self {
            root,
            quality,
            seventh: None,
            extensions: Vec::new(),
            alterations: Vec::new(),
            added_tones: Vec::new(),
            suspensions: Vec::new(),
            omissions: Vec::new(),
            bass_note: None,
        }
    }

    /// Add a seventh to the chord
    pub fn with_seventh(mut self, seventh: SeventhType) -> Self {
        self.seventh = Some(seventh);
        self
    }

    /// Add an extension to the chord
    pub fn with_extension(mut self, extension: Extension) -> Self {
        if !self.extensions.contains(&extension) {
            self.extensions.push(extension);
            self.extensions.sort();
        }
        self
    }

    /// Add an alteration to the chord
    pub fn with_alteration(mut self, alteration: Alteration) -> Self {
        if !self.alterations.contains(&alteration) {
            self.alterations.push(alteration);
        }
        self
    }

    /// Add an added tone to the chord
    pub fn with_added_tone(mut self, added_tone: AddedTone) -> Self {
        if !self.added_tones.contains(&added_tone) {
            self.added_tones.push(added_tone);
        }
        self
    }

    /// Add a suspension to the chord
    pub fn with_suspension(mut self, suspension: Suspension) -> Self {
        if !self.suspensions.contains(&suspension) {
            self.suspensions.push(suspension);
        }
        self
    }

    /// Add an omission to the chord
    pub fn with_omission(mut self, omission: Omission) -> Self {
        if !self.omissions.contains(&omission) {
            self.omissions.push(omission);
        }
        self
    }

    /// Set the bass note for a slash chord
    pub fn with_bass(mut self, bass: ChordRoot) -> Self {
        self.bass_note = Some(bass);
        self
    }

    /// Get the highest extension present (used to determine base chord type)
    pub fn highest_extension(&self) -> Option<Extension> {
        self.extensions.iter().max().copied()
    }

    /// Check if this is a basic triad (no seventh, extensions, etc.)
    pub fn is_triad(&self) -> bool {
        self.seventh.is_none() && 
        self.extensions.is_empty() && 
        self.added_tones.is_empty() &&
        self.suspensions.is_empty()
    }

    /// Check if the chord has any alterations
    pub fn has_alterations(&self) -> bool {
        !self.alterations.is_empty()
    }

    /// Check if the chord has any omissions
    pub fn has_omissions(&self) -> bool {
        !self.omissions.is_empty()
    }

    /// Check if the chord has any added tones
    pub fn has_added_tones(&self) -> bool {
        !self.added_tones.is_empty()
    }

    /// Check if the chord has any suspensions
    pub fn has_suspensions(&self) -> bool {
        !self.suspensions.is_empty()
    }
}

// Display implementations for all types

impl Display for ChordRoot {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            ChordRoot::Note(note) => write!(f, "{}", note),
            ChordRoot::Roman(roman) => write!(f, "{}", roman),
        }
    }
}

impl Display for SeventhType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            SeventhType::Minor => write!(f, "7"),
            SeventhType::Major => write!(f, "maj7"),
            SeventhType::Diminished => write!(f, "°7"),
            SeventhType::HalfDiminished => write!(f, "ø7"),
        }
    }
}

impl Display for Extension {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Extension::Ninth => write!(f, "9"),
            Extension::Eleventh => write!(f, "11"),
            Extension::Thirteenth => write!(f, "13"),
        }
    }
}

impl Display for Alteration {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Alteration::FlatFifth => write!(f, "♭5"),
            Alteration::SharpFifth => write!(f, "#5"),
            Alteration::FlatNinth => write!(f, "♭9"),
            Alteration::SharpNinth => write!(f, "#9"),
            Alteration::SharpEleventh => write!(f, "#11"),
            Alteration::FlatThirteenth => write!(f, "♭13"),
            Alteration::SharpThirteenth => write!(f, "#13"),
        }
    }
}

impl Display for AddedTone {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            AddedTone::Second => write!(f, "add2"),
            AddedTone::Fourth => write!(f, "add4"),
            AddedTone::Sixth => write!(f, "6"),
            AddedTone::Ninth => write!(f, "add9"),
            AddedTone::Eleventh => write!(f, "add11"),
        }
    }
}

impl Display for Suspension {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Suspension::Second => write!(f, "sus2"),
            Suspension::Fourth => write!(f, "sus4"),
        }
    }
}

impl Display for Omission {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Omission::Third => write!(f, "no3"),
            Omission::Fifth => write!(f, "no5"),
            Omission::Seventh => write!(f, "no7"),
            Omission::Eleventh => write!(f, "no11"),
        }
    }
}

impl Display for ChordName {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        use super::renderer::ChordRenderer;
        write!(f, "{}", ChordRenderer::default().render(self))
    }
}