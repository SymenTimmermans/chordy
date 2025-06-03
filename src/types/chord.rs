use std::{fmt::Display, str::FromStr};

use super::{scale::ScaleDegree, Interval, NoteName};
use crate::{
    error::ParseError, note, traits::{HasIntervals, HasRoot, Invertible}
};

mod quality;
pub use quality::ChordQuality;

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

    /// Returns the chord quality if it can be determined.
    ///
    /// It mainly considers the third and fifth intervals to determine the quality.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chordy::{Chord, note, ChordQuality};
    ///
    /// let d_major = Chord::from_notes_and_root(
    ///     &[note!("D"), note!("F#"), note!("A")],
    ///     note!("D")
    /// );
    /// assert_eq!(d_major.quality(), Some(ChordQuality::Major));
    ///
    /// let b_minor_diad = Chord::from_notes_and_root(
    ///    &[note!("B"), note!("D")],
    ///    note!("B")
    /// );
    /// assert_eq!(b_minor_diad.quality(), Some(ChordQuality::Minor));
    ///
    /// ```
    pub fn quality(&self) -> Option<ChordQuality> {
        ChordQuality::detect(self)
    }

    /// Returns the extended type of the chord, such as "7", "9", "11", etc.
    pub fn extended_type(&self) -> String {
        let has_minor_7th = self.intervals.contains(&Interval::MINOR_SEVENTH);
        let has_major_7th = self.intervals.contains(&Interval::MAJOR_SEVENTH);
        let has_9th = self.intervals.contains(&Interval::MAJOR_NINTH) || 
                     self.intervals.contains(&Interval::MINOR_NINTH) ||
                     self.intervals.contains(&Interval::MINOR_SECOND) ||
                     self.intervals.contains(&Interval::MAJOR_SECOND);
        let has_11th = self.intervals.contains(&Interval::PERFECT_ELEVENTH) ||
                      self.intervals.contains(&Interval::PERFECT_FOURTH);
        let has_13th = self.intervals.contains(&Interval::MAJOR_THIRTEENTH) ||
                      self.intervals.contains(&Interval::MINOR_THIRTEENTH) ||
                      self.intervals.contains(&Interval::MAJOR_SIXTH) ||
                      self.intervals.contains(&Interval::MINOR_SIXTH);

        // Handle 13th chords first since they're most specific
        if has_13th {
            if has_11th && has_9th {
                if has_minor_7th {
                    return "13".to_string(); // Dominant 13th (e.g. C13)
                } else if has_major_7th {
                    if self.is_major() {
                        return "maj13".to_string(); // Major 13th (e.g. Cmaj13)
                    } else {
                        return "(maj13)".to_string(); // Minor-major 13th
                    }
                } else {
                    return "add13".to_string(); // Just a 13th without 7th, 9th or 11th
                }
            } else if has_major_7th {
                if self.is_major() {
                    if has_9th {
                        return "maj13(no11)".to_string();
                    } else if has_11th {
                        return "maj13(no9)".to_string();
                    } else {
                        return "maj7(add13)".to_string();
                    }
                } else {
                    if has_9th {
                        return "(maj13)(no11)".to_string();
                    } else if has_11th {
                        return "(maj13)(no9)".to_string();
                    } else {
                        return "(maj7)(add13)".to_string();
                    }
                }
            } else if has_minor_7th {
                if has_9th {
                    return "13(no11)".to_string();
                } else if has_11th {
                    return "13(no9)".to_string();
                } else {
                    return "7(add13)".to_string();
                }
            } else {
                return "add13".to_string();
            }
        }

        // Handle 11th chords next
        if has_11th {
            if has_9th {
                if has_major_7th {
                    if self.is_major() {
                        return "maj11".to_string(); // Major 11th (e.g. Cmaj11)
                    } else {
                        return "(maj11)".to_string(); // Minor-major 11th
                    }
                } else if has_minor_7th {
                    return "11".to_string(); // Dominant 11th (e.g. C11)
                } else {
                    return "add11".to_string(); // Just an 11th without 7th or 9th
                }
            } else if has_major_7th {
                if self.is_major() {
                    return "maj7(add11)".to_string();
                } else {
                    return "(maj7)(add11)".to_string();
                }
            } else if has_minor_7th {
                return "7(add11)".to_string();
            } else {
                return "add11".to_string();
            }
        }

        // Handle 9th chords next
        if has_9th {
            if has_minor_7th {
                return "9".to_string(); // Dominant 9th (e.g. C9)
            } else if has_major_7th {
                if self.is_major() {
                    return "maj9".to_string(); // Major 9th (e.g. Cmaj9)
                } else {
                    return "(maj9)".to_string(); // Minor-major 9th (e.g. Cm(maj9))
                }
            } else {
                return "add9".to_string(); // Just a 9th without 7th
            }
        }

        // Handle 7th chords if no higher extensions present
        if has_minor_7th {
            return "7".to_string();
        } else if has_major_7th {
            if self.is_major() {
                return "maj7".to_string();
            } else {
                return "(maj7)".to_string(); // Minor-major 7th
            }
        }

        String::new() // No recognized extension
    }

    /// Return abbreviated name of the chord.
    ///
    /// Tries to figure out by intervals what the chord name is and then creates suffixes for any
    /// remaining intervals.
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
        let quality = match self.quality() {
            Some(ChordQuality::Major) => "",
            Some(ChordQuality::Minor) => "m",
            Some(ChordQuality::Diminished) => "dim",
            Some(ChordQuality::Augmented) => "aug",
            None => "",
        };

        format!("{}{}{}", self.root, quality, self.extended_type())
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

        Ok(Chord::from_notes(&notes))
    }
}

/// Harmonic functions represent the roles chords play in a key
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
