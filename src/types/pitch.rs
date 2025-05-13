use std::fmt;
use std::ops::{Add, AddAssign};
use std::str::FromStr;

use crate::error::ParseError;

use super::{NoteName, Letter, Accidental};

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
            octave
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

    /// Transposes the pitch by the specified number of semitones,
    /// maintaining proper enharmonic spelling where possible.
    pub fn transpose(&self, semitones: i8) -> Self {
        // Calculate the new MIDI number
        let new_midi = self.midi_number() + semitones;
        
        // Handle octave wrapping
        let octave = (new_midi / 12) - 2;
        let semitone_in_octave = new_midi % 12;
        
        // Determine the best letter and accidental for the new pitch
        let (letter, accidental) = match semitone_in_octave {
            0 => (Letter::C, Accidental::Natural),
            1 => (self.name.letter.next(), Accidental::Flat),  // C♯/D♭
            2 => (Letter::D, Accidental::Natural),
            3 => (self.name.letter.next(), Accidental::Flat),  // D♯/E♭
            4 => (Letter::E, Accidental::Natural),
            5 => (Letter::F, Accidental::Natural),
            6 => (self.name.letter.next(), Accidental::Flat),  // F♯/G♭
            7 => (Letter::G, Accidental::Natural),
            8 => (self.name.letter.next(), Accidental::Flat),  // G♯/A♭
            9 => (Letter::A, Accidental::Natural),
            10 => (self.name.letter.next(), Accidental::Flat), // A♯/B♭
            11 => (Letter::B, Accidental::Natural),
            _ => unreachable!(),
        };
        
        // Special case: when transposing up a half step from B or E
        // we should prefer the next letter with flat rather than current letter with sharp
        if (self.name.letter == Letter::B || self.name.letter == Letter::E) 
            && semitones == 1 
            && self.name.accidental == Accidental::Natural {
            return Pitch::new(letter, Accidental::Natural, octave);
        }
        
        // Special case: when transposing down a half step from C or F
        // we should prefer the previous letter with sharp
        if (self.name.letter == Letter::C || self.name.letter == Letter::F) 
            && semitones == -1 
            && self.name.accidental == Accidental::Natural {
            return Pitch::new(self.name.letter, Accidental::Sharp, octave);
        }
        
        Pitch::new(letter, accidental, octave)
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
        // Calculate the new MIDI number
        let new_midi_number = self.midi_number() + semitones;
        
        // Convert back to a Pitch
        // This requires converting from MIDI number to letter+accidental+octave
        let octave = (new_midi_number / 12) - 1;
        let semitone_in_octave = new_midi_number % 12;
        
        // Map the semitone to a letter+accidental 
        // (This is a bit simplified - real implementation should handle enharmonic spellings)
        let (letter, accidental) = match semitone_in_octave {
            0 => (Letter::C, Accidental::Natural),
            1 => (Letter::C, Accidental::Sharp),
            2 => (Letter::D, Accidental::Natural),
            3 => (Letter::D, Accidental::Sharp),
            4 => (Letter::E, Accidental::Natural),
            5 => (Letter::F, Accidental::Natural),
            6 => (Letter::F, Accidental::Sharp),
            7 => (Letter::G, Accidental::Natural),
            8 => (Letter::G, Accidental::Sharp),
            9 => (Letter::A, Accidental::Natural),
            10 => (Letter::A, Accidental::Sharp),
            11 => (Letter::B, Accidental::Natural),
            _ => unreachable!(),
        };
        
        Pitch::new(letter, accidental, octave)
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
                .unwrap_or(0)
        );

        if note_part.is_empty() || octave_part.is_empty() {
            return Err(ParseError::InvalidPitch(s.to_string()));
        }

        // Parse octave allowing for negative numbers
        let octave = octave_part.parse::<i8>()
            .map_err(|_| ParseError::InvalidPitch(s.to_string()))?;

        // Parse note name
        let name = NoteName::from_str(note_part)?;

        Ok(Pitch { name, octave })
    }
}
