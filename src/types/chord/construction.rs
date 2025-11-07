//! Chord construction and factory methods
//!
//! This module contains methods for creating chords from various sources:
//! - Factory methods for common chord types (major, minor, dominant7, etc.)
//! - Construction from note lists
//! - Construction with specified roots

use crate::{note, types::{Chord, Interval, NoteName}};

impl Chord {
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
            let base_semitones = (base_interval.fifths as i32 * 7
                + base_interval.octaves as i32 * 12)
                .rem_euclid(12);
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

    /// Create a major 9th chord with the given root note
    pub fn major_9th(root: NoteName) -> Self {
        Self::new(
            root,
            vec![
                Interval::PERFECT_UNISON,
                Interval::MAJOR_THIRD,
                Interval::PERFECT_FIFTH,
                Interval::MAJOR_SEVENTH,
                Interval::MAJOR_NINTH,
            ],
        )
    }

    /// Create a minor 9th chord with the given root note
    pub fn minor_9th(root: NoteName) -> Self {
        Self::new(
            root,
            vec![
                Interval::PERFECT_UNISON,
                Interval::MINOR_THIRD,
                Interval::PERFECT_FIFTH,
                Interval::MINOR_SEVENTH,
                Interval::MAJOR_NINTH,
            ],
        )
    }

    /// Create a dominant 9th chord with the given root note
    pub fn dominant_9th(root: NoteName) -> Self {
        Self::new(
            root,
            vec![
                Interval::PERFECT_UNISON,
                Interval::MAJOR_THIRD,
                Interval::PERFECT_FIFTH,
                Interval::MINOR_SEVENTH,
                Interval::MAJOR_NINTH,
            ],
        )
    }

    /// Create a minor 11th chord with the given root note
    pub fn minor_11th(root: NoteName) -> Self {
        Self::new(
            root,
            vec![
                Interval::PERFECT_UNISON,
                Interval::MINOR_THIRD,
                Interval::PERFECT_FIFTH,
                Interval::MINOR_SEVENTH,
                Interval::MAJOR_NINTH,
                Interval::PERFECT_ELEVENTH,
            ],
        )
    }

    /// Create a major 11th chord with the given root note
    pub fn major_11th(root: NoteName) -> Self {
        Self::new(
            root,
            vec![
                Interval::PERFECT_UNISON,
                Interval::MAJOR_THIRD,
                Interval::PERFECT_FIFTH,
                Interval::MAJOR_SEVENTH,
                Interval::MAJOR_NINTH,
                Interval::PERFECT_ELEVENTH,
            ],
        )
    }

    /// Create a dominant 11th chord with the given root note
    pub fn dominant_11th(root: NoteName) -> Self {
        Self::new(
            root,
            vec![
                Interval::PERFECT_UNISON,
                Interval::MAJOR_THIRD,
                Interval::PERFECT_FIFTH,
                Interval::MINOR_SEVENTH,
                Interval::MAJOR_NINTH,
                Interval::PERFECT_ELEVENTH,
            ],
        )
    }

    /// Create a major 13th chord with the given root note
    pub fn major_13th(root: NoteName) -> Self {
        Self::new(
            root,
            vec![
                Interval::PERFECT_UNISON,
                Interval::MAJOR_THIRD,
                Interval::PERFECT_FIFTH,
                Interval::MAJOR_SEVENTH,
                Interval::MAJOR_NINTH,
                Interval::PERFECT_ELEVENTH,
                Interval::MAJOR_THIRTEENTH,
            ],
        )
    }

    /// Create a minor 13th chord with the given root note
    pub fn minor_13th(root: NoteName) -> Self {
        Self::new(
            root,
            vec![
                Interval::PERFECT_UNISON,
                Interval::MINOR_THIRD,
                Interval::PERFECT_FIFTH,
                Interval::MINOR_SEVENTH,
                Interval::MAJOR_NINTH,
                Interval::PERFECT_ELEVENTH,
                Interval::MAJOR_THIRTEENTH,
            ],
        )
    }

    /// Create a dominant 13th chord with the given root note
    pub fn dominant_13th(root: NoteName) -> Self {
        Self::new(
            root,
            vec![
                Interval::PERFECT_UNISON,
                Interval::MAJOR_THIRD,
                Interval::PERFECT_FIFTH,
                Interval::MINOR_SEVENTH,
                Interval::MAJOR_NINTH,
                Interval::PERFECT_ELEVENTH,
                Interval::MAJOR_THIRTEENTH,
            ],
        )
    }

    /// Create a dominant 7th flat 9 chord with the given root note
    pub fn dominant_7th_flat_9(root: NoteName) -> Self {
        Self::new(
            root,
            vec![
                Interval::PERFECT_UNISON,
                Interval::MAJOR_THIRD,
                Interval::PERFECT_FIFTH,
                Interval::MINOR_SEVENTH,
                Interval::MINOR_NINTH,
            ],
        )
    }

    /// Create a dominant 7th sharp 9 chord with the given root note
    pub fn dominant_7th_sharp_9(root: NoteName) -> Self {
        Self::new(
            root,
            vec![
                Interval::PERFECT_UNISON,
                Interval::MAJOR_THIRD,
                Interval::PERFECT_FIFTH,
                Interval::MINOR_SEVENTH,
                Interval::AUGMENTED_NINTH,
            ],
        )
    }

    /// Create a dominant 7th flat 13 chord with the given root note
    pub fn dominant_7th_flat_13(root: NoteName) -> Self {
        Self::new(
            root,
            vec![
                Interval::PERFECT_UNISON,
                Interval::MAJOR_THIRD,
                Interval::PERFECT_FIFTH,
                Interval::MINOR_SEVENTH,
                Interval::MINOR_THIRTEENTH,
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
}