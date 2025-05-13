use std::fmt;
use std::ops::{Add, AddAssign};

use crate::error::ParseError;
use super::{NoteName, Letter, Accidental};

/// A specific pitch with both note name and octave
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pitch {
    name: NoteName,
    octave: i8,
}

impl Pitch {
    pub fn new(name: NoteName, octave: i8) -> Self {
        Pitch { name, octave }
    }

    /// Returns the full MIDI note number for this pitch
    pub fn midi_number(&self) -> i8 {
        // MIDI octaves start at -2, where C-2 is note 0
        self.name.base_midi_number() + ((self.octave + 2) * 12)
    }
    
    /// Checks if two pitches represent the same frequency
    pub fn is_enharmonic_with(&self, other: &Self) -> bool {
        self.midi_number() == other.midi_number()
    }

    // More musically aware transposition
    pub fn transpose(&self, semitones: i8) -> Self {
        // Basic implementation for small positive transpositions
        if semitones > 0 && semitones <= 2 {
            let letter = self.name.letter;
            let next_letter = letter.next();
            let current_semitones = next_letter.base_midi_number() - letter.base_midi_number();
            let remaining = semitones - current_semitones;
            
            let new_accidental = match remaining {
                -1 => Accidental::Flat,
                0 => Accidental::Natural,
                1 => Accidental::Sharp,
                _ => unreachable!(),  // Simplified for this example
            };
            
            return Pitch::new(NoteName::new(next_letter, new_accidental), self.octave);
        }
        
        // Fall back to standard implementation for other cases
        *self + semitones
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
        
        Pitch::new(NoteName::new(letter, accidental), octave)
    }
}

impl AddAssign<i8> for Pitch {
    fn add_assign(&mut self, semitones: i8) {
        *self = *self + semitones;
    }
}
