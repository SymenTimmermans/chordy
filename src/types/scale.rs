use super::{chord::HarmonicFunction, Accidental, Chord, Interval, IntervalSet, Key, NoteName};
use crate::traits::{ChordLike, HasIntervals, HasRoot};

pub mod definition;
pub use definition::ScaleDefinition;

pub mod bitmask;
pub use bitmask::ScaleBitmask;

/// Scale degree representation and conversion
pub mod degree;
pub use degree::ScaleDegree;

#[allow(dead_code)]
pub mod scales;

/// A musical scale with a tonic and intervals
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scale {
    /// The tonic (starting note) of the scale
    pub tonic: NoteName,
    /// Human-readable name of the scale
    pub name: String,
    /// Intervals that define the scale
    pub intervals: IntervalSet<12>,
    /// For modes, the degree offset in parent scale
    pub degree_offset: Option<u8>,
    /// Optional name of parent scale for modes
    pub mode_of: Option<String>,
    /// Bitmask representing pitch classes
    pub bitmask: ScaleBitmask,
}

/// A scale is a sequence of notes that defines a musical key.
/// ```
/// use chordy::prelude::*;
///
/// // Create a C major scale
/// let c = NoteName::new(Letter::C, Accidental::Natural);
/// let c_major = Scale::from_definition(c, scales::IONIAN);
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
    /// Creates a new scale from a ScaleDefinition (for preset scales)
    pub fn from_definition(tonic: NoteName, definition: ScaleDefinition) -> Self {
        Scale {
            tonic,
            name: definition.name.to_string(),
            intervals: IntervalSet::from_slice(definition.intervals),
            degree_offset: definition.degree_offset,
            mode_of: definition.mode_of.map(|s| s.to_string()),
            bitmask: definition.bitmask,
        }
    }

    /// Creates a major scale with the given tonic
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::prelude::*;
    ///
    /// let c_major = Scale::major(note!("C"));
    /// let notes = c_major.notes();
    /// // [C, D, E, F, G, A, B]
    /// ```
    pub fn major(tonic: NoteName) -> Self {
        Self::from_definition(tonic, scales::IONIAN)
    }

    /// Creates a minor scale (natural minor/Aeolian) with the given tonic
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::prelude::*;
    ///
    /// let a_minor = Scale::minor(note!("A"));
    /// let notes = a_minor.notes();
    /// // [A, B, C, D, E, F, G]
    /// ```
    pub fn minor(tonic: NoteName) -> Self {
        Self::from_definition(tonic, scales::AEOLIAN)
    }

    /// Creates a harmonic minor scale with the given tonic
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::prelude::*;
    ///
    /// let a_harmonic_minor = Scale::harmonic_minor(note!("A"));
    /// let notes = a_harmonic_minor.notes();
    /// // [A, B, C, D, E, F, G#]
    /// ```
    pub fn harmonic_minor(tonic: NoteName) -> Self {
        Self::from_definition(tonic, scales::HARMONIC_MINOR)
    }

    /// Creates a melodic minor scale with the given tonic
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::prelude::*;
    ///
    /// let a_melodic_minor = Scale::melodic_minor(note!("A"));
    /// let notes = a_melodic_minor.notes();
    /// // [A, B, C, D, E, F#, G#]
    /// ```
    pub fn melodic_minor(tonic: NoteName) -> Self {
        Self::from_definition(tonic, scales::MELODIC_MINOR)
    }

    /// Creates a custom scale with the given properties
    pub fn custom(
        tonic: NoteName,
        name: impl Into<String>,
        intervals: impl Into<Vec<Interval>>,
        degree_offset: Option<u8>,
        mode_of: Option<String>,
    ) -> Self {
        let intervals_vec = intervals.into();
        Scale {
            tonic,
            name: name.into(),
            bitmask: ScaleBitmask::from_intervals(&intervals_vec),
            intervals: IntervalSet::from_slice(&intervals_vec),
            degree_offset,
            mode_of,
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
    /// let c_major = Scale::from_definition(note!("C"), scales::IONIAN);
    /// // Exact matches return natural accidentals
    /// assert_eq!(c_major.degree_of(&note!("C")), Some(ScaleDegree::TONIC));
    /// assert_eq!(c_major.degree_of(&note!("G")), Some(ScaleDegree::DOMINANT));
    ///
    /// // Altered notes return appropriate accidentals
    /// assert_eq!(c_major.degree_of(&note!("C#")), Some(ScaleDegree::new(1, Some(Accidental::Sharp))));
    /// assert_eq!(c_major.degree_of(&note!("F#")), Some(ScaleDegree::new(4, Some(Accidental::Sharp))));
    ///
    /// // Works with enharmonic equivalents
    /// let a_minor = Scale::from_definition(note!("A"), scales::AEOLIAN);
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
    /// See: <https://openmusictheory.github.io/harmonicFunctions.html>
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Chord, Scale, scales, HarmonicFunction, note};
    ///
    /// let c_major_scale = Scale::from_definition(note!("C"), scales::IONIAN);
    /// let g_chord = Chord::major(note!("G"));
    /// assert_eq!(c_major_scale.harmonic_function(&g_chord), Some(HarmonicFunction::Dominant));
    ///
    /// ```
    pub fn harmonic_function(&self, chord: &Chord) -> Option<HarmonicFunction> {
        let scale_degrees: Vec<ScaleDegree> = chord
            .notes_iter()
            .filter_map(|note| self.degree_of(&note))
            .collect();

        HarmonicFunction::detect_by_scale_degrees(&scale_degrees)
    }

    /// Creates a triad from the given scale degree (1-7)
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Scale, Chord, note};
    ///
    /// let c_major = Scale::major(note!("C"));
    /// let tonic_chord = c_major.chord_at_degree(1);
    /// assert_eq!(tonic_chord, Chord::major(note!("C")));
    /// ```
    pub fn chord_at_degree(&self, degree: u8) -> Chord {
        if !(1..=7).contains(&degree) {
            panic!("Scale degree must be in range 1-7, got {}", degree);
        }

        let notes = self.notes();
        let root = notes[(degree - 1) as usize];

        // Build triad using scale notes
        let third_degree = ((degree - 1 + 2) % 7) as usize;
        let fifth_degree = ((degree - 1 + 4) % 7) as usize;

        let third = notes[third_degree];
        let fifth = notes[fifth_degree];

        let third_interval = root.interval_to(third);
        let fifth_interval = root.interval_to(fifth);

        Chord::new(
            root,
            vec![Interval::PERFECT_UNISON, third_interval, fifth_interval],
        )
    }

    /// Gets the tonic chord (I)
    pub fn tonic_chord(&self) -> Chord {
        self.chord_at_degree(1)
    }

    /// Gets the supertonic chord (ii)
    pub fn supertonic_chord(&self) -> Chord {
        self.chord_at_degree(2)
    }

    /// Gets the mediant chord (iii)
    pub fn mediant_chord(&self) -> Chord {
        self.chord_at_degree(3)
    }

    /// Gets the subdominant chord (IV)
    pub fn subdominant_chord(&self) -> Chord {
        self.chord_at_degree(4)
    }

    /// Gets the dominant chord (V)
    pub fn dominant_chord(&self) -> Chord {
        self.chord_at_degree(5)
    }

    /// Gets the submediant chord (vi)
    pub fn submediant_chord(&self) -> Chord {
        self.chord_at_degree(6)
    }

    /// Gets the leading tone chord (vii°)
    pub fn leading_tone_chord(&self) -> Chord {
        self.chord_at_degree(7)
    }

    /// Returns the major or minor key signature for this scale
    ///
    /// Determines the key based on the third interval of the scale:
    /// - Scales with a major third map to major keys
    /// - Scales with a minor third map to minor keys
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::prelude::*;
    ///
    /// let c_major = Scale::major(note!("C"));
    /// assert_eq!(c_major.key(), Key::Major(note!("C")));
    ///
    /// let a_minor = Scale::minor(note!("A"));
    /// assert_eq!(a_minor.key(), Key::Minor(note!("A")));
    ///
    /// // Modes also map to their appropriate key signatures
    /// let d_dorian = Scale::from_definition(note!("D"), scales::DORIAN);
    /// assert_eq!(d_dorian.key(), Key::Minor(note!("D")));
    ///
    /// let f_lydian = Scale::from_definition(note!("F"), scales::LYDIAN);
    /// assert_eq!(f_lydian.key(), Key::Major(note!("F")));
    /// ```
    pub fn key(&self) -> Key {
        // Check if the scale contains a major or minor third
        if self.intervals.contains(Interval::MAJOR_THIRD) {
            Key::Major(self.tonic)
        } else {
            // Default to minor for scales with minor third or no third
            Key::Minor(self.tonic)
        }
    }

    /// Returns the relative major/minor of this scale
    pub fn relative(&self) -> Option<Scale> {
        if *self == scales::IONIAN {
            // to get the new tonic, transpose the tonic to the 6th interval
            let new_tonic = self.tonic + self.intervals.as_slice()[5];

            // If the scale is Ionian, return the relative minor (Aeolian)
            let relative_minor = Scale::from_definition(new_tonic, scales::AEOLIAN);
            Some(relative_minor)
        } else if self.name == "Aeolian" {
            // to get the new tonic, transpose the tonic to the 3rd interval
            let new_tonic = self.tonic + self.intervals.as_slice()[5];

            // If the scale is Aeolian, return the relative major (Ionian)
            let relative_major = Scale::from_definition(new_tonic, scales::IONIAN);
            Some(relative_major)
        } else {
            None
        }
    }

    /// Returns the parallel major/minor of this scale
    pub fn parallel(&self) -> Option<Scale> {
        if self == scales::IONIAN {
            let parallel_minor = Scale::from_definition(self.tonic, scales::AEOLIAN);
            Some(parallel_minor)
        } else if self == scales::AEOLIAN {
            let parallel_major = Scale::from_definition(self.tonic, scales::IONIAN);
            Some(parallel_major)
        } else {
            None
        }
    }
    ///
    /// Determine if a given note belongs to the scale
    pub fn contains(&self, note: &NoteName) -> bool {
        let interval: Interval = self.tonic - *note;
        self.intervals.contains(interval)
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

    /// Returns intervals spanning multiple octaves
    ///
    /// Generates intervals from the scale that span the specified number of octaves.
    /// For example, with 2 octaves, a major scale would include intervals up to the 14th.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chordy::prelude::*;
    ///
    /// let c_major = Scale::major(note!("C"));
    /// let extended = c_major.extended_intervals(2);
    ///
    /// // Should include 9th, 11th, 13th intervals
    /// assert!(extended.iter().any(|i| i.semitones() == 14)); // Major 9th
    /// assert!(extended.iter().any(|i| i.semitones() == 17)); // Perfect 11th  
    /// assert!(extended.iter().any(|i| i.semitones() == 21)); // Major 13th
    /// ```
    pub fn extended_intervals(&self, octaves: u8) -> Vec<Interval> {
        let mut extended = Vec::new();

        for octave in 0..octaves {
            for interval in self.intervals.iter() {
                let extended_interval: Interval = interval + Interval::new(0, octave as i8);
                extended.push(extended_interval);
            }
        }

        extended.sort_by_key(|i| i.semitones());
        extended.dedup();
        extended
    }

    /// Returns notes spanning multiple octaves  
    ///
    /// Generates all notes in the scale across the specified number of octaves.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chordy::prelude::*;
    ///
    /// let c_major = Scale::major(note!("C"));
    /// let extended_notes = c_major.extended_notes(2);
    ///
    /// // Should have 14 notes (7 per octave x 2 octaves)
    /// assert_eq!(extended_notes.len(), 14);
    /// ```
    pub fn extended_notes(&self, octaves: u8) -> Vec<NoteName> {
        let extended_intervals = self.extended_intervals(octaves);
        extended_intervals
            .iter()
            .map(|&interval| self.tonic + interval)
            .collect()
    }

    /// Join two scales together, returning a new scale that combines their intervals
    ///
    /// The new scale will have the tonic of the first scale, and it will have the notes of both
    /// scales, by taking into account the interval difference between the tonics.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chordy::prelude::*;
    ///
    /// let c_major = Scale::from_definition(note!("C"), scales::IONIAN);
    /// let d_lydian = Scale::from_definition(note!("D"), scales::LYDIAN);
    /// let joined = c_major.join(&d_lydian);
    ///
    /// // The joined scale will have the tonic of C and intervals (notes) from both scales
    /// assert_eq!(joined.root(), note!("C"));
    ///
    /// let notes = joined.notes();
    /// // F# from D Lydian should be included
    /// assert!(notes.contains(&note!("F#")));
    /// // B from C Major should be included
    /// assert!(notes.contains(&note!("B")));
    /// // C sharp from D Lydian should be included
    /// assert!(notes.contains(&note!("C#")));
    ///
    /// ```
    pub fn join(&self, other: &Scale) -> Scale {
        let tonic_difference = other.tonic - self.tonic;

        let mut intervals: Vec<Interval> = self.intervals.iter().collect();

        // add each of the other intervals, adjusted by the tonic difference
        intervals.extend(other.intervals.iter().map(|i| i + tonic_difference));

        intervals.sort();
        intervals.dedup();

        let bitmask = ScaleBitmask::from_intervals(&intervals);

        Scale {
            tonic: self.tonic,
            name: format!("{} + {} @ {}", self.name, other.name, tonic_difference),
            intervals: IntervalSet::from_slice(&intervals),
            degree_offset: None,
            mode_of: None,
            bitmask,
        }
    }
}

impl HasRoot for Scale {
    fn root(&self) -> NoteName {
        self.tonic
    }
}

impl HasIntervals for Scale {
    fn intervals(&self) -> &[Interval] {
        self.intervals.as_slice()
    }

    fn set_intervals(&mut self, intervals: Vec<Interval>) {
        self.intervals = intervals.into_iter().collect();
    }

    fn remove_interval(&mut self, interval: Interval) {
        self.intervals.remove(interval);
    }

    fn add_interval(&mut self, interval: Interval) {
        if !self.intervals.contains(interval) {
            self.intervals.push(interval);
        }
    }
}

use std::ops::Deref;

impl Deref for Scale {
    type Target = [Interval];

    fn deref(&self) -> &Self::Target {
        self.intervals.as_slice()
    }
}

impl PartialEq<ScaleDefinition> for Scale {
    fn eq(&self, other: &ScaleDefinition) -> bool {
        self.intervals.as_slice() == other.intervals
    }
}

impl PartialEq<ScaleDefinition> for &Scale {
    fn eq(&self, other: &ScaleDefinition) -> bool {
        self.intervals.as_slice() == other.intervals
    }
}

impl PartialEq<Scale> for ScaleDefinition {
    fn eq(&self, other: &Scale) -> bool {
        self.intervals == other.intervals()
    }
}

impl PartialEq<&Scale> for ScaleDefinition {
    fn eq(&self, other: &&Scale) -> bool {
        self.intervals == other.intervals()
    }
}
