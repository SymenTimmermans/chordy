use std::fmt::Display;

use quality::ChordQuality;

use super::{scale::ScaleDegree, Interval, NoteName};
use crate::{note, traits::{HasIntervals, HasRoot, Invertible}};

mod quality;

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

    /// Returns the chord qualitty if it can be determined.
    pub fn quality(&self) -> Option<ChordQuality> {
        None
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

        format!(
            "{}{}",
            self.root,
            quality
        )
    }
}

impl Display for Chord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:?}", self.root, self.intervals)
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
