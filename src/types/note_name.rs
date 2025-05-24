use std::fmt;
use std::str::FromStr;

use super::{
    key::KeySignature, Accidental, Interval, IntervalDirection, IntervalSize, Letter, Pitch,
};
use crate::error::ParseError;

/// Represents a musical note name with a letter and accidental
///
/// # Examples
///
/// ```
/// use chordy::NoteName;
/// use chordy::{Letter, Accidental};
///
/// let c_sharp = NoteName::new(Letter::C, Accidental::Sharp);
/// assert_eq!(c_sharp.to_string(), "C♯");
///
/// // Enharmonic equivalence check
/// let d_flat = NoteName::new(Letter::D, Accidental::Flat);
/// assert!(c_sharp.is_enharmonic_with(&d_flat));
/// assert_ne!(c_sharp, d_flat); // Different note names
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NoteName {
    pub letter: Letter,
    pub accidental: Accidental,
}

impl NoteName {
    pub fn new(letter: Letter, accidental: Accidental) -> Self {
        NoteName { letter, accidental }
    }

    /// Returns the MIDI note number for this note name in octave 0
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{NoteName, Letter, Accidental};
    /// let c_natural = NoteName::new(Letter::C, Accidental::Natural);
    /// assert_eq!(c_natural.base_midi_number(), 0);
    ///
    /// let f_sharp = NoteName::new(Letter::F, Accidental::Sharp);
    /// assert_eq!(f_sharp.base_midi_number(), 6);
    ///
    /// let b_flat = NoteName::new(Letter::B, Accidental::Flat);
    /// assert_eq!(b_flat.base_midi_number(), 10);
    /// ```
    pub fn base_midi_number(&self) -> i8 {
        self.letter.base_midi_number() + self.accidental.semitone_offset()
    }

    /// Checks if two note names are enharmonically equivalent
    pub fn is_enharmonic_with(&self, other: &Self) -> bool {
        // Notes are enharmonically equivalent if they represent the same pitch
        (self.base_midi_number() + 12) % 12 == (other.base_midi_number() + 12) % 12
    }

    /// Apply an interval to this note to get a new note with correct spelling
    pub fn apply_interval(&self, interval: Interval) -> Self {
        // First, determine the target letter based on interval size
        let target_letter = self.get_letter_at_interval(interval.size);

        // Calculate semitones in the interval
        let semitones = interval.semitones();

        // Calculate the correct accidental for the target note
        self.note_with_interval_to(target_letter, semitones)
    }

    /// Returns the letter that is the given interval size away from this note's letter
    fn get_letter_at_interval(&self, size: IntervalSize) -> Letter {
        // Apply the letter steps
        let letter_value = (self.letter as u8 + size.letter_steps()) % 7;
        match letter_value {
            0 => Letter::C,
            1 => Letter::D,
            2 => Letter::E,
            3 => Letter::F,
            4 => Letter::G,
            5 => Letter::A,
            6 => Letter::B,
            _ => unreachable!(),
        }
    }

    /// Creates a note with the specified letter that is the given number of semitones away
    fn note_with_interval_to(&self, target_letter: Letter, semitones: i8) -> Self {
        // Get the base MIDI values as if both notes were natural
        let self_natural_base = self.letter.base_midi_number();
        let target_natural_base = target_letter.base_midi_number();

        // Account for the current note's accidental
        let actual_self_value = self_natural_base + self.accidental.semitone_offset();

        // Calculate what the target value should be based on the requested interval
        let target_value = (actual_self_value + semitones) % 12;

        // Calculate how many semitones need to be added/subtracted with an accidental
        // to get from the natural target letter to the desired pitch
        let natural_target_mod12 = target_natural_base % 12;
        let adjustment = (target_value - natural_target_mod12 + 12) % 12;

        // Determine the correct accidental
        let accidental = match adjustment {
            0 => Accidental::Natural,
            1 => Accidental::Sharp,
            2 => Accidental::DoubleSharp,
            11 => Accidental::Flat,
            10 => Accidental::DoubleFlat,
            _ => panic!(
                "Cannot represent adjustment of {} semitones with simple accidentals",
                adjustment
            ),
        };

        NoteName::new(target_letter, accidental)
    }

    pub fn transpose_by_interval(&self, interval: Interval, key_sig: &KeySignature) -> NoteName {
        // Calculate target letter
        let mut target_letter = self.letter;
        let steps = interval.letter_steps();

        for _ in 0..steps {
            target_letter = if interval.direction == IntervalDirection::Ascending {
                target_letter.next()
            } else {
                target_letter.prev()
            };
        }
        //
        // Get default accidental from key signature
        let default_accidental = key_sig.letter_map[target_letter as usize];

        // Calculate required semitone adjustment
        let current_semitones = self.base_midi_number() % 12;
        let interval_semitones = interval.semitones().abs() % 12;
        let target_semitones = if interval.direction == IntervalDirection::Ascending {
            (current_semitones + interval_semitones) % 12
        } else {
            (current_semitones - interval_semitones + 12) % 12
        };

        let default_semitones =
            (target_letter.base_midi_number() + default_accidental.semitone_offset()) % 12;
        let semitone_diff = (target_semitones - default_semitones + 12) % 12;

        // Determine needed accidental
        let accidental = match semitone_diff {
            0 => default_accidental,
            1 => Accidental::Sharp,
            2 => Accidental::DoubleSharp,
            10 => Accidental::DoubleFlat,
            11 => Accidental::Flat,
            _ => panic!("Invalid semitone difference for diatonic transposition"),
        };

        NoteName::new(target_letter, accidental)
    }

    /// Convert this NoteName to a Pitch at the given octave
    pub fn to_pitch(&self, octave: i8) -> Pitch {
        Pitch::new(self.letter, self.accidental, octave)
    }
}

impl fmt::Display for NoteName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.accidental {
            Accidental::Natural => write!(f, "{}", self.letter),
            _ => write!(f, "{}{}", self.letter, self.accidental),
        }
    }
}

impl FromStr for NoteName {
    type Err = ParseError;

    /// Parses a string into a NoteName, taking the first character to be a Letter, and the rest of
    /// the string to be an accidental.
    ///
    /// # Examples
    /// ```
    /// use chordy::{NoteName, Letter, Accidental};
    /// use chordy::prelude::traits::FromStr;
    ///
    /// let c_sharp = NoteName::from_str("C♯").unwrap();
    /// assert_eq!(c_sharp.letter, Letter::C);
    /// assert_eq!(c_sharp.accidental, Accidental::Sharp);
    ///
    /// let b_double_sharp = NoteName::from_str("B♯♯").unwrap();
    /// assert_eq!(b_double_sharp.letter, Letter::B);
    /// assert_eq!(b_double_sharp.accidental, Accidental::DoubleSharp);
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // The string should be at least 1 character long
        if s.is_empty() {
            return Err(ParseError::InvalidNoteName(s.to_string()));
        }

        let letter = Letter::from_str(&s[0..1])?;

        let accidental = if s.len() > 1 {
            Accidental::from_str(&s[1..])?
        } else {
            Accidental::Natural
        };

        Ok(NoteName::new(letter, accidental))
    }
}
