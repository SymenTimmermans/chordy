use core::fmt;
use std::str::FromStr;

use crate::error::ParseError;

use super::{NoteName, Pitch};

/// Represents a musical interval with quality and size
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Interval {
    /// Interval quality (Perfect, Major, Minor, etc.)
    pub quality: IntervalQuality,

    /// Interval size (Unison, Second, Third, etc.)
    pub size: IntervalSize,

    /// Direction (ascending or descending)
    pub direction: IntervalDirection,
}

/// Represents interval quality (Perfect, Major, Minor, etc.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IntervalQuality {
    Perfect,
    Major,
    Minor,
    Augmented(u8),  // For single, double, triple augmented (1, 2, 3)
    Diminished(u8), // For single, double, triple diminished (1, 2, 3)
}

/// Represents interval size in staff positions (second, third, etc.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum IntervalSize {
    Unison,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Octave,
    Ninth,
    Tenth,
    Eleventh,
    Twelfth,
    Thirteenth,
    // Compound intervals beyond this can be represented as Octave + n
}

impl IntervalSize {
    pub fn letter_steps(&self) -> u8 {
        match self {
            IntervalSize::Unison => 0,
            IntervalSize::Second => 1,
            IntervalSize::Third => 2,
            IntervalSize::Fourth => 3,
            IntervalSize::Fifth => 4,
            IntervalSize::Sixth => 5,
            IntervalSize::Seventh => 6,
            IntervalSize::Octave => 7,
            IntervalSize::Ninth => 8,
            IntervalSize::Tenth => 9,
            IntervalSize::Eleventh => 10,
            IntervalSize::Twelfth => 11,
            IntervalSize::Thirteenth => 12,
        }
    }
}

/// Direction of the interval
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IntervalDirection {
    Ascending,
    Descending,
}

impl Interval {
    /// Create a new interval with the given quality, size, and direction
    pub fn new(quality: IntervalQuality, size: IntervalSize, direction: IntervalDirection) -> Self {
        Interval { quality, size, direction }
    }
    
    /// Create an ascending perfect interval
    pub fn perfect(size: IntervalSize) -> Self {
        // Only valid for unison, fourth, fifth, octave, and compound equivalents
        assert!(Self::is_perfect_size(size), "Cannot create perfect interval of size: {:?}", size);
        Interval::new(IntervalQuality::Perfect, size, IntervalDirection::Ascending)
    }
    
    /// Create an ascending major interval
    pub fn major(size: IntervalSize) -> Self {
        // Only valid for second, third, sixth, seventh, and compound equivalents
        assert!(!Self::is_perfect_size(size) || size == IntervalSize::Unison, 
               "Cannot create major interval of size: {:?}", size);
        Interval::new(IntervalQuality::Major, size, IntervalDirection::Ascending)
    }
    
    /// Create an ascending minor interval
    pub fn minor(size: IntervalSize) -> Self {
        // Only valid for second, third, sixth, seventh, and compound equivalents
        assert!(!Self::is_perfect_size(size) || size == IntervalSize::Unison, 
               "Cannot create minor interval of size: {:?}", size);
        Interval::new(IntervalQuality::Minor, size, IntervalDirection::Ascending)
    }
    
    /// Create an ascending augmented interval
    pub fn augmented(size: IntervalSize) -> Self {
        Interval::new(IntervalQuality::Augmented(1), size, IntervalDirection::Ascending)
    }
    
    /// Create an ascending diminished interval
    pub fn diminished(size: IntervalSize) -> Self {
        Interval::new(IntervalQuality::Diminished(1), size, IntervalDirection::Ascending)
    }
    
    /// Helper to check if an interval size can be perfect
    fn is_perfect_size(size: IntervalSize) -> bool {
        matches!(size, 
            IntervalSize::Unison | 
            IntervalSize::Fourth | 
            IntervalSize::Fifth | 
            IntervalSize::Octave |
            IntervalSize::Eleventh |
            IntervalSize::Twelfth)
    }

    /// Get the number of semitones in this interval
    pub fn semitones(&self) -> i8 {
        let semitones = match (self.quality, self.size) {
            // Perfect intervals
            (IntervalQuality::Perfect, IntervalSize::Unison) => 0,
            (IntervalQuality::Perfect, IntervalSize::Fourth) => 5,
            (IntervalQuality::Perfect, IntervalSize::Fifth) => 7,
            (IntervalQuality::Perfect, IntervalSize::Octave) => 12,
            (IntervalQuality::Perfect, IntervalSize::Eleventh) => 17, // P4 + Octave
            (IntervalQuality::Perfect, IntervalSize::Twelfth) => 19,  // P5 + Octave
            
            // Major intervals
            (IntervalQuality::Major, IntervalSize::Second) => 2,
            (IntervalQuality::Major, IntervalSize::Third) => 4,
            (IntervalQuality::Major, IntervalSize::Sixth) => 9,
            (IntervalQuality::Major, IntervalSize::Seventh) => 11,
            (IntervalQuality::Major, IntervalSize::Ninth) => 14,      // M2 + Octave
            (IntervalQuality::Major, IntervalSize::Tenth) => 16,      // M3 + Octave
            (IntervalQuality::Major, IntervalSize::Thirteenth) => 21, // M6 + Octave
            
            // Minor intervals
            (IntervalQuality::Minor, IntervalSize::Second) => 1,
            (IntervalQuality::Minor, IntervalSize::Third) => 3,
            (IntervalQuality::Minor, IntervalSize::Sixth) => 8,
            (IntervalQuality::Minor, IntervalSize::Seventh) => 10,
            (IntervalQuality::Minor, IntervalSize::Ninth) => 13,      // m2 + Octave
            (IntervalQuality::Minor, IntervalSize::Tenth) => 15,      // m3 + Octave
            (IntervalQuality::Minor, IntervalSize::Thirteenth) => 20, // m6 + Octave
            
            // Augmented intervals
            (IntervalQuality::Augmented(n), size) => {
                let base = match size {
                    IntervalSize::Unison => 0,
                    IntervalSize::Second => 2,
                    IntervalSize::Third => 4,
                    IntervalSize::Fourth => 5,
                    IntervalSize::Fifth => 7,
                    IntervalSize::Sixth => 9,
                    IntervalSize::Seventh => 11,
                    IntervalSize::Octave => 12,
                    // Handle compound intervals
                    _ => self.decompose_compound().semitones(),
                };
                base + n as i8
            },
            
            // Diminished intervals
            (IntervalQuality::Diminished(n), size) => {
                let base = match size {
                    IntervalSize::Unison => 0,
                    IntervalSize::Second => 1,
                    IntervalSize::Third => 3,
                    IntervalSize::Fourth => 5,
                    IntervalSize::Fifth => 7,
                    IntervalSize::Sixth => 8,
                    IntervalSize::Seventh => 10,
                    IntervalSize::Octave => 12,
                    // Handle compound intervals
                    _ => self.decompose_compound().semitones(),
                };
                base - n as i8
            },
            
            // Handle any remaining combinations
            _ => self.decompose_compound().semitones(),
        };
        
        match self.direction {
            IntervalDirection::Ascending => semitones,
            IntervalDirection::Descending => -semitones,
        }
    }
    
    /// For compound intervals, decompose into simple interval + octaves
    fn decompose_compound(&self) -> Self {
        // Implementation for handling compound intervals
        todo!()
    }

    /// Create an interval from two note names (without octave info)
    pub fn between_notes(from: &NoteName, to: &NoteName) -> Self {
        // This will use letter distance for size and accidentals for quality
        let from_letter_idx = from.letter as u8;
        let to_letter_idx = to.letter as u8;
        
        // Calculate size based on letter distance
        let letter_distance = (to_letter_idx + 7 - from_letter_idx) % 7;
        let size = match letter_distance {
            0 => IntervalSize::Unison,
            1 => IntervalSize::Second,
            2 => IntervalSize::Third,
            3 => IntervalSize::Fourth,
            4 => IntervalSize::Fifth,
            5 => IntervalSize::Sixth,
            6 => IntervalSize::Seventh,
            _ => unreachable!(),
        };
        
        // Calculate semitones
        let from_base = from.base_midi_number();
        let to_base = to.base_midi_number();
        
        // Handle octave wrapping
        let raw_semitones = (to_base - from_base + 12) % 12;
        
        // Determine quality based on size and semitones
        let quality = Self::quality_from_semitones_and_size(raw_semitones, size);
        
        Interval::new(quality, size, IntervalDirection::Ascending)
    }
    
    /// Create an interval from two pitches (with octave info)
    pub fn between_pitches(from: &Pitch, to: &Pitch) -> Self {
        let note_interval = Self::between_notes(&from.name, &to.name);
        
        // Handle the octave component
        let octave_diff = to.octave - from.octave;
        
        // Adjust interval size based on octave difference
        // Implementation details would go here
        
        todo!()
    }
    
    /// Helper to determine quality from semitones and size
    fn quality_from_semitones_and_size(semitones: i8, size: IntervalSize) -> IntervalQuality {
        match (size, semitones) {
            // Perfect intervals
            (IntervalSize::Unison, 0) => IntervalQuality::Perfect,
            (IntervalSize::Fourth, 5) => IntervalQuality::Perfect,
            (IntervalSize::Fifth, 7) => IntervalQuality::Perfect,
            (IntervalSize::Octave, 0) => IntervalQuality::Perfect,
            
            // Major intervals
            (IntervalSize::Second, 2) => IntervalQuality::Major,
            (IntervalSize::Third, 4) => IntervalQuality::Major,
            (IntervalSize::Sixth, 9) => IntervalQuality::Major,
            (IntervalSize::Seventh, 11) => IntervalQuality::Major,
            
            // Minor intervals
            (IntervalSize::Second, 1) => IntervalQuality::Minor,
            (IntervalSize::Third, 3) => IntervalQuality::Minor,
            (IntervalSize::Sixth, 8) => IntervalQuality::Minor,
            (IntervalSize::Seventh, 10) => IntervalQuality::Minor,
            
            // Augmented
            (IntervalSize::Unison, 1) => IntervalQuality::Augmented(1),
            (IntervalSize::Second, 3) => IntervalQuality::Augmented(1),
            (IntervalSize::Third, 5) => IntervalQuality::Augmented(1),
            (IntervalSize::Fourth, 6) => IntervalQuality::Augmented(1),
            (IntervalSize::Fifth, 8) => IntervalQuality::Augmented(1),
            (IntervalSize::Sixth, 10) => IntervalQuality::Augmented(1),
            (IntervalSize::Seventh, 12) => IntervalQuality::Augmented(1),
            
            // Diminished
            (IntervalSize::Second, 0) => IntervalQuality::Diminished(1),
            (IntervalSize::Third, 2) => IntervalQuality::Diminished(1),
            (IntervalSize::Fourth, 4) => IntervalQuality::Diminished(1),
            (IntervalSize::Fifth, 6) => IntervalQuality::Diminished(1),
            (IntervalSize::Sixth, 7) => IntervalQuality::Diminished(1),
            (IntervalSize::Seventh, 9) => IntervalQuality::Diminished(1),
            (IntervalSize::Octave, 11) => IntervalQuality::Diminished(1),
            
            // Handle additional cases for multiple augmentations/diminutions
            // This would be expanded with more cases
            
            _ => panic!("Unhandled interval: {:?} with {} semitones", size, semitones),
        }
    }

    /// Invert the interval
    pub fn invert(&self) -> Self {
        let new_size = match self.size {
            IntervalSize::Unison => IntervalSize::Octave,
            IntervalSize::Second => IntervalSize::Seventh,
            IntervalSize::Third => IntervalSize::Sixth,
            IntervalSize::Fourth => IntervalSize::Fifth,
            IntervalSize::Fifth => IntervalSize::Fourth,
            IntervalSize::Sixth => IntervalSize::Third,
            IntervalSize::Seventh => IntervalSize::Second,
            IntervalSize::Octave => IntervalSize::Unison,
            // Handle compound intervals
            _ => todo!(),
        };
        
        let new_quality = match self.quality {
            IntervalQuality::Perfect => IntervalQuality::Perfect,
            IntervalQuality::Major => IntervalQuality::Minor,
            IntervalQuality::Minor => IntervalQuality::Major,
            IntervalQuality::Augmented(n) => IntervalQuality::Diminished(n),
            IntervalQuality::Diminished(n) => IntervalQuality::Augmented(n),
        };
        
        Interval::new(new_quality, new_size, self.direction)
    }
    
    /// Return the simple version of a compound interval
    pub fn simple(&self) -> Self {
        match self.size {
            // Simple intervals stay the same
            IntervalSize::Unison |
            IntervalSize::Second |
            IntervalSize::Third |
            IntervalSize::Fourth |
            IntervalSize::Fifth |
            IntervalSize::Sixth |
            IntervalSize::Seventh |
            IntervalSize::Octave => *self,
            
            // Compound intervals get reduced
            IntervalSize::Ninth => Interval::new(self.quality, IntervalSize::Second, self.direction),
            IntervalSize::Tenth => Interval::new(self.quality, IntervalSize::Third, self.direction),
            IntervalSize::Eleventh => Interval::new(self.quality, IntervalSize::Fourth, self.direction),
            IntervalSize::Twelfth => Interval::new(self.quality, IntervalSize::Fifth, self.direction),
            IntervalSize::Thirteenth => Interval::new(self.quality, IntervalSize::Sixth, self.direction),
        }
    }
    
    /// Apply this interval to a note to get another note
    pub fn apply_to(&self, note: &NoteName) -> NoteName {
        // Implementation would handle proper letter naming and accidentals
        todo!()
    }
    
    /// Combine two intervals
    pub fn add(&self, other: &Interval) -> Interval {
        // Implementation would combine intervals musically
        todo!()
    }

    /// Multiply an interval by a factor (e.g., 2 octaves)
    pub fn multiply(&self, factor: u8) -> Self {
        let mut result = *self;
        for _ in 1..factor {
            result = result.add(self);
        }
        result
    }

    /// Get the number of letter steps this interval spans
    pub fn letter_steps(&self) -> u8 {
        match self.size {
            IntervalSize::Unison => 0,
            IntervalSize::Second => 1,
            IntervalSize::Third => 2,
            IntervalSize::Fourth => 3,
            IntervalSize::Fifth => 4,
            IntervalSize::Sixth => 5,
            IntervalSize::Seventh => 6,
            IntervalSize::Octave => 7,
            IntervalSize::Ninth => 8,
            IntervalSize::Tenth => 9,
            IntervalSize::Eleventh => 10,
            IntervalSize::Twelfth => 11,
            IntervalSize::Thirteenth => 12,
        }
    }

    /// Common interval constants for easy use
    pub const UNISON: Interval = Interval {
        quality: IntervalQuality::Perfect,
        size: IntervalSize::Unison,
        direction: IntervalDirection::Ascending,
    };
    
    pub const MINOR_SECOND: Interval = Interval {
        quality: IntervalQuality::Minor,
        size: IntervalSize::Second,
        direction: IntervalDirection::Ascending,
    };
    
    pub const MAJOR_SECOND: Interval = Interval {
        quality: IntervalQuality::Major,
        size: IntervalSize::Second,
        direction: IntervalDirection::Ascending,
    };
    
    pub const MINOR_THIRD: Interval = Interval {
        quality: IntervalQuality::Minor,
        size: IntervalSize::Third,
        direction: IntervalDirection::Ascending,
    };
    
    pub const MAJOR_THIRD: Interval = Interval {
        quality: IntervalQuality::Major,
        size: IntervalSize::Third,
        direction: IntervalDirection::Ascending,
    };
    
    pub const PERFECT_FOURTH: Interval = Interval {
        quality: IntervalQuality::Perfect,
        size: IntervalSize::Fourth,
        direction: IntervalDirection::Ascending,
    };
    
    pub const TRITONE: Interval = Interval {
        quality: IntervalQuality::Augmented(1),
        size: IntervalSize::Fourth,
        direction: IntervalDirection::Ascending,
    };

    pub const DIMINISHED_FIFTH: Interval = Interval {
        quality: IntervalQuality::Diminished(1),
        size: IntervalSize::Fifth,
        direction: IntervalDirection::Ascending,
    };
    
    pub const PERFECT_FIFTH: Interval = Interval {
        quality: IntervalQuality::Perfect,
        size: IntervalSize::Fifth,
        direction: IntervalDirection::Ascending,
    };

    pub const AUGMENTED_FIFTH: Interval = Interval {
        quality: IntervalQuality::Augmented(1),
        size: IntervalSize::Fifth,
        direction: IntervalDirection::Ascending,
    };
    
    pub const MINOR_SIXTH: Interval = Interval {
        quality: IntervalQuality::Minor,
        size: IntervalSize::Sixth,
        direction: IntervalDirection::Ascending,
    };
    
    pub const MAJOR_SIXTH: Interval = Interval {
        quality: IntervalQuality::Major,
        size: IntervalSize::Sixth,
        direction: IntervalDirection::Ascending,
    };
    
    pub const MINOR_SEVENTH: Interval = Interval {
        quality: IntervalQuality::Minor,
        size: IntervalSize::Seventh,
        direction: IntervalDirection::Ascending,
    };
    
    pub const MAJOR_SEVENTH: Interval = Interval {
        quality: IntervalQuality::Major,
        size: IntervalSize::Seventh,
        direction: IntervalDirection::Ascending,
    };
    
    pub const OCTAVE: Interval = Interval {
        quality: IntervalQuality::Perfect,
        size: IntervalSize::Octave,
        direction: IntervalDirection::Ascending,
    };
}

impl FromStr for Interval {
    type Err = ParseError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Parse interval strings like "P5", "M3", "d7", "A4", etc.
        if s.len() < 2 {
            return Err(ParseError::InvalidInterval(s.to_string()));
        }
        
        let quality_char = s.chars().next().unwrap();
        let size_str = &s[1..];
        
        let quality = match quality_char {
            'P' => IntervalQuality::Perfect,
            'M' => IntervalQuality::Major,
            'm' => IntervalQuality::Minor,
            'A' => IntervalQuality::Augmented(1),
            'd' => IntervalQuality::Diminished(1),
            _ => return Err(ParseError::InvalidInterval(s.to_string())),
        };
        
        let size = match size_str.parse::<u8>() {
            Ok(1) => IntervalSize::Unison,
            Ok(2) => IntervalSize::Second,
            Ok(3) => IntervalSize::Third,
            Ok(4) => IntervalSize::Fourth,
            Ok(5) => IntervalSize::Fifth,
            Ok(6) => IntervalSize::Sixth,
            Ok(7) => IntervalSize::Seventh,
            Ok(8) => IntervalSize::Octave,
            Ok(9) => IntervalSize::Ninth,
            Ok(10) => IntervalSize::Tenth,
            Ok(11) => IntervalSize::Eleventh,
            Ok(12) => IntervalSize::Twelfth,
            Ok(13) => IntervalSize::Thirteenth,
            _ => return Err(ParseError::InvalidInterval(s.to_string())),
        };
        
        // Validate that the quality and size are compatible
        if quality == IntervalQuality::Perfect && !Self::is_perfect_size(size) {
            return Err(ParseError::InvalidInterval(s.to_string()));
        }
        
        Ok(Interval::new(quality, size, IntervalDirection::Ascending))
    }
}

impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let quality_str = match self.quality {
            IntervalQuality::Perfect => "P",
            IntervalQuality::Major => "M",
            IntervalQuality::Minor => "m",
            IntervalQuality::Augmented(1) => "A",
            IntervalQuality::Diminished(1) => "d",
            IntervalQuality::Augmented(n) => {
                // Multiple augmentations (e.g., "AA" for doubly augmented)
                write!(f, "{}", "A".repeat(n as usize))?;
                ""
            },
            IntervalQuality::Diminished(n) => {
                // Multiple diminutions (e.g., "dd" for doubly diminished)
                write!(f, "{}", "d".repeat(n as usize))?;
                ""
            },
        };
        
        let size_num = match self.size {
            IntervalSize::Unison => 1,
            IntervalSize::Second => 2,
            IntervalSize::Third => 3,
            IntervalSize::Fourth => 4,
            IntervalSize::Fifth => 5,
            IntervalSize::Sixth => 6,
            IntervalSize::Seventh => 7,
            IntervalSize::Octave => 8,
            IntervalSize::Ninth => 9,
            IntervalSize::Tenth => 10,
            IntervalSize::Eleventh => 11,
            IntervalSize::Twelfth => 12,
            IntervalSize::Thirteenth => 13,
        };
        
        write!(f, "{}{}", quality_str, size_num)
    }
}
