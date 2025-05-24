use std::fmt;
use std::str::FromStr;
use std::ops::{Add, Sub};

use super::{
    key::KeySignature, Accidental, Interval, Letter, Pitch,
};
use crate::error::ParseError;
use crate::traits::Torsor;

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
pub struct NoteName(i8);

impl NoteName {
    pub fn new(letter: Letter, accidental: Accidental) -> Self {
        use crate::types::Accidental::*;
        use crate::types::Letter::*;

        match (letter, accidental) {
            (F, DoubleFlat) => Self(-15),
            (C, DoubleFlat) => Self(-14),
            (G, DoubleFlat) => Self(-13),
            (D, DoubleFlat) => Self(-12),
            (A, DoubleFlat) => Self(-11),
            (E, DoubleFlat) => Self(-10),
            (B, DoubleFlat) => Self(-9),
            (F, Flat) => Self(-8),
            (C, Flat) => Self(-7),
            (G, Flat) => Self(-6),
            (D, Flat) => Self(-5),
            (A, Flat) => Self(-4),
            (E, Flat) => Self(-3),
            (B, Flat) => Self(-2),
            (F, Natural) => Self(-1),
            (C, Natural) => Self(0),
            (G, Natural) => Self(1),
            (D, Natural) => Self(2),
            (A, Natural) => Self(3),
            (E, Natural) => Self(4),
            (B, Natural) => Self(5),
            (F, Sharp) => Self(6),
            (C, Sharp) => Self(7),
            (G, Sharp) => Self(8),
            (D, Sharp) => Self(9),
            (A, Sharp) => Self(10),
            (E, Sharp) => Self(11),
            (B, Sharp) => Self(12),
            (F, DoubleSharp) => Self(13),
            (C, DoubleSharp) => Self(14),
            (G, DoubleSharp) => Self(15),
            (D, DoubleSharp) => Self(16),
            (A, DoubleSharp) => Self(17),
            (E, DoubleSharp) => Self(18),
            (B, DoubleSharp) => Self(19),
        }
    }

    pub fn from_fifths(fifths: i8) -> Self {
        Self(fifths)
    }
    
    pub fn fifths(&self) -> i8 {
        self.0
    }

    pub fn letter(&self) -> Letter {
        match (self.0 + 15) % 7 {
            0 => Letter::F,
            1 => Letter::C,
            2 => Letter::G,
            3 => Letter::D,
            4 => Letter::A,
            5 => Letter::E,
            6 => Letter::B,
            _ => unreachable!(),
        }
    }

    pub fn accidental(&self) -> Accidental {
        match self.0 {
            -15..-8 => Accidental::DoubleFlat,
            -8..-1 => Accidental::Flat,
            -1..6 => Accidental::Natural,
            6..13 => Accidental::Sharp,
            13..=19 => Accidental::DoubleSharp,
            _ => {
                panic!("Invalid NoteName value: {}", self.0);
            }
        }
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
        self.letter().base_midi_number() + self.accidental().semitone_offset()
    }

    /// Checks if two note names are enharmonically equivalent
    pub fn is_enharmonic_with(&self, other: &Self) -> bool {
        // Notes are enharmonically equivalent if they represent the same pitch
        (self.base_midi_number() + 12) % 12 == (other.base_midi_number() + 12) % 12
    }

    /// Returns the letter that is the given interval size away from this note's letter
    fn get_letter_at_interval(&self, steps: i8) -> Letter {
        // Apply the letter steps
        let letter_value = (self.letter() as i8 + steps) % 7;
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
        let self_natural_base = self.letter().base_midi_number();
        let target_natural_base = target_letter.base_midi_number();

        // Account for the current note's accidental
        let actual_self_value = self_natural_base + self.accidental().semitone_offset();

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

    pub fn transpose_by_interval(&self, interval: Interval) -> NoteName {
        *self + interval
    }

    /// Convert this NoteName to a Pitch at the given octave
    pub fn to_pitch(&self, octave: i8) -> Pitch {
        Pitch::new(self.letter(), self.accidental(), octave)
    }

    /// Transpose a note by an interval
    pub fn transpose(self, interval: Interval) -> Self {
        self.act(interval)
    }
    
    /// Find the interval between two notes
    pub fn interval_to(self, other: Self) -> Interval {
        other.difference(&self)
    }
}

// Torsor action: Note + Interval → Note
impl Add<Interval> for NoteName {
    type Output = Self;
    
    fn add(self, interval: Interval) -> Self {
        Self(self.0 + interval.fifths)
    }
}

// Torsor difference: Note - Note → Interval
impl Sub<NoteName> for NoteName {
    type Output = Interval;
    
    fn sub(self, other: NoteName) -> Interval {
        Interval::with_fifths(self.0 - other.0)
    }
}

impl Torsor<Interval> for NoteName {
    fn act(&self, interval: Interval) -> Self {
        *self + interval
    }
    
    fn difference(&self, other: &Self) -> Interval {
        *self - *other
    }
}

impl fmt::Display for NoteName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.accidental() {
            Accidental::Natural => write!(f, "{}", self.letter()),
            _ => write!(f, "{}{}", self.letter(), self.accidental()),
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
    /// assert_eq!(c_sharp.letter(), Letter::C);
    /// assert_eq!(c_sharp.accidental(), Accidental::Sharp);
    ///
    /// let b_double_sharp = NoteName::from_str("B♯♯").unwrap();
    /// assert_eq!(b_double_sharp.letter(), Letter::B);
    /// assert_eq!(b_double_sharp.accidental(), Accidental::DoubleSharp);
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
