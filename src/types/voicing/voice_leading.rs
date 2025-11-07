//! Voice leading optimization for smooth chord progressions

use crate::types::{Chord, Pitch};
use super::{VoicedChord, VoicingConfig, VoicingError, VoicingStyle, Voicer};

/// Different styles of voice leading
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VoiceLeadingStyle {
    /// Common practice voice leading with minimal movement
    CommonPractice,
    /// Jazz voice leading with smooth voice movement
    Jazz,
    /// Minimal voice leading with absolute minimal movement
    Minimal,
    /// Free voice leading with no constraints
    Free,
}

impl Default for VoiceLeadingStyle {
    fn default() -> Self {
        Self::Jazz
    }
}

/// Voice leading optimization for smooth chord progressions
pub struct VoiceLeader {
    config: VoicingConfig,
    voice_leading_style: VoiceLeadingStyle,
}

impl VoiceLeader {
    /// Create a new voice leader with the given configuration
    pub fn new(config: VoicingConfig) -> Self {
        Self {
            config,
            voice_leading_style: VoiceLeadingStyle::default(),
        }
    }

    /// Create a voice leader with default configuration
    pub fn default() -> Self {
        Self::new(VoicingConfig::default())
    }

    /// Set the voice leading style
    pub fn style(mut self, style: VoiceLeadingStyle) -> Self {
        self.voice_leading_style = style;
        self
    }

    /// Voice a chord progression with optimal voice leading
    pub fn voice_progression(
        &self,
        chords: &[Chord],
    ) -> Result<Vec<VoicedChord>, VoicingError> {
        if chords.is_empty() {
            return Ok(vec![]);
        }

        let mut voiced_chords = Vec::new();

        // Voice the first chord
        let first_voiced = Voicer::new(self.config.clone()).voice_chord(&chords[0])?;
        voiced_chords.push(first_voiced);

        // Voice subsequent chords with voice leading optimization
        for i in 1..chords.len() {
            let previous_voiced = &voiced_chords[i - 1];
            let current_chord = &chords[i];

            let current_voiced = self.voice_with_leading(previous_voiced, current_chord)?;
            voiced_chords.push(current_voiced);
        }

        Ok(voiced_chords)
    }

    /// Voice a single chord with voice leading from a previous chord
    pub fn voice_with_leading(
        &self,
        previous: &VoicedChord,
        chord: &Chord,
    ) -> Result<VoicedChord, VoicingError> {
        match self.voice_leading_style {
            VoiceLeadingStyle::CommonPractice => self.voice_common_practice(previous, chord),
            VoiceLeadingStyle::Jazz => self.voice_jazz(previous, chord),
            VoiceLeadingStyle::Minimal => self.voice_minimal(previous, chord),
            VoiceLeadingStyle::Free => {
                let mut candidate = Voicer::new(self.config.clone()).voice_chord(chord)?;

                // Calculate movement even for free style
                let mut prev_voices = previous.pitches.clone();
                let mut curr_voices = candidate.pitches.clone();

                prev_voices.sort_by_key(|p| p.midi_number());
                curr_voices.sort_by_key(|p| p.midi_number());

                let movement = self.calculate_movement(&prev_voices, &curr_voices);
                candidate.info = candidate.info.with_movement(movement);

                Ok(candidate)
            }
        }
    }

    /// Common practice voice leading with traditional rules
    fn voice_common_practice(
        &self,
        previous: &VoicedChord,
        chord: &Chord,
    ) -> Result<VoicedChord, VoicingError> {
        // Start with basic voicing
        let mut candidate = Voicer::new(self.config.clone()).voice_chord(chord)?;

        // Sort voices for comparison
        let mut prev_voices = previous.pitches.clone();
        let mut curr_voices = candidate.pitches.clone();

        prev_voices.sort_by_key(|p| p.midi_number());
        curr_voices.sort_by_key(|p| p.midi_number());

        // Ensure we have the same number of voices
        if prev_voices.len() != curr_voices.len() {
            // Pad with appropriate notes
            curr_voices = self.pad_voices(&curr_voices, prev_voices.len());
        }

        // Apply voice leading rules
        let optimized_voices = self.optimize_common_practice(&prev_voices, &curr_voices);

        // Update the voiced chord
        candidate.pitches = optimized_voices;

        // Calculate voice movement
        let movement = self.calculate_movement(&prev_voices, &candidate.pitches);
        candidate.info = candidate.info.with_movement(movement);

        Ok(candidate)
    }

    /// Jazz voice leading with smooth voice movement
    fn voice_jazz(
        &self,
        previous: &VoicedChord,
        chord: &Chord,
    ) -> Result<VoicedChord, VoicingError> {
        // Start with drop-2 voicing for jazz
        let drop2_config = self.config.clone().style(VoicingStyle::Drop2);
        let mut candidate = Voicer::new(drop2_config).voice_chord(chord)?;

        // Sort voices
        let mut prev_voices = previous.pitches.clone();
        let mut curr_voices = candidate.pitches.clone();

        prev_voices.sort_by_key(|p| p.midi_number());
        curr_voices.sort_by_key(|p| p.midi_number());

        // Ensure we have the same number of voices
        if prev_voices.len() != curr_voices.len() {
            curr_voices = self.pad_voices(&curr_voices, prev_voices.len());
        }

        // Apply jazz voice leading rules
        let optimized_voices = self.optimize_jazz(&prev_voices, &curr_voices);

        candidate.pitches = optimized_voices;
        let movement = self.calculate_movement(&prev_voices, &candidate.pitches);
        candidate.info = candidate.info.with_movement(movement);

        Ok(candidate)
    }

    /// Minimal voice leading with absolute minimal movement
    fn voice_minimal(
        &self,
        previous: &VoicedChord,
        chord: &Chord,
    ) -> Result<VoicedChord, VoicingError> {
        let mut candidate = Voicer::new(self.config.clone()).voice_chord(chord)?;

        let mut prev_voices = previous.pitches.clone();
        let mut curr_voices = candidate.pitches.clone();

        prev_voices.sort_by_key(|p| p.midi_number());
        curr_voices.sort_by_key(|p| p.midi_number());

        if prev_voices.len() != curr_voices.len() {
            curr_voices = self.pad_voices(&curr_voices, prev_voices.len());
        }

        let optimized_voices = self.optimize_minimal(&prev_voices, &curr_voices);
        candidate.pitches = optimized_voices;
        let movement = self.calculate_movement(&prev_voices, &candidate.pitches);
        candidate.info = candidate.info.with_movement(movement);

        Ok(candidate)
    }

    /// Pad voices to match target count
    fn pad_voices(&self, voices: &[Pitch], target_count: usize) -> Vec<Pitch> {
        let mut padded = voices.to_vec();

        while padded.len() < target_count {
            // Add octave duplicates of existing voices
            if let Some(&last_voice) = padded.last() {
                let new_voice = last_voice + 12i8;
                if new_voice.midi_number() <= self.config.range.high.midi_number() {
                    padded.push(new_voice);
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        padded
    }

    /// Optimize voices for common practice voice leading
    fn optimize_common_practice(&self, prev_voices: &[Pitch], curr_voices: &[Pitch]) -> Vec<Pitch> {
        let mut optimized = curr_voices.to_vec();

        // Common practice rules:
        // - Avoid parallel fifths/octaves
        // - Resolve leading tones
        // - Smooth voice movement

        for i in 0..optimized.len() {
            if i < prev_voices.len() {
                let prev_midi = prev_voices[i].midi_number();
                let curr_midi = optimized[i].midi_number();
                let movement = (curr_midi - prev_midi).abs();

                // Prefer stepwise motion
                if movement > 4 {
                    // Try to find a closer voicing
                    if let Some(closer) = self.find_closer_voicing(prev_midi, &optimized) {
                        optimized[i] = closer;
                    }
                }
            }
        }

        optimized
    }

    /// Optimize voices for jazz voice leading
    fn optimize_jazz(&self, prev_voices: &[Pitch], curr_voices: &[Pitch]) -> Vec<Pitch> {
        let mut optimized = curr_voices.to_vec();

        // Jazz voice leading rules:
        // - Smooth voice movement (prefer stepwise)
        // - Maintain voice independence
        // - Avoid awkward leaps

        for i in 0..optimized.len() {
            if i < prev_voices.len() {
                let prev_midi = prev_voices[i].midi_number();
                let curr_midi = optimized[i].midi_number();
                let movement = (curr_midi - prev_midi).abs();

                // In jazz, small leaps are acceptable but prefer stepwise
                if movement > 7 {
                    // Try to find a closer voicing
                    if let Some(closer) = self.find_closer_voicing(prev_midi, &optimized) {
                        optimized[i] = closer;
                    }
                }
            }
        }

        optimized
    }

    /// Optimize voices for minimal voice leading
    fn optimize_minimal(&self, prev_voices: &[Pitch], curr_voices: &[Pitch]) -> Vec<Pitch> {
        let mut optimized = curr_voices.to_vec();

        // Minimal voice leading: absolute minimal movement
        for i in 0..optimized.len() {
            if i < prev_voices.len() {
                let prev_midi = prev_voices[i].midi_number();
                let curr_midi = optimized[i].midi_number();

                // Try to minimize movement
                if (curr_midi - prev_midi).abs() > 2 {
                    if let Some(closer) = self.find_closest_voicing(prev_midi, &optimized) {
                        optimized[i] = closer;
                    }
                }
            }
        }

        optimized
    }

    /// Find a closer voicing for a given voice
    fn find_closer_voicing(&self, target_midi: i8, voices: &[Pitch]) -> Option<Pitch> {
        // Look for a voice that's closer to the target
        voices
            .iter()
            .min_by_key(|&p| (p.midi_number() as i16 - target_midi as i16).abs())
            .copied()
    }

    /// Find the closest voicing for a given voice
    fn find_closest_voicing(&self, target_midi: i8, voices: &[Pitch]) -> Option<Pitch> {
        // Look for the closest voice
        voices
            .iter()
            .min_by_key(|&p| (p.midi_number() as i16 - target_midi as i16).abs())
            .copied()
    }

    /// Calculate total voice movement between two chord voicings
    fn calculate_movement(&self, prev_voices: &[Pitch], curr_voices: &[Pitch]) -> i32 {
        let mut total_movement = 0;

        for i in 0..prev_voices.len().min(curr_voices.len()) {
            let movement = (curr_voices[i].midi_number() as i32 - prev_voices[i].midi_number() as i32).abs();
            total_movement += movement;
        }

        total_movement
    }
}

impl Default for VoiceLeader {
    fn default() -> Self {
        Self::new(VoicingConfig::default())
    }
}