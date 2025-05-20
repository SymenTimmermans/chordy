use std::fmt::Display;

use super::{ChordExtension, Interval, NoteName};

/// A chord with a root note and quality
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chord {
    root: NoteName,
    quality: ChordQuality,
    extensions: Vec<ChordExtension>,
}

impl Chord {
    pub fn new(root: NoteName, quality: ChordQuality, extensions: Vec<ChordExtension>) -> Self {
        Chord {
            root,
            quality,
            extensions,
        }
    }

    /// Returns the notes in the chord with proper theoretical spelling
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::types::Chord;
    /// use chordy::note;
    ///
    /// let d_minor = Chord::new(note!("D"), ChordQuality::Minor, vec![]);
    /// assert_eq!(d_minor.notes(), vec![note!("D"), note!("F"), note!("A")]);
    /// ```
    pub fn notes(&self) -> Vec<NoteName> {
        // Start with the intervals that define the chord quality
        let base_intervals = self.quality.get_intervals();

        // Add extensions if present
        let mut all_intervals = base_intervals.clone();
        for extension in &self.extensions {
            all_intervals.extend(extension.get_intervals());
        }

        // Remove duplicates (e.g., if an extension replaces a note in the base triad)
        all_intervals.sort_by_key(|interval| interval.semitones());
        all_intervals.dedup_by_key(|interval| interval.size);

        // Build the chord by applying each interval to the root
        all_intervals
            .iter()
            .map(|interval| self.root.apply_interval(*interval))
            .collect()
    }
}

impl Display for Chord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:?}", self.root, self.quality)
    }
}

/// The quality/type of a chord (major, minor, etc.)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChordQuality {
    Major,
    Minor,
    Diminished,
    Augmented,
    Sus2,
    Sus4,
    // etc.
}

impl ChordQuality {
    /// Returns the intervals that make up the chord quality.
    /// This also includes the root note, in order to support rootless chords, where we could omit
    /// it.
    pub fn get_intervals(&self) -> Vec<Interval> {
        match self {
            ChordQuality::Major => vec![
                Interval::UNISON,
                Interval::MAJOR_THIRD,
                Interval::PERFECT_FIFTH,
            ],
            ChordQuality::Minor => vec![
                Interval::UNISON,
                Interval::MINOR_THIRD,
                Interval::PERFECT_FIFTH,
            ],
            ChordQuality::Diminished => vec![
                Interval::UNISON,
                Interval::MINOR_THIRD,
                Interval::DIMINISHED_FIFTH,
            ],
            ChordQuality::Augmented => vec![
                Interval::UNISON,
                Interval::MAJOR_THIRD,
                Interval::AUGMENTED_FIFTH,
            ],
            ChordQuality::Sus2 => vec![
                Interval::UNISON,
                Interval::MAJOR_SECOND,
                Interval::PERFECT_FIFTH,
            ],
            ChordQuality::Sus4 => vec![
                Interval::UNISON,
                Interval::PERFECT_FOURTH,
                Interval::PERFECT_FIFTH,
            ],
        }
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

    pub fn detect_by_scale_degrees(scale_degrees: &[u8]) -> Option<Self> {
        let all = Self::all();
        let scores = all.iter().map(|hf| {
            let mut score = 0;
            for degree in scale_degrees {
                if hf.triggers().contains(degree) {
                    score += 8;
                } else if hf.associates().contains(degree) {
                    score += 4;
                } else if hf.dissonances().iter().any(|(d, i)| {
                    d == degree && (i.is_none() || scale_degrees.contains(&i.unwrap()))
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
        scores
            .max_by_key(|(_, score)| *score)
            .map(|(hf, _)| *hf)
    }
}
