use std::fmt;
use std::ops::{Add, Sub};
use std::str::FromStr;

use super::{Accidental, Interval, Letter, Pitch};
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
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NoteName(i8);

impl NoteName {
    /// Creates a new `NoteName` from a letter and accidental.
    pub fn new(letter: Letter, accidental: Accidental) -> Self {
        use crate::types::Letter::*;

        // Calculate the base fifths position for this letter
        let base_fifths = match letter {
            F => -1,
            C => 0,
            G => 1,
            D => 2,
            A => 3,
            E => 4,
            B => 5,
        };

        // Calculate the final fifths position
        let fifths = base_fifths + accidental.semitone_offset() * 7;
        Self(fifths)
    }

    /// Creates a new `NoteName` from a number of fifths.
    pub fn from_fifths(fifths: i8) -> Self {
        Self(fifths)
    }

    /// Returns the number of fifths represented by this note name.
    pub fn fifths(&self) -> i8 {
        self.0
    }

    /// Returns the letter of this note name.
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

    /// Returns the accidental of this note name.
    pub fn accidental(&self) -> Accidental {
        // Calculate the base fifths position for this letter
        let base_fifths = match self.letter() {
            Letter::F => -1,
            Letter::C => 0,
            Letter::G => 1,
            Letter::D => 2,
            Letter::A => 3,
            Letter::E => 4,
            Letter::B => 5,
        };

        // Calculate the accidental level (how many sharps/flats beyond natural)
        let accidental_level = (self.0 - base_fifths) / 7;

        match accidental_level {
            -2 => Accidental::DoubleFlat,
            -1 => Accidental::Flat,
            0 => Accidental::Natural,
            1 => Accidental::Sharp,
            2 => Accidental::DoubleSharp,
            level => Accidental::Extreme(level),
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

    /// Transpose this note by a certain interval
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

    /// Returns the HTML representation of the note name with accidentals as superscripts
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{NoteName, Letter, Accidental};
    /// let c_sharp = NoteName::new(Letter::C, Accidental::Sharp);
    /// assert_eq!(c_sharp.to_html(), "C<sup>&sharp;</sup>");
    ///
    /// let b_flat = NoteName::new(Letter::B, Accidental::Flat);
    /// assert_eq!(b_flat.to_html(), "B<sup>&flat;</sup>");
    ///
    /// let d_natural = NoteName::new(Letter::D, Accidental::Natural);
    /// assert_eq!(d_natural.to_html(), "D");
    /// ```
    pub fn to_html(&self) -> String {
        use crate::types::chord::naming::ChordFormat;
        match self.accidental() {
            Accidental::Natural => self.letter().to_string(),
            Accidental::Extreme(0) => self.letter().to_string(), // Extreme with 0 is natural
            acc => format!(
                "{}<sup>{}</sup>",
                self.letter(),
                acc.render_for_format(ChordFormat::Html)
            ),
        }
    }
}

// Torsor action: Note + Interval → Note
impl Add<Interval> for NoteName {
    type Output = Self;

    fn add(self, interval: Interval) -> Self {
        Self(self.0 + interval.fifths)
    }
}

impl Sub<Interval> for NoteName {
    type Output = Self;

    fn sub(self, interval: Interval) -> Self {
        Self(self.0 - interval.fifths)
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

impl fmt::Debug for NoteName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
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
    /// use chordy::prelude::FromStr;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extreme_transpositions_no_panic() {
        let c = NoteName::new(Letter::C, Accidental::Natural);

        // Test multiple tritone transpositions that would previously panic with invalid fifths value 24
        let extreme_note = c
            + Interval::AUGMENTED_FOURTH
            + Interval::AUGMENTED_FOURTH
            + Interval::AUGMENTED_FOURTH
            + Interval::AUGMENTED_FOURTH;

        // Should not panic - this is the key fix
        let accidental = extreme_note.accidental();
        assert!(matches!(accidental, Accidental::Extreme(_)));

        // The note should display with repeated sharp symbols
        let display = extreme_note.to_string();
        assert!(display.contains("♯"));
    }

    #[test]
    fn test_extreme_accidental_display() {
        let triple_sharp = NoteName::new(Letter::C, Accidental::Extreme(3));
        let display = triple_sharp.to_string();
        assert_eq!(display.chars().filter(|&c| c == '♯').count(), 3);

        let quad_flat = NoteName::new(Letter::F, Accidental::Extreme(-4));
        let display = quad_flat.to_string();
        assert_eq!(display.chars().filter(|&c| c == '♭').count(), 4);
    }
}
