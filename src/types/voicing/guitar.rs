//! Guitar-specific voicing functionality
//!
//! This module provides types and algorithms for guitar chord voicing,
//! including fretboard representation and chord finding algorithms.
//!
//! # Core Concepts
//!
//! - **GuitarFingering**: Represents finger positions on a guitar fretboard
//! - **GuitarTuning**: Defines the open string pitches for different tunings
//! - **StringState**: Individual string state (muted, open, or fretted)
//! - **IntervalFirstGuitarFinder**: Algorithm to find guitar voicings using interval analysis
//!
//! # Example
//!
//! ```rust
//! use chordy::{prelude::*, IntervalFirstGuitarFinder, GuitarTuning};
//!
//! let finder = IntervalFirstGuitarFinder::new();
//! let c_major = Chord::major(note!("C"));
//! let voicings = finder.find_voicings(&c_major);
//!
//! if let Some((fingering, _score)) = voicings.first() {
//!     let pitches = fingering.to_pitches(&GuitarTuning::standard());
//! }
//! ```

use crate::{
    error::ParseError,
    traits::{HasRoot, HasIntervals},
    types::{Chord, NoteName, Pitch},
};
use std::{collections::HashSet, fmt, str::FromStr};

use super::{VoicingError, VoicedChord, VoicingInfo, VoicingStyle, VoicingDetails, PitchRange};

/// Simple guitar shape representing finger positions
/// 0 = open string (doesn't move), 1+ = fretted positions (move together)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuitarShape {
    /// Finger positions (0 = open, 1+ = fretted)
    pub positions: &'static [u8],
}

impl GuitarShape {
    /// Create a new guitar shape from a static slice
    pub const fn new(positions: &'static [u8]) -> Self {
        Self { positions }
    }

    /// Check if this shape has any open strings (0s)
    pub fn has_open_strings(&self) -> bool {
        self.positions.iter().any(|&p| p == 0)
    }

    /// Count the number of fretted positions (> 0)
    pub fn count_fretted(&self) -> usize {
        self.positions.iter().filter(|&&p| p > 0).count()
    }

    /// Check if a fingering matches this shape (including partial matches)
    pub fn matches_fingering(&self, fingering: &[u8]) -> bool {
        // Direct match
        if self.positions == fingering {
            return true;
        }

        // Check if fingering is a valid partial (continuous subset)
        // allowing outer strings to be muted
        for window_start in 0..self.positions.len() {
            for window_end in (window_start + 1)..=self.positions.len() {
                let window = &self.positions[window_start..window_end];
                if window == fingering {
                    return true;
                }
            }
        }

        false
    }
}

/// Represents the state of a guitar string in a fingering pattern
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StringState {
    /// String is muted (not played), represented as 'X' in tablature
    Muted,
    /// String is played open (unfretted), represented as '0' in tablature  
    Open,
    /// String is fretted at the specified fret number (1-24)
    Fretted(u8),
}

impl StringState {
    /// Check if this string state represents a fretted note
    pub fn is_fretted(&self) -> bool {
        matches!(self, StringState::Fretted(_))
    }

    /// Check if this string state represents an open string
    pub fn is_open(&self) -> bool {
        matches!(self, StringState::Open)
    }

    /// Check if this string state represents a muted string
    pub fn is_muted(&self) -> bool {
        matches!(self, StringState::Muted)
    }

    /// Get the fret number if this is a fretted string
    pub fn fret_number(&self) -> Option<u8> {
        match self {
            StringState::Fretted(fret) => Some(*fret),
            _ => None,
        }
    }
}

impl fmt::Display for StringState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StringState::Muted => write!(f, "X"),
            StringState::Open => write!(f, "0"),
            StringState::Fretted(fret) => write!(f, "{}", fret),
        }
    }
}

impl FromStr for StringState {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" | "x" => Ok(StringState::Muted),
            "0" => Ok(StringState::Open),
            fret_str => {
                let fret = fret_str.parse::<u8>().map_err(|_| {
                    ParseError::InvalidChordFormat(format!("Invalid fret number: {}", fret_str))
                })?;
                if fret > 24 {
                    return Err(ParseError::InvalidChordFormat(format!(
                        "Fret number too high: {}",
                        fret
                    )));
                }
                Ok(StringState::Fretted(fret))
            }
        }
    }
}

/// Represents standard guitar tuning (low to high: E A D G B E)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GuitarTuning {
    /// The pitches of the six strings from lowest (thickest) to highest (thinnest)
    /// Standard tuning: [E2, A2, D3, G3, B3, E4]
    pub strings: [Pitch; 6],
}

impl GuitarTuning {
    /// Create a new guitar tuning from six pitches
    pub fn new(strings: [Pitch; 6]) -> Self {
        Self { strings }
    }

    /// Standard guitar tuning (E A D G B E)
    pub fn standard() -> Self {
        Self::new([
            "E2".parse().unwrap(), // Low E string
            "A2".parse().unwrap(), // A string
            "D3".parse().unwrap(), // D string
            "G3".parse().unwrap(), // G string
            "B3".parse().unwrap(), // B string
            "E4".parse().unwrap(), // High E string
        ])
    }

    /// Drop D tuning (D A D G B E)
    pub fn drop_d() -> Self {
        Self::new([
            "D2".parse().unwrap(), // Low D string
            "A2".parse().unwrap(), // A string
            "D3".parse().unwrap(), // D string
            "G3".parse().unwrap(), // G string
            "B3".parse().unwrap(), // B string
            "E4".parse().unwrap(), // High E string
        ])
    }
}

impl Default for GuitarTuning {
    fn default() -> Self {
        Self::standard()
    }
}

/// Represents a guitar chord fingering pattern
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GuitarFingering {
    /// The fingering pattern for each string (E A D G B E, low to high)
    pub frets: [StringState; 6],
    /// Which string index (0-5) contains the root note of the chord
    pub root_string: u8,
}

impl GuitarFingering {
    /// Create a new guitar fingering
    pub fn new(frets: [StringState; 6], root_string: u8) -> Result<Self, ParseError> {
        if root_string > 5 {
            return Err(ParseError::InvalidChordFormat(format!(
                "Root string index {} is invalid, must be 0-5",
                root_string
            )));
        }
        Ok(Self { frets, root_string })
    }

    /// Check if this fingering represents a barre chord (no open strings)
    pub fn is_barre(&self) -> bool {
        !self.frets.iter().any(|state| state.is_open())
    }

    /// Get the barre fret (lowest fret that is fretted) if this is a barre chord
    pub fn barre_fret(&self) -> Option<u8> {
        if !self.is_barre() {
            return None;
        }

        self.frets
            .iter()
            .filter_map(|state| state.fret_number())
            .min()
    }

    /// Generate the actual pitches this fingering produces on the fretboard
    pub fn to_pitches(&self, tuning: &GuitarTuning) -> Vec<Pitch> {
        self.frets
            .iter()
            .zip(tuning.strings.iter())
            .filter_map(|(fret_state, &open_pitch)| {
                match fret_state {
                    StringState::Muted => None,
                    StringState::Open => Some(open_pitch),
                    StringState::Fretted(fret) => {
                        // Add semitones for the fretted position
                        Some(open_pitch + (*fret as i8))
                    }
                }
            })
            .collect()
    }

    /// Get the root note pitch for this fingering
    pub fn root_pitch(&self, tuning: &GuitarTuning) -> Option<Pitch> {
        let root_string_state = &self.frets[self.root_string as usize];
        let open_pitch = tuning.strings[self.root_string as usize];

        match root_string_state {
            StringState::Muted => None,
            StringState::Open => Some(open_pitch),
            StringState::Fretted(fret) => Some(open_pitch + (*fret as i8)),
        }
    }

    /// Transpose this fingering to a different fret position (only works for barre chords)
    pub fn transpose_to_fret(&self, target_fret: u8) -> Result<Self, ParseError> {
        if !self.is_barre() {
            return Err(ParseError::InvalidChordFormat(
                "Cannot transpose open chord fingerings".to_string(),
            ));
        }

        let current_barre = self.barre_fret().unwrap_or(0);
        let offset = (target_fret as i8) - (current_barre as i8);

        let mut new_frets = self.frets;
        for fret_state in &mut new_frets {
            if let StringState::Fretted(fret) = fret_state {
                let new_fret = (*fret as i8) + offset;
                if !(0..=24).contains(&new_fret) {
                    return Err(ParseError::InvalidChordFormat(format!(
                        "Transposition would result in invalid fret: {}",
                        new_fret
                    )));
                }
                *fret_state = StringState::Fretted(new_fret as u8);
            }
        }

        Ok(Self {
            frets: new_frets,
            root_string: self.root_string,
        })
    }

    /// Create variations of this fingering by replacing muted strings with open strings
    /// This is useful for slash chords where we need specific bass notes
    /// For example, X35543 (Cm barre) can become 035543 to get E in the bass for Cm/E
    pub fn with_muted_to_open_variations(&self) -> Vec<Self> {
        let mut variations = vec![self.clone()];

        // For each muted string, create a variation with it as open
        for (i, state) in self.frets.iter().enumerate() {
            if matches!(state, StringState::Muted) {
                let mut variant = self.clone();
                variant.frets[i] = StringState::Open;
                variations.push(variant);
            }
        }

        // Also create a variation with ALL muted strings as open (if multiple muted)
        let muted_count = self
            .frets
            .iter()
            .filter(|s| matches!(s, StringState::Muted))
            .count();
        if muted_count > 1 {
            let mut all_open_variant = self.clone();
            for fret_state in &mut all_open_variant.frets {
                if matches!(fret_state, StringState::Muted) {
                    *fret_state = StringState::Open;
                }
            }
            variations.push(all_open_variant);
        }

        variations
    }

    /// Generate variations by optionally opening muted strings
    /// This allows patterns like X32010 to become 032010 when needed for slash chords
    pub fn with_optional_opens(&self) -> Vec<Self> {
        let mut variations = vec![self.clone()];

        // For each muted string, try opening it
        for (i, &state) in self.frets.iter().enumerate() {
            if matches!(state, StringState::Muted) {
                // Create a variation with this string open
                let mut variant = self.clone();
                variant.frets[i] = StringState::Open;
                variations.push(variant);
            }
        }

        variations
    }
}

impl fmt::Display for GuitarFingering {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, fret_state) in self.frets.iter().enumerate() {
            if i > 0 {
                write!(f, "")?;
            }
            write!(f, "{}", fret_state)?;
        }
        Ok(())
    }
}

impl FromStr for GuitarFingering {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 2 {
            return Err(ParseError::InvalidChordFormat(format!(
                "Guitar fingering format should be 'fingering,root_string', got: {}",
                s
            )));
        }

        let fingering_str = parts[0];
        let root_string: u8 = parts[1].parse().map_err(|_| {
            ParseError::InvalidChordFormat(format!("Invalid root string index: {}", parts[1]))
        })?;

        if fingering_str.len() != 6 {
            return Err(ParseError::InvalidChordFormat(format!(
                "Guitar fingering must have exactly 6 characters, got: {}",
                fingering_str
            )));
        }

        let mut frets = [StringState::Muted; 6];
        for (i, c) in fingering_str.chars().enumerate() {
            frets[i] = c.to_string().parse()?;
        }

        Self::new(frets, root_string)
    }
}

/// Interval-first guitar chord finder
pub struct IntervalFirstGuitarFinder {
    tuning: GuitarTuning,
    shapes: &'static [&'static GuitarShape],
}

impl Default for IntervalFirstGuitarFinder {
    fn default() -> Self {
        Self::new()
    }
}

impl IntervalFirstGuitarFinder {
    /// Create a new finder with standard tuning
    pub fn new() -> Self {
        use crate::guitar_shapes::ALL_GUITAR_SHAPES;
        Self {
            tuning: GuitarTuning::standard(),
            shapes: ALL_GUITAR_SHAPES,
        }
    }

    /// Create a new finder with custom tuning
    pub fn with_tuning(tuning: GuitarTuning) -> Self {
        use crate::guitar_shapes::ALL_GUITAR_SHAPES;
        Self {
            tuning,
            shapes: ALL_GUITAR_SHAPES,
        }
    }

    /// Find bass note positions on lower strings (typically strings 0-2)
    fn find_bass_positions(&self, bass_note: NoteName) -> Vec<(u8, u8)> {
        let mut positions = Vec::new();

        // Check lower strings (0-2) for bass positions
        for string in 0u8..=2u8 {
            let open_pitch = self.tuning.strings[string as usize];

            // Check each fret position up to 12th fret
            for fret in 0u8..=12u8 {
                let fretted_pitch = open_pitch + (fret as i8);
                if fretted_pitch.name == bass_note {
                    positions.push((string, fret));
                }
            }
        }

        positions
    }

    /// Find ways to voice the remaining chord intervals from a bass position
    fn find_chord_candidates(&self, chord: &Chord, bass_pos: (u8, u8)) -> Vec<GuitarFingering> {
        let required_notes = self.get_chord_notes(chord);
        let (bass_string, bass_fret) = bass_pos;

        // Define search window around bass position
        let (min_fret, max_fret) = if bass_fret == 0 {
            // For open bass, search a wider range since chord tones can be anywhere
            // The fret span validation will ensure only playable fingerings (max 4 fret span for fretted notes)
            (0, 12) // Allow full neck search for open bass positions
        } else {
            (bass_fret.saturating_sub(3), (bass_fret + 4).min(12))
        };

        // Generate combinations for remaining strings
        let remaining_strings: Vec<u8> = ((bass_string + 1)..6).collect();

        // Find ALL valid combinations, not just the best one
        self.find_all_combinations(
            &required_notes,
            &remaining_strings,
            min_fret,
            max_fret,
            bass_pos,
        )
    }

    /// Get the unique notes required for this chord
    fn get_chord_notes(&self, chord: &Chord) -> Vec<NoteName> {
        let root = chord.root();
        let mut notes = vec![root];

        for interval in chord.intervals() {
            let note = root + *interval;
            if !notes.contains(&note) {
                notes.push(note);
            }
        }

        notes
    }

    /// Check if a note is a chord tone, considering enharmonic equivalents
    fn is_chord_tone(&self, note: &NoteName, required_notes: &[NoteName]) -> bool {
        required_notes.contains(note) || 
        required_notes.iter().any(|req| note.is_enharmonic_with(req))
    }

    /// Find all valid combinations of notes on the given strings
    fn find_all_combinations(
        &self,
        required_notes: &[NoteName],
        strings: &[u8],
        min_fret: u8,
        max_fret: u8,
        bass_pos: (u8, u8),
    ) -> Vec<GuitarFingering> {
        let mut all_fingerings = Vec::new();
        self.try_all_combinations(
            required_notes,
            strings,
            min_fret,
            max_fret,
            bass_pos,
            0,
            [StringState::Muted; 6],
            &mut all_fingerings,
        );
        all_fingerings
    }

    /// Recursive helper to try all combinations and collect ALL valid ones
    #[allow(clippy::too_many_arguments)]
    fn try_all_combinations(
        &self,
        required_notes: &[NoteName],
        strings: &[u8],
        min_fret: u8,
        max_fret: u8,
        bass_pos: (u8, u8),
        string_index: usize,
        mut current_frets: [StringState; 6],
        all_fingerings: &mut Vec<GuitarFingering>,
    ) {
        let (bass_string, bass_fret) = bass_pos;

        // Set the bass note
        current_frets[bass_string as usize] = if bass_fret == 0 {
            StringState::Open
        } else {
            StringState::Fretted(bass_fret)
        };

        // Base case: we've assigned all strings
        if string_index >= strings.len() {
            if let Ok(fingering) = GuitarFingering::new(current_frets, bass_string) {
                if self.is_valid_voicing(&fingering, required_notes) {
                    all_fingerings.push(fingering);
                }
            }
            return;
        }

        let string = strings[string_index];
        let open_pitch = self.tuning.strings[string as usize];

        // Try muting this string
        self.try_all_combinations(
            required_notes,
            strings,
            min_fret,
            max_fret,
            bass_pos,
            string_index + 1,
            current_frets,
            all_fingerings,
        );

        // Try fretting this string at each position
        for fret in min_fret..=max_fret {
            let fretted_pitch = open_pitch + (fret as i8);

            // Only consider chord tones (including enharmonic equivalents)
            if self.is_chord_tone(&fretted_pitch.name, required_notes) {
                let mut new_frets = current_frets;
                new_frets[string as usize] = if fret == 0 {
                    StringState::Open
                } else {
                    StringState::Fretted(fret)
                };

                self.try_all_combinations(
                    required_notes,
                    strings,
                    min_fret,
                    max_fret,
                    bass_pos,
                    string_index + 1,
                    new_frets,
                    all_fingerings,
                );
            }
        }
    }

    /// Validate that muted strings only appear on the outside (beginning or end)
    fn has_valid_muting_pattern(&self, fingering: &GuitarFingering) -> bool {
        let frets = &fingering.frets;

        // Find first and last non-muted strings
        let first_active = frets.iter().position(|s| !matches!(s, StringState::Muted));
        let last_active = frets.iter().rposition(|s| !matches!(s, StringState::Muted));

        if let (Some(first), Some(last)) = (first_active, last_active) {
            // Check that all strings between first and last active strings are not muted
            for i in first..=last {
                if matches!(frets[i], StringState::Muted) {
                    return false; // Found muted string between active strings
                }
            }
        }

        true
    }

    /// Validate that fret span is playable (max 4 frets for fretted notes, excluding open)
    fn has_playable_fret_span(&self, fingering: &GuitarFingering) -> bool {
        let frets = &fingering.frets;

        // Collect all fretted positions (excluding open strings)
        let fretted_positions: Vec<u8> = frets
            .iter()
            .filter_map(|s| match s {
                StringState::Fretted(fret) => Some(*fret),
                _ => None,
            })
            .collect();

        if fretted_positions.is_empty() {
            return true; // All open strings is valid
        }

        let min_fret = *fretted_positions.iter().min().unwrap();
        let max_fret = *fretted_positions.iter().max().unwrap();

        // Allow maximum span of 4 frets for fretted notes
        max_fret - min_fret <= 4
    }

    /// Validate that a voicing is playable and contains sufficient chord tones
    fn is_valid_voicing(&self, fingering: &GuitarFingering, required_notes: &[NoteName]) -> bool {
        // Check chord tone coverage
        let pitches = fingering.to_pitches(&self.tuning);
        let unique_notes: HashSet<NoteName> = pitches.iter().map(|p| p.name).collect();
        
        let coverage = required_notes
            .iter()
            .filter(|&required_note| {
                unique_notes.iter().any(|&actual_note| {
                    actual_note == *required_note || actual_note.is_enharmonic_with(required_note)
                })
            })
            .count();
        let coverage_percentage = coverage as f32 / required_notes.len() as f32;
        
        // Require at least 75% chord tone coverage
        if coverage_percentage < 0.95 {
            // println!(
            //     "Rejected fingering {} due to insufficient chord tone coverage: {:.2}%",
            //     fingering,
            //     coverage_percentage * 100.0
            // );
            return false;
        }

        // Check that muted strings are only on the outside
        if !self.has_valid_muting_pattern(fingering) {
            return false;
        }

        // Check that fret span is playable
        if !self.has_playable_fret_span(fingering) {
            return false;
        }

        // Check if it matches a known shape
        self.matches_known_shape(fingering)
    }


    /// Check if any known shape matches the fingering
    fn matches_known_shape(&self, fingering: &GuitarFingering) -> bool {
        // Try both extraction methods for compatibility
        let full_pattern = self.extract_shape_from_fingering(fingering); // Includes muted as 255
        let filtered_pattern = fingering.frets.iter().filter_map(|state| match state {
            StringState::Fretted(f) => Some(*f),
            StringState::Open => Some(0),
            StringState::Muted => None,
        }).collect::<Vec<u8>>(); // Original method - filters out muted

        // Try exact matches with full pattern (for muted string shapes)
        for shape in self.shapes {
            if shape.matches_fingering(&full_pattern) {
                return true;
            }
        }

        // Try exact matches with filtered pattern (for traditional shapes)
        for shape in self.shapes {
            if shape.matches_fingering(&filtered_pattern) {
                return true;
            }
        }

        // Try matches with open string extensions for filtered pattern
        for shape in self.shapes {
            if self.matches_shape_with_open_extensions(&filtered_pattern, shape) {
                return true;
            }
        }
        
        false
    }

    /// Check if the extracted pattern matches a known shape when allowing open string extensions
    fn matches_shape_with_open_extensions(
        &self,
        extracted_pattern: &[u8],
        shape: &GuitarShape,
    ) -> bool {
        let shape_positions = shape.positions;
        let shape_len = shape_positions.len();
        let pattern_len = extracted_pattern.len();

        if pattern_len < shape_len {
            return false;
        }

        // Try all possible alignments of the shape within the pattern
        for start in 0..=(pattern_len - shape_len) {
            let segment = &extracted_pattern[start..start + shape_len];
            
            if self.patterns_match(segment, shape_positions) {
                // Check that all positions outside the shape are open (0)
                let prefix_open = extracted_pattern[0..start].iter().all(|&pos| pos == 0);
                let suffix_open = extracted_pattern[start + shape_len..].iter().all(|&pos| pos == 0);
                
                if prefix_open && suffix_open {
                    return true;
                }
            }
        }

        false
    }

    /// Helper to check if two patterns match (accounting for movable shapes)
    fn patterns_match(&self, pattern1: &[u8], pattern2: &[u8]) -> bool {
        pattern1.len() == pattern2.len() && (
            pattern1 == pattern2 || 
            self.normalize_pattern(pattern1) == self.normalize_pattern(pattern2)
        )
    }

    /// Normalize a pattern by subtracting the minimum non-zero fret number
    fn normalize_pattern(&self, pattern: &[u8]) -> Vec<u8> {
        let min_fret = pattern.iter().filter(|&&f| f > 0).min().copied().unwrap_or(0);
        
        if min_fret == 0 {
            pattern.to_vec()
        } else {
            pattern.iter().map(|&f| if f == 0 { 0 } else { f - min_fret + 1 }).collect()
        }
    }

    /// Extract the shape pattern from a fingering
    fn extract_shape_from_fingering(&self, fingering: &GuitarFingering) -> Vec<u8> {
        fingering.frets.iter().map(|state| match state {
            StringState::Fretted(f) => *f,
            StringState::Open => 0,
            StringState::Muted => 255, // Include muted strings in pattern matching
        }).collect()
    }

    /// Find guitar voicings using interval-first approach
    pub fn find_voicings(&self, chord: &Chord) -> Vec<(GuitarFingering, f32)> {
        let bass_note = chord.bass_note();
        let bass_positions = self.find_bass_positions(bass_note);
        let mut all_voicings = Vec::new();

        for bass_pos in bass_positions {
            let candidates = self.find_chord_candidates(chord, bass_pos);
            
            for candidate in candidates {
                if candidate.root_pitch(&self.tuning).is_some() {
                    let score = self.calculate_score_for_chord(chord, bass_pos, &candidate);
                    all_voicings.push((candidate, score));
                }
            }
        }

        // Sort by score and return top results
        all_voicings.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        all_voicings.truncate(10);
        all_voicings
    }

    /// Calculate score for a voicing (lower is better)
    fn calculate_score_for_chord(
        &self,
        _chord: &Chord,
        _bass_pos: (u8, u8),
        fingering: &GuitarFingering,
    ) -> f32 {
        let pitches = fingering.to_pitches(&self.tuning);
        
        // Prioritize: 1) Low position 2) More pitches 3) Playability
        // Balance between fullness and playability
        let pitch_bonus = -(pitches.len() as f32) * 10.0; // Favor more notes (fuller sound)
        let playability_penalty = self.calculate_playability_score(fingering) * 4.0;
        let position_penalty = self.calculate_neck_position_score(fingering) * 12.0; // Strongly favor lower positions
        
        pitch_bonus + playability_penalty + position_penalty
    }

    /// Calculate playability score (lower is easier to play)
    fn calculate_playability_score(&self, fingering: &GuitarFingering) -> f32 {
        let open_count = fingering.frets.iter().filter(|s| matches!(s, StringState::Open)).count();
        let muted_count = fingering.frets.iter().filter(|s| matches!(s, StringState::Muted)).count();
        let fretted_count = fingering.frets.iter().filter(|s| matches!(s, StringState::Fretted(_))).count();
        
        let fretted_positions: Vec<u8> = fingering.frets.iter()
            .filter_map(|s| match s { StringState::Fretted(f) => Some(*f), _ => None })
            .collect();

        if fretted_positions.is_empty() {
            return 0.0; // All open strings - perfect playability
        }

        let fret_span = fretted_positions.iter().max().unwrap() - fretted_positions.iter().min().unwrap();
        
        // Prioritize fret span over number of fingers
        // A 4-fret span is much harder than 4 fingers in a 1-fret span
        let span_penalty = fret_span as f32 * 2.0;  // Fret span is most important
        let finger_penalty = (fretted_count as f32 - 1.0).max(0.0) * 0.3; // Additional fingers are less important
        let muted_penalty = muted_count as f32 * 0.1; // Muted strings are slightly harder
        let open_bonus = -(open_count as f32 * 0.8); // Open strings are much better (increased from 0.2)
        
        span_penalty + finger_penalty + muted_penalty + open_bonus
    }

    /// Calculate neck position score (lower frets are better)
    fn calculate_neck_position_score(&self, fingering: &GuitarFingering) -> f32 {
        fingering.frets.iter()
            .filter_map(|s| match s { StringState::Fretted(f) => Some(*f), _ => None })
            .min()
            .map(|min_fret| min_fret as f32 * 0.1)
            .unwrap_or(0.0)
    }

}

/// Voice a chord using guitar fingering patterns within the main Voicer engine
pub fn voice_guitar_chord(chord: &Chord, range: PitchRange) -> Result<VoicedChord, VoicingError> {
    // Use the new interval-first approach directly
    let finder = IntervalFirstGuitarFinder::new();
    let voicings = finder.find_voicings(chord);

    for voicing in &voicings {
    }

    if voicings.is_empty() {
        return Err(VoicingError::UnsupportedStyle);
    }

    // Use the best voicing (first in the sorted list)
    let (best_fingering, _score) = &voicings[0];
    let tuning = GuitarTuning::standard(); // Use standard tuning for now
    let pitches = best_fingering.to_pitches(&tuning);

    // Create voicing details with the guitar fingering information
    let details = VoicingDetails::Guitar {
        fingering: best_fingering.clone(),
        tuning: tuning.clone(),
    };

    let info = VoicingInfo::new_with_details(VoicingStyle::Guitar, range, 0, details);
    Ok(VoicedChord::new(*chord, pitches, info))
}

