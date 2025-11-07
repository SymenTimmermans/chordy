//! Piano-specific voicing algorithms and metadata
//!
//! This module provides piano-specific voicing functionality that considers
//! human physical limitations, hand ergonomics, and piano-specific constraints.

use crate::types::Pitch;
use std::fmt;

/// Hand position for piano voicing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PianoHandPosition {
    /// Left hand only (typically plays bass notes)
    LeftHand,
    /// Right hand only (typically plays melody/harmony)
    RightHand,
    /// Both hands combined
    BothHands,
}

impl Default for PianoHandPosition {
    fn default() -> Self {
        Self::BothHands
    }
}

impl fmt::Display for PianoHandPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LeftHand => write!(f, "Left Hand"),
            Self::RightHand => write!(f, "Right Hand"),
            Self::BothHands => write!(f, "Both Hands"),
        }
    }
}

/// Piano voicing types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PianoVoicingType {
    /// Block chords - all notes played simultaneously
    Block,
    /// Broken chords - notes played in sequence
    Broken,
    /// Arpeggiated - notes played in rapid succession
    Arpeggiated,
    /// Spread - wide spacing across multiple octaves
    Spread,
    /// Shell voicing - root + 3rd/7th for jazz
    Shell,
    /// Rootless voicing - omitting root for jazz harmony
    Rootless,
}

impl Default for PianoVoicingType {
    fn default() -> Self {
        Self::Block
    }
}

impl fmt::Display for PianoVoicingType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Block => write!(f, "Block"),
            Self::Broken => write!(f, "Broken"),
            Self::Arpeggiated => write!(f, "Arpeggiated"),
            Self::Spread => write!(f, "Spread"),
            Self::Shell => write!(f, "Shell"),
            Self::Rootless => write!(f, "Rootless"),
        }
    }
}


/// Hand span constraints for piano voicing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PianoHandSpan {
    /// Maximum comfortable span for left hand in semitones
    pub left_hand_max_span: u8,
    /// Maximum comfortable span for right hand in semitones
    pub right_hand_max_span: u8,
    /// Maximum number of notes for left hand
    pub left_hand_max_notes: u8,
    /// Maximum number of notes for right hand
    pub right_hand_max_notes: u8,
    /// Minimum interval between adjacent fingers in semitones
    pub min_finger_spacing: u8,
}

impl Default for PianoHandSpan {
    fn default() -> Self {
        // Reasonable defaults for average adult hands
        Self {
            left_hand_max_span: 10,  // Octave + minor 3rd
            right_hand_max_span: 12,  // Octave + perfect 4th
            left_hand_max_notes: 4,   // Root + 5th + octave, or 4-note chord
            right_hand_max_notes: 5,  // Full 5-note chord
            min_finger_spacing: 2,    // Major 2nd minimum
        }
    }
}

impl PianoHandSpan {
    /// Create a hand span for small hands
    pub fn small_hands() -> Self {
        Self {
            left_hand_max_span: 8,   // Octave
            right_hand_max_span: 10, // Octave + minor 3rd
            left_hand_max_notes: 3,
            right_hand_max_notes: 4,
            min_finger_spacing: 2,
        }
    }

    /// Create a hand span for large hands
    pub fn large_hands() -> Self {
        Self {
            left_hand_max_span: 12,  // Octave + perfect 4th
            right_hand_max_span: 14, // Octave + minor 6th
            left_hand_max_notes: 5,
            right_hand_max_notes: 6,
            min_finger_spacing: 2,
        }
    }

    /// Check if a span (in semitones) is comfortable for left hand
    pub fn is_left_hand_comfortable(&self, span: u8) -> bool {
        span <= self.left_hand_max_span
    }

    /// Check if a span (in semitones) is comfortable for right hand
    pub fn is_right_hand_comfortable(&self, span: u8) -> bool {
        span <= self.right_hand_max_span
    }

    /// Check if number of notes is comfortable for left hand
    pub fn is_left_hand_note_count_comfortable(&self, count: usize) -> bool {
        count <= self.left_hand_max_notes as usize
    }

    /// Check if number of notes is comfortable for right hand
    pub fn is_right_hand_note_count_comfortable(&self, count: usize) -> bool {
        count <= self.right_hand_max_notes as usize
    }
}

/// Piano voicing configuration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PianoVoicingConfig {
    /// Hand position
    pub hand_position: PianoHandPosition,
    /// Voicing type
    pub voicing_type: PianoVoicingType,
    /// Hand span constraints
    pub hand_span: PianoHandSpan,
    /// Whether to prefer root position
    pub prefer_root_position: bool,
    /// Whether to avoid voice crossing
    pub avoid_voice_crossing: bool,
    /// Maximum total span for both hands combined
    pub max_total_span: u8,
}

impl Default for PianoVoicingConfig {
    fn default() -> Self {
        Self {
            hand_position: PianoHandPosition::default(),
            voicing_type: PianoVoicingType::default(),
            hand_span: PianoHandSpan::default(),
            prefer_root_position: true,
            avoid_voice_crossing: true,
            max_total_span: 24, // Two octaves
        }
    }
}

impl PianoVoicingConfig {
    /// Create a configuration for left hand only
    pub fn left_hand() -> Self {
        Self {
            hand_position: PianoHandPosition::LeftHand,
            ..Default::default()
        }
    }

    /// Create a configuration for right hand only
    pub fn right_hand() -> Self {
        Self {
            hand_position: PianoHandPosition::RightHand,
            ..Default::default()
        }
    }

    /// Create a configuration for jazz voicing
    pub fn jazz() -> Self {
        Self {
            voicing_type: PianoVoicingType::Shell,
            prefer_root_position: false,
            ..Default::default()
        }
    }

    /// Create a configuration for classical voicing
    pub fn classical() -> Self {
        Self {
            voicing_type: PianoVoicingType::Block,
            prefer_root_position: true,
            avoid_voice_crossing: true,
            ..Default::default()
        }
    }

    /// Create a configuration for broken chords
    pub fn broken() -> Self {
        Self {
            voicing_type: PianoVoicingType::Broken,
            ..Default::default()
        }
    }
}

/// Piano-specific voicing engine
pub struct PianoVoicer {
    config: PianoVoicingConfig,
}

impl PianoVoicer {
    /// Create a new piano voicer with the given configuration
    pub fn new(config: PianoVoicingConfig) -> Self {
        Self { config }
    }

    /// Create a piano voicer with default configuration
    pub fn default() -> Self {
        Self::new(PianoVoicingConfig::default())
    }

    /// Check if a set of pitches is ergonomically comfortable for piano
    pub fn is_ergonomic(&self, pitches: &[Pitch]) -> bool {
        if pitches.is_empty() {
            return true;
        }

        let sorted_pitches: Vec<Pitch> = {
            let mut sorted = pitches.to_vec();
            sorted.sort_by_key(|p| p.midi_number());
            sorted
        };

        let span = (sorted_pitches.last().unwrap().midi_number() - sorted_pitches.first().unwrap().midi_number()) as u8;
        let note_count = pitches.len();

        match self.config.hand_position {
            PianoHandPosition::LeftHand => {
                self.config.hand_span.is_left_hand_comfortable(span)
                    && self.config.hand_span.is_left_hand_note_count_comfortable(note_count)
            }
            PianoHandPosition::RightHand => {
                self.config.hand_span.is_right_hand_comfortable(span)
                    && self.config.hand_span.is_right_hand_note_count_comfortable(note_count)
            }
            PianoHandPosition::BothHands => {
                // For both hands, we need to check if we can split the chord
                // This is a simplified check - actual implementation would split the chord
                span <= self.config.max_total_span
                    && note_count <= (self.config.hand_span.left_hand_max_notes + self.config.hand_span.right_hand_max_notes) as usize
            }
        }
    }

    /// Calculate the ergonomic score for a set of pitches (higher = more comfortable)
    pub fn ergonomic_score(&self, pitches: &[Pitch]) -> f32 {
        if pitches.is_empty() {
            return 1.0;
        }

        let sorted_pitches: Vec<Pitch> = {
            let mut sorted = pitches.to_vec();
            sorted.sort_by_key(|p| p.midi_number());
            sorted
        };

        let span = (sorted_pitches.last().unwrap().midi_number() - sorted_pitches.first().unwrap().midi_number()) as u8;
        let note_count = pitches.len();

        // Calculate span score (closer to comfortable span is better)
        let max_span = match self.config.hand_position {
            PianoHandPosition::LeftHand => self.config.hand_span.left_hand_max_span,
            PianoHandPosition::RightHand => self.config.hand_span.right_hand_max_span,
            PianoHandPosition::BothHands => self.config.max_total_span,
        };

        let span_score = if span <= max_span {
            1.0 - (span as f32 / max_span as f32) * 0.5 // Prefer smaller spans
        } else {
            0.0 // Outside comfortable range
        };

        // Calculate note count score
        let max_notes = match self.config.hand_position {
            PianoHandPosition::LeftHand => self.config.hand_span.left_hand_max_notes,
            PianoHandPosition::RightHand => self.config.hand_span.right_hand_max_notes,
            PianoHandPosition::BothHands => self.config.hand_span.left_hand_max_notes + self.config.hand_span.right_hand_max_notes,
        };

        let count_score = if note_count <= max_notes as usize {
            1.0 - (note_count as f32 / max_notes as f32) * 0.3 // Slight preference for fewer notes
        } else {
            0.0 // Too many notes
        };

        // Calculate spacing score (prefer even spacing)
        let spacing_score = if sorted_pitches.len() >= 2 {
            let intervals: Vec<i32> = sorted_pitches
                .windows(2)
                .map(|window| (window[1].midi_number() - window[0].midi_number()) as i32)
                .collect();

            let avg_interval = intervals.iter().sum::<i32>() as f32 / intervals.len() as f32;
            let variance = intervals.iter().map(|&i| (i as f32 - avg_interval).powi(2)).sum::<f32>() / intervals.len() as f32;

            // Lower variance is better (more even spacing)
            1.0 / (1.0 + variance.sqrt())
        } else {
            1.0
        };

        // Combine scores
        (span_score * 0.4) + (count_score * 0.3) + (spacing_score * 0.3)
    }
}

impl Default for PianoVoicer {
    fn default() -> Self {
        Self::new(PianoVoicingConfig::default())
    }
}