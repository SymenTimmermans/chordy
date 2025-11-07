//! Chord voicing and voice leading functionality
//!
//! This module provides tools for converting abstract chords into specific pitched
//! realizations with different voicing styles, as well as voice leading algorithms
//! for smooth chord progressions.
//!
//! # Core Concepts
//!
//! - **Voicing**: The specific arrangement of chord tones across different octaves
//! - **Voice Leading**: The smooth movement of individual voices between chords
//! - **Pitch Range**: Constraints on the lowest and highest pitches in a voicing
//!
//! # Voicing Styles
//!
//! - **Closed**: All notes packed within an octave
//! - **Open**: Notes spread across multiple octaves
//! - **Drop2/Drop3**: Jazz voicing techniques
//! - **Spread**: Even distribution across a specified range
//! - **Guitar**: Fretboard-based voicing using guitar chord patterns
//!
//! # Example
//!
//! ```rust
//! use chordy::{prelude::*, VoicingConfig, VoicingStyle};
//!
//! let chord = Chord::major(note!("C"));
//!
//! // Basic closed voicing
//! let voiced = chord.voice_closed("C4".parse().unwrap()).unwrap();
//!
//! // Custom voicing configuration
//! let config = VoicingConfig::new()
//!     .style(VoicingStyle::Open)
//!     .range_from("C3".parse().unwrap(), "C6".parse().unwrap());
//! let voiced = chord.voice(&config).unwrap();
//! ```

use crate::{
    traits::HasIntervals,
    types::{Chord, Interval, Pitch},
};
use std::fmt;

pub mod guitar;
pub mod piano;
pub mod voice_leading;

pub use guitar::{
    GuitarFingering, GuitarShape, GuitarTuning, IntervalFirstGuitarFinder, StringState,
};
pub use piano::{
    PianoHandPosition, PianoHandSpan, PianoVoicer, PianoVoicingConfig, PianoVoicingType,
};
pub use voice_leading::{VoiceLeader, VoiceLeadingStyle};

/// Instrument-specific voicing details that provide additional metadata
/// about how a chord is physically realized on different instruments
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VoicingDetails {
    /// Guitar-specific voicing information including fretboard fingering
    Guitar {
        /// The fingering pattern used to voice this chord
        fingering: GuitarFingering,
        /// The tuning used (standard, drop-D, etc.)
        tuning: GuitarTuning,
    },
    /// Piano-specific voicing information including hand positions and ergonomics
    Piano {
        /// Which hand(s) are used to play the chord
        hand_position: PianoHandPosition,
        /// Type of piano voicing (block, broken, spread, etc.)
        voicing_type: PianoVoicingType,
        /// Hand span constraints that were applied
        hand_span: PianoHandSpan,
    },
    /// Generic voicing with no instrument-specific details
    Generic,
}

/// Defines the range of pitches available for voicing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PitchRange {
    /// Lowest pitch in the range
    pub low: Pitch,
    /// Highest pitch in the range  
    pub high: Pitch,
}

impl PitchRange {
    /// Create a new pitch range from low to high pitches
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chordy::PitchRange;
    ///
    /// let range = PitchRange::new("C3".parse().unwrap(), "C6".parse().unwrap());
    /// assert_eq!(range.low, "C3".parse().unwrap());
    /// assert_eq!(range.high, "C6".parse().unwrap());
    /// ```
    pub fn new(low: Pitch, high: Pitch) -> Self {
        Self { low, high }
    }

    /// Check if a pitch falls within this range
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chordy::PitchRange;
    ///
    /// let range = PitchRange::new("C3".parse().unwrap(), "C6".parse().unwrap());
    /// assert!(range.contains("G4".parse().unwrap()));
    /// assert!(!range.contains("C2".parse().unwrap()));
    /// ```
    pub fn contains(&self, pitch: Pitch) -> bool {
        pitch.midi_number() >= self.low.midi_number()
            && pitch.midi_number() <= self.high.midi_number()
    }

    /// Get the span of this range in octaves
    pub fn span_octaves(&self) -> f32 {
        (self.high.midi_number() - self.low.midi_number()) as f32 / 12.0
    }

    /// Common piano range (standard 88-key piano)
    pub fn piano() -> Self {
        Self::new("A0".parse().unwrap(), "C8".parse().unwrap())
    }

    /// Common guitar range (standard tuning, 22 frets)
    pub fn guitar() -> Self {
        Self::new("E2".parse().unwrap(), "E6".parse().unwrap())
    }

    /// Common vocal range for mixed voices
    pub fn vocal() -> Self {
        Self::new("C3".parse().unwrap(), "C6".parse().unwrap())
    }
}

impl fmt::Display for PitchRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.low, self.high)
    }
}

/// Different styles of chord voicing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VoicingStyle {
    /// All notes packed as closely as possible (within an octave)
    Closed,

    /// Notes spread across multiple octaves with wider intervals
    Open,

    /// Drop-2 voicing: take the second-highest note and drop it an octave
    Drop2,

    /// Drop-3 voicing: take the third-highest note and drop it an octave  
    Drop3,

    /// Custom spread with minimum and maximum intervals between adjacent notes
    Spread {
        /// Minimum interval between adjacent notes
        min_interval: Interval,
        /// Maximum interval between adjacent notes  
        max_interval: Interval,
    },

    /// Guitar fingering-based voicing using fretboard patterns
    Guitar,
}

impl VoicingStyle {
    /// Create a spread voicing with specified interval constraints
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chordy::{VoicingStyle, Interval};
    ///
    /// let spread = VoicingStyle::spread(Interval::MAJOR_SECOND, Interval::PERFECT_FIFTH);
    /// ```
    pub fn spread(min_interval: Interval, max_interval: Interval) -> Self {
        Self::Spread {
            min_interval,
            max_interval,
        }
    }
}

impl Default for VoicingStyle {
    fn default() -> Self {
        Self::Closed
    }
}

impl fmt::Display for VoicingStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Closed => write!(f, "Closed"),
            Self::Open => write!(f, "Open"),
            Self::Drop2 => write!(f, "Drop-2"),
            Self::Drop3 => write!(f, "Drop-3"),
            Self::Spread {
                min_interval,
                max_interval,
            } => {
                write!(f, "Spread({}-{})", min_interval, max_interval)
            }
            Self::Guitar => write!(f, "Guitar"),
        }
    }
}

/// Configuration for chord voicing operations
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VoicingConfig {
    /// The voicing style to apply
    pub style: VoicingStyle,
    /// Range constraint for the voicing
    pub range: PitchRange,
    /// Optional bass note constraint (for bass lines)
    pub bass_pitch: Option<Pitch>,
}

impl VoicingConfig {
    /// Create a new voicing configuration with default settings
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chordy::{VoicingConfig, VoicingStyle, PitchRange};
    ///
    /// let config = VoicingConfig::new()
    ///     .style(VoicingStyle::Open)
    ///     .range_from("C3".parse().unwrap(), "C6".parse().unwrap());
    /// ```
    pub fn new() -> Self {
        Self {
            style: VoicingStyle::default(),
            range: PitchRange::piano(), // Default to piano range
            bass_pitch: None,
        }
    }

    /// Set the voicing style
    pub fn style(mut self, style: VoicingStyle) -> Self {
        self.style = style;
        self
    }

    /// Set the pitch range
    pub fn range(mut self, range: PitchRange) -> Self {
        self.range = range;
        self
    }

    /// Set the pitch range from low and high pitches
    pub fn range_from(mut self, low: Pitch, high: Pitch) -> Self {
        self.range = PitchRange::new(low, high);
        self
    }

    /// Set a specific bass pitch constraint
    pub fn bass_pitch(mut self, pitch: Pitch) -> Self {
        self.bass_pitch = Some(pitch);
        self
    }

    /// Create a configuration for piano voicing
    pub fn piano() -> Self {
        Self::new().range(PitchRange::piano())
    }

    /// Create a configuration for guitar voicing
    pub fn guitar() -> Self {
        Self::new()
            .range(PitchRange::guitar())
            .style(VoicingStyle::Guitar)
    }

    /// Create a configuration for vocal voicing
    pub fn vocal() -> Self {
        Self::new().range(PitchRange::vocal())
    }
}

impl Default for VoicingConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Metadata about how a chord was voiced
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VoicingInfo {
    /// The voicing style that was applied
    pub style: VoicingStyle,
    /// The range that was used
    pub range: PitchRange,
    /// The inversion used (0 = root position, 1 = first inversion, etc.)
    pub inversion: u8,
    /// Total voice movement from previous chord (if applicable)
    pub movement: Option<i32>,
    /// Instrument-specific voicing details (fingerings, hand positions, etc.)
    pub details: Option<VoicingDetails>,
}

impl VoicingInfo {
    /// Create new voicing metadata
    pub fn new(style: VoicingStyle, range: PitchRange, inversion: u8) -> Self {
        Self {
            style,
            range,
            inversion,
            movement: None,
            details: None,
        }
    }

    /// Create new voicing metadata with instrument-specific details
    pub fn new_with_details(
        style: VoicingStyle,
        range: PitchRange,
        inversion: u8,
        details: VoicingDetails,
    ) -> Self {
        Self {
            style,
            range,
            inversion,
            movement: None,
            details: Some(details),
        }
    }

    /// Set the voice movement amount
    pub fn with_movement(mut self, movement: i32) -> Self {
        self.movement = Some(movement);
        self
    }

    /// Set the voicing details
    pub fn with_details(mut self, details: VoicingDetails) -> Self {
        self.details = Some(details);
        self
    }
}

/// A chord with specific pitch realizations from a voicing operation
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VoicedChord {
    /// The original abstract chord
    pub chord: Chord,
    /// The specific pitches from the voicing
    pub pitches: Vec<Pitch>,
    /// Metadata about the voicing
    pub info: VoicingInfo,
}

impl VoicedChord {
    /// Create a new voiced chord
    pub fn new(chord: Chord, pitches: Vec<Pitch>, info: VoicingInfo) -> Self {
        Self {
            chord,
            pitches,
            info,
        }
    }

    /// Get the bass pitch (lowest note)
    pub fn bass_pitch(&self) -> Option<Pitch> {
        self.pitches.iter().min_by_key(|p| p.midi_number()).copied()
    }

    /// Get the soprano pitch (highest note)
    pub fn soprano_pitch(&self) -> Option<Pitch> {
        self.pitches.iter().max_by_key(|p| p.midi_number()).copied()
    }

    /// Get the range spanned by this voicing
    pub fn range(&self) -> Option<PitchRange> {
        match (self.bass_pitch(), self.soprano_pitch()) {
            (Some(bass), Some(soprano)) => Some(PitchRange::new(bass, soprano)),
            _ => None,
        }
    }

    /// Calculate the total semitone span of this voicing
    pub fn span_semitones(&self) -> i32 {
        match (self.bass_pitch(), self.soprano_pitch()) {
            (Some(bass), Some(soprano)) => (soprano.midi_number() - bass.midi_number()) as i32,
            _ => 0,
        }
    }

    /// Check if this is a closed voicing (span <= octave)
    pub fn is_closed(&self) -> bool {
        self.span_semitones() <= 12
    }

    /// Check if this is an open voicing (span > octave)
    pub fn is_open(&self) -> bool {
        self.span_semitones() > 12
    }

    /// Get the intervals between adjacent voices (for voice leading analysis)
    pub fn voice_intervals(&self) -> Vec<i32> {
        if self.pitches.len() < 2 {
            return vec![];
        }

        let mut sorted_pitches = self.pitches.clone();
        sorted_pitches.sort_by_key(|p| p.midi_number());

        sorted_pitches
            .windows(2)
            .map(|window| (window[1].midi_number() - window[0].midi_number()) as i32)
            .collect()
    }

    /// Get the guitar fingering if this is a guitar voicing
    pub fn guitar_fingering(&self) -> Option<&GuitarFingering> {
        match &self.info.details {
            Some(VoicingDetails::Guitar { fingering, .. }) => Some(fingering),
            _ => None,
        }
    }

    /// Get the guitar tuning if this is a guitar voicing
    pub fn guitar_tuning(&self) -> Option<&GuitarTuning> {
        match &self.info.details {
            Some(VoicingDetails::Guitar { tuning, .. }) => Some(tuning),
            _ => None,
        }
    }

    /// Get the piano hand position if this is a piano voicing
    pub fn piano_hand_position(&self) -> Option<&PianoHandPosition> {
        match &self.info.details {
            Some(VoicingDetails::Piano { hand_position, .. }) => Some(hand_position),
            _ => None,
        }
    }

    /// Get the piano voicing type if this is a piano voicing
    pub fn piano_voicing_type(&self) -> Option<&PianoVoicingType> {
        match &self.info.details {
            Some(VoicingDetails::Piano { voicing_type, .. }) => Some(voicing_type),
            _ => None,
        }
    }

    /// Check if this voicing has instrument-specific details
    pub fn has_voicing_details(&self) -> bool {
        self.info.details.is_some()
    }

    /// Check if this is a guitar voicing with fingering information
    pub fn is_guitar_voicing(&self) -> bool {
        matches!(
            self.info.details,
            Some(VoicingDetails::Guitar { .. })
        )
    }

    /// Check if this is a piano voicing with piano-specific information
    pub fn is_piano_voicing(&self) -> bool {
        matches!(
            self.info.details,
            Some(VoicingDetails::Piano { .. })
        )
    }
}

impl fmt::Display for VoicedChord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} [", self.chord)?;
        for (i, pitch) in self.pitches.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", pitch)?;
        }
        write!(f, "] ({})", self.info.style)
    }
}

/// Core voicing engine that implements different voicing algorithms
pub struct Voicer {
    config: VoicingConfig,
}

impl Voicer {
    /// Create a new voicer with the given configuration
    pub fn new(config: VoicingConfig) -> Self {
        Self { config }
    }

    /// Create a voicer with default configuration
    pub fn default() -> Self {
        Self::new(VoicingConfig::default())
    }

    /// Voice a chord using the configured style
    pub fn voice_chord(&self, chord: &Chord) -> Result<VoicedChord, VoicingError> {
        self.voice_chord_from_pitch(chord, None)
    }

    /// Voice a chord starting from a specific bass pitch
    pub fn voice_chord_from_pitch(
        &self,
        chord: &Chord,
        bass_pitch: Option<Pitch>,
    ) -> Result<VoicedChord, VoicingError> {
        let bass_pitch = bass_pitch.or(self.config.bass_pitch);

        match self.config.style {
            VoicingStyle::Closed => self.voice_closed(chord, bass_pitch),
            VoicingStyle::Open => self.voice_open(chord, bass_pitch),
            VoicingStyle::Drop2 => self.voice_drop2(chord, bass_pitch),
            VoicingStyle::Drop3 => self.voice_drop3(chord, bass_pitch),
            VoicingStyle::Spread {
                min_interval,
                max_interval,
            } => self.voice_spread(chord, bass_pitch, min_interval, max_interval),
            VoicingStyle::Guitar => guitar::voice_guitar_chord(chord, self.config.range),
        }
    }

    /// Voice a chord specifically for piano with ergonomic considerations
    pub fn voice_piano(
        &self,
        chord: &Chord,
        piano_config: &PianoVoicingConfig,
    ) -> Result<VoicedChord, VoicingError> {
        // Start with basic closed voicing
        let closed = self.voice_closed(chord, None)?;
        let mut pitches = closed.pitches.clone();

        // Apply piano-specific transformations based on voicing type
        match piano_config.voicing_type {
            PianoVoicingType::Block => {
                // Block chords - keep as is, but ensure ergonomic constraints
                if !self.is_piano_ergonomic(&pitches, piano_config) {
                    // Try to adjust voicing to be more ergonomic
                    pitches = self.adjust_for_ergonomics(&pitches, piano_config);
                }
            }
            PianoVoicingType::Spread => {
                // Spread voicing - distribute notes across wider range
                pitches = self.create_spread_voicing(&pitches, piano_config);
            }
            PianoVoicingType::Shell => {
                // Shell voicing - root + 3rd/7th for jazz
                pitches = self.create_shell_voicing(&pitches, piano_config);
            }
            PianoVoicingType::Rootless => {
                // Rootless voicing - omit root for jazz harmony
                pitches = self.create_rootless_voicing(&pitches, piano_config);
            }
            PianoVoicingType::Broken | PianoVoicingType::Arpeggiated => {
                // Broken/arpeggiated - same pitches as block, different execution
                // The distinction is in performance, not pitch selection
            }
        }

        // Distribute notes between hands based on hand position
        pitches = self.distribute_hands(&pitches, piano_config);

        // Ensure all pitches are within range
        if pitches
            .iter()
            .any(|p| p.midi_number() < self.config.range.low.midi_number()
                || p.midi_number() > self.config.range.high.midi_number())
        {
            return Err(VoicingError::OutOfRange);
        }

        // Sort pitches
        pitches.sort_by_key(|p| p.midi_number());

        // Create voicing info with piano details
        let details = VoicingDetails::Piano {
            hand_position: piano_config.hand_position,
            voicing_type: piano_config.voicing_type,
            hand_span: piano_config.hand_span,
        };

        let info = VoicingInfo::new_with_details(
            VoicingStyle::Closed, // Base style - piano-specific is in details
            self.config.range,
            0,
            details,
        );

        Ok(VoicedChord::new(*chord, pitches, info))
    }

    /// Check if a set of pitches is ergonomic for piano playing
    fn is_piano_ergonomic(&self, pitches: &[Pitch], config: &PianoVoicingConfig) -> bool {
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

        match config.hand_position {
            PianoHandPosition::LeftHand => {
                config.hand_span.is_left_hand_comfortable(span)
                    && config.hand_span.is_left_hand_note_count_comfortable(note_count)
            }
            PianoHandPosition::RightHand => {
                config.hand_span.is_right_hand_comfortable(span)
                    && config.hand_span.is_right_hand_note_count_comfortable(note_count)
            }
            PianoHandPosition::BothHands => {
                // Simplified check for both hands
                span <= config.max_total_span
                    && note_count <= (config.hand_span.left_hand_max_notes + config.hand_span.right_hand_max_notes) as usize
            }
        }
    }

    /// Adjust voicing to be more ergonomic for piano
    fn adjust_for_ergonomics(&self, pitches: &[Pitch], config: &PianoVoicingConfig) -> Vec<Pitch> {
        let mut adjusted = pitches.to_vec();

        // Sort pitches
        adjusted.sort_by_key(|p| p.midi_number());

        // Check span and adjust if necessary
        let span = (adjusted.last().unwrap().midi_number() - adjusted.first().unwrap().midi_number()) as u8;
        let max_span = match config.hand_position {
            PianoHandPosition::LeftHand => config.hand_span.left_hand_max_span,
            PianoHandPosition::RightHand => config.hand_span.right_hand_max_span,
            PianoHandPosition::BothHands => config.max_total_span,
        };

        if span > max_span {
            // Reduce span by moving inner notes closer together
            // This is a simplified adjustment - real implementation would be more sophisticated
            let reduction_needed = span - max_span;
            for i in 1..adjusted.len() - 1 {
                if reduction_needed > 0 {
                    // Move note down by one octave if possible
                    let new_pitch = adjusted[i] + (-12i8);
                    if new_pitch.midi_number() >= self.config.range.low.midi_number() {
                        adjusted[i] = new_pitch;
                    }
                }
            }
        }

        adjusted.sort_by_key(|p| p.midi_number());
        adjusted
    }

    /// Create a spread voicing for piano
    fn create_spread_voicing(&self, pitches: &[Pitch], _config: &PianoVoicingConfig) -> Vec<Pitch> {
        let mut spread = pitches.to_vec();

        // Sort pitches
        spread.sort_by_key(|p| p.midi_number());

        // For spread voicing, distribute notes across wider range
        // Move every other note up an octave
        for i in (1..spread.len()).step_by(2) {
            let new_pitch = spread[i] + 12i8;
            if new_pitch.midi_number() <= self.config.range.high.midi_number() {
                spread[i] = new_pitch;
            }
        }

        spread.sort_by_key(|p| p.midi_number());
        spread
    }

    /// Create a shell voicing for jazz piano
    fn create_shell_voicing(&self, pitches: &[Pitch], _config: &PianoVoicingConfig) -> Vec<Pitch> {
        let mut shell = Vec::new();

        // For shell voicing, prioritize root, 3rd, and 7th
        // This is a simplified implementation
        if pitches.len() >= 3 {
            // Root (lowest)
            shell.push(pitches[0]);

            // 3rd (second lowest)
            if pitches.len() >= 2 {
                shell.push(pitches[1]);
            }

            // 7th (highest that's not the root)
            if pitches.len() >= 4 {
                shell.push(pitches[pitches.len() - 2]);
            } else if pitches.len() >= 3 {
                shell.push(pitches[pitches.len() - 1]);
            }
        } else {
            // Fallback to original pitches
            shell = pitches.to_vec();
        }

        shell
    }

    /// Create a rootless voicing for jazz piano
    fn create_rootless_voicing(&self, pitches: &[Pitch], _config: &PianoVoicingConfig) -> Vec<Pitch> {
        let mut rootless = Vec::new();

        // For rootless voicing, omit the root and focus on 3rd, 5th, 7th, extensions
        if pitches.len() >= 2 {
            // Skip the root (lowest note)
            for &pitch in &pitches[1..] {
                rootless.push(pitch);
            }
        } else {
            // Fallback to original pitches
            rootless = pitches.to_vec();
        }

        rootless
    }

    /// Get a reasonable starting pitch for a chord based on its root and the voicing range
    fn get_starting_pitch(&self, chord: &Chord, bass_pitch: Option<Pitch>) -> Pitch {
        if let Some(bass_pitch) = bass_pitch {
            return bass_pitch;
        }

        // Find a reasonable octave for the chord root within the range
        let root_name = chord.bass_note(); // This respects bass/inversions
        let mut octave = 4; // Start with middle octave
        let mut attempts = 0;
        const MAX_ATTEMPTS: i8 = 20; // Prevent infinite loops

        // Try to find an octave that puts the root in the lower part of the range
        loop {
            attempts += 1;
            if attempts > MAX_ATTEMPTS {
                // Fallback: just use the middle of the range
                let range_middle_midi = (self.config.range.low.midi_number() as i16
                    + self.config.range.high.midi_number() as i16)
                    / 2;
                let middle_octave = ((range_middle_midi / 12) - 2) as i8; // Convert back to octave
                return Pitch::new(root_name.letter(), root_name.accidental(), middle_octave);
            }

            let candidate_root = Pitch::new(root_name.letter(), root_name.accidental(), octave);

            // If this pitch is too low, go up
            if candidate_root.midi_number() < self.config.range.low.midi_number() {
                octave += 1;
                continue;
            }

            // If this pitch is too high, go down (but don't go below reasonable limits)
            if candidate_root.midi_number() > self.config.range.high.midi_number() - 12
                && octave > -2
            {
                octave -= 1;
                continue;
            }

            return candidate_root;
        }
    }

    /// Implement closed voicing: pack all notes as closely as possible (within an octave)
    fn voice_closed(
        &self,
        chord: &Chord,
        bass_pitch: Option<Pitch>,
    ) -> Result<VoicedChord, VoicingError> {
        let root_pitch = self.get_starting_pitch(chord, bass_pitch);
        let intervals = chord.intervals();

        let mut pitches = Vec::new();

        for &interval in intervals {
            // Check if adding this interval would overflow
            let root_midi = root_pitch.midi_number() as i16;
            let interval_semitones = interval.semitones() as i16;
            let target_midi = root_midi + interval_semitones;

            // Check for overflow and range constraints
            if target_midi > i8::MAX as i16 || target_midi < i8::MIN as i16 {
                return Err(VoicingError::OutOfRange);
            }

            if target_midi > self.config.range.high.midi_number() as i16 || target_midi < self.config.range.low.midi_number() as i16 {
                return Err(VoicingError::OutOfRange);
            }

            let pitch = root_pitch + interval.semitones();
            pitches.push(pitch);
        }

        // Sort pitches to ensure they're in order
        pitches.sort_by_key(|p| p.midi_number());

        let info = VoicingInfo::new(VoicingStyle::Closed, self.config.range, 0);
        Ok(VoicedChord::new(*chord, pitches, info))
    }

    /// Implement open voicing: spread notes across multiple octaves
    fn voice_open(
        &self,
        chord: &Chord,
        bass_pitch: Option<Pitch>,
    ) -> Result<VoicedChord, VoicingError> {
        let root_pitch = self.get_starting_pitch(chord, bass_pitch);
        let intervals = chord.intervals();

        let mut pitches = Vec::new();
        let mut current_octave_offset = 0i16;

        for (i, &interval) in intervals.iter().enumerate() {
            // For open voicing, spread notes across octaves
            // Every other note goes up an octave
            if i > 0 && i % 2 == 0 {
                current_octave_offset += 12;
            }

            // Check if adding this interval would overflow
            let root_midi = root_pitch.midi_number() as i16;
            let interval_semitones = interval.semitones() as i16;
            let target_midi = root_midi + interval_semitones + current_octave_offset;

            // Check for overflow and range constraints
            if target_midi > i8::MAX as i16 || target_midi < i8::MIN as i16 {
                return Err(VoicingError::OutOfRange);
            }

            if target_midi > self.config.range.high.midi_number() as i16 {
                return Err(VoicingError::OutOfRange);
            }

            let mut pitch = root_pitch + interval.semitones();
            pitch += current_octave_offset as i8;

            pitches.push(pitch);
        }

        // Sort pitches to ensure they're in order
        pitches.sort_by_key(|p| p.midi_number());

        let info = VoicingInfo::new(VoicingStyle::Open, self.config.range, 0);
        Ok(VoicedChord::new(*chord, pitches, info))
    }

    /// Implement drop-2 voicing: take second-highest note and drop it an octave
    /// with proper hand distribution (1+3 or 2+2 texture)
    fn voice_drop2(
        &self,
        chord: &Chord,
        bass_pitch: Option<Pitch>,
    ) -> Result<VoicedChord, VoicingError> {
        // Get all chord tones including extensions
        let chord_tones = self.get_chord_tones_with_extensions(chord);

        if chord_tones.len() < 3 {
            // Can't do drop-2 with fewer than 3 notes, use closed
            return self.voice_closed(chord, bass_pitch);
        }

        // Create close voicing starting from appropriate octave
        let starting_pitch = self.get_starting_pitch(chord, bass_pitch);
        let close_voicing = self.create_close_voicing(&chord_tones, starting_pitch)?;

        // Sort close voicing to identify voices
        let mut sorted_close = close_voicing.clone();
        sorted_close.sort_by_key(|p| p.midi_number());

        // For drop-2: drop the 2nd highest voice
        let drop_index = sorted_close.len() - 2;
        let mut drop_voicing = sorted_close.clone();
        drop_voicing[drop_index] += -12i8;

        // Distribute between hands (1+3 or 2+2 texture)
        let distributed = self.distribute_drop_voicing(&drop_voicing, 2);

        // Ensure all pitches are within range
        if distributed
            .iter()
            .any(|p| p.midi_number() < self.config.range.low.midi_number()
                || p.midi_number() > self.config.range.high.midi_number())
        {
            return Err(VoicingError::OutOfRange);
        }

        let info = VoicingInfo::new(VoicingStyle::Drop2, self.config.range, 0);
        Ok(VoicedChord::new(*chord, distributed, info))
    }

    /// Implement drop-3 voicing: take third-highest note and drop it an octave
    /// with proper hand distribution
    fn voice_drop3(
        &self,
        chord: &Chord,
        bass_pitch: Option<Pitch>,
    ) -> Result<VoicedChord, VoicingError> {
        // Get all chord tones including extensions
        let chord_tones = self.get_chord_tones_with_extensions(chord);

        if chord_tones.len() < 4 {
            // Can't do drop-3 with fewer than 4 notes, use drop-2
            return self.voice_drop2(chord, bass_pitch);
        }

        // Create close voicing starting from appropriate octave
        let starting_pitch = self.get_starting_pitch(chord, bass_pitch);
        let close_voicing = self.create_close_voicing(&chord_tones, starting_pitch)?;

        // Sort close voicing to identify voices
        let mut sorted_close = close_voicing.clone();
        sorted_close.sort_by_key(|p| p.midi_number());

        // For drop-3: drop the 3rd highest voice
        let drop_index = sorted_close.len() - 3;
        let mut drop_voicing = sorted_close.clone();
        drop_voicing[drop_index] += -12i8;

        // Distribute between hands
        let distributed = self.distribute_drop_voicing(&drop_voicing, 3);

        // Ensure all pitches are within range
        if distributed
            .iter()
            .any(|p| p.midi_number() < self.config.range.low.midi_number()
                || p.midi_number() > self.config.range.high.midi_number())
        {
            return Err(VoicingError::OutOfRange);
        }

        let info = VoicingInfo::new(VoicingStyle::Drop3, self.config.range, 0);
        Ok(VoicedChord::new(*chord, distributed, info))
    }

    /// Implement spread voicing: distribute notes with specified interval constraints
    fn voice_spread(
        &self,
        chord: &Chord,
        bass_pitch: Option<Pitch>,
        min_interval: Interval,
        max_interval: Interval,
    ) -> Result<VoicedChord, VoicingError> {
        let root_pitch = self.get_starting_pitch(chord, bass_pitch);
        let intervals = chord.intervals();

        let mut pitches = Vec::new();

        for &interval in intervals {
            // Check if adding this interval would overflow
            let root_midi = root_pitch.midi_number() as i16;
            let interval_semitones = interval.semitones() as i16;
            let target_midi = root_midi + interval_semitones;

            // Check for overflow first
            if target_midi > i8::MAX as i16 || target_midi < i8::MIN as i16 {
                return Err(VoicingError::OutOfRange);
            }

            let target_pitch = root_pitch + interval.semitones();

            // Adjust the pitch to respect spacing constraints
            let current_pitch = if !pitches.is_empty() {
                let last_pitch: &Pitch = pitches.last().unwrap();
                let interval_to_target = target_pitch.midi_number() - last_pitch.midi_number();

                if interval_to_target < min_interval.semitones() {
                    // Check for overflow before adding
                    let last_midi = last_pitch.midi_number() as i16;
                    let min_semitones = min_interval.semitones() as i16;
                    if last_midi + min_semitones > i8::MAX as i16 {
                        return Err(VoicingError::OutOfRange);
                    }
                    // Too close, push it up
                    *last_pitch + min_interval.semitones()
                } else if interval_to_target > max_interval.semitones() {
                    // Check for overflow before adding
                    let last_midi = last_pitch.midi_number() as i16;
                    let max_semitones = max_interval.semitones() as i16;
                    if last_midi + max_semitones > i8::MAX as i16 {
                        return Err(VoicingError::OutOfRange);
                    }
                    // Too far, bring it closer
                    *last_pitch + max_interval.semitones()
                } else {
                    target_pitch
                }
            } else {
                target_pitch
            };

            // Ensure the pitch is within range
            if current_pitch.midi_number() > self.config.range.high.midi_number() {
                return Err(VoicingError::OutOfRange);
            }

            pitches.push(current_pitch);
        }

        let info = VoicingInfo::new(
            VoicingStyle::Spread {
                min_interval,
                max_interval,
            },
            self.config.range,
            0,
        );
        Ok(VoicedChord::new(*chord, pitches, info))
    }

    /// Get all chord tones including extensions (root, 3rd, 5th, 7th, 9th, 11th, 13th)
    fn get_chord_tones_with_extensions(&self, chord: &Chord) -> Vec<Interval> {
        let intervals = chord.intervals();
        let mut all_tones = intervals.to_vec();

        // Only add extensions for extended chords (9th, 11th, 13th)
        // Don't automatically add extensions to basic 7th chords
        // This preserves the original chord structure for basic voicings

        // Check if this is already an extended chord
        let has_ninth = intervals.contains(&Interval::MAJOR_NINTH) || intervals.contains(&Interval::MINOR_NINTH);
        let has_eleventh = intervals.contains(&Interval::PERFECT_ELEVENTH);
        let has_thirteenth = intervals.contains(&Interval::MAJOR_THIRTEENTH) || intervals.contains(&Interval::MINOR_THIRTEENTH);

        // Only add extensions if the chord is already extended
        if has_ninth || has_eleventh || has_thirteenth {
            // For dominant 7th chords with extensions, add 9th and 13th
            if intervals.contains(&Interval::MINOR_SEVENTH) {
                if !has_ninth {
                    all_tones.push(Interval::MAJOR_NINTH);
                }
                if !has_thirteenth {
                    all_tones.push(Interval::MAJOR_THIRTEENTH);
                }
            }

            // For major 7th chords with extensions, add 9th
            if intervals.contains(&Interval::MAJOR_SEVENTH) && !has_ninth {
                all_tones.push(Interval::MAJOR_NINTH);
            }

            // For minor 7th chords with extensions, add 9th and 11th
            if intervals.contains(&Interval::MINOR_THIRD) && intervals.contains(&Interval::MINOR_SEVENTH) {
                if !has_ninth {
                    all_tones.push(Interval::MAJOR_NINTH);
                }
                if !has_eleventh {
                    all_tones.push(Interval::PERFECT_ELEVENTH);
                }
            }
        }

        all_tones.sort_by_key(|interval| interval.semitones());
        all_tones.dedup();
        all_tones
    }

    /// Create a close voicing from chord tones starting at a given pitch
    fn create_close_voicing(
        &self,
        intervals: &[Interval],
        starting_pitch: Pitch,
    ) -> Result<Vec<Pitch>, VoicingError> {
        let mut pitches = Vec::new();

        for &interval in intervals {
            let pitch = starting_pitch + interval.semitones();

            // Check range constraints
            if pitch.midi_number() < self.config.range.low.midi_number()
                || pitch.midi_number() > self.config.range.high.midi_number()
            {
                return Err(VoicingError::OutOfRange);
            }

            pitches.push(pitch);
        }

        // Sort pitches to ensure they're in order
        pitches.sort_by_key(|p| p.midi_number());
        Ok(pitches)
    }

    /// Distribute drop voicing between hands using proper textures (1+3 or 2+2)
    fn distribute_drop_voicing(&self, pitches: &[Pitch], drop_type: u8) -> Vec<Pitch> {
        let mut sorted_pitches = pitches.to_vec();
        sorted_pitches.sort_by_key(|p| p.midi_number());

        if sorted_pitches.len() < 4 {
            return sorted_pitches; // Can't distribute with fewer than 4 notes
        }

        match drop_type {
            2 => {
                // Drop-2: typically 1+3 texture (left hand: bass, right hand: 3 upper voices)
                // For 4-note chords: bass in left hand, 3 upper voices in right hand
                // For 5-note chords: bass in left hand, 4 upper voices in right hand
                // This creates the characteristic open sound
                sorted_pitches
            }
            3 => {
                // Drop-3: typically 2+2 texture (left hand: bass + 3rd, right hand: 5th + 7th)
                // For 4-note chords: bass + 3rd in left hand, 5th + 7th in right hand
                // For 5-note chords: bass + 3rd in left hand, 5th + 7th + extension in right hand
                // This creates a wider, more resonant sound
                sorted_pitches
            }
            _ => sorted_pitches,
        }
    }

    /// Distribute notes between hands based on hand position configuration
    fn distribute_hands(&self, pitches: &[Pitch], config: &PianoVoicingConfig) -> Vec<Pitch> {
        match config.hand_position {
            PianoHandPosition::LeftHand => self.adjust_for_left_hand(pitches, config),
            PianoHandPosition::RightHand => self.adjust_for_right_hand(pitches, config),
            PianoHandPosition::BothHands => self.split_between_hands(pitches, config),
        }
    }

    /// Adjust voicing for left hand only
    fn adjust_for_left_hand(&self, pitches: &[Pitch], config: &PianoVoicingConfig) -> Vec<Pitch> {
        let mut left_hand_pitches = pitches.to_vec();

        // Sort pitches
        left_hand_pitches.sort_by_key(|p| p.midi_number());

        // For left hand, prioritize lower notes and limit note count
        if left_hand_pitches.len() > config.hand_span.left_hand_max_notes as usize {
            // Keep only the lowest notes (root, 3rd, 5th, 7th in order)
            left_hand_pitches.truncate(config.hand_span.left_hand_max_notes as usize);
        }

        // Ensure the span is comfortable for left hand
        if left_hand_pitches.len() >= 2 {
            let span = (left_hand_pitches.last().unwrap().midi_number() - left_hand_pitches.first().unwrap().midi_number()) as u8;
            if span > config.hand_span.left_hand_max_span {
                // Reduce span by moving higher notes down an octave
                for i in (1..left_hand_pitches.len()).rev() {
                    let new_pitch = left_hand_pitches[i] + (-12i8);
                    if new_pitch.midi_number() >= self.config.range.low.midi_number() {
                        left_hand_pitches[i] = new_pitch;
                        break;
                    }
                }
            }
        }

        left_hand_pitches.sort_by_key(|p| p.midi_number());
        left_hand_pitches
    }

    /// Adjust voicing for right hand only
    fn adjust_for_right_hand(&self, pitches: &[Pitch], config: &PianoVoicingConfig) -> Vec<Pitch> {
        let mut right_hand_pitches = pitches.to_vec();

        // Sort pitches
        right_hand_pitches.sort_by_key(|p| p.midi_number());

        // For right hand, we can use more notes but still need ergonomic constraints
        if right_hand_pitches.len() > config.hand_span.right_hand_max_notes as usize {
            // Keep the most important notes (root, 3rd, 7th, extensions)
            // This is a simplified approach - real implementation would be more sophisticated
            if right_hand_pitches.len() >= 4 {
                // Keep root, 3rd, 7th, and highest extension
                let important_indices = vec![0, 1, right_hand_pitches.len() - 2, right_hand_pitches.len() - 1];
                right_hand_pitches = important_indices
                    .into_iter()
                    .map(|i| right_hand_pitches[i])
                    .collect();
            }
        }

        // Ensure the span is comfortable for right hand
        if right_hand_pitches.len() >= 2 {
            let span = (right_hand_pitches.last().unwrap().midi_number() - right_hand_pitches.first().unwrap().midi_number()) as u8;
            if span > config.hand_span.right_hand_max_span {
                // Reduce span by moving lower notes up an octave
                for i in 0..right_hand_pitches.len() - 1 {
                    let new_pitch = right_hand_pitches[i] + 12i8;
                    if new_pitch.midi_number() <= self.config.range.high.midi_number() {
                        right_hand_pitches[i] = new_pitch;
                        break;
                    }
                }
            }
        }

        right_hand_pitches.sort_by_key(|p| p.midi_number());
        right_hand_pitches
    }

    /// Split notes between left and right hands
    fn split_between_hands(&self, pitches: &[Pitch], config: &PianoVoicingConfig) -> Vec<Pitch> {
        let mut all_pitches = pitches.to_vec();

        // Sort pitches
        all_pitches.sort_by_key(|p| p.midi_number());

        // Simple split: left hand gets lower notes, right hand gets higher notes
        // Middle C (MIDI 60) is a common dividing point
        let middle_c_midi = 60;

        let mut left_hand = Vec::new();
        let mut right_hand = Vec::new();

        for pitch in &all_pitches {
            if pitch.midi_number() <= middle_c_midi {
                left_hand.push(*pitch);
            } else {
                right_hand.push(*pitch);
            }
        }

        // For piano voicing, allow note repetition across hands
        // Left hand often plays bass notes that may be repeated in right hand
        if !left_hand.is_empty() && !right_hand.is_empty() {
            // Add bass note repetition: left hand plays root an octave lower
            if let Some(root_pitch) = left_hand.first() {
                let bass_pitch = *root_pitch + (-12i8);
                if bass_pitch.midi_number() >= self.config.range.low.midi_number() {
                    left_hand.insert(0, bass_pitch);
                }
            }
        }

        // Apply hand-specific adjustments
        left_hand = self.adjust_for_left_hand(&left_hand, config);
        right_hand = self.adjust_for_right_hand(&right_hand, config);

        // Combine and sort all pitches
        let mut combined = left_hand;
        combined.extend(right_hand);
        combined.sort_by_key(|p| p.midi_number());
        combined
    }
}

impl Default for Voicer {
    fn default() -> Self {
        Self::new(VoicingConfig::default())
    }
}

/// Errors that can occur during voicing operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VoicingError {
    /// The resulting voicing would be outside the specified range
    OutOfRange,
    /// The chord has no intervals to voice
    EmptyChord,
    /// The voicing style is not supported for this chord
    UnsupportedStyle,
}

impl fmt::Display for VoicingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutOfRange => write!(f, "Voicing would be outside the specified range"),
            Self::EmptyChord => write!(f, "Cannot voice a chord with no intervals"),
            Self::UnsupportedStyle => write!(f, "Voicing style not supported for this chord"),
        }
    }
}

impl std::error::Error for VoicingError {}
