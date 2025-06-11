use core::fmt;
use std::str::FromStr;

use crate::error::ParseError;

use std::ops::{Add, Sub};

/// Intervals form a group - they can be added, subtracted, negated
/// This represents the "acting group" in our torsor
///
/// | −15 | −14 | −13 | −12 | −11 | −10 |  −9 |
/// |-----|-----|-----|-----|-----|-----|-----|
/// |  F𝄫 |  C𝄫 |  G𝄫 |  D𝄫 |  A𝄫 |  E𝄫 |  B𝄫 |
/// |  −8 |  −7 |  −6 |  −5 |  −4 |  −3 |  −2 |
/// |  F♭ |  C♭ |  G♭ |  D♭ |  A♭ |  E♭ |  B♭ |
/// |  −1 |   0 |   1 |   2 |   3 |   4 |   5 |
/// |   F |   C |   G |   D |   A |   E |   B |
/// |   6 |   7 |   8 |   9 |  10 |  11 |  12 |
/// |  F♯ |  C♯ |  G♯ |  D♯ |  A♯ |  E♯ |  B♯ |
/// |  13 |  14 |  15 |  16 |  17 |  18 |  19 |
/// |  F𝄪 |  C𝄪 |  G𝄪 |  D𝄪 |  A𝄪 |  E𝄪 |  B𝄪 |
///  
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Interval {
    /// The number of fifths (7 semitones) this interval spans.
    ///
    /// It is normalized to the range -12 to 12, any octave displacement is stored in `octaves`.
    pub fifths: i8,
    /// The number of octaves (12 semitones) this interval spans.
    pub octaves: i8,
}

#[rustfmt::skip]
#[allow(missing_docs)]
impl Interval {
    pub const PERFECT_UNISON: Self = Self { fifths: 0, octaves: 0};

    pub const DIMINISHED_SECOND: Self = Self { fifths: -12, octaves: 0};
    pub const MINOR_SECOND: Self = Self { fifths: -5, octaves: 0};
    pub const MAJOR_SECOND: Self = Self { fifths: 2, octaves: 0};
    pub const AUGMENTED_SECOND: Self = Self { fifths: 9, octaves: 0};

    pub const DIMINISHED_THIRD: Self = Self { fifths: -10, octaves: 0};
    pub const MINOR_THIRD: Self = Self { fifths: -3, octaves: 0};
    pub const MAJOR_THIRD: Self = Self { fifths: 4, octaves: 0};
    pub const AUGMENTED_THIRD: Self = Self { fifths: 11, octaves: 0};

    pub const DIMINISHED_FOURTH: Self = Self { fifths: -8, octaves: 0};
    pub const PERFECT_FOURTH: Self = Self { fifths: -1, octaves: 0};
    pub const AUGMENTED_FOURTH: Self = Self { fifths: 6, octaves: 0};

    pub const DIMINISHED_FIFTH: Self = Self { fifths: -6, octaves: 0};
    pub const PERFECT_FIFTH: Self = Self { fifths: 1, octaves: 0};
    pub const AUGMENTED_FIFTH: Self = Self { fifths: 8, octaves: 0};

    pub const DIMINISHED_SIXTH: Self = Self { fifths: -11, octaves: 0};
    pub const MINOR_SIXTH: Self = Self { fifths: -4, octaves: 0};
    pub const MAJOR_SIXTH: Self = Self { fifths: 3, octaves: 0};
    pub const AUGMENTED_SIXTH: Self = Self { fifths: 10, octaves: 0};

    pub const DIMINISHED_SEVENTH: Self = Self { fifths: -9, octaves: 0};
    pub const MINOR_SEVENTH: Self = Self { fifths: -2, octaves: 0};
    pub const MAJOR_SEVENTH: Self = Self { fifths: 5, octaves: 0};
    pub const AUGMENTED_SEVENTH: Self = Self { fifths: 12, octaves: 0};

    pub const DIMINISHED_OCTAVE: Self = Self { fifths: -7, octaves: 0};
    pub const OCTAVE: Self = Self { fifths: 0, octaves: 1};
    pub const AUGMENTED_OCTAVE: Self = Self { fifths: 7, octaves: 1};

    pub const DIMINISHED_NINTH: Self = Self { fifths: -12, octaves: 1};
    pub const MINOR_NINTH: Self = Self { fifths: -5, octaves: 1};
    pub const MAJOR_NINTH: Self = Self { fifths: 2, octaves: 1};
    pub const AUGMENTED_NINTH: Self = Self { fifths: 9, octaves: 1};

    pub const DIMINISHED_TENTH: Self = Self { fifths: -10, octaves: 1};
    pub const MINOR_TENTH: Self = Self { fifths: -3, octaves: 1};
    pub const MAJOR_TENTH: Self = Self { fifths: 4, octaves: 1};
    pub const AUGMENTED_TENTH: Self = Self { fifths: 11, octaves: 1};

    pub const DIMINISHED_ELEVENTH: Self = Self { fifths: -8, octaves: 1};
    pub const PERFECT_ELEVENTH: Self = Self { fifths: -1, octaves: 1};
    pub const AUGMENTED_ELEVENTH: Self = Self { fifths: 6, octaves: 1};

    pub const DIMINISHED_TWELFTH: Self = Self { fifths: -6, octaves: 1};
    pub const PERFECT_TWELFTH: Self = Self { fifths: 1, octaves: 1};
    pub const AUGMENTED_TWELFTH: Self = Self { fifths: 8, octaves: 1};

    pub const DIMINISHED_THIRTEENTH: Self = Self { fifths: -11, octaves: 1};
    pub const MINOR_THIRTEENTH: Self = Self { fifths: -4, octaves: 1};
    pub const MAJOR_THIRTEENTH: Self = Self { fifths: 3, octaves: 1};
    pub const AUGMENTED_THIRTEENTH: Self = Self { fifths: 10, octaves: 1};

    pub const DIMINISHED_FOURTEENTH: Self = Self { fifths: -9, octaves: 1};
    pub const MINOR_FOURTEENTH: Self = Self { fifths: -2, octaves: 1};
    pub const MAJOR_FOURTEENTH: Self = Self { fifths: 5, octaves: 1};
    pub const AUGMENTED_FOURTEENTH: Self = Self { fifths: 12, octaves: 1};
    

    /// Create a new interval with specified fifths and octaves.
    pub fn new(fifths: i8, octaves: i8) -> Self {
        Self { fifths, octaves }
    }

    /// Create a new interval with specified fifths, defaulting octaves to 0.
    pub fn with_fifths(fifths: i8) -> Self {
        Self { fifths, octaves: 0 }
    }
    
    /// These intervals are used to represent the "distance" between two pitches.
    pub fn fifths(&self) -> i8 {
        self.fifths
    }
    
    /// The number of octaves this interval spans.
    pub fn octaves(&self) -> i8 {
        self.octaves
    }

    /// The number of semitones this interval spans.
    pub fn semitones(&self) -> i8 {
        // Convert fifths to semitones, making sure it's positive.
        (((self.fifths * 7 % 12) + 12) % 12) + self.octaves * 12
    }

    /// Returns a new interval that represents the class of this interval, setting octaves to 0.
    pub fn interval_class(&self) -> Self {
        Self { fifths: self.fifths, octaves: 0 }
    }

    /// Calculate the generic interval number from fifths position
    /// This gives us the "letter distance" - how many letter names apart
    fn generic_interval_number(&self) -> i8 {
        // The magic formula: each fifth moves us 4 letter names
        // We need to account for octaves and normalize to 1-7 range
        let base_generic = ((self.fifths * 4) % 7 + 7) % 7;
        let octave_generics = self.octaves * 7;
        base_generic + octave_generics + 1  // +1 because intervals start at 1, not 0
    }

    /// Calculate the number of semitones this interval spans
    fn total_semitones(&self) -> i8 {
        (((self.fifths * 7) % 12 + 12) % 12) + (self.octaves * 12)
    }

    /// Convert interval number (1-14, etc.) to base fifths and octaves
    /// This gives the "major" or "perfect" version of each interval
    fn interval_number_to_fifths_and_octaves(number: u8) -> (i8, i8) {
        let octaves = ((number - 1) / 7) as i8;
        let base_number = ((number - 1) % 7) + 1;
        
        let base_fifths = match base_number {
            1 => 0,   // Unison (perfect)
            2 => 2,   // Second (major)
            3 => 4,   // Third (major)
            4 => -1,  // Fourth (perfect)
            5 => 1,   // Fifth (perfect)
            6 => 3,   // Sixth (major)
            7 => 5,   // Seventh (major)
            _ => unreachable!(),
        };
        
        (base_fifths, octaves)
    }
    
    /// Check if an interval number can be perfect (1, 4, 5, 8, 11, 12, etc.)
    fn can_be_perfect(number: u8) -> bool {
        let base_number = ((number - 1) % 7) + 1;
        matches!(base_number, 1 | 4 | 5)
    }

    /// Returns true if the interval is a fifth of some kind.
    pub fn is_fifth(&self) -> bool {
        *self == Self::PERFECT_FIFTH || 
        *self == Self::AUGMENTED_FIFTH || 
        *self == Self::DIMINISHED_FIFTH
    }

    /// Returns true if the interval is a third of some kind.
    pub fn is_third(&self) -> bool {
        *self == Self::MAJOR_THIRD || 
        *self == Self::MINOR_THIRD || 
        *self == Self::AUGMENTED_THIRD || 
        *self == Self::DIMINISHED_THIRD
    }

    /// Returns true if the interval is a seventh of some kind.
    pub fn is_seventh(&self) -> bool {
        *self == Self::DIMINISHED_SEVENTH ||
        *self == Self::MINOR_SEVENTH ||
        *self == Self::MAJOR_SEVENTH ||
        *self == Self::AUGMENTED_SEVENTH
    }
}

// Intervals form a group
impl Add for Interval {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            fifths: self.fifths + other.fifths,
            octaves: self.octaves + other.octaves,
        }
    }
}

impl Sub for Interval {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            fifths: self.fifths - other.fifths,
            octaves: self.octaves - other.octaves,
        }
    }
}

impl std::ops::Neg for Interval {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            fifths: -self.fifths,
            octaves: -self.octaves,
        }
    }
}

/// Quite the challenge.
impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            // Unison
            Self::PERFECT_UNISON => write!(f, "P1"),
            // Seconds
            Self::DIMINISHED_SECOND => write!(f, "d2"),
            Self::MINOR_SECOND => write!(f, "m2"),
            Self::MAJOR_SECOND => write!(f, "M2"),
            Self::AUGMENTED_SECOND => write!(f, "A2"),
            // Thirds
            Self::DIMINISHED_THIRD => write!(f, "d3"),
            Self::MINOR_THIRD => write!(f, "m3"),
            Self::MAJOR_THIRD => write!(f, "M3"),
            Self::AUGMENTED_THIRD => write!(f, "A3"),
            // Fourths
            Self::DIMINISHED_FOURTH => write!(f, "d4"),
            Self::PERFECT_FOURTH => write!(f, "P4"),
            Self::AUGMENTED_FOURTH => write!(f, "A4"),
            // Fifths
            Self::DIMINISHED_FIFTH => write!(f, "d5"),
            Self::PERFECT_FIFTH => write!(f, "P5"),
            Self::AUGMENTED_FIFTH => write!(f, "A5"),
            // Sixths
            Self::DIMINISHED_SIXTH => write!(f, "d6"),
            Self::MINOR_SIXTH => write!(f, "m6"),
            Self::MAJOR_SIXTH => write!(f, "M6"),
            Self::AUGMENTED_SIXTH => write!(f, "A6"),
            // Sevenths
            Self::DIMINISHED_SEVENTH => write!(f, "d7"),
            Self::MINOR_SEVENTH => write!(f, "m7"),
            Self::MAJOR_SEVENTH => write!(f, "M7"),
            Self::AUGMENTED_SEVENTH => write!(f, "A7"),
            // Octaves
            Self::DIMINISHED_OCTAVE => write!(f, "d8"),
            Self::OCTAVE => write!(f, "P8"),
            Self::AUGMENTED_OCTAVE => write!(f, "A8"),
            // Ninths
            Self::DIMINISHED_NINTH => write!(f, "d9"),
            Self::MINOR_NINTH => write!(f, "m9"),
            Self::MAJOR_NINTH => write!(f, "M9"),
            Self::AUGMENTED_NINTH => write!(f, "A9"),
            // Tenths
            Self::DIMINISHED_TENTH => write!(f, "d10"),
            Self::MINOR_TENTH => write!(f, "m10"),
            Self::MAJOR_TENTH => write!(f, "M10"),
            Self::AUGMENTED_TENTH => write!(f, "A10"),
            // Elevenths
            Self::DIMINISHED_ELEVENTH => write!(f, "d11"),
            Self::PERFECT_ELEVENTH => write!(f, "P11"),
            Self::AUGMENTED_ELEVENTH => write!(f, "A11"),
            // Twelfths
            Self::DIMINISHED_TWELFTH => write!(f, "d12"),
            Self::PERFECT_TWELFTH => write!(f, "P12"),
            Self::AUGMENTED_TWELFTH => write!(f, "A12"),
            // Thirteenths
            Self::DIMINISHED_THIRTEENTH => write!(f, "d13"),
            Self::MINOR_THIRTEENTH => write!(f, "m13"),
            Self::MAJOR_THIRTEENTH => write!(f, "M13"),
            Self::AUGMENTED_THIRTEENTH => write!(f, "A13"),
            // Fourteenths
            Self::DIMINISHED_FOURTEENTH => write!(f, "d14"),
            Self::MINOR_FOURTEENTH => write!(f, "m14"),
            Self::MAJOR_FOURTEENTH => write!(f, "M14"),
            Self::AUGMENTED_FOURTEENTH => write!(f, "A14"),
            _ => {
                // Fall back to algorithmic approach for uncommon intervals
                let generic_num = self.generic_interval_number();
                let semitones = self.total_semitones();
                // Calculate quality based on semitones vs expected
                write!(
                    f,
                    "interval({}f,{}o,g:{},s:{})",
                    self.fifths, self.octaves, generic_num, semitones
                )
            }
        }
    }
}

impl FromStr for Interval {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(ParseError::InvalidInterval(s.to_string()));
        }

        // Parse quality (can be multiple chars for multiple augmented/diminished)
        let mut chars = s.chars();
        let first_char = chars.next().unwrap();

        let (quality_type, quality_count) = match first_char {
            'P' => ('P', 1),
            'M' => ('M', 1),
            'm' => ('m', 1),
            'A' => {
                // Count consecutive A's for multiple augmented
                let mut count = 1;
                let remaining: String = chars.collect();
                for ch in remaining.chars() {
                    if ch == 'A' {
                        count += 1;
                    } else {
                        break;
                    }
                }
                ('A', count)
            }
            'd' => {
                // Count consecutive d's for multiple diminished
                let mut count = 1;
                let remaining: String = chars.collect();
                for ch in remaining.chars() {
                    if ch == 'd' {
                        count += 1;
                    } else {
                        break;
                    }
                }
                ('d', count)
            }
            _ => return Err(ParseError::InvalidInterval(s.to_string())),
        };

        // Extract the number part
        let number_start = if quality_type == 'A' || quality_type == 'd' {
            quality_count
        } else {
            1
        };

        let number_str = &s[number_start..];
        let interval_number: u8 = number_str
            .parse()
            .map_err(|_| ParseError::InvalidInterval(s.to_string()))?;

        if interval_number == 0 {
            return Err(ParseError::InvalidInterval(s.to_string()));
        }

        // Calculate base fifths and octaves for the interval number
        let (base_fifths, octaves) = Self::interval_number_to_fifths_and_octaves(interval_number);

        // Adjust for quality
        let adjusted_fifths = match quality_type {
            'P' => {
                // Validate that this interval can be perfect
                if !Self::can_be_perfect(interval_number) {
                    return Err(ParseError::InvalidInterval(s.to_string()));
                }
                base_fifths
            }
            'M' => {
                // Validate that this interval can be major
                if Self::can_be_perfect(interval_number) {
                    return Err(ParseError::InvalidInterval(s.to_string()));
                }
                base_fifths
            }
            'm' => {
                // Validate that this interval can be minor
                if Self::can_be_perfect(interval_number) {
                    return Err(ParseError::InvalidInterval(s.to_string()));
                }
                base_fifths - 7 // Minor is 7 fifths flat from major
            }
            'A' => {
                // Augmented: add 7 fifths per augmentation
                base_fifths + (7 * quality_count as i8)
            }
            'd' => {
                // Diminished: subtract 7 fifths per diminution
                let base = if Self::can_be_perfect(interval_number) {
                    base_fifths // Start from perfect
                } else {
                    base_fifths - 7 // Start from minor for major/minor intervals
                };
                base - (7 * quality_count as i8)
            }
            _ => unreachable!(),
        };

        Ok(Interval::new(adjusted_fifths, octaves))
    }
}

impl PartialOrd for Interval {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}


impl Ord for Interval {
    /// Compare intervals based on their semitone distance.
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.semitones().cmp(&other.semitones())
    }
}
