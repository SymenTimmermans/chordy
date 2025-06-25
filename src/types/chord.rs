use std::{fmt::Display, str::FromStr};

use super::{scale::ScaleDegree, Interval, NoteName};
use crate::{
    error::ParseError, note, traits::{HasIntervals, HasRoot, Invertible}
};

mod quality;
pub use quality::ChordQuality;

pub mod naming;
pub use naming::*;

/// A chord represented by a root note and intervals from that root
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chord {
    /// The root note of the chord
    pub root: NoteName,
    /// The intervals from the root note that define the chord.
    ///
    /// Intervals are typically in ascending order, starting from the root, which is included as a
    /// PERFECT_UNISON.
    pub intervals: Vec<Interval>,
}

impl Chord {
    /// Create a new chord from root and intervals
    pub fn new(root: NoteName, intervals: Vec<Interval>) -> Self {
        Chord { root, intervals }
    }

    /// Create a chord from a list of notes
    pub fn from_notes(notes: &[NoteName]) -> Self {
        // We could get notes in any order, so we need to determine the root
        // In order to do this, we will create interval sets from each note.
        // The interval set that contains a fifth and some third will be the root.
        let candidate: Option<NoteName> = notes.first().cloned();
        let score: i32 = i32::MIN;
        notes
            .iter()
            .fold((candidate, score), |(mut candidate, mut score), note| {
                let note_intervals = notes
                    .iter()
                    .filter(|&&n| n != *note)
                    .map(|&n| note.interval_to(n))
                    .collect::<Vec<Interval>>();
                let note_score = note_intervals.iter().fold(0, |acc, interval| {
                    if interval.is_fifth() {
                        acc + 5
                    } else if interval.is_third() {
                        acc + 3
                    } else {
                        acc
                    }
                });
                match note_score.cmp(&score) {
                    // equal score, prefer lower note
                    std::cmp::Ordering::Equal => {
                        if let Some(c) = candidate {
                            if note.base_midi_number() < c.base_midi_number() {
                                candidate = Some(*note);
                            }
                        } else {
                            candidate = Some(*note);
                        }
                    }
                    std::cmp::Ordering::Greater => {
                        candidate = Some(*note);
                        score = note_score;
                    }
                    _ => {}
                }
                (candidate, score)
            });

        // if we have a candidate, create the chord
        let root = candidate.unwrap_or(notes.first().cloned().unwrap_or(note!("C")));

        Self::from_notes_and_root(notes, root)
    }

    /// Create a chord from a list of notes and a specified root
    pub fn from_notes_and_root(notes: &[NoteName], root: NoteName) -> Chord {
        Self::new(root, notes.iter().map(|&n| root.interval_to(n)).collect())
    }

    /// Create a chord from an ordered list of notes and a specified root
    /// 
    /// This method assumes the notes are in ascending order and calculates 
    /// compound intervals when necessary. For example, in "G,B,D,F,Ab", 
    /// the Ab will be treated as a minor 9th rather than a minor 2nd.
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Chord, note};
    ///
    /// // This creates a G7♭9 chord
    /// let notes = [note!("G"), note!("B"), note!("D"), note!("F"), note!("Ab")];
    /// let chord = Chord::from_notes_ordered(&notes, note!("G"));
    /// assert_eq!(chord.abbreviated_name(), "G7♭9");
    /// ```
    pub fn from_notes_ordered(notes: &[NoteName], root: NoteName) -> Chord {
        if notes.is_empty() {
            return Self::new(root, vec![]);
        }

        let mut intervals = Vec::new();
        let mut last_semitone_position = 0; // Position of the root
        
        for &note in notes {
            if note == root {
                // Root note, add perfect unison
                intervals.push(Interval::PERFECT_UNISON);
                continue;
            }

            // Calculate the basic interval from root to this note
            let base_interval = root.interval_to(note);
            
            // Calculate how many semitones this represents (using positive modulo)
            let base_semitones = (base_interval.fifths as i32 * 7 + base_interval.octaves as i32 * 12).rem_euclid(12);
            let mut final_semitones = base_semitones;
            let mut octaves_to_add = 0;
            
            // If this note would be at the same position or lower than the last note,
            // move it up octaves until it's higher
            while final_semitones <= last_semitone_position {
                final_semitones += 12;
                octaves_to_add += 1;
            }
            
            // Create the final interval
            let final_interval = Interval {
                fifths: base_interval.fifths,
                octaves: base_interval.octaves + octaves_to_add,
            };
            
            intervals.push(final_interval);
            last_semitone_position = final_semitones;
        }

        Self::new(root, intervals)
    }

    /// Create a chord from an ordered list of notes, using the first note as root
    /// 
    /// This is a convenience method that uses the first note as the root and 
    /// applies ordered parsing to the rest.
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Chord, note};
    ///
    /// // This creates a G7♭9 chord
    /// let notes = [note!("G"), note!("B"), note!("D"), note!("F"), note!("Ab")];
    /// let chord = Chord::from_notes_ordered_root_first(&notes);
    /// assert_eq!(chord.abbreviated_name(), "G7♭9");
    /// ```
    pub fn from_notes_ordered_root_first(notes: &[NoteName]) -> Chord {
        if notes.is_empty() {
            return Self::new(note!("C"), vec![]);
        }
        
        let root = notes[0];
        Self::from_notes_ordered(notes, root)
    }

    /// Create a major chord with the given root note
    pub fn major(root: NoteName) -> Self {
        Self::new(
            root,
            vec![
                Interval::PERFECT_UNISON,
                Interval::MAJOR_THIRD,
                Interval::PERFECT_FIFTH,
            ],
        )
    }

    /// Create a minor chord with the given root note
    pub fn minor(root: NoteName) -> Self {
        Self::new(
            root,
            vec![
                Interval::PERFECT_UNISON,
                Interval::MINOR_THIRD,
                Interval::PERFECT_FIFTH,
            ],
        )
    }

    /// Create a diminished chord with the given root note
    pub fn diminished(root: NoteName) -> Self {
        Self::new(
            root,
            vec![
                Interval::PERFECT_UNISON,
                Interval::MINOR_THIRD,
                Interval::DIMINISHED_FIFTH,
            ],
        )
    }

    /// Create an augmented chord with the given root note
    pub fn augmented(root: NoteName) -> Self {
        Self::new(
            root,
            vec![
                Interval::PERFECT_UNISON,
                Interval::MAJOR_THIRD,
                Interval::AUGMENTED_FIFTH,
            ],
        )
    }

    /// Create a dominant 7th chord with the given root note
    pub fn dominant_7th(root: NoteName) -> Self {
        Self::new(
            root,
            vec![
                Interval::PERFECT_UNISON,
                Interval::MAJOR_THIRD,
                Interval::PERFECT_FIFTH,
                Interval::MINOR_SEVENTH,
            ],
        )
    }

    /// Create a major 7th chord with the given root note
    pub fn major_7th(root: NoteName) -> Self {
        Self::new(
            root,
            vec![
                Interval::PERFECT_UNISON,
                Interval::MAJOR_THIRD,
                Interval::PERFECT_FIFTH,
                Interval::MAJOR_SEVENTH,
            ],
        )
    }

    /// Create a minor 7th chord with the given root note
    pub fn minor_7th(root: NoteName) -> Self {
        Self::new(
            root,
            vec![
                Interval::PERFECT_UNISON,
                Interval::MINOR_THIRD,
                Interval::PERFECT_FIFTH,
                Interval::MINOR_SEVENTH,
            ],
        )
    }

    /// Create a minor-major 7th chord with the given root note
    pub fn minor_major_7th(root: NoteName) -> Self {
        Self::new(
            root,
            vec![
                Interval::PERFECT_UNISON,
                Interval::MINOR_THIRD,
                Interval::PERFECT_FIFTH,
                Interval::MAJOR_SEVENTH,
            ],
        )
    }

    /// Create a half-diminished 7th chord (minor 7th flat 5) with the given root note
    pub fn minor_7th_flat_5(root: NoteName) -> Self {
        Self::new(
            root,
            vec![
                Interval::PERFECT_UNISON,
                Interval::MINOR_THIRD,
                Interval::DIMINISHED_FIFTH,
                Interval::MINOR_SEVENTH,
            ],
        )
    }

    // More chord constructors can be added as needed...

    /// Return a Harte representation (string) of the chord
    pub fn to_harte(&self) -> String {
        todo!()
    }

    /// Parse a Harte representation (string) of the chord
    pub fn from_harte(_harte: &str) -> Self {
        todo!()
    }

    /// Returns true if the intervals contain the major third
    pub fn is_major(&self) -> bool {
        self.intervals.contains(&Interval::MAJOR_THIRD)
    }



    /// Return abbreviated name of the chord.
    ///
    /// Uses the new chord naming system for consistent and accurate chord symbol generation.
    /// This method replaces the old nested if/else logic with a clean analyzer-based approach.
    /// Uses legacy compatibility rendering to maintain backward compatibility.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chordy::{Chord, note};
    ///
    /// let a_minor = Chord::from_notes_and_root(
    ///     [note!("A"), note!("C"), note!("E")].as_ref(),
    ///     note!("A")
    /// );
    ///
    /// assert_eq!(a_minor.abbreviated_name(), "Am");
    /// ```
    pub fn abbreviated_name(&self) -> String {
        // Use the new ChordAnalyzer with legacy renderer for backward compatibility
        let chord_name = self.to_chord_name();
        ChordRenderer::legacy().render(&chord_name)
    }

    /// Returns the HTML representation of the chord.
    pub fn to_html(&self) -> String {
        // Use the new ChordAnalyzer with HTML renderer
        let chord_name = self.to_chord_name();
        ChordRenderer::html().render(&chord_name)
    }

    /// Convert this chord to a roman numeral chord in the given key
    pub fn to_roman(&self, key: &super::Key) -> Option<super::RomanChord> {
        use super::RomanNumeral;
        
        // Calculate the interval from the key root to this chord's root
        let interval_from_key = key.root().interval_to(self.root);
        
        let roman_numeral: RomanNumeral = interval_from_key.into();
        println!("Converting chord {} to roman numeral: {} via interval: {}", self, roman_numeral, interval_from_key);
        Some(super::RomanChord::new(roman_numeral, self.intervals.clone()))
    }

    /// Analyze this chord in the given key, returning both roman numeral and harmonic function
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Chord, Key, note};
    ///
    /// let g_chord = Chord::major(note!("G"));
    /// let c_major_key = Key::Major(note!("C"));
    /// let (roman, function) = g_chord.analyze_in_key(&c_major_key);
    /// // roman would be V, function would be Some(Dominant)
    /// ```
    pub fn analyze_in_key(&self, key: &super::Key) -> (super::RomanNumeral, Option<super::HarmonicFunction>) {
        let roman = key.analyze_chord(self);
        let function = key.harmonic_function(self);
        (roman, function)
    }

    /// Get the roman numeral for this chord in the given key
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Chord, Key, RomanNumeral, note};
    ///
    /// let g_chord = Chord::major(note!("G"));
    /// let c_major_key = Key::Major(note!("C"));
    /// let roman = g_chord.roman_in_key(&c_major_key);
    /// assert_eq!(roman, RomanNumeral::V());
    /// ```
    pub fn roman_in_key(&self, key: &super::Key) -> super::RomanNumeral {
        key.analyze_chord(self)
    }

    /// Get the harmonic function of this chord in the given key
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Chord, Key, HarmonicFunction, note};
    ///
    /// let g_chord = Chord::major(note!("G"));
    /// let c_major_key = Key::Major(note!("C"));
    /// let function = g_chord.function_in_key(&c_major_key);
    /// assert_eq!(function, Some(HarmonicFunction::Dominant));
    /// ```
    pub fn function_in_key(&self, key: &super::Key) -> Option<super::HarmonicFunction> {
        key.harmonic_function(self)
    }

    /// Convert this chord to a ChordName using the new naming system
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Chord, note};
    ///
    /// let g7 = Chord::dominant_7th(note!("G"));
    /// let chord_name = g7.to_chord_name();
    /// assert_eq!(format!("{}", chord_name), "G7");
    /// ```
    pub fn to_chord_name(&self) -> ChordName {
        ChordAnalyzer::analyze(self.root, &self.intervals)
    }

    // Fluent interface methods for method chaining
    
    /// Add an interval to this chord (fluent interface)
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Chord, Interval, note};
    ///
    /// let chord = Chord::major(note!("C"))
    ///     .with_interval(Interval::MINOR_SEVENTH);
    /// // Creates a C7 chord
    /// ```
    pub fn with_interval(mut self, interval: Interval) -> Self {
        if !self.intervals.contains(&interval) {
            self.intervals.push(interval);
            self.intervals.sort();
        }
        self
    }

    /// Remove an interval from this chord (fluent interface)
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Chord, Interval, note};
    ///
    /// let chord = Chord::major(note!("C"))
    ///     .without_interval(Interval::PERFECT_FIFTH);
    /// // Creates a C chord without the fifth (C power chord)
    /// ```
    pub fn without_interval(mut self, interval: Interval) -> Self {
        self.intervals.retain(|&i| i != interval);
        self
    }

    /// Transpose this chord (fluent interface)
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Chord, Interval, note};
    ///
    /// let chord = Chord::major(note!("C"))
    ///     .transposed_by(Interval::MAJOR_THIRD);
    /// // Creates an E major chord
    /// ```
    pub fn transposed_by(mut self, interval: Interval) -> Self {
        use crate::traits::Transposable;
        self.transpose(interval);
        self
    }

    /// Convert to a different chord type while keeping the root (fluent interface)
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Chord, note};
    ///
    /// let minor_chord = Chord::major(note!("C"))
    ///     .as_minor();
    /// // Converts C major to C minor
    /// ```
    pub fn as_minor(self) -> Self {
        Chord::minor(self.root)
    }

    /// Convert to major chord while keeping the root (fluent interface)
    pub fn as_major(self) -> Self {
        Chord::major(self.root)
    }

    /// Convert to diminished chord while keeping the root (fluent interface)
    pub fn as_diminished(self) -> Self {
        Chord::diminished(self.root)
    }

    /// Convert to augmented chord while keeping the root (fluent interface)
    pub fn as_augmented(self) -> Self {
        Chord::augmented(self.root)
    }
}

impl Display for Chord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.abbreviated_name())
    }
}

impl HasRoot for Chord {
    fn root(&self) -> NoteName {
        self.root
    }

    fn root_mut(&mut self) -> &mut NoteName {
        &mut self.root
    }
}

impl HasIntervals for Chord {
    fn intervals(&self) -> &[Interval] {
        &self.intervals
    }

    fn intervals_mut(&mut self) -> &mut Vec<Interval> {
        &mut self.intervals
    }
}

impl Invertible for Chord {
    fn inverted(&self, inversion: u8) -> Self {
        let mut intervals = self.intervals.clone();
        // Rotate intervals based on inversion
        intervals.rotate_left(inversion as usize % self.intervals.len());
        // Adjust octaves for proper inversion
        if inversion > 0 {
            if let Some(last) = intervals.last_mut() {
                *last = *last - Interval::OCTAVE;
            }
        }
        Chord::new(self.root, intervals)
    }
}

/// Implement FromStr for Chord to allow parsing from a string representation.
///
impl FromStr for Chord {
    type Err = ParseError;

    /// Parses a string into a Chord, currently returning an error as a placeholder.
    ///
    /// Supports only list of notes right now, where the notes are separated by comma.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chordy::prelude::*;
    ///
    /// let chord: Chord = "C,E,G".parse().unwrap();
    /// let c_major = Chord::major(note!("C"));
    ///
    /// assert_eq!(chord, c_major);
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // split the string by commas and parse each note
        let notes: Vec<NoteName> = s
            .split(',')
            .map(|note_str| note_str.trim().parse::<NoteName>())
            .collect::<Result<Vec<NoteName>, ParseError>>()?;

        if notes.is_empty() {
            return Err(ParseError::InvalidChordFormat(s.to_string()));
        }

        // Use ordered parsing with the first note as root for test compatibility
        Ok(Chord::from_notes_ordered_root_first(&notes))
    }
}

/// Harmonic functions represent the roles chords play in a key
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum HarmonicFunction {
    Tonic,
    Subdominant,
    Dominant,
}

impl HarmonicFunction {
    fn all() -> Vec<HarmonicFunction> {
        vec![
            HarmonicFunction::Tonic,
            HarmonicFunction::Subdominant,
            HarmonicFunction::Dominant,
        ]
    }

    fn triggers(&self) -> Vec<u8> {
        match self {
            HarmonicFunction::Tonic => vec![1, 3],
            HarmonicFunction::Subdominant => vec![4, 6],
            HarmonicFunction::Dominant => vec![5, 7],
        }
    }

    fn associates(&self) -> Vec<u8> {
        match self {
            HarmonicFunction::Tonic => vec![5, 6],
            HarmonicFunction::Subdominant => vec![1, 2],
            HarmonicFunction::Dominant => vec![2],
        }
    }

    fn dissonances(&self) -> Vec<(u8, Option<u8>)> {
        match self {
            HarmonicFunction::Tonic => vec![(5, Some(6)), (7, None)],
            HarmonicFunction::Subdominant => vec![(1, Some(2)), (3, None)],
            HarmonicFunction::Dominant => vec![(4, None), (6, None)],
        }
    }

    /// Detects the harmonic function of a chord based on its scale degrees
    pub fn detect_by_scale_degrees(scale_degrees: &[ScaleDegree]) -> Option<Self> {
        let all = Self::all();
        let simple_degrees = scale_degrees.iter().map(|sd| sd.step).collect::<Vec<u8>>();
        let scores = all.iter().map(|hf| {
            let mut score = 0;
            for degree in simple_degrees.iter() {
                if hf.triggers().contains(degree) {
                    score += 8;
                } else if hf.associates().contains(degree) {
                    score += 4;
                } else if hf.dissonances().iter().any(|(d, i)| {
                    d == degree && (i.is_none() || simple_degrees.contains(&i.unwrap()))
                }) {
                    score += 1;
                }
            }
            (hf, score)
        });

        // if no harmonic function has a positive score, return None
        if scores.clone().all(|(_, score)| score <= 0) {
            return None;
        }

        // return the harmonic function with the highest score
        scores.max_by_key(|(_, score)| *score).map(|(hf, _)| *hf)
    }
}
