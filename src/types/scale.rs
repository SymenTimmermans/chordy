use crate::error::TypeError;

use super::{
    chord::HarmonicFunction, key::KeySignature, Accidental, Chord, ChordQuality, NoteName
};

pub mod definition;
pub use definition::ScaleDefinition;

pub mod bitmask;
pub use bitmask::ScaleBitmask;

#[allow(dead_code)]
pub mod scales;

/// A scale with a tonic and mode
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scale {
    /// The tonic (starting note) of the scale
    pub tonic: NoteName,

    /// The scale definition
    pub definition: ScaleDefinition,
}

/// A scale is a sequence of notes that defines a musical key.
/// ```
/// use chordy::{NoteName, Letter, Accidental, Scale, scales};
///
/// // Create a C major scale
/// let c = NoteName::new(Letter::C, Accidental::Natural);
/// let c_major = Scale::new(c, scales::IONIAN);
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
    pub fn new(tonic: NoteName, definition: ScaleDefinition) -> Self {
        Scale {
            tonic,
            definition
        }
    }

    pub fn notes(&self) -> Vec<NoteName> {
        // Generate notes based on scale type intervals
        let mut result = Vec::with_capacity(self.definition.intervals.len());

        // Add remaining notes with proper spelling based on key signature
        for &interval in self.definition.intervals {
            let note = self.tonic + interval;
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
    /// Returns the scale degree for a given note, accounting for alterations
    ///
    /// Returns a ScaleDegree struct containing:
    /// - step: the base scale degree (1-7)
    /// - accidental: the alteration from the scale degree (Natural if exact match)
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Scale, scales, note, Accidental, ScaleDegree, HarmonicFunction};
    ///
    /// let c_major = Scale::new(note!("C"), scales::IONIAN);
    /// // Exact matches return natural accidentals
    /// assert_eq!(c_major.degree_of(&note!("C")), Some(ScaleDegree::TONIC));
    /// assert_eq!(c_major.degree_of(&note!("G")), Some(ScaleDegree::DOMINANT));
    ///
    /// // Altered notes return appropriate accidentals
    /// assert_eq!(c_major.degree_of(&note!("C#")), Some(ScaleDegree::new(1, Some(Accidental::Sharp))));
    /// assert_eq!(c_major.degree_of(&note!("F#")), Some(ScaleDegree::new(4, Some(Accidental::Sharp))));
    ///
    /// // Works with enharmonic equivalents
    /// let a_minor = Scale::new(note!("A"), scales::AEOLIAN);
    /// assert_eq!(a_minor.degree_of(&note!("G#")), Some(ScaleDegree::new(7, Some(Accidental::Sharp)))); // Leading tone
    /// assert_eq!(a_minor.degree_of(&note!("Ab")), Some(ScaleDegree::new(1, Some(Accidental::Flat))));
    /// ```
    pub fn degree_of(&self, note: &NoteName) -> Option<ScaleDegree> {
        let notes = self.notes();

        // First check exact matches
        if let Some(pos) = notes.iter().position(|n| n == note) {
            return Some(ScaleDegree::new((pos + 1) as u8, None));
        }

        // Then check enharmonic equivalents
        for (i, scale_note) in notes.iter().enumerate() {
            if scale_note.is_enharmonic_with(note) {
                return Some(ScaleDegree::new((i + 1) as u8, None));
            }
        }

        // Then check all notes for single accidentals, preferring matching accidental type
        for (i, scale_note) in notes.iter().enumerate() {
            let semitone_diff = note.base_midi_number() - scale_note.base_midi_number();
            let octave_adjusted_diff = (semitone_diff + 12) % 12;

            // Prefer matching the note's accidental type if possible
            if note.accidental() == Accidental::Flat && octave_adjusted_diff == 11 {
                return Some(ScaleDegree::new((i + 1) as u8, Some(Accidental::Flat)));
            }
            if note.accidental() == Accidental::Sharp && octave_adjusted_diff == 1 {
                return Some(ScaleDegree::new((i + 1) as u8, Some(Accidental::Sharp)));
            }
        }

        // if we can't match on accidental, try to match on letter name
        for (i, scale_note) in notes.iter().enumerate() {
            if scale_note.letter() == note.letter() {
                let semitone_diff = note.base_midi_number() - scale_note.base_midi_number();
                let octave_adjusted_diff = (semitone_diff + 12) % 12;

                // Prefer matching the note's accidental type if possible
                if octave_adjusted_diff == 11 {
                    return Some(ScaleDegree::new((i + 1) as u8, Some(Accidental::Flat)));
                }
                if octave_adjusted_diff == 1 {
                    return Some(ScaleDegree::new((i + 1) as u8, Some(Accidental::Sharp)));
                }
            }
        }

        // Fall back to any single accidental if no preferred match found
        for (i, scale_note) in notes.iter().enumerate() {
            let semitone_diff = note.base_midi_number() - scale_note.base_midi_number();
            let octave_adjusted_diff = (semitone_diff + 12) % 12;

            match dbg!(octave_adjusted_diff) {
                1 => return Some(ScaleDegree::new((i + 1) as u8, Some(Accidental::Sharp))),
                11 => return Some(ScaleDegree::new((i + 1) as u8, Some(Accidental::Flat))),
                _ => continue,
            }
        }

        // Finally check for double accidentals if nothing else matched
        for (i, scale_note) in notes.iter().enumerate() {
            let semitone_diff = note.base_midi_number() - scale_note.base_midi_number();
            let octave_adjusted_diff = (semitone_diff + 12) % 12;

            match octave_adjusted_diff {
                2 => {
                    return Some(ScaleDegree::new(
                        (i + 1) as u8,
                        Some(Accidental::DoubleSharp),
                    ))
                }
                10 => {
                    return Some(ScaleDegree::new(
                        (i + 1) as u8,
                        Some(Accidental::DoubleFlat),
                    ))
                }
                _ => continue,
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
    /// use chordy::{Chord, ChordQuality, Scale, scales, HarmonicFunction, note};
    ///
    /// let c_major_scale = Scale::new(note!("C"), scales::IONIAN);
    /// let g_chord = Chord::new(note!("G"), ChordQuality::Major, vec![]);
    /// assert_eq!(c_major_scale.harmonic_function(&g_chord), Some(HarmonicFunction::Dominant));
    ///
    /// ```
    pub fn harmonic_function(&self, chord: &Chord) -> Option<HarmonicFunction> {
        let notes = chord.notes();

        let scale_degrees: Vec<ScaleDegree> = notes
            .iter()
            .filter_map(|note| self.degree_of(note))
            .collect();

        HarmonicFunction::detect_by_scale_degrees(&scale_degrees)
    }

    /// Creates a chord from the given scale degree (1-7)
    pub fn chord_at_degree(&self, _degree: u8, _chord_type: ChordQuality) -> Chord {
        // Implementation
        todo!()
    }
    /// Returns the relative major/minor of this scale
    pub fn relative(&self) -> Option<Scale> {
        if self.definition == scales::IONIAN {
            // to get the new tonic, transpose the tonic to the 6th interval
            let new_tonic = self.tonic + self.definition.intervals[5];

            // If the scale is Ionian, return the relative minor (Aeolian)
            let relative_minor = Scale::new(new_tonic, scales::AEOLIAN);
            Some(relative_minor)
        } else if self.definition == scales::AEOLIAN {
            // to get the new tonic, transpose the tonic to the 3rd interval
            let new_tonic = self.tonic + self.definition.intervals[5];

            // If the scale is Aeolian, return the relative major (Ionian)
            let relative_major = Scale::new(new_tonic, scales::IONIAN);
            Some(relative_major)
        } else {
            None
        }
    }

    /// Returns the parallel major/minor of this scale
    pub fn parallel(&self) -> Option<Scale> {
        if self.definition == scales::IONIAN {
            let parallel_minor = Scale::new(self.tonic, scales::AEOLIAN);
            Some(parallel_minor)
        } else if self.definition == scales::AEOLIAN {
            let parallel_major = Scale::new(self.tonic, scales::IONIAN);
            Some(parallel_major)
        } else {
            None
        }
    }
    ///
    /// Determine if a given note belongs to the scale
    pub fn contains(&self, _note: &NoteName) -> bool {
        // Implementation
        todo!()
    }

    /// Find the closest scale tone to a given note
    pub fn closest_tone_to(&self, _note: &NoteName) -> NoteName {
        // Implementation
        todo!()
    }

    /*
    /// Calculate the tension/stability of a note in this scale context
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
    pub alteration: Option<Accidental>,
}

impl ScaleDegree {
    /// Create a new scale degree with the given step and optional alteration
    pub const fn new(step: u8, alteration: Option<Accidental>) -> Self {
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
    pub const TONIC: Self = Self::new(1, None);
    pub const SUPERTONIC: Self = Self::new(2, None);
    pub const MEDIANT: Self = Self::new(3, None);
    pub const SUBDOMINANT: Self = Self::new(4, None);
    pub const DOMINANT: Self = Self::new(5, None);
    pub const SUBMEDIANT: Self = Self::new(6, None);
    pub const LEADING_TONE: Self = Self::new(7, None);
    // unaltered 7th scale degree in minor scales
    pub const SUBTONIC: Self = Self::new(7, None);

    // Altered scale degrees
    pub const FLAT_SECOND: Self = Self::new(2, Some(Accidental::Flat));
    pub const FLAT_THIRD: Self = Self::new(3, Some(Accidental::Flat));
    pub const SHARP_FOURTH: Self = Self::new(4, Some(Accidental::Sharp));
    pub const FLAT_SIXTH: Self = Self::new(6, Some(Accidental::Flat));
    pub const FLAT_SEVENTH: Self = Self::new(7, Some(Accidental::Flat));

    // Special scale degrees with traditional names
    pub const NEAPOLITAN: Self = Self::new(2, Some(Accidental::Flat)); // ♭II
}

