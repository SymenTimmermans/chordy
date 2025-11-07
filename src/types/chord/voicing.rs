//! Voicing system integration for chords
//!
//! This module contains methods for voicing chords on different instruments:
//! - Generic voicing methods
//! - Piano-specific voicing
//! - Guitar-specific voicing
//! - Vocal voicing

use crate::types::{
    voicing::{
        PianoVoicingConfig, PianoVoicingType, VoicedChord, Voicer, VoicingConfig, VoicingError,
        VoicingStyle,
    },
    Chord, Interval, Pitch,
};

impl Chord {
    // Voicing Methods
    //
    // These convenience methods provide easy access to the voicing system
    // without requiring users to create Voicer instances manually.

    /// Voice this chord using a custom voicing configuration
    ///
    /// This is the most flexible voicing method, allowing full control over
    /// all voicing parameters including style, range, and bass constraints.
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Chord, VoicingConfig, VoicingStyle, PitchRange, note};
    ///
    /// let chord = Chord::major(note!("C"));
    /// let config = VoicingConfig::new()
    ///     .style(VoicingStyle::Open)
    ///     .range_from("C3".parse().unwrap(), "C6".parse().unwrap());
    ///
    /// let voiced = chord.voice(&config).unwrap();
    /// assert_eq!(voiced.chord, chord);
    /// assert_eq!(voiced.info.style, VoicingStyle::Open);
    /// ```
    pub fn voice(
        &self,
        config: &VoicingConfig,
    ) -> Result<VoicedChord, VoicingError> {
        let voicer = Voicer::new(config.clone());
        voicer.voice_chord(self)
    }

    /// Voice this chord in closed position
    ///
    /// Creates a closed voicing where all notes are packed as closely as possible,
    /// typically within an octave. This is the most compact voicing style.
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Chord, note};
    ///
    /// let chord = Chord::major(note!("C"));
    /// let voiced = chord.voice_closed("C4".parse().unwrap()).unwrap();
    ///
    /// assert!(voiced.is_closed());
    /// assert_eq!(voiced.bass_pitch().unwrap().name, note!("C"));
    /// ```
    pub fn voice_closed(
        &self,
        bass_pitch: Pitch,
    ) -> Result<VoicedChord, VoicingError> {
        let config = VoicingConfig::new()
            .style(VoicingStyle::Closed)
            .bass_pitch(bass_pitch);
        self.voice(&config)
    }

    /// Voice this chord in open position
    ///
    /// Creates an open voicing where notes are spread across multiple octaves,
    /// providing a fuller, more spacious sound than closed voicings.
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Chord, note};
    ///
    /// let chord = Chord::major(note!("C"));
    /// let voiced = chord.voice_open("C3".parse().unwrap()).unwrap();
    ///
    /// // Open voicings typically span more than an octave
    /// assert!(voiced.span_semitones() > 12 || voiced.pitches.len() <= 2);
    /// ```
    pub fn voice_open(
        &self,
        bass_pitch: Pitch,
    ) -> Result<VoicedChord, VoicingError> {
        let config = VoicingConfig::new()
            .style(VoicingStyle::Open)
            .bass_pitch(bass_pitch);
        self.voice(&config)
    }

    /// Voice this chord using drop-2 voicing
    ///
    /// Drop-2 voicing takes the second-highest note from a closed voicing
    /// and drops it down an octave. This is a common jazz piano technique
    /// that creates smoother voice leading.
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Chord, note};
    ///
    /// let chord = Chord::major_7th(note!("C")); // Works best with 4+ notes
    /// let voiced = chord.voice_drop2("C4".parse().unwrap()).unwrap();
    ///
    /// assert_eq!(voiced.info.style, chordy::VoicingStyle::Drop2);
    /// ```
    pub fn voice_drop2(
        &self,
        bass_pitch: Pitch,
    ) -> Result<VoicedChord, VoicingError> {
        let config = VoicingConfig::new()
            .style(VoicingStyle::Drop2)
            .bass_pitch(bass_pitch);
        self.voice(&config)
    }

    /// Voice this chord using drop-3 voicing
    ///
    /// Drop-3 voicing takes the third-highest note from a closed voicing
    /// and drops it down an octave. This creates distinctive harmonic spacing
    /// common in jazz arrangements.
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Chord, note};
    ///
    /// let chord = Chord::dominant_7th(note!("G")); // Works best with 4+ notes
    /// let voiced = chord.voice_drop3("G3".parse().unwrap()).unwrap();
    ///
    /// assert_eq!(voiced.info.style, chordy::VoicingStyle::Drop3);
    /// ```
    pub fn voice_drop3(
        &self,
        bass_pitch: Pitch,
    ) -> Result<VoicedChord, VoicingError> {
        let config = VoicingConfig::new()
            .style(VoicingStyle::Drop3)
            .bass_pitch(bass_pitch);
        self.voice(&config)
    }

    /// Voice this chord with custom spread constraints
    ///
    /// Creates a spread voicing where adjacent notes maintain intervals
    /// within the specified minimum and maximum range. This provides
    /// fine control over the harmonic density.
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Chord, Interval, note};
    ///
    /// let chord = Chord::major(note!("C"));
    /// let voiced = chord.voice_spread(
    ///     "C4".parse().unwrap(),
    ///     Interval::MAJOR_THIRD,  // Minimum spacing
    ///     Interval::PERFECT_FIFTH // Maximum spacing
    /// ).unwrap();
    ///
    /// // All adjacent intervals should be between major 3rd and perfect 5th
    /// let intervals = voiced.voice_intervals();
    /// for &interval in &intervals {
    ///     assert!(interval >= 4 && interval <= 7);
    /// }
    /// ```
    pub fn voice_spread(
        &self,
        bass_pitch: Pitch,
        min_interval: Interval,
        max_interval: Interval,
    ) -> Result<VoicedChord, VoicingError> {
        let config = VoicingConfig::new()
            .style(VoicingStyle::spread(min_interval, max_interval))
            .bass_pitch(bass_pitch);
        self.voice(&config)
    }

    /// Voice this chord for piano with default settings
    ///
    /// Uses piano-friendly voicing settings with the standard 88-key range
    /// and closed voicing style, optimized for typical piano playing.
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Chord, note};
    ///
    /// let chord = Chord::major(note!("C"));
    /// let voiced = chord.voice_for_piano().unwrap();
    ///
    /// assert_eq!(voiced.info.range, chordy::PitchRange::piano());
    /// ```
    pub fn voice_for_piano(&self) -> Result<VoicedChord, VoicingError> {
        let config = VoicingConfig::piano();
        self.voice(&config)
    }

    /// Voice this chord specifically for piano with ergonomic considerations
    ///
    /// Uses piano-specific voicing algorithms that consider hand span,
    /// finger reach, and other human physical limitations.
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Chord, note};
    ///
    /// let chord = Chord::major(note!("C"));
    /// let voiced = chord.voice_piano_block().unwrap();
    ///
    /// assert!(voiced.is_piano_voicing());
    /// ```
    pub fn voice_piano_block(&self) -> Result<VoicedChord, VoicingError> {
        let config = VoicingConfig::piano();
        let piano_config = PianoVoicingConfig::classical();
        Voicer::new(config).voice_piano(self, &piano_config)
    }

    /// Voice this chord for piano with spread voicing
    ///
    /// Creates a spread voicing where notes are distributed across
    /// multiple octaves for a more open, resonant sound.
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Chord, note};
    ///
    /// let chord = Chord::major(note!("C"));
    /// let voiced = chord.voice_piano_spread().unwrap();
    ///
    /// assert!(voiced.is_piano_voicing());
    /// assert_eq!(voiced.piano_voicing_type(), Some(&chordy::PianoVoicingType::Spread));
    /// ```
    pub fn voice_piano_spread(&self) -> Result<VoicedChord, VoicingError> {
        let config = VoicingConfig::piano();
        let mut piano_config = PianoVoicingConfig::classical();
        piano_config.voicing_type = PianoVoicingType::Spread;
        Voicer::new(config).voice_piano(self, &piano_config)
    }

    /// Voice this chord for piano with jazz shell voicing
    ///
    /// Creates a shell voicing that emphasizes the root, 3rd, and 7th,
    /// commonly used in jazz piano for clear harmonic definition.
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Chord, note};
    ///
    /// let chord = Chord::dominant_7th(note!("C"));
    /// let voiced = chord.voice_piano_jazz().unwrap();
    ///
    /// assert!(voiced.is_piano_voicing());
    /// assert_eq!(voiced.piano_voicing_type(), Some(&chordy::PianoVoicingType::Shell));
    /// ```
    pub fn voice_piano_jazz(&self) -> Result<VoicedChord, VoicingError> {
        let config = VoicingConfig::piano();
        let piano_config = PianoVoicingConfig::jazz();
        Voicer::new(config).voice_piano(self, &piano_config)
    }

    /// Voice this chord for piano with rootless jazz voicing
    ///
    /// Creates a rootless voicing that omits the root note, commonly
    /// used in jazz piano when playing with a bassist.
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Chord, note};
    ///
    /// let chord = Chord::dominant_7th(note!("C"));
    /// let voiced = chord.voice_piano_rootless().unwrap();
    ///
    /// assert!(voiced.is_piano_voicing());
    /// assert_eq!(voiced.piano_voicing_type(), Some(&chordy::PianoVoicingType::Rootless));
    /// ```
    pub fn voice_piano_rootless(&self) -> Result<VoicedChord, VoicingError> {
        let config = VoicingConfig::piano();
        let mut piano_config = PianoVoicingConfig::jazz();
        piano_config.voicing_type = PianoVoicingType::Rootless;
        Voicer::new(config).voice_piano(self, &piano_config)
    }

    /// Voice this chord for piano with broken chord pattern
    ///
    /// Creates a broken chord voicing suitable for arpeggiated or
    /// rolled chord patterns.
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Chord, note};
    ///
    /// let chord = Chord::major(note!("C"));
    /// let voiced = chord.voice_piano_broken().unwrap();
    ///
    /// assert!(voiced.is_piano_voicing());
    /// assert_eq!(voiced.piano_voicing_type(), Some(&chordy::PianoVoicingType::Broken));
    /// ```
    pub fn voice_piano_broken(&self) -> Result<VoicedChord, VoicingError> {
        let config = VoicingConfig::piano();
        let piano_config = PianoVoicingConfig::broken();
        Voicer::new(config).voice_piano(self, &piano_config)
    }

    /// Voice this chord for piano with custom configuration
    ///
    /// Provides full control over piano voicing parameters including
    /// hand position, voicing type, articulation, and hand span constraints.
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Chord, note};
    ///
    /// let chord = Chord::major(note!("C"));
    /// let piano_config = chordy::PianoVoicingConfig::left_hand();
    /// let voiced = chord.voice_piano_with_config(&piano_config).unwrap();
    ///
    /// assert!(voiced.is_piano_voicing());
    /// assert_eq!(voiced.piano_hand_position(), Some(&chordy::PianoHandPosition::LeftHand));
    /// ```
    pub fn voice_piano_with_config(
        &self,
        piano_config: &PianoVoicingConfig,
    ) -> Result<VoicedChord, VoicingError> {
        let config = VoicingConfig::piano();
        Voicer::new(config).voice_piano(self, piano_config)
    }

    /// Voice this chord for guitar with default settings
    ///
    /// Uses guitar-friendly voicing settings with the standard guitar range
    /// and closed voicing style, optimized for typical guitar playing.
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Chord, note};
    ///
    /// let chord = Chord::major(note!("C"));
    /// let voiced = chord.voice_for_guitar().unwrap();
    ///
    /// assert_eq!(voiced.info.range, chordy::PitchRange::guitar());
    /// ```
    pub fn voice_for_guitar(&self) -> Result<VoicedChord, VoicingError> {
        let config = VoicingConfig::guitar();
        self.voice(&config)
    }

    /// Voice this chord for vocals with default settings
    ///
    /// Uses vocal-friendly voicing settings with a comfortable singing range
    /// and closed voicing style, suitable for choir or vocal arrangements.
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Chord, note};
    ///
    /// let chord = Chord::major(note!("C"));
    /// let voiced = chord.voice_for_vocals().unwrap();
    ///
    /// assert_eq!(voiced.info.range, chordy::PitchRange::vocal());
    /// ```
    pub fn voice_for_vocals(&self) -> Result<VoicedChord, VoicingError> {
        let config = VoicingConfig::vocal();
        self.voice(&config)
    }
}