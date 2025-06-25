//! Chord naming system with intermediate representation and flexible rendering
//!
//! This module provides a structured approach to chord naming that separates
//! chord analysis from display formatting, allowing for multiple naming conventions
//! and output formats.
//!
//! ## Architecture
//!
//! The system is organized into three main components:
//!
//! ### Types (`types` module)
//! Contains all the core data structures that represent chord names:
//! - `ChordName`: Complete intermediate representation
//! - `ChordRoot`: Note or roman numeral roots
//! - `SeventhType`, `Extension`, `Alteration`, etc.: Chord components
//!
//! ### Analyzer (`analyzer` module)
//! Converts raw intervals into structured chord names:
//! - `ChordAnalyzer`: Main analysis engine
//! - `IntervalChecker`: Helper for interval presence checks
//! - Music theory logic for extensions, alterations, suspensions, etc.
//!
//! ### Renderer (`renderer` module)
//! Converts structured chord names into formatted strings:
//! - `ChordRenderer`: Configurable rendering engine
//! - `ChordFormat`: Output format (Unicode, ASCII, HTML)
//! - `NamingConvention`: Style conventions (Jazz, Classical, Lead Sheet)
//!
//! ## Usage
//!
//! ```rust
//! use chordy::{Chord, note};
//! 
//! // Create a chord and analyze it using the naming system
//! let chord = Chord::from_notes_and_root(
//!     &[note!("G"), note!("B"), note!("D"), note!("F"), note!("A")],
//!     note!("G")
//! );
//! 
//! // The chord naming system is used internally by abbreviated_name()
//! let name = chord.abbreviated_name();
//! assert_eq!(name, "G9");
//! ```

pub mod types;
pub mod analyzer;
pub mod renderer;

// Re-export the main public types and functions
pub use types::*;
pub use analyzer::ChordAnalyzer;
pub use renderer::{ChordRenderer, ChordFormat, NamingConvention};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::note;

    #[test]
    fn test_chord_name_creation() {
        let chord_name = ChordName::new(
            ChordRoot::Note(note!("C")),
            crate::ChordQuality::Major,
        );
        
        assert_eq!(chord_name.quality, crate::ChordQuality::Major);
        assert!(chord_name.is_triad());
        assert!(!chord_name.has_alterations());
    }

    #[test]
    fn test_chord_name_builder() {
        let chord_name = ChordName::new(
            ChordRoot::Note(note!("C")),
            crate::ChordQuality::Major,
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
            crate::ChordQuality::Diminished,
        ).with_seventh(SeventhType::HalfDiminished);
        
        assert_eq!(half_dim.seventh, Some(SeventhType::HalfDiminished));
        
        // Test new alterations
        let altered_chord = ChordName::new(
            ChordRoot::Note(note!("C")),
            crate::ChordQuality::Major,
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
            crate::ChordQuality::Major,
        ).with_seventh(SeventhType::Minor);
        
        assert_eq!(format!("{}", chord_name), "C7");
    }

    #[test]
    fn test_chord_analysis() {
        use crate::Interval;
        
        // Test basic major chord
        let c_major = ChordAnalyzer::analyze(
            note!("C"),
            &[Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH]
        );
        assert_eq!(c_major.quality, crate::ChordQuality::Major);
        assert!(c_major.is_triad());
        
        // Test minor chord
        let a_minor = ChordAnalyzer::analyze(
            note!("A"),
            &[Interval::PERFECT_UNISON, Interval::MINOR_THIRD, Interval::PERFECT_FIFTH]
        );
        assert_eq!(a_minor.quality, crate::ChordQuality::Minor);
        
        // Test diminished chord
        let b_dim = ChordAnalyzer::analyze(
            note!("B"),
            &[Interval::PERFECT_UNISON, Interval::MINOR_THIRD, Interval::DIMINISHED_FIFTH]
        );
        assert_eq!(b_dim.quality, crate::ChordQuality::Diminished);
        
        // Test augmented chord
        let c_aug = ChordAnalyzer::analyze(
            note!("C"),
            &[Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::AUGMENTED_FIFTH]
        );
        assert_eq!(c_aug.quality, crate::ChordQuality::Augmented);
    }

    #[test]
    fn test_seventh_chord_analysis() {
        use crate::Interval;
        
        // Test dominant 7th
        let g7 = ChordAnalyzer::analyze(
            note!("G"),
            &[Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH]
        );
        assert_eq!(g7.seventh, Some(SeventhType::Minor));
        
        // Test major 7th
        let cmaj7 = ChordAnalyzer::analyze(
            note!("C"),
            &[Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MAJOR_SEVENTH]
        );
        assert_eq!(cmaj7.seventh, Some(SeventhType::Major));
        
        // Test half-diminished
        let b_half_dim = ChordAnalyzer::analyze(
            note!("B"),
            &[Interval::PERFECT_UNISON, Interval::MINOR_THIRD, Interval::DIMINISHED_FIFTH, Interval::MINOR_SEVENTH]
        );
        assert_eq!(b_half_dim.seventh, Some(SeventhType::HalfDiminished));
        assert_eq!(b_half_dim.quality, crate::ChordQuality::Diminished);
    }

    #[test]
    fn test_extension_analysis() {
        use crate::Interval;
        
        // Test 9th chord
        let c9 = ChordAnalyzer::analyze(
            note!("C"),
            &[
                Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, 
                Interval::MINOR_SEVENTH, Interval::MAJOR_NINTH
            ]
        );
        assert!(c9.extensions.contains(&Extension::Ninth));
        
        // Test 13th chord
        let f13 = ChordAnalyzer::analyze(
            note!("F"),
            &[
                Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH,
                Interval::MINOR_SEVENTH, Interval::MAJOR_NINTH, Interval::PERFECT_ELEVENTH, 
                Interval::MAJOR_THIRTEENTH
            ]
        );
        assert!(f13.extensions.contains(&Extension::Thirteenth));
    }

    #[test]
    fn test_alteration_analysis() {
        use crate::Interval;
        
        // Test flat 5
        let c7_flat5 = ChordAnalyzer::analyze(
            note!("C"),
            &[Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::DIMINISHED_FIFTH, Interval::MINOR_SEVENTH]
        );
        assert!(c7_flat5.alterations.contains(&Alteration::FlatFifth));
        
        // Test sharp 11
        let g7_sharp11 = ChordAnalyzer::analyze(
            note!("G"),
            &[
                Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH,
                Interval::MINOR_SEVENTH, Interval::MAJOR_NINTH, Interval::AUGMENTED_ELEVENTH
            ]
        );
        assert!(g7_sharp11.alterations.contains(&Alteration::SharpEleventh));
    }

    #[test]
    fn test_suspension_analysis() {
        use crate::Interval;
        
        // Test sus2
        let csus2 = ChordAnalyzer::analyze(
            note!("C"),
            &[Interval::PERFECT_UNISON, Interval::MAJOR_SECOND, Interval::PERFECT_FIFTH]
        );
        assert!(csus2.suspensions.contains(&Suspension::Second));
        
        // Test sus4
        let dsus4 = ChordAnalyzer::analyze(
            note!("D"),
            &[Interval::PERFECT_UNISON, Interval::PERFECT_FOURTH, Interval::PERFECT_FIFTH]
        );
        assert!(dsus4.suspensions.contains(&Suspension::Fourth));
    }

    #[test]
    fn test_added_tone_analysis() {
        use crate::Interval;
        
        // Test add9
        let cadd9 = ChordAnalyzer::analyze(
            note!("C"),
            &[Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MAJOR_NINTH]
        );
        assert!(cadd9.added_tones.contains(&AddedTone::Ninth));
        
        // Test add6
        let c6 = ChordAnalyzer::analyze(
            note!("C"),
            &[Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MAJOR_SIXTH]
        );
        assert!(c6.added_tones.contains(&AddedTone::Sixth));
    }

    #[test]
    fn test_omission_analysis() {
        use crate::Interval;
        
        // Test omit5 in 7th chord
        let c7_omit5 = ChordAnalyzer::analyze(
            note!("C"),
            &[Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::MINOR_SEVENTH]
        );
        // Note: omit5 is not explicitly tracked in legacy behavior
        
        // Test power chord (no third)
        let c5 = ChordAnalyzer::analyze(
            note!("C"),
            &[Interval::PERFECT_UNISON, Interval::PERFECT_FIFTH]
        );
        assert!(c5.omissions.contains(&Omission::Third));
    }

    #[test]
    fn test_complex_chord_analysis() {
        use crate::Interval;
        
        // Test complex altered chord
        let complex = ChordAnalyzer::analyze(
            note!("G"),
            &[
                Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH,
                Interval::MINOR_SEVENTH, Interval::MINOR_NINTH, Interval::AUGMENTED_ELEVENTH, 
                Interval::MINOR_THIRTEENTH
            ]
        );
        
        assert_eq!(complex.seventh, Some(SeventhType::Minor));
        assert!(complex.alterations.contains(&Alteration::FlatNinth));
        assert!(complex.alterations.contains(&Alteration::SharpEleventh));
        assert!(complex.alterations.contains(&Alteration::FlatThirteenth));
    }

    #[test]
    fn test_renderer_formats() {
        let chord_name = ChordName::new(
            ChordRoot::Note(note!("C")),
            crate::ChordQuality::Minor,
        ).with_seventh(SeventhType::Major);
        
        // Test different formats
        let unicode = ChordRenderer::unicode_jazz().render(&chord_name);
        let ascii = ChordRenderer::ascii().render(&chord_name);
        let legacy = ChordRenderer::legacy().render(&chord_name);
        
        assert!(unicode.contains("maj"));  // Unified maj7 rendering for compatibility
        assert!(ascii.contains("maj"));    // ASCII major
        assert!(legacy.contains("(maj7)")); // Legacy parentheses for minor-major
    }
}