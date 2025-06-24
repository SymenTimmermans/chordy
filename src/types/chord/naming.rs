//! Chord naming system with intermediate representation and flexible rendering
//!
//! This module provides a structured approach to chord naming that separates
//! chord analysis from display formatting, allowing for multiple naming conventions
//! and output formats.

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

/// Extensions beyond the basic triad and seventh
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Extension {
    /// Ninth (9)
    Ninth,
    /// Eleventh (11) 
    Eleventh,
    /// Thirteenth (13)
    Thirteenth,
}

/// Alterations to chord tones
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

/// Added tones that don't follow standard extension rules
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AddedTone {
    /// Added second (add2)
    Second,
    /// Added fourth (add4)
    Fourth,
    /// Added sixth (add6)
    Sixth,
    /// Added ninth (add9)
    Ninth,
}

/// Suspended chords where the third is replaced
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Suspension {
    /// Second suspension (sus2)
    Second,
    /// Fourth suspension (sus4)
    Fourth,
}

/// Omitted notes from an otherwise implied chord
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Omission {
    /// No third (no3)
    Third,
    /// No fifth (no5)
    Fifth,
    /// No seventh (no7)
    Seventh,
    /// No ninth (no9)
    Ninth,
    /// No eleventh (no11)
    Eleventh,
    /// No thirteenth (no13)
    Thirteenth,
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
        self.seventh.is_none() 
            && self.extensions.is_empty() 
            && self.added_tones.is_empty()
            && self.suspensions.is_empty()
    }

    /// Check if this has alterations
    pub fn has_alterations(&self) -> bool {
        !self.alterations.is_empty()
    }

    /// Check if this has omissions
    pub fn has_omissions(&self) -> bool {
        !self.omissions.is_empty()
    }
    
    /// Render this chord name using a specific renderer
    pub fn render_with(&self, renderer: &ChordRenderer) -> String {
        renderer.render(self)
    }
    
    /// Render as ASCII text (no Unicode symbols)
    pub fn to_ascii(&self) -> String {
        ChordRenderer::ascii().render(self)
    }
    
    /// Render as HTML with entities
    pub fn to_html(&self) -> String {
        ChordRenderer::html().render(self)
    }
    
    /// Render with classical notation (△7 instead of maj7)
    pub fn to_classical(&self) -> String {
        ChordRenderer::new(ChordFormat::Unicode, NamingConvention::Classical).render(self)
    }
}

// Display implementations for rendering flexibility

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
            AddedTone::Sixth => write!(f, "add6"),
            AddedTone::Ninth => write!(f, "add9"),
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
            Omission::Ninth => write!(f, "no9"),
            Omission::Eleventh => write!(f, "no11"),
            Omission::Thirteenth => write!(f, "no13"),
        }
    }
}

impl Display for ChordName {
    /// Basic display implementation - produces a standard chord symbol
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        // Start with root
        write!(f, "{}", self.root)?;
        
        // Handle special case for half-diminished seventh
        if let Some(SeventhType::HalfDiminished) = self.seventh {
            write!(f, "ø7")?;
            
            // Add extensions and alterations (but skip ♭5 since it's implied in ø7)
            if let Some(ext) = self.highest_extension() {
                write!(f, "{}", ext)?;
            }
            
            for alteration in &self.alterations {
                // Skip flat fifth for half-diminished as it's already implied
                if !matches!(alteration, Alteration::FlatFifth) {
                    write!(f, "{}", alteration)?;
                }
            }
        } else {
            // Add quality suffix for diminished/augmented
            match self.quality {
                ChordQuality::Diminished => write!(f, "°")?,
                ChordQuality::Augmented => write!(f, "+")?,
                _ => {},
            }
            
            // Add highest extension if present, otherwise add seventh
            if let Some(ext) = self.highest_extension() {
                write!(f, "{}", ext)?;
            } else if let Some(seventh) = &self.seventh {
                write!(f, "{}", seventh)?;
            }
            
            // Add alterations
            for alteration in &self.alterations {
                write!(f, "{}", alteration)?;
            }
        }
        
        // Add suspensions
        for suspension in &self.suspensions {
            write!(f, "{}", suspension)?;
        }
        
        // Add added tones
        for added_tone in &self.added_tones {
            write!(f, "{}", added_tone)?;
        }
        
        // Add omissions
        for omission in &self.omissions {
            write!(f, "{}", omission)?;
        }
        
        // Add bass note for slash chords
        if let Some(bass) = &self.bass_note {
            write!(f, "/{}", bass)?;
        }
        
        Ok(())
    }
}

/// Rendering format for chord names
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChordFormat {
    /// Unicode symbols (♯, ♭, °, ø, △, etc.)
    Unicode,
    /// ASCII symbols (#, b, dim, hdim, maj, etc.)
    Ascii,
    /// HTML entities (&sharp;, &flat;, etc.)
    Html,
}

/// Naming convention for chord symbols
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NamingConvention {
    /// Jazz/pop convention (7, maj7, 13, etc.)
    Jazz,
    /// Classical convention (more explicit notation)
    Classical,
    /// Lead sheet style (simplified for readability)
    LeadSheet,
}

/// Renderer for converting ChordName to formatted strings
pub struct ChordRenderer {
    format: ChordFormat,
    convention: NamingConvention,
}

impl ChordRenderer {
    /// Create a new renderer with specified format and convention
    pub fn new(format: ChordFormat, convention: NamingConvention) -> Self {
        Self { format, convention }
    }
    
    /// Create a Unicode jazz renderer (default style)
    pub fn unicode_jazz() -> Self {
        Self::new(ChordFormat::Unicode, NamingConvention::Jazz)
    }
    
    /// Create an ASCII renderer for plain text
    pub fn ascii() -> Self {
        Self::new(ChordFormat::Ascii, NamingConvention::Jazz)
    }
    
    /// Create an HTML renderer for web display
    pub fn html() -> Self {
        Self::new(ChordFormat::Html, NamingConvention::Jazz)
    }
    
    /// Create a legacy compatibility renderer that matches the old abbreviated_name() behavior
    pub fn legacy() -> Self {
        Self::new(ChordFormat::Unicode, NamingConvention::LeadSheet)
    }
    
    /// Render a chord name to string
    pub fn render(&self, chord_name: &ChordName) -> String {
        let mut result = String::new();
        
        // Render root with quality-aware formatting
        result.push_str(&self.render_root_with_quality(&chord_name.root, chord_name.quality));
        
        // Handle special case for half-diminished seventh
        if let Some(SeventhType::HalfDiminished) = chord_name.seventh {
            result.push_str(&self.render_half_diminished_for_root(&chord_name.root));
            
            // Add extensions and alterations (but skip ♭5 since it's implied)
            if let Some(ext) = chord_name.highest_extension() {
                result.push_str(&self.render_extension(ext));
            }
            
            for alteration in &chord_name.alterations {
                if !matches!(alteration, Alteration::FlatFifth) {
                    result.push_str(&self.render_alteration(*alteration));
                }
            }
        } else {
            // Render quality suffix for diminished/augmented (skip minor for Roman numerals)
            result.push_str(&self.render_quality_for_root(chord_name.quality, &chord_name.root));
            
            // Render highest extension if present, otherwise render seventh
            if let Some(ext) = chord_name.highest_extension() {
                result.push_str(&self.render_extension_with_seventh(ext, chord_name.seventh, chord_name.quality));
            } else if let Some(seventh) = chord_name.seventh {
                // Special handling for minor-major chords in legacy mode
                if self.convention == NamingConvention::LeadSheet 
                    && chord_name.quality == ChordQuality::Minor 
                    && seventh == SeventhType::Major {
                    result.push_str("(maj7)");
                } else {
                    result.push_str(&self.render_seventh(seventh));
                }
            }
            
            // Render alterations
            for alteration in &chord_name.alterations {
                result.push_str(&self.render_alteration(*alteration));
            }
        }
        
        // Render suspensions
        for suspension in &chord_name.suspensions {
            result.push_str(&self.render_suspension(*suspension));
        }
        
        // Render added tones
        for added_tone in &chord_name.added_tones {
            result.push_str(&self.render_added_tone(*added_tone, chord_name.seventh.is_some()));
        }
        
        // Render omissions
        for omission in &chord_name.omissions {
            result.push_str(&self.render_omission(*omission));
        }
        
        // Render bass note for slash chords
        if let Some(bass) = &chord_name.bass_note {
            result.push('/');
            result.push_str(&self.render_root(bass));
        }
        
        result
    }
    
    fn render_root(&self, root: &ChordRoot) -> String {
        match root {
            ChordRoot::Note(note) => self.render_note_name(note),
            ChordRoot::Roman(roman) => format!("{}", roman),
        }
    }
    
    fn render_root_with_quality(&self, root: &ChordRoot, quality: ChordQuality) -> String {
        match root {
            ChordRoot::Note(note) => self.render_note_name(note),
            ChordRoot::Roman(roman) => roman.display_for_quality(quality),
        }
    }
    
    fn render_note_name(&self, note: &NoteName) -> String {
        match self.format {
            ChordFormat::Unicode => format!("{}", note),
            ChordFormat::Ascii => note.to_string().replace('♯', "#").replace('♭', "b"),
            ChordFormat::Html => note.to_string()
                .replace('♯', "&sharp;")
                .replace('♭', "&flat;")
                .replace('𝄪', "&x;")
                .replace('𝄫', "&bb;"),
        }
    }
    
    fn render_quality(&self, quality: ChordQuality) -> String {
        match (quality, self.format, self.convention) {
            // Legacy/LeadSheet convention uses "m", "dim", "aug"
            (ChordQuality::Minor, _, NamingConvention::LeadSheet) => "m".to_string(),
            (ChordQuality::Diminished, _, NamingConvention::LeadSheet) => "dim".to_string(),
            (ChordQuality::Augmented, _, NamingConvention::LeadSheet) => "aug".to_string(),
            // Standard conventions
            (ChordQuality::Diminished, ChordFormat::Unicode, _) => "°".to_string(),
            (ChordQuality::Diminished, ChordFormat::Ascii, _) => "dim".to_string(),
            (ChordQuality::Diminished, ChordFormat::Html, _) => "&deg;".to_string(),
            (ChordQuality::Augmented, ChordFormat::Unicode, _) => "+".to_string(),
            (ChordQuality::Augmented, ChordFormat::Ascii, _) => "aug".to_string(),
            (ChordQuality::Augmented, ChordFormat::Html, _) => "+".to_string(),
            _ => String::new(),
        }
    }
    
    fn render_quality_for_root(&self, quality: ChordQuality, root: &ChordRoot) -> String {
        match root {
            ChordRoot::Note(_) => self.render_quality(quality),
            ChordRoot::Roman(_) => {
                // For Roman numerals, quality is shown by case and symbols, not suffix letters
                // Only render symbols for diminished/augmented (already handled in display_for_quality)
                String::new()
            }
        }
    }
    
    fn render_seventh(&self, seventh: SeventhType) -> String {
        match seventh {
            SeventhType::Minor => "7".to_string(),
            SeventhType::Major => match (self.format, self.convention) {
                (ChordFormat::Unicode, NamingConvention::Classical) => "△7".to_string(),
                (ChordFormat::Html, NamingConvention::Classical) => "&Delta;7".to_string(),
                _ => "maj7".to_string(),
            },
            SeventhType::Diminished => match self.format {
                ChordFormat::Unicode => "°7".to_string(),
                ChordFormat::Ascii => "dim7".to_string(),
                ChordFormat::Html => "&deg;7".to_string(),
            },
            SeventhType::HalfDiminished => unreachable!("Handled separately"),
        }
    }
    
    fn render_half_diminished(&self) -> String {
        match self.format {
            ChordFormat::Unicode => "ø7".to_string(),
            ChordFormat::Ascii => "m7b5".to_string(),
            ChordFormat::Html => "&oslash;7".to_string(),
        }
    }
    
    fn render_half_diminished_for_root(&self, root: &ChordRoot) -> String {
        match root {
            ChordRoot::Note(_) => self.render_half_diminished(),
            ChordRoot::Roman(_) => {
                // In Roman numeral analysis, half-diminished 7th chords are often written as °7
                // especially for the vii chord in major keys
                "7".to_string()
            }
        }
    }
    
    fn render_extension(&self, extension: Extension) -> String {
        match extension {
            Extension::Ninth => "9".to_string(),
            Extension::Eleventh => "11".to_string(),
            Extension::Thirteenth => "13".to_string(),
        }
    }
    
    fn render_extension_with_seventh(&self, extension: Extension, seventh: Option<SeventhType>, quality: ChordQuality) -> String {
        match (extension, seventh, quality, self.convention) {
            // Legacy: Major extensions with major 7th in major chords show "maj"
            (Extension::Ninth, Some(SeventhType::Major), ChordQuality::Major, NamingConvention::LeadSheet) => "maj9".to_string(),
            (Extension::Eleventh, Some(SeventhType::Major), ChordQuality::Major, NamingConvention::LeadSheet) => "maj11".to_string(),
            (Extension::Thirteenth, Some(SeventhType::Major), ChordQuality::Major, NamingConvention::LeadSheet) => "maj13".to_string(),
            // Legacy: Minor chords with major 7th and extensions use parentheses
            (Extension::Ninth, Some(SeventhType::Major), ChordQuality::Minor, NamingConvention::LeadSheet) => "(maj9)".to_string(),
            (Extension::Eleventh, Some(SeventhType::Major), ChordQuality::Minor, NamingConvention::LeadSheet) => "(maj11)".to_string(),
            (Extension::Thirteenth, Some(SeventhType::Major), ChordQuality::Minor, NamingConvention::LeadSheet) => "(maj13)".to_string(),
            // Standard behavior
            _ => self.render_extension(extension),
        }
    }
    
    fn render_alteration(&self, alteration: Alteration) -> String {
        match (alteration, self.format) {
            (Alteration::FlatFifth, ChordFormat::Unicode) => "♭5".to_string(),
            (Alteration::FlatFifth, ChordFormat::Ascii) => "b5".to_string(),
            (Alteration::FlatFifth, ChordFormat::Html) => "&flat;5".to_string(),
            (Alteration::SharpFifth, ChordFormat::Unicode) => "♯5".to_string(),
            (Alteration::SharpFifth, ChordFormat::Ascii) => "#5".to_string(),
            (Alteration::SharpFifth, ChordFormat::Html) => "&sharp;5".to_string(),
            (Alteration::FlatNinth, ChordFormat::Unicode) => "♭9".to_string(),
            (Alteration::FlatNinth, ChordFormat::Ascii) => "b9".to_string(),
            (Alteration::FlatNinth, ChordFormat::Html) => "&flat;9".to_string(),
            (Alteration::SharpNinth, ChordFormat::Unicode) => "♯9".to_string(),
            (Alteration::SharpNinth, ChordFormat::Ascii) => "#9".to_string(),
            (Alteration::SharpNinth, ChordFormat::Html) => "&sharp;9".to_string(),
            (Alteration::SharpEleventh, ChordFormat::Unicode) => "♯11".to_string(),
            (Alteration::SharpEleventh, ChordFormat::Ascii) => "#11".to_string(),
            (Alteration::SharpEleventh, ChordFormat::Html) => "&sharp;11".to_string(),
            (Alteration::FlatThirteenth, ChordFormat::Unicode) => "♭13".to_string(),
            (Alteration::FlatThirteenth, ChordFormat::Ascii) => "b13".to_string(),
            (Alteration::FlatThirteenth, ChordFormat::Html) => "&flat;13".to_string(),
            (Alteration::SharpThirteenth, ChordFormat::Unicode) => "♯13".to_string(),
            (Alteration::SharpThirteenth, ChordFormat::Ascii) => "#13".to_string(),
            (Alteration::SharpThirteenth, ChordFormat::Html) => "&sharp;13".to_string(),
        }
    }
    
    fn render_suspension(&self, suspension: Suspension) -> String {
        match suspension {
            Suspension::Second => "sus2".to_string(),
            Suspension::Fourth => "sus4".to_string(),
        }
    }
    
    fn render_added_tone(&self, added_tone: AddedTone, has_seventh: bool) -> String {
        let base = match (added_tone, self.convention) {
            // Legacy convention: fourth becomes add11
            (AddedTone::Fourth, NamingConvention::LeadSheet) => "add11".to_string(),
            // Standard conventions
            (AddedTone::Second, _) => "add2".to_string(),
            (AddedTone::Fourth, _) => "add4".to_string(),
            (AddedTone::Sixth, _) => "add6".to_string(),
            (AddedTone::Ninth, _) => "add9".to_string(),
        };
        
        // In legacy mode, use parentheses for added tones when seventh is present
        if self.convention == NamingConvention::LeadSheet && has_seventh {
            format!("({})", base)
        } else {
            base
        }
    }
    
    fn render_omission(&self, omission: Omission) -> String {
        match (omission, self.convention) {
            // Legacy convention uses parentheses for omissions
            (Omission::Eleventh, NamingConvention::LeadSheet) => "(no11)".to_string(),
            // Standard conventions
            (Omission::Third, _) => "no3".to_string(),
            (Omission::Fifth, _) => "no5".to_string(),
            (Omission::Seventh, _) => "no7".to_string(),
            (Omission::Ninth, _) => "no9".to_string(),
            (Omission::Eleventh, _) => "no11".to_string(),
            (Omission::Thirteenth, _) => "no13".to_string(),
        }
    }
}

impl Default for ChordRenderer {
    fn default() -> Self {
        Self::unicode_jazz()
    }
}

/// Analyzes chord intervals and converts them to ChordName representation
pub struct ChordAnalyzer;

impl ChordAnalyzer {
    /// Analyze a chord's intervals and produce a ChordName
    pub fn analyze(root: NoteName, intervals: &[Interval]) -> ChordName {
        let chord_root = ChordRoot::Note(root);
        Self::analyze_with_root(chord_root, intervals)
    }
    
    /// Analyze a chord's intervals with a given root (Note or Roman) and produce a ChordName
    pub fn analyze_with_root(chord_root: ChordRoot, intervals: &[Interval]) -> ChordName {
        // Start with basic quality analysis
        let quality = Self::determine_quality(intervals);
        let mut chord_name = ChordName::new(chord_root, quality);
        
        // Analyze seventh
        if let Some(seventh_type) = Self::analyze_seventh(intervals) {
            chord_name = chord_name.with_seventh(seventh_type);
        }
        
        // Analyze extensions
        let extensions = Self::analyze_extensions(intervals);
        for extension in extensions {
            chord_name = chord_name.with_extension(extension);
        }
        
        // Analyze alterations
        let alterations = Self::analyze_alterations(intervals);
        for alteration in alterations {
            chord_name = chord_name.with_alteration(alteration);
        }
        
        // Analyze added tones
        let added_tones = Self::analyze_added_tones(intervals);
        for added_tone in added_tones {
            chord_name = chord_name.with_added_tone(added_tone);
        }
        
        // Analyze suspensions
        let suspensions = Self::analyze_suspensions(intervals);
        for suspension in suspensions {
            chord_name = chord_name.with_suspension(suspension);
        }
        
        // Analyze omissions (compared to expected chord type)
        let omissions = Self::analyze_omissions(intervals, &chord_name);
        for omission in omissions {
            chord_name = chord_name.with_omission(omission);
        }
        
        chord_name
    }
    
    /// Determine the basic chord quality from intervals
    fn determine_quality(intervals: &[Interval]) -> ChordQuality {
        let has_major_third = intervals.contains(&Interval::MAJOR_THIRD);
        let has_minor_third = intervals.contains(&Interval::MINOR_THIRD);
        let has_perfect_fifth = intervals.contains(&Interval::PERFECT_FIFTH);
        let has_diminished_fifth = intervals.contains(&Interval::DIMINISHED_FIFTH);
        let has_augmented_fifth = intervals.contains(&Interval::AUGMENTED_FIFTH);
        
        match (has_major_third, has_minor_third, has_perfect_fifth, has_diminished_fifth, has_augmented_fifth) {
            (true, false, false, false, true) => ChordQuality::Augmented,
            (false, true, false, true, false) => ChordQuality::Diminished,
            (false, true, _, _, _) => ChordQuality::Minor,
            (true, false, _, _, _) => ChordQuality::Major,
            _ => ChordQuality::Major, // Default fallback
        }
    }
    
    /// Analyze seventh intervals
    fn analyze_seventh(intervals: &[Interval]) -> Option<SeventhType> {
        if intervals.contains(&Interval::DIMINISHED_SEVENTH) {
            Some(SeventhType::Diminished)
        } else if intervals.contains(&Interval::MINOR_SEVENTH) {
            // Check if it's half-diminished (minor 7th with diminished 5th and minor 3rd)
            if intervals.contains(&Interval::DIMINISHED_FIFTH) && intervals.contains(&Interval::MINOR_THIRD) {
                Some(SeventhType::HalfDiminished)
            } else {
                Some(SeventhType::Minor)
            }
        } else if intervals.contains(&Interval::MAJOR_SEVENTH) {
            Some(SeventhType::Major)
        } else {
            None
        }
    }
    
    /// Analyze extension intervals (9, 11, 13)
    fn analyze_extensions(intervals: &[Interval]) -> Vec<Extension> {
        let mut extensions = Vec::new();
        
        let has_seventh = intervals.contains(&Interval::MINOR_SEVENTH) || 
                         intervals.contains(&Interval::MAJOR_SEVENTH);
        
        // Only treat as extensions if we have a seventh (extensions imply the seventh)
        if has_seventh {
            // Check for both simple and compound intervals that indicate extensions
            // 9th: major second OR major ninth
            let has_ninth = intervals.contains(&Interval::MAJOR_NINTH) || 
                           intervals.contains(&Interval::MINOR_NINTH) ||
                           intervals.contains(&Interval::MAJOR_SECOND);
            
            if has_ninth {
                extensions.push(Extension::Ninth);
            }
            
            // 11th: perfect fourth OR perfect eleventh (but only if 9th is also present)
            let has_eleventh = intervals.contains(&Interval::PERFECT_ELEVENTH) || 
                              intervals.contains(&Interval::AUGMENTED_ELEVENTH) ||
                              intervals.contains(&Interval::PERFECT_FOURTH);
                              
            if has_eleventh && has_ninth {
                extensions.push(Extension::Eleventh);
            }
            
            // 13th: major sixth OR major thirteenth (requires 9th and typically 11th)
            let has_thirteenth = intervals.contains(&Interval::MAJOR_THIRTEENTH) || 
                               intervals.contains(&Interval::MINOR_THIRTEENTH) ||
                               intervals.contains(&Interval::MAJOR_SIXTH) ||
                               intervals.contains(&Interval::MINOR_SIXTH);
                               
            if has_thirteenth && has_ninth {
                extensions.push(Extension::Thirteenth);
            }
        }
        
        extensions.sort();
        extensions
    }
    
    /// Analyze altered intervals
    fn analyze_alterations(intervals: &[Interval]) -> Vec<Alteration> {
        let mut alterations = Vec::new();
        
        // Don't add flat fifth for diminished chords (it's implied by the quality)
        let has_diminished_quality = intervals.contains(&Interval::MINOR_THIRD) && 
                                   intervals.contains(&Interval::DIMINISHED_FIFTH);
        
        // Don't add sharp fifth for augmented chords (it's implied by the quality)
        let has_augmented_quality = intervals.contains(&Interval::MAJOR_THIRD) && 
                                  intervals.contains(&Interval::AUGMENTED_FIFTH);
        
        if intervals.contains(&Interval::DIMINISHED_FIFTH) && !has_diminished_quality {
            alterations.push(Alteration::FlatFifth);
        }
        
        if intervals.contains(&Interval::AUGMENTED_FIFTH) && !has_augmented_quality {
            alterations.push(Alteration::SharpFifth);
        }
        
        if intervals.contains(&Interval::MINOR_NINTH) {
            alterations.push(Alteration::FlatNinth);
        }
        
        if intervals.contains(&Interval::AUGMENTED_NINTH) {
            alterations.push(Alteration::SharpNinth);
        }
        
        if intervals.contains(&Interval::AUGMENTED_ELEVENTH) {
            alterations.push(Alteration::SharpEleventh);
        }
        
        if intervals.contains(&Interval::MINOR_THIRTEENTH) {
            alterations.push(Alteration::FlatThirteenth);
        }
        
        if intervals.contains(&Interval::AUGMENTED_THIRTEENTH) {
            alterations.push(Alteration::SharpThirteenth);
        }
        
        alterations.sort();
        alterations
    }
    
    /// Analyze added tones (intervals that don't follow extension patterns)
    fn analyze_added_tones(intervals: &[Interval]) -> Vec<AddedTone> {
        let mut added_tones = Vec::new();
        
        let has_major_third = intervals.contains(&Interval::MAJOR_THIRD);
        let has_minor_third = intervals.contains(&Interval::MINOR_THIRD);
        let has_seventh = intervals.contains(&Interval::MINOR_SEVENTH) || intervals.contains(&Interval::MAJOR_SEVENTH);
        
        // Add9 - major second or ninth without seventh (becomes add9, not add2 in legacy)
        if (intervals.contains(&Interval::MAJOR_SECOND) || intervals.contains(&Interval::MAJOR_NINTH)) && 
           (has_major_third || has_minor_third) && // Not a suspension
           !has_seventh {
            added_tones.push(AddedTone::Ninth);
        }
        
        // Add11 - perfect fourth with third present (not suspension) 
        // Can be with or without seventh, but if seventh is present, 9th must be absent
        let has_ninth = intervals.contains(&Interval::MAJOR_SECOND) || 
                       intervals.contains(&Interval::MAJOR_NINTH) ||
                       intervals.contains(&Interval::MINOR_NINTH);
                       
        if intervals.contains(&Interval::PERFECT_FOURTH) && 
           (has_major_third || has_minor_third) &&
           (!has_seventh || (has_seventh && !has_ninth)) {
            added_tones.push(AddedTone::Fourth); // Will be rendered as add11 in legacy mode
        }
        
        // Add6 - major sixth without seventh
        if intervals.contains(&Interval::MAJOR_SIXTH) && !has_seventh {
            added_tones.push(AddedTone::Sixth);
        }
        
        added_tones.sort();
        added_tones
    }
    
    /// Analyze suspensions (replacements for the third)
    fn analyze_suspensions(intervals: &[Interval]) -> Vec<Suspension> {
        let mut suspensions = Vec::new();
        
        let has_major_third = intervals.contains(&Interval::MAJOR_THIRD);
        let has_minor_third = intervals.contains(&Interval::MINOR_THIRD);
        
        // Sus2 - has major second but no third
        if intervals.contains(&Interval::MAJOR_SECOND) && !has_major_third && !has_minor_third {
            suspensions.push(Suspension::Second);
        }
        
        // Sus4 - has perfect fourth but no third
        if intervals.contains(&Interval::PERFECT_FOURTH) && !has_major_third && !has_minor_third {
            suspensions.push(Suspension::Fourth);
        }
        
        suspensions.sort();
        suspensions
    }
    
    /// Analyze omissions (notes expected but missing)
    fn analyze_omissions(intervals: &[Interval], chord_name: &ChordName) -> Vec<Omission> {
        let mut omissions = Vec::new();
        
        // Legacy behavior: Don't explicitly note missing fifths in extended chords
        // Only show omissions for specific patterns like 13(no11)
        
        // Check for missing 11th in 13th chords (specific legacy pattern)
        if chord_name.extensions.contains(&Extension::Thirteenth) {
            let has_eleventh = intervals.contains(&Interval::PERFECT_FOURTH) ||
                             intervals.contains(&Interval::PERFECT_ELEVENTH);
            if !has_eleventh {
                omissions.push(Omission::Eleventh);
            }
        }
        
        // Check for missing third in power chords (only when there are no suspensions)
        if !intervals.contains(&Interval::MAJOR_THIRD) && 
           !intervals.contains(&Interval::MINOR_THIRD) &&
           chord_name.suspensions.is_empty() &&
           chord_name.seventh.is_none() &&
           chord_name.extensions.is_empty() {
            omissions.push(Omission::Third);
        }
        
        omissions.sort();
        omissions
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::note;

    #[test]
    fn test_chord_name_creation() {
        let chord_name = ChordName::new(
            ChordRoot::Note(note!("C")),
            ChordQuality::Major,
        );
        
        assert_eq!(chord_name.quality, ChordQuality::Major);
        assert!(chord_name.is_triad());
        assert!(!chord_name.has_alterations());
    }

    #[test]
    fn test_chord_name_builder() {
        let chord_name = ChordName::new(
            ChordRoot::Note(note!("C")),
            ChordQuality::Major,
        )
        .with_seventh(SeventhType::Minor)
        .with_extension(Extension::Ninth)
        .with_alteration(Alteration::SharpEleventh);
        
        assert_eq!(chord_name.seventh, Some(SeventhType::Minor));
        assert_eq!(chord_name.highest_extension(), Some(Extension::Ninth));
        assert!(chord_name.has_alterations());
        assert!(!chord_name.is_triad());
    }

    #[test]
    fn test_new_enum_variants() {
        // Test HalfDiminished seventh
        let half_dim = ChordName::new(
            ChordRoot::Note(note!("C")),
            ChordQuality::Diminished,
        ).with_seventh(SeventhType::HalfDiminished);
        
        assert_eq!(half_dim.seventh, Some(SeventhType::HalfDiminished));
        
        // Test new alterations
        let altered_chord = ChordName::new(
            ChordRoot::Note(note!("C")),
            ChordQuality::Major,
        )
        .with_alteration(Alteration::SharpThirteenth)
        .with_added_tone(AddedTone::Ninth)
        .with_omission(Omission::Seventh);
        
        assert!(altered_chord.alterations.contains(&Alteration::SharpThirteenth));
        assert!(altered_chord.added_tones.contains(&AddedTone::Ninth));
        assert!(altered_chord.omissions.contains(&Omission::Seventh));
    }

    #[test]
    fn test_display_implementations() {
        // Test seventh type display
        assert_eq!(format!("{}", SeventhType::HalfDiminished), "ø7");
        assert_eq!(format!("{}", SeventhType::Major), "maj7");
        
        // Test alteration display
        assert_eq!(format!("{}", Alteration::SharpThirteenth), "#13");
        assert_eq!(format!("{}", Alteration::FlatFifth), "♭5");
        
        // Test added tone display
        assert_eq!(format!("{}", AddedTone::Ninth), "add9");
        
        // Test omission display
        assert_eq!(format!("{}", Omission::Seventh), "no7");
        
        // Test chord name display
        let chord_name = ChordName::new(
            ChordRoot::Note(note!("C")),
            ChordQuality::Major,
        ).with_seventh(SeventhType::Minor);
        
        assert_eq!(format!("{}", chord_name), "C7");
    }

    #[test]
    fn test_complex_chord_display() {
        let complex_chord = ChordName::new(
            ChordRoot::Note(note!("F#")),
            ChordQuality::Minor,
        )
        .with_seventh(SeventhType::Minor)
        .with_extension(Extension::Thirteenth)
        .with_alteration(Alteration::SharpEleventh)
        .with_bass(ChordRoot::Note(note!("A")));
        
        let display = format!("{}", complex_chord);
        // Note: F# displays as F♯ (Unicode sharp symbol)
        assert!(display.contains("F♯"));
        assert!(display.contains("13"));
        assert!(display.contains("#11"));
        assert!(display.contains("/A"));
    }

    #[test]
    fn test_chord_analyzer_basic_triads() {
        // C Major
        let c_major = ChordAnalyzer::analyze(
            note!("C"),
            &[Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH]
        );
        assert_eq!(c_major.quality, ChordQuality::Major);
        assert!(c_major.is_triad());

        // A Minor
        let a_minor = ChordAnalyzer::analyze(
            note!("A"),
            &[Interval::PERFECT_UNISON, Interval::MINOR_THIRD, Interval::PERFECT_FIFTH]
        );
        assert_eq!(a_minor.quality, ChordQuality::Minor);
        assert!(a_minor.is_triad());

        // B Diminished
        let b_dim = ChordAnalyzer::analyze(
            note!("B"),
            &[Interval::PERFECT_UNISON, Interval::MINOR_THIRD, Interval::DIMINISHED_FIFTH]
        );
        assert_eq!(b_dim.quality, ChordQuality::Diminished);

        // C Augmented
        let c_aug = ChordAnalyzer::analyze(
            note!("C"),
            &[Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::AUGMENTED_FIFTH]
        );
        assert_eq!(c_aug.quality, ChordQuality::Augmented);
    }

    #[test]
    fn test_chord_analyzer_seventh_chords() {
        // G7 (dominant 7th)
        let g7 = ChordAnalyzer::analyze(
            note!("G"),
            &[Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH]
        );
        assert_eq!(g7.seventh, Some(SeventhType::Minor));
        assert_eq!(format!("{}", g7), "G7");

        // Cmaj7
        let cmaj7 = ChordAnalyzer::analyze(
            note!("C"),
            &[Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MAJOR_SEVENTH]
        );
        assert_eq!(cmaj7.seventh, Some(SeventhType::Major));
        assert_eq!(format!("{}", cmaj7), "Cmaj7");

        // Bø7 (half-diminished)
        let b_half_dim = ChordAnalyzer::analyze(
            note!("B"),
            &[Interval::PERFECT_UNISON, Interval::MINOR_THIRD, Interval::DIMINISHED_FIFTH, Interval::MINOR_SEVENTH]
        );
        assert_eq!(b_half_dim.seventh, Some(SeventhType::HalfDiminished));
        assert_eq!(format!("{}", b_half_dim), "Bø7");
    }

    #[test]
    fn test_chord_analyzer_extensions() {
        // C9
        let c9 = ChordAnalyzer::analyze(
            note!("C"),
            &[Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, 
              Interval::MINOR_SEVENTH, Interval::MAJOR_NINTH]
        );
        assert_eq!(c9.seventh, Some(SeventhType::Minor));
        assert_eq!(c9.highest_extension(), Some(Extension::Ninth));

        // F13
        let f13 = ChordAnalyzer::analyze(
            note!("F"),
            &[Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, 
              Interval::MINOR_SEVENTH, Interval::MAJOR_NINTH, Interval::MAJOR_THIRTEENTH]
        );
        assert_eq!(f13.highest_extension(), Some(Extension::Thirteenth));
    }

    #[test]
    fn test_chord_analyzer_alterations() {
        // C7♭5
        let c7_flat5 = ChordAnalyzer::analyze(
            note!("C"),
            &[Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::DIMINISHED_FIFTH, Interval::MINOR_SEVENTH]
        );
        assert!(c7_flat5.alterations.contains(&Alteration::FlatFifth));

        // G7#11
        let g7_sharp11 = ChordAnalyzer::analyze(
            note!("G"),
            &[Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, 
              Interval::MINOR_SEVENTH, Interval::AUGMENTED_ELEVENTH]
        );
        assert!(g7_sharp11.alterations.contains(&Alteration::SharpEleventh));
    }

    #[test]
    fn test_chord_analyzer_suspensions() {
        // Csus2
        let csus2 = ChordAnalyzer::analyze(
            note!("C"),
            &[Interval::PERFECT_UNISON, Interval::MAJOR_SECOND, Interval::PERFECT_FIFTH]
        );
        assert!(csus2.suspensions.contains(&Suspension::Second));
        assert_eq!(format!("{}", csus2), "Csus2");

        // Dsus4
        let dsus4 = ChordAnalyzer::analyze(
            note!("D"),
            &[Interval::PERFECT_UNISON, Interval::PERFECT_FOURTH, Interval::PERFECT_FIFTH]
        );
        assert!(dsus4.suspensions.contains(&Suspension::Fourth));
        assert_eq!(format!("{}", dsus4), "Dsus4");
    }

    #[test]
    fn test_chord_analyzer_added_tones() {
        // Cadd9
        let cadd9 = ChordAnalyzer::analyze(
            note!("C"),
            &[Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MAJOR_NINTH]
        );
        assert!(cadd9.added_tones.contains(&AddedTone::Ninth));
        assert!(cadd9.seventh.is_none()); // No seventh = added tone

        // C6
        let c6 = ChordAnalyzer::analyze(
            note!("C"),
            &[Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MAJOR_SIXTH]
        );
        assert!(c6.added_tones.contains(&AddedTone::Sixth));
    }

    #[test]
    fn test_chord_analyzer_omissions() {
        // C7 omit 5 - legacy behavior doesn't explicitly note missing fifths
        let c7_omit5 = ChordAnalyzer::analyze(
            note!("C"),
            &[Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::MINOR_SEVENTH]
        );
        // Legacy system doesn't mark fifth omissions in most cases
        assert!(!c7_omit5.omissions.contains(&Omission::Fifth));

        // C5 (power chord - no third) - this should be detected as omission
        let c5 = ChordAnalyzer::analyze(
            note!("C"),
            &[Interval::PERFECT_UNISON, Interval::PERFECT_FIFTH]
        );
        assert!(c5.omissions.contains(&Omission::Third));
    }

    #[test]
    fn test_chord_analyzer_complex_chord() {
        // Complex jazz chord: F#m13#11
        let complex = ChordAnalyzer::analyze(
            note!("F#"),
            &[Interval::PERFECT_UNISON, Interval::MINOR_THIRD, Interval::PERFECT_FIFTH,
              Interval::MINOR_SEVENTH, Interval::MAJOR_NINTH, Interval::AUGMENTED_ELEVENTH, 
              Interval::MAJOR_THIRTEENTH]
        );
        
        assert_eq!(complex.quality, ChordQuality::Minor);
        assert_eq!(complex.seventh, Some(SeventhType::Minor));
        assert_eq!(complex.highest_extension(), Some(Extension::Thirteenth));
        assert!(complex.alterations.contains(&Alteration::SharpEleventh));
        
        let display = format!("{}", complex);
        assert!(display.contains("F♯"));
        assert!(display.contains("13"));
        assert!(display.contains("#11"));
    }

    #[test]
    fn test_chord_to_chord_name_integration() {
        use crate::types::Chord;
        
        // Test basic chord creation and conversion
        let g7 = Chord::dominant_7th(note!("G"));
        let chord_name = g7.to_chord_name();
        assert_eq!(format!("{}", chord_name), "G7");
        
        let cmaj7 = Chord::major_7th(note!("C"));
        let chord_name = cmaj7.to_chord_name();
        assert_eq!(format!("{}", chord_name), "Cmaj7");
        
        let a_minor = Chord::minor(note!("A"));
        let chord_name = a_minor.to_chord_name();
        assert_eq!(format!("{}", chord_name), "A");
        assert_eq!(chord_name.quality, ChordQuality::Minor);
    }

    #[test]
    fn test_chord_renderer_formats() {
        let chord_name = ChordName::new(
            ChordRoot::Note(note!("F#")),
            ChordQuality::Major,
        ).with_seventh(SeventhType::Major);
        
        // Unicode format (default)
        let unicode = ChordRenderer::unicode_jazz().render(&chord_name);
        assert_eq!(unicode, "F♯maj7");
        
        // ASCII format
        let ascii = ChordRenderer::ascii().render(&chord_name);
        assert_eq!(ascii, "F#maj7");
        
        // HTML format
        let html = ChordRenderer::html().render(&chord_name);
        assert_eq!(html, "F&sharp;maj7");
        
        // Classical notation
        let classical = ChordRenderer::new(ChordFormat::Unicode, NamingConvention::Classical).render(&chord_name);
        assert_eq!(classical, "F♯△7");
    }

    #[test]
    fn test_chord_renderer_half_diminished() {
        let chord_name = ChordName::new(
            ChordRoot::Note(note!("B")),
            ChordQuality::Diminished,
        ).with_seventh(SeventhType::HalfDiminished);
        
        // Unicode format
        assert_eq!(ChordRenderer::unicode_jazz().render(&chord_name), "Bø7");
        
        // ASCII format (uses m7b5 notation)
        assert_eq!(ChordRenderer::ascii().render(&chord_name), "Bm7b5");
        
        // HTML format
        assert_eq!(ChordRenderer::html().render(&chord_name), "B&oslash;7");
    }

    #[test]
    fn test_chord_renderer_complex_chord() {
        let complex = ChordName::new(
            ChordRoot::Note(note!("Db")),
            ChordQuality::Minor,
        )
        .with_seventh(SeventhType::Minor)
        .with_extension(Extension::Thirteenth)
        .with_alteration(Alteration::SharpEleventh)
        .with_alteration(Alteration::FlatNinth);
        
        // Unicode
        let unicode = ChordRenderer::unicode_jazz().render(&complex);
        assert!(unicode.contains("D♭"));
        assert!(unicode.contains("13"));
        assert!(unicode.contains("♯11"));
        assert!(unicode.contains("♭9"));
        
        // ASCII
        let ascii = ChordRenderer::ascii().render(&complex);
        assert!(ascii.contains("Db"));
        assert!(ascii.contains("13"));
        assert!(ascii.contains("#11"));
        assert!(ascii.contains("b9"));
        
        // HTML
        let html = ChordRenderer::html().render(&complex);
        assert!(html.contains("D&flat;"));
        assert!(html.contains("13"));
        assert!(html.contains("&sharp;11"));
        assert!(html.contains("&flat;9"));
    }

    #[test]
    fn test_chord_name_convenience_methods() {
        let chord_name = ChordName::new(
            ChordRoot::Note(note!("C")),
            ChordQuality::Major,
        ).with_seventh(SeventhType::Major);
        
        // Test convenience methods
        assert_eq!(chord_name.to_ascii(), "Cmaj7");
        assert_eq!(chord_name.to_html(), "Cmaj7");
        assert_eq!(chord_name.to_classical(), "C△7");
        
        // Test custom renderer
        let custom_renderer = ChordRenderer::new(ChordFormat::Ascii, NamingConvention::Classical);
        assert_eq!(chord_name.render_with(&custom_renderer), "Cmaj7"); // ASCII doesn't use △
    }

    #[test]
    fn test_chord_renderer_suspensions_and_additions() {
        let sus_chord = ChordName::new(
            ChordRoot::Note(note!("G")),
            ChordQuality::Major,
        ).with_suspension(Suspension::Fourth);
        
        assert_eq!(ChordRenderer::unicode_jazz().render(&sus_chord), "Gsus4");
        assert_eq!(ChordRenderer::ascii().render(&sus_chord), "Gsus4");
        
        let add_chord = ChordName::new(
            ChordRoot::Note(note!("D")),
            ChordQuality::Minor,
        ).with_added_tone(AddedTone::Ninth);
        
        assert_eq!(ChordRenderer::unicode_jazz().render(&add_chord), "Dadd9");
        assert_eq!(ChordRenderer::ascii().render(&add_chord), "Dadd9");
    }

    #[test]
    fn test_chord_renderer_omissions() {
        let omit_chord = ChordName::new(
            ChordRoot::Note(note!("A")),
            ChordQuality::Major,
        )
        .with_seventh(SeventhType::Minor)
        .with_omission(Omission::Fifth);
        
        let rendered = ChordRenderer::unicode_jazz().render(&omit_chord);
        assert!(rendered.contains("A"));
        assert!(rendered.contains("7"));
        assert!(rendered.contains("no5"));
    }

    #[test]
    fn test_chord_renderer_slash_chords() {
        let slash_chord = ChordName::new(
            ChordRoot::Note(note!("C")),
            ChordQuality::Major,
        ).with_bass(ChordRoot::Note(note!("E")));
        
        assert_eq!(ChordRenderer::unicode_jazz().render(&slash_chord), "C/E");
        assert_eq!(ChordRenderer::ascii().render(&slash_chord), "C/E");
        assert_eq!(ChordRenderer::html().render(&slash_chord), "C/E");
    }

    #[test]
    fn test_chord_renderer_roman_numerals() {
        use crate::types::RomanNumeral;
        
        let roman_chord = ChordName::new(
            ChordRoot::Roman(RomanNumeral::V()),
            ChordQuality::Major,
        ).with_seventh(SeventhType::Minor);
        
        // Roman numerals render the same regardless of format
        assert_eq!(ChordRenderer::unicode_jazz().render(&roman_chord), "V7");
        assert_eq!(ChordRenderer::ascii().render(&roman_chord), "V7");
        assert_eq!(ChordRenderer::html().render(&roman_chord), "V7");
    }

    #[test]
    fn test_chord_abbreviated_name_integration() {
        use crate::types::Chord;
        
        // Test that abbreviated_name() now uses the new system
        let chords = vec![
            (Chord::major(note!("C")), "C"),
            (Chord::minor(note!("A")), "Am"),  // Legacy system uses "m" for minor
            (Chord::diminished(note!("B")), "Bdim"), // Legacy system uses "dim" not "°"
            (Chord::augmented(note!("C")), "Caug"), // Legacy system uses "aug" not "+"
            (Chord::dominant_7th(note!("G")), "G7"),
            (Chord::major_7th(note!("F")), "Fmaj7"),
        ];
        
        for (chord, expected) in chords {
            assert_eq!(chord.abbreviated_name(), expected, 
                "Chord {} should render as {}", chord, expected);
        }
    }

    #[test]
    fn test_chord_abbreviated_name_vs_old_system() {
        use crate::types::Chord;
        
        // Test some complex chords to ensure the new system works well
        let g7 = Chord::dominant_7th(note!("G"));
        assert_eq!(g7.abbreviated_name(), "G7");
        
        let cmaj7 = Chord::major_7th(note!("C"));
        assert_eq!(cmaj7.abbreviated_name(), "Cmaj7");
        
        // Test that we get consistent results
        let f_sharp_minor = Chord::minor(note!("F#"));
        assert_eq!(f_sharp_minor.abbreviated_name(), "F♯m");
        
        // Complex chord with extensions
        let complex = Chord::new(
            note!("D"),
            vec![
                Interval::PERFECT_UNISON,
                Interval::MINOR_THIRD,
                Interval::PERFECT_FIFTH,
                Interval::MINOR_SEVENTH,
                Interval::MAJOR_NINTH,
            ]
        );
        assert_eq!(complex.abbreviated_name(), "Dm9");
    }
}