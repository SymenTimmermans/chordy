use std::fmt::Display;

use super::{scale::ScaleDegree, Interval, NoteName};
use crate::note;

/// A chord represented by a root note and intervals from that root
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chord {
    pub root: NoteName,
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
        let outcome = notes.iter().fold((candidate, score), |(mut candidate, mut score), note| {
            let note_intervals = notes.iter()
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
            if note_score > score {
                candidate = Some(*note);
                score = note_score;
            } else if note_score == score {
                // If scores are equal, prefer the lower note
                if let Some(c) = candidate {
                    if note.base_midi_number() < c.base_midi_number() {
                        candidate = Some(*note);
                    }
                } else {
                    candidate = Some(*note);
                }
            }
            (candidate, score)
        });

        // if we have a candidate, create the chord
        let root = candidate.unwrap_or(notes.first().cloned().unwrap_or(note!("C")));



        Self::from_notes_and_root(notes, root)
    }

    /// Create a chord from a list of notes and a specified root
    pub fn from_notes_and_root(notes: &[NoteName], root: NoteName) -> Chord {
        Self::new(root, notes.iter()
            .map(|&n| root.interval_to(n))
            .collect())
    }



    /// Returns the notes in the chord with proper theoretical spelling
    pub fn notes(&self) -> Vec<NoteName> {
        self.intervals
            .iter()
            .map(|interval| self.root + *interval)
            .collect()
    }

    // Common chord constructors
    
    pub fn major(root: NoteName) -> Self {
        Self::new(root, vec![
            Interval::PERFECT_UNISON,
            Interval::MAJOR_THIRD,
            Interval::PERFECT_FIFTH,
        ])
    }

    pub fn minor(root: NoteName) -> Self {
        Self::new(root, vec![
            Interval::PERFECT_UNISON,
            Interval::MINOR_THIRD,
            Interval::PERFECT_FIFTH,
        ])
    }

    pub fn diminished(root: NoteName) -> Self {
        Self::new(root, vec![
            Interval::PERFECT_UNISON,
            Interval::MINOR_THIRD,
            Interval::DIMINISHED_FIFTH,
        ])
    }

    pub fn augmented(root: NoteName) -> Self {
        Self::new(root, vec![
            Interval::PERFECT_UNISON,
            Interval::MAJOR_THIRD,
            Interval::AUGMENTED_FIFTH,
        ])
    }

    pub fn dominant_7th(root: NoteName) -> Self {
        Self::new(root, vec![
            Interval::PERFECT_UNISON,
            Interval::MAJOR_THIRD,
            Interval::PERFECT_FIFTH,
            Interval::MINOR_SEVENTH,
        ])
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
}

impl Display for Chord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:?}", self.root, self.intervals)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
