use crate::error::TypeError;

use super::{
    chord::HarmonicFunction, key::KeySignature, Accidental, Chord, ChordQuality, Interval, NoteName,
};

/// A scale with a tonic and mode
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scale {
    /// The tonic (starting note) of the scale
    pub tonic: NoteName,

    /// The type of scale (defines the interval pattern)
    pub scale_type: ScaleType,

    /// Optional key signature for theoretical consistency
    /// If None, will be inferred from scale_type and tonic
    pub key_signature: Option<KeySignature>,
}

/// A scale is a sequence of notes that defines a musical key.
/// ```
/// use chordy::{NoteName, Letter, Accidental, Scale, ScaleType};
///
/// // Create a C major scale
/// let c = NoteName::new(Letter::C, Accidental::Natural);
/// let c_major = Scale::new(c, ScaleType::Major);
///
/// // Get the notes in the scale
/// let notes = c_major.notes();
/// // [C, D, E, F, G, A, B]
/// assert_eq!(notes, vec![
///     NoteName::new(Letter::C, Accidental::Natural),
///     NoteName::new(Letter::D, Accidental::Natural),
///     NoteName::new(Letter::E, Accidental::Natural),
///     NoteName::new(Letter::F, Accidental::Natural),
///     NoteName::new(Letter::G, Accidental::Natural),
///     NoteName::new(Letter::A, Accidental::Natural),
///     NoteName::new(Letter::B, Accidental::Natural),
/// ]);
/// ```
impl Scale {
    // Core constructor
    pub fn new(tonic: NoteName, scale_type: ScaleType) -> Self {
        Scale {
            tonic,
            scale_type,
            key_signature: None,
        }
    }

    // With explicit key signature
    pub fn with_key_signature(
        tonic: NoteName,
        scale_type: ScaleType,
        key_signature: KeySignature,
    ) -> Self {
        Scale {
            tonic,
            scale_type,
            key_signature: Some(key_signature),
        }
    }

    pub fn notes(&self) -> Vec<NoteName> {
        // First determine proper key signature if not provided
        let key_sig = self
            .key_signature
            .unwrap_or_else(|| self.infer_key_signature());

        // Generate notes based on scale type intervals
        let intervals = self.scale_type.intervals();
        let mut result = Vec::with_capacity(intervals.len() + 1);

        // Add tonic
        result.push(self.tonic);

        // Add remaining notes with proper spelling based on key signature
        for &interval in intervals {
            let note = self.tonic.transpose_by_interval(interval, &key_sig);
            result.push(note);
        }

        result
    }

    /// Infers the most appropriate key signature for this scale
    fn infer_key_signature(&self) -> KeySignature {
        // Implement key signature inference logic based on scale type and tonic
        // For example, C Major uses no accidentals, while F Major uses one flat
        KeySignature {
            accidentals: 0, // Placeholder
            letter_map: [
                Accidental::Natural,
                Accidental::Natural,
                Accidental::Natural,
                Accidental::Natural,
                Accidental::Natural,
                Accidental::Natural,
                Accidental::Natural,
            ],
        }
    }
    /// Returns the scale degree (1-7) for a given note
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Scale, ScaleType, note};
    ///
    /// let c_major = Scale::new(note!("C"), ScaleType::Major);
    /// assert_eq!(c_major.degree_of(&note!("C")), Some(1));
    /// assert_eq!(c_major.degree_of(&note!("G")), Some(5));
    /// assert_eq!(c_major.degree_of(&note!("F#")), None); // Not in scale
    ///
    /// // Works with enharmonic equivalents
    /// let a_minor = Scale::new(note!("A"), ScaleType::NaturalMinor);
    /// assert_eq!(a_minor.degree_of(&note!("G#")), Some(7)); // Leading tone
    /// assert_eq!(a_minor.degree_of(&note!("Ab")), None); // Theoretically not in the scale.
    /// ```
    pub fn degree_of(&self, note: &NoteName) -> Option<u8> {
        let notes = self.notes();

        // First check exact matches
        if let Some(pos) = notes.iter().position(|n| n == note) {
            return Some((pos + 1) as u8);
        }

        // Then check enharmonic equivalents
        for (i, scale_note) in notes.iter().enumerate() {
            if scale_note.is_enharmonic_with(note) {
                return Some((i + 1) as u8);
            }
        }

        None
    }

    /// Returns the chromatic degree (accounting for alterations)
    ///
    /// Returns a tuple of (degree, alteration) where:
    /// - degree is the base scale degree (1-7)
    /// - alteration is the semitone adjustment (-2 to +2)
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Scale, ScaleType, note};
    ///
    /// let c_major = Scale::new(note!("C"), ScaleType::Major);
    /// assert_eq!(c_major.chromatic_degree(&note!("C")), Some((1, 0)));
    /// assert_eq!(c_major.chromatic_degree(&note!("C#")), Some((1, 1)));
    /// assert_eq!(c_major.chromatic_degree(&note!("F#")), Some((4, 1))); // #11
    /// ```
    pub fn chromatic_degree(&self, note: &NoteName) -> Option<(u8, i8)> {
        // First try exact degree match
        if let Some(degree) = self.degree_of(note) {
            return Some((degree, 0));
        }

        // Check chromatic variants
        for degree in 1..=7 {
            if let Some(scale_note) = self.notes().get(degree as usize - 1) {
                let semitone_diff = note.base_midi_number() - scale_note.base_midi_number();
                let octave_adjusted_diff = (semitone_diff + 12) % 12;

                // Check common chromatic alterations
                match octave_adjusted_diff {
                    1 => return Some((degree, 1)),   // Sharp
                    11 => return Some((degree, -1)), // Flat
                    2 => return Some((degree, 2)),   // Double sharp
                    10 => return Some((degree, -2)), // Double flat
                    _ => continue,
                }
            }
        }

        None
    }

    /// Returns the harmonic function of a given chord within this scale
    ///
    /// For this, we use a method devised by Ian Quinn (Yale University), which distinguishes three
    /// harmonic functions, and categorises chords based on the internal scale degrees.
    ///
    /// See: https://openmusictheory.github.io/harmonicFunctions.html
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Chord, Scale};
    ///
    /// let cmaj_scale = Scale::new(note!("C"), ScaleType::Major);
    /// let g_chord = Chord::new(note!("G"), ChordQuality::Major, vec![]);
    /// assert_eq!(c_major.harmonic_function(g_chord), HarmonicFunction::Dominant);
    ///
    /// ```
    pub fn harmonic_function(&self, chord: &Chord) -> Option<HarmonicFunction> {
        let notes = chord.notes();

        let scale_degrees: Vec<u8> = notes
            .iter()
            .filter_map(|note| self.degree_of(note))
            .collect();

        println!("Scale degrees for chord {:?}: {:?}", chord, scale_degrees);

        HarmonicFunction::detect_by_scale_degrees(&scale_degrees)
    }

    /// Creates a chord from the given scale degree (1-7)
    pub fn chord_at_degree(&self, degree: u8, chord_type: ChordQuality) -> Chord {
        // Implementation
        todo!()
    }
    /// Returns the relative major/minor of this scale
    pub fn relative(&self) -> Scale {
        // Implementation
        todo!()
    }

    /// Returns the parallel major/minor of this scale
    pub fn parallel(&self) -> Scale {
        // Implementation
        todo!()
    }

    /// Returns a scale a perfect fifth higher (dominant)
    pub fn dominant(&self) -> Scale {
        // Implementation
        todo!()
    }

    /// Returns a scale a perfect fifth lower (subdominant)
    pub fn subdominant(&self) -> Scale {
        // Implementation
        todo!()
    }
    /// Determine if a given note belongs to the scale
    pub fn contains(&self, note: &NoteName) -> bool {
        // Implementation
        todo!()
    }

    /// Find the closest scale tone to a given note
    pub fn closest_tone_to(&self, note: &NoteName) -> NoteName {
        // Implementation
        todo!()
    }

    /// Calculate the tension/stability of a note in this scale context
    /*
    pub fn tension(&self, note: &NoteName) -> TensionRating {
        // Implementation
    }
    */

    /// Returns all possible chords that can be built within this scale
    pub fn possible_chords(&self) -> Vec<Chord> {
        // Implementation
        todo!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScaleDegree {
    pub step: u8,
    pub alteration: Accidental,
}

impl ScaleDegree {
    /// Create a new scale degree with the given step and alteration
    pub const fn new(step: u8, alteration: Accidental) -> Self {
        // Note: const fn can't use assert! before Rust 1.57
        // Use runtime validation in a separate function if needed
        ScaleDegree { step, alteration }
    }
    
    // Add runtime validation if needed
    pub fn validate(&self) -> Result<(), TypeError> {
        if self.step < 1 || self.step > 7 {
            return Err(TypeError::InvalidScaleDegree(self.step));
        }
        Ok(())
    }

    // Constants for common scale degrees
    pub const TONIC: Self = Self::new(1, Accidental::Natural);
    pub const SUPERTONIC: Self = Self::new(2, Accidental::Natural);
    pub const MEDIANT: Self = Self::new(3, Accidental::Natural);
    pub const SUBDOMINANT: Self = Self::new(4, Accidental::Natural);
    pub const DOMINANT: Self = Self::new(5, Accidental::Natural);
    pub const SUBMEDIANT: Self = Self::new(6, Accidental::Natural);
    pub const LEADING_TONE: Self = Self::new(7, Accidental::Natural);
    
    // Altered scale degrees
    pub const FLAT_SECOND: Self = Self::new(2, Accidental::Flat);
    pub const FLAT_THIRD: Self = Self::new(3, Accidental::Flat);
    pub const SHARP_FOURTH: Self = Self::new(4, Accidental::Sharp);
    pub const FLAT_SIXTH: Self = Self::new(6, Accidental::Flat);
    pub const FLAT_SEVENTH: Self = Self::new(7, Accidental::Flat);
    
    // Special scale degrees with traditional names
    pub const NEAPOLITAN: Self = Self::new(2, Accidental::Flat);  // ♭II
    pub const SUBTONIC: Self = Self::new(7, Accidental::Flat);    // ♭VII
}


/// Types of musical scales
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScaleType {
    Major,
    NaturalMinor,
    HarmonicMinor,
    MelodicMinor,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Locrian,
    // etc.
}

impl ScaleType {
    /// Returns the interval pattern for this scale type
    pub fn intervals(&self) -> &[Interval] {
        match self {
            ScaleType::Major => &[
                Interval::MAJOR_SECOND,
                Interval::MAJOR_THIRD,
                Interval::PERFECT_FOURTH,
                Interval::PERFECT_FIFTH,
                Interval::MAJOR_SIXTH,
                Interval::MAJOR_SEVENTH,
            ],
            ScaleType::NaturalMinor => &[
                Interval::MAJOR_SECOND,
                Interval::MINOR_THIRD,
                Interval::PERFECT_FOURTH,
                Interval::PERFECT_FIFTH,
                Interval::MINOR_SIXTH,
                Interval::MINOR_SEVENTH,
            ],
            ScaleType::Dorian => &[
                Interval::MAJOR_SECOND,
                Interval::MINOR_THIRD,
                Interval::PERFECT_FOURTH,
                Interval::PERFECT_FIFTH,
                Interval::MAJOR_SIXTH,
                Interval::MINOR_SEVENTH,
            ],
            ScaleType::Phrygian => &[
                Interval::MINOR_SECOND,
                Interval::MINOR_THIRD,
                Interval::PERFECT_FOURTH,
                Interval::PERFECT_FIFTH,
                Interval::MINOR_SIXTH,
                Interval::MINOR_SEVENTH,
            ],
            ScaleType::Lydian => &[
                Interval::MAJOR_SECOND,
                Interval::MAJOR_THIRD,
                Interval::TRITONE,
                Interval::PERFECT_FIFTH,
                Interval::MAJOR_SIXTH,
                Interval::MAJOR_SEVENTH,
            ],
            ScaleType::Mixolydian => &[
                Interval::MAJOR_SECOND,
                Interval::MAJOR_THIRD,
                Interval::PERFECT_FOURTH,
                Interval::PERFECT_FIFTH,
                Interval::MAJOR_SIXTH,
                Interval::MINOR_SEVENTH,
            ],
            ScaleType::Locrian => &[
                Interval::MINOR_SECOND,
                Interval::MINOR_THIRD,
                Interval::PERFECT_FOURTH,
                Interval::DIMINISHED_FIFTH,
                Interval::MINOR_SIXTH,
                Interval::MINOR_SEVENTH,
            ],
            ScaleType::HarmonicMinor => &[
                Interval::MAJOR_SECOND,
                Interval::MINOR_THIRD,
                Interval::PERFECT_FOURTH,
                Interval::PERFECT_FIFTH,
                Interval::MINOR_SIXTH,
                Interval::MAJOR_SEVENTH,
            ],
            // Other scale types would be defined similarly
            _ => todo!(),
        }
    }
}
