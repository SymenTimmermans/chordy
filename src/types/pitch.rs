use std::fmt;
use std::ops::{Add, AddAssign};
use std::str::FromStr;

use crate::error::ParseError;

use super::{Accidental, Letter, NoteName};

/// A specific pitch with both note name and octave
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pitch {
    name: NoteName,
    octave: i8,
}

impl Pitch {
    pub fn new(letter: Letter, accidental: Accidental, octave: i8) -> Self {
        Pitch {
            name: NoteName::new(letter, accidental),
            octave,
        }
    }

    /// Returns the full MIDI note number for this pitch.
    /// Starting from C-2 (MIDI note 0).
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Pitch, Letter, Accidental};
    ///
    /// let pitch = Pitch::new(Letter::C, Accidental::Natural, 3);
    /// assert_eq!(pitch.midi_number(), 60);
    ///
    /// let pitch = Pitch::new(Letter::G, Accidental::Sharp, 5);
    /// assert_eq!(pitch.midi_number(), 92);
    ///
    /// ```
    pub fn midi_number(&self) -> i8 {
        // MIDI octaves start at -2, where C-2 is note 0
        self.name.base_midi_number() + ((self.octave + 2) * 12)
    }

    /// Checks if two pitches represent the same frequency.
    /// Often used for enharmonic equivalence. This assumes equal temperament tuning.
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Pitch, Letter, Accidental};
    ///
    /// let p1 = Pitch::new(Letter::C, Accidental::Natural, 4);
    /// let p2 = Pitch::new(Letter::B, Accidental::Sharp, 3);
    ///
    /// assert!(p1.is_enharmonic_with(&p2));
    ///
    /// let p1 = Pitch::new(Letter::A, Accidental::Flat, 4);
    /// let p2 = Pitch::new(Letter::G, Accidental::Sharp, 4);
    ///
    /// assert!(p1.is_enharmonic_with(&p2));
    /// ```
    pub fn is_enharmonic_with(&self, other: &Self) -> bool {
        self.midi_number() == other.midi_number()
    }

    pub fn transpose(&self, semitone_offset: i8) -> Pitch {
        if semitone_offset == 0 {
            return *self;
        }

        let target_midi = self.midi_number() + semitone_offset;

        let mut best_pitch = None;
        let mut best_score = i32::MAX;

        for letter in Letter::all() {
            let base_index = letter.base_midi_number();
            // Guess octave roughly
            let candidate_octave = (target_midi / 12) - 2;

            for accidental in Accidental::all() {
                let semitone =
                    base_index + accidental.semitone_offset() + 12 * (candidate_octave + 2);
                let mut pitch_guess = Pitch::new(letter, accidental, candidate_octave);

                // If it's too far off, maybe we got the wrong octave
                let diff = semitone - target_midi;
                if diff > 6 {
                    pitch_guess.octave -= 1;
                } else if diff < -6 {
                    pitch_guess.octave += 1;
                }

                let final_midi = pitch_guess.midi_number();
                if final_midi != target_midi {
                    continue; // skip invalid guesses
                }

                let interval_class_penalty = interval_penalty(self, &pitch_guess);

                let spelling_penalty = if self.name.accidental != accidental {
                    accidental.penalty()
                } else {
                    0
                };

                let letter_distance =
                    (pitch_guess.name.letter as i8 - self.name.letter as i8).rem_euclid(7);
                let unnatural_motion_penalty = if letter_distance == 6 || letter_distance == 0 {
                    2
                } else {
                    0
                };
                let enharmonic_suspicion_penalty = if pitch_guess.is_suspicious_spelling() {
                    3
                } else {
                    0
                };

                let expected = expected_letter(self.name.letter, semitone_offset);
                let letter_bias_penalty = if letter != expected { 2 } else { 0 };
                let direction_bias_penalty = flat_vs_sharp_bias(accidental, semitone_offset);
                let total_penalty = interval_class_penalty
                    + spelling_penalty
                    + unnatural_motion_penalty
                    + enharmonic_suspicion_penalty
                    + letter_bias_penalty
                    + direction_bias_penalty;

                if total_penalty <= best_score {
                    if semitone_offset < 0
                        && (!accidental.is_sharp() || self.name.accidental.is_sharp())
                    {
                        best_pitch = Some(pitch_guess);
                    }
                    if semitone_offset > 0
                        && (!accidental.is_flat() || self.name.accidental.is_flat())
                    {
                        best_pitch = Some(pitch_guess);
                    }
                    best_score = total_penalty;
                }
            }
        }

        best_pitch.unwrap_or_else(|| {
            panic!(
                "No valid pitch spelling found transposing {} {}",
                self, semitone_offset
            )
        })
    }

    pub fn is_suspicious_spelling(&self) -> bool {
        matches!(
            (self.name.letter, self.name.accidental),
            (Letter::B, Accidental::Sharp)
                | (Letter::E, Accidental::Sharp)
                | (Letter::C, Accidental::Flat)
                | (Letter::F, Accidental::Flat)
        )
    }
}

impl fmt::Display for Pitch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.name, self.octave)
    }
}

impl Add<i8> for Pitch {
    type Output = Pitch;

    fn add(self, semitones: i8) -> Self::Output {
        self.transpose(semitones)
    }
}

impl AddAssign<i8> for Pitch {
    fn add_assign(&mut self, semitones: i8) {
        *self = *self + semitones;
    }
}

impl FromStr for Pitch {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Split into note part and octave part
        let (note_part, octave_part) = s.split_at(
            s.rfind(|c: char| !c.is_ascii_digit() && c != '-')
                .map(|pos| pos + 1)
                .unwrap_or(0),
        );

        if note_part.is_empty() || octave_part.is_empty() {
            return Err(ParseError::InvalidPitch(s.to_string()));
        }

        // Parse octave allowing for negative numbers
        let octave = octave_part
            .parse::<i8>()
            .map_err(|_| ParseError::InvalidPitch(s.to_string()))?;

        // Parse note name
        let name = NoteName::from_str(note_part)?;

        Ok(Pitch { name, octave })
    }
}

fn interval_penalty(from: &Pitch, to: &Pitch) -> i32 {
    let from_letter = from.name.letter as i8;
    let to_letter = to.name.letter as i8;

    // Diatonic interval in scale degrees (0 = unison, 1 = 2nd, ..., 6 = 7th)
    let letter_diff = (to_letter - from_letter).rem_euclid(7);

    // Chromatic semitone difference
    let semitone_diff = to.midi_number() - from.midi_number();

    // Now classify based on semitone + letter_diff
    // This table is based on common intervals in Western tonal music
    match (letter_diff, semitone_diff) {
        (0, 0) => 0,             // Perfect unison
        (1, 1) | (6, -1) => 0,   // Minor 2nd
        (1, 2) | (6, -2) => 0,   // Major 2nd
        (2, 3) | (5, -3) => 0,   // Minor 3rd
        (2, 4) | (5, -4) => 0,   // Major 3rd
        (3, 5) | (4, -5) => 0,   // Perfect 4th
        (4, 6) | (3, -6) => 1,   // Tritone (aug 4th or dim 5th)
        (4, 7) | (3, -7) => 0,   // Perfect 5th
        (5, 8) | (2, -8) => 0,   // Minor 6th
        (5, 9) | (2, -9) => 0,   // Major 6th
        (6, 10) | (1, -10) => 0, // Minor 7th
        (6, 11) | (1, -11) => 0, // Major 7th
        (0, 12) | (0, -12) => 0, // Perfect octave

        // Mildly weird stuff
        (_, 6) | (_, -6) => 1,     // Tritone variants
        (_, 1 | -1 | 2 | -2) => 2, // Aug 1st, dim 2nd, etc.
        _ => 4,                    // Rare or ugly intervals
    }
}

fn expected_letter(from: Letter, semitone_offset: i8) -> Letter {
    let direction = if semitone_offset >= 0 { 1 } else { -1 };
    let steps = match semitone_offset.abs() {
        0 => 0,
        1 | 2 => 1,
        3 | 4 => 2,
        5 => 3,
        6 | 7 => 4,
        8 | 9 => 5,
        10 | 11 => 6,
        _ => 7,
    };

    let mut letter = from;
    for _ in 0..steps {
        letter = if direction > 0 {
            letter.next()
        } else {
            letter.prev()
        };
    }

    letter
}

fn flat_vs_sharp_bias(accidental: Accidental, semitone_offset: i8) -> i32 {
    match (semitone_offset, accidental) {
        (x, Accidental::Flat | Accidental::DoubleFlat) if x < 0 => 0, // prefer flats when going down
        (x, Accidental::Sharp | Accidental::DoubleSharp) if x > 0 => 0, // prefer sharps when going up
        (_, Accidental::Natural) => 0,                                  // neutral
        _ => 2, // penalize using the "wrong" side
    }
}
