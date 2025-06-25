//! Chord name rendering logic for multiple output formats and conventions
//!
//! This module handles the conversion of structured ChordName representations into
//! formatted strings, supporting different output formats (Unicode, ASCII, HTML) and
//! naming conventions (Jazz, Classical, Lead Sheet).

use crate::types::{ChordQuality, NoteName, Accidental, RomanNumeral};
use super::types::*;

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
        
        // Render root with quality-aware formatting (except for diminished seventh Roman chords)
        if matches!(chord_name.seventh, Some(SeventhType::HalfDiminished | SeventhType::Diminished)) && matches!(chord_name.root, ChordRoot::Roman(_)) {
            // For diminished seventh Roman chords, render root with diminished case but no quality symbol
            if let ChordRoot::Roman(roman) = &chord_name.root {
                let accidental_str = match roman.accidental {
                    Accidental::Natural => "",
                    Accidental::Sharp => "♯",
                    Accidental::Flat => "♭",
                    Accidental::DoubleSharp => "𝄪",
                    Accidental::DoubleFlat => "𝄫",
                };
                let base = roman.degree.lowercase_string(); // Use lowercase for diminished quality
                result.push_str(&format!("{}{}", accidental_str, base));
            }
        } else {
            result.push_str(&self.render_root_with_quality(&chord_name.root, chord_name.quality));
        }
        
        // Handle special cases for diminished seventh chords
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
        } else if let Some(SeventhType::Diminished) = chord_name.seventh {
            // Handle fully diminished seventh chords
            result.push_str(&self.render_diminished_seventh_for_root(&chord_name.root));
            
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
            ChordRoot::Roman(roman) => self.render_roman_with_quality(roman, quality),
        }
    }
    
    fn render_roman_with_quality(&self, roman: &RomanNumeral, quality: ChordQuality) -> String {
        // Determine case based on quality: Major/Augmented = uppercase, Minor/Diminished = lowercase
        let base = match quality {
            ChordQuality::Major | ChordQuality::Augmented => roman.degree.base_string(),
            ChordQuality::Minor | ChordQuality::Diminished => roman.degree.lowercase_string(),
        };
        
        // Render accidental
        let accidental_str = match roman.accidental {
            Accidental::Natural => "",
            Accidental::Sharp => "♯",
            Accidental::Flat => "♭",
            Accidental::DoubleSharp => "𝄪",
            Accidental::DoubleFlat => "𝄫",
        };
        
        // Add quality suffix
        let quality_suffix = match quality {
            ChordQuality::Diminished => "°",
            ChordQuality::Augmented => "+",
            _ => "",
        };
        
        format!("{}{}{}", accidental_str, base, quality_suffix)
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
            ChordRoot::Note(_) => {
                // For note names, render minor as "m", others as per quality
                match quality {
                    ChordQuality::Minor => "m".to_string(),
                    _ => self.render_quality(quality),
                }
            },
            ChordRoot::Roman(_) => {
                // For Roman numerals, quality is encoded in the numeral itself via display_for_quality
                // Don't add additional quality symbols
                String::new()
            }
        }
    }
    
    fn render_half_diminished_for_root(&self, root: &ChordRoot) -> String {
        match root {
            ChordRoot::Note(_) => {
                match self.format {
                    ChordFormat::Unicode => "ø7".to_string(),
                    ChordFormat::Ascii => "m7b5".to_string(),
                    ChordFormat::Html => "&oslash;7".to_string(),
                }
            },
            ChordRoot::Roman(_) => {
                // For half-diminished Roman numerals, we need the ø symbol + 7
                match self.format {
                    ChordFormat::Unicode => "ø7".to_string(),
                    ChordFormat::Ascii => "ø7".to_string(), // Keep ø even in ASCII for half-diminished
                    ChordFormat::Html => "&oslash;7".to_string(),
                }
            }
        }
    }
    
    fn render_diminished_seventh_for_root(&self, root: &ChordRoot) -> String {
        match root {
            ChordRoot::Note(_) => {
                match self.format {
                    ChordFormat::Unicode => "°7".to_string(),
                    ChordFormat::Ascii => "dim7".to_string(),
                    ChordFormat::Html => "&deg;7".to_string(),
                }
            },
            ChordRoot::Roman(_) => {
                // For fully diminished Roman numerals, we need the ° symbol + 7
                match self.format {
                    ChordFormat::Unicode => "°7".to_string(),
                    ChordFormat::Ascii => "°7".to_string(), // Keep ° even in ASCII for fully diminished
                    ChordFormat::Html => "&deg;7".to_string(),
                }
            }
        }
    }
    
    fn render_seventh(&self, seventh: SeventhType) -> String {
        match seventh {
            SeventhType::Minor => "7".to_string(),
            SeventhType::Major => "maj7".to_string(), // Always use "maj7" for compatibility
            SeventhType::Diminished => match self.format {
                ChordFormat::Unicode => "°7".to_string(),
                ChordFormat::Ascii => "dim7".to_string(),
                ChordFormat::Html => "&deg;7".to_string(),
            },
            SeventhType::HalfDiminished => match self.format {
                ChordFormat::Unicode => "ø7".to_string(),
                ChordFormat::Ascii => "m7b5".to_string(),
                ChordFormat::Html => "&oslash;7".to_string(),
            },
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
        let ext_str = self.render_extension(extension);
        
        // Add seventh type prefix for extensions when needed
        match (seventh, quality, self.convention) {
            (Some(SeventhType::Major), ChordQuality::Minor, NamingConvention::LeadSheet) => {
                // Special case for minor-major chords: use parentheses
                format!("(maj{})", ext_str)
            },
            (Some(SeventhType::Major), _, _) => format!("maj{}", ext_str), // Always use "maj" for compatibility
            (Some(SeventhType::Diminished), _, _) => match self.format {
                ChordFormat::Unicode => format!("°{}", ext_str),
                ChordFormat::Ascii => format!("dim{}", ext_str),
                ChordFormat::Html => format!("&deg;{}", ext_str),
            },
            // Minor 7th and no seventh both just use the extension number
            _ => ext_str,
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
        match added_tone {
            AddedTone::Second => "add2".to_string(),
            AddedTone::Fourth => "add4".to_string(),
            AddedTone::Sixth => "6".to_string(),
            AddedTone::Ninth => "add9".to_string(),
            AddedTone::Eleventh => {
                if has_seventh {
                    "(add11)".to_string()
                } else {
                    "add11".to_string()
                }
            },
        }
    }
    
    fn render_omission(&self, omission: Omission) -> String {
        match omission {
            Omission::Third => "5".to_string(), // Power chord notation
            Omission::Fifth => "(no5)".to_string(),
            Omission::Seventh => "(no7)".to_string(),
            Omission::Eleventh => "(no11)".to_string(),
        }
    }
}

impl Default for ChordRenderer {
    fn default() -> Self {
        Self::unicode_jazz()
    }
}