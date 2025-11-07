//! Chord parsing from string representations
//!
//! This module contains methods for parsing chords from various string formats:
//! - Note list format ("C,E,G")
//! - Harte notation (future)
//! - Other chord symbol formats

use std::str::FromStr;

use crate::types::{Chord, NoteName};

impl FromStr for Chord {
    type Err = String;

    /// Parse a chord from a comma-separated list of note names
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::Chord;
    ///
    /// let chord: Chord = "C,E,G".parse().unwrap();
    /// assert_eq!(chord.abbreviated_name(), "C");
    ///
    /// let chord: Chord = "D,F,A".parse().unwrap();
    /// assert_eq!(chord.abbreviated_name(), "Dm");
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Split the string by commas and trim whitespace
        let note_strings: Vec<&str> = s.split(',').map(|s| s.trim()).collect();

        if note_strings.is_empty() {
            return Err("Empty chord string".to_string());
        }

        // Parse each note name
        let mut notes = Vec::new();
        for note_str in note_strings {
            let note: NoteName = note_str
                .parse()
                .map_err(|e| format!("Failed to parse note '{}': {}", note_str, e))?;
            notes.push(note);
        }

        // Create chord from notes
        Ok(Chord::from_notes(&notes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_major_chord() {
        let chord: Chord = "C,E,G".parse().unwrap();
        assert_eq!(chord.abbreviated_name(), "C");
    }

    #[test]
    fn test_parse_minor_chord() {
        let chord: Chord = "D,F,A".parse().unwrap();
        assert_eq!(chord.abbreviated_name(), "Dm");
    }

    #[test]
    fn test_parse_dominant_7th() {
        let chord: Chord = "G,B,D,F".parse().unwrap();
        assert_eq!(chord.abbreviated_name(), "G7");
    }

    #[test]
    fn test_parse_invalid_note() {
        let result: Result<Chord, _> = "C,X,G".parse();
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_empty_string() {
        let result: Result<Chord, _> = "".parse();
        assert!(result.is_err());
    }
}