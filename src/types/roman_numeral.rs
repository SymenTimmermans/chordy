use std::{fmt::Display, str::FromStr};
use super::{Interval, Key, Chord, Accidental, ChordQuality};
use crate::error::ParseError;

/// Roman degree representation (I-VII), analogous to Letter enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RomanDegree {
    /// First degree
    I,
    /// Second degree
    II,
    /// Third degree
    III,
    /// Fourth degree
    IV,
    /// Fifth degree
    V,
    /// Sixth degree
    VI,
    /// Seventh degree
    VII,
}

impl RomanDegree {
    /// Convert to numeric value (1-7)
    pub fn to_number(self) -> u8 {
        match self {
            RomanDegree::I => 1,
            RomanDegree::II => 2,
            RomanDegree::III => 3,
            RomanDegree::IV => 4,
            RomanDegree::V => 5,
            RomanDegree::VI => 6,
            RomanDegree::VII => 7,
        }
    }
    
    /// Create from numeric value (1-7)
    pub fn from_number(n: u8) -> Option<Self> {
        match n {
            1 => Some(RomanDegree::I),
            2 => Some(RomanDegree::II),
            3 => Some(RomanDegree::III),
            4 => Some(RomanDegree::IV),
            5 => Some(RomanDegree::V),
            6 => Some(RomanDegree::VI),
            7 => Some(RomanDegree::VII),
            _ => None,
        }
    }
    
    /// Get the base roman numeral string (uppercase)
    pub fn base_string(self) -> &'static str {
        match self {
            RomanDegree::I => "I",
            RomanDegree::II => "II",
            RomanDegree::III => "III",
            RomanDegree::IV => "IV",
            RomanDegree::V => "V",
            RomanDegree::VI => "VI",
            RomanDegree::VII => "VII",
        }
    }
    
    /// Get the lowercase version
    pub fn lowercase_string(self) -> &'static str {
        match self {
            RomanDegree::I => "i",
            RomanDegree::II => "ii",
            RomanDegree::III => "iii",
            RomanDegree::IV => "iv",
            RomanDegree::V => "v",
            RomanDegree::VI => "vi",
            RomanDegree::VII => "vii",
        }
    }
}

/// Roman numeral representation with degree and accidental, analogous to NoteName
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RomanNumeral {
    /// The roman degree (I-VII)
    pub degree: RomanDegree,
    /// The accidental modifier
    pub accidental: Accidental,
}

impl RomanNumeral {
    /// Create a new roman numeral
    pub fn new(degree: RomanDegree, accidental: Accidental) -> Self {
        RomanNumeral { degree, accidental }
    }
    
    /// Get the degree
    pub fn degree(self) -> RomanDegree {
        self.degree
    }
    
    /// Get the accidental
    pub fn accidental(self) -> Accidental {
        self.accidental
    }
    
    /// Display this roman numeral for a given chord quality
    /// Quality determines case: Major/Augmented = uppercase, Minor/Diminished = lowercase
    pub fn display_for_quality(self, quality: ChordQuality) -> String {
        let base = match quality {
            ChordQuality::Major | ChordQuality::Augmented => self.degree.base_string(),
            ChordQuality::Minor | ChordQuality::Diminished => self.degree.lowercase_string(),
        };
        
        let accidental_str = match self.accidental {
            Accidental::Natural => "",
            Accidental::Sharp => "♯",
            Accidental::Flat => "♭",
            Accidental::DoubleSharp => "𝄪",
            Accidental::DoubleFlat => "𝄫",
        };
        
        let quality_suffix = match quality {
            ChordQuality::Diminished => "°",
            ChordQuality::Augmented => "+",
            _ => "",
        };
        
        format!("{}{}{}", accidental_str, base, quality_suffix)
    }
}

impl Display for RomanNumeral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Default display without quality context - use uppercase
        let base = self.degree.base_string();
        let accidental_str = match self.accidental {
            Accidental::Natural => "",
            Accidental::Sharp => "♯",
            Accidental::Flat => "♭",
            Accidental::DoubleSharp => "𝄪",
            Accidental::DoubleFlat => "𝄫",
        };
        
        write!(f, "{}{}", accidental_str, base)
    }
}

impl FromStr for RomanNumeral {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<char> = s.chars().collect();
        if chars.is_empty() {
            return Err(ParseError::InvalidRomanNumeral(s.to_string()));
        }
        
        // Parse accidental prefix
        let mut accidental = Accidental::Natural;
        let mut degree_start = 0;
        
        match chars[0] {
            '♭' => {
                accidental = Accidental::Flat;
                degree_start = 1;
            },
            '♯' => {
                accidental = Accidental::Sharp;
                degree_start = 1;
            },
            '𝄫' => {
                accidental = Accidental::DoubleFlat;
                degree_start = 1;
            },
            '𝄪' => {
                accidental = Accidental::DoubleSharp;
                degree_start = 1;
            },
            _ => {}
        }
        
        // Parse the roman numeral part
        let roman_part: String = chars[degree_start..].iter().collect();
        let degree = match roman_part.to_uppercase().as_str() {
            "I" => RomanDegree::I,
            "II" => RomanDegree::II,
            "III" => RomanDegree::III,
            "IV" => RomanDegree::IV,
            "V" => RomanDegree::V,
            "VI" => RomanDegree::VI,
            "VII" => RomanDegree::VII,
            _ => return Err(ParseError::InvalidRomanNumeral(s.to_string())),
        };
        
        Ok(RomanNumeral::new(degree, accidental))
    }
}

/// A chord represented by a roman numeral root and intervals
///
/// This struct represents a chord in the context of a key, defined by its roman numeral root and
/// the intervals that make up the chord.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RomanChord {
    /// The roman numeral root of the chord
    pub root: RomanNumeral,
    /// The intervals from the root that define the chord
    pub intervals: Vec<Interval>,
}

impl RomanChord {
    /// Create a new roman chord from root and intervals
    pub fn new(root: RomanNumeral, intervals: Vec<Interval>) -> Self {
        RomanChord { root, intervals }
    }

    /// Get the root roman numeral
    pub fn root(&self) -> RomanNumeral {
        self.root
    }

    /// Get the intervals
    pub fn intervals(&self) -> &Vec<Interval> {
        &self.intervals
    }

    /// Create a major roman chord
    pub fn major(root: RomanNumeral) -> Self {
        Self::new(
            root,
            vec![
                Interval::PERFECT_UNISON,
                Interval::MAJOR_THIRD,
                Interval::PERFECT_FIFTH,
            ],
        )
    }

    /// Create a minor roman chord
    pub fn minor(root: RomanNumeral) -> Self {
        Self::new(
            root,
            vec![
                Interval::PERFECT_UNISON,
                Interval::MINOR_THIRD,
                Interval::PERFECT_FIFTH,
            ],
        )
    }

    /// Create a diminished roman chord
    pub fn diminished(root: RomanNumeral) -> Self {
        Self::new(
            root,
            vec![
                Interval::PERFECT_UNISON,
                Interval::MINOR_THIRD,
                Interval::DIMINISHED_FIFTH,
            ],
        )
    }
    
    /// Create an augmented roman chord
    pub fn augmented(root: RomanNumeral) -> Self {
        Self::new(
            root,
            vec![
                Interval::PERFECT_UNISON,
                Interval::MAJOR_THIRD,
                Interval::AUGMENTED_FIFTH,
            ],
        )
    }

    /// Convert this roman chord to an actual chord in the given key
    pub fn in_key(&self, key: &Key) -> Chord {
        // Get the root note of the key
        let key_root = match key {
            Key::Major(note) | Key::Minor(note) => *note,
        };
        
        // Calculate the interval for this degree
        let degree_interval = match self.root.degree {
            RomanDegree::I => Interval::PERFECT_UNISON,
            RomanDegree::II => Interval::MAJOR_SECOND,
            RomanDegree::III => Interval::MAJOR_THIRD,
            RomanDegree::IV => Interval::PERFECT_FOURTH,
            RomanDegree::V => Interval::PERFECT_FIFTH,
            RomanDegree::VI => Interval::MAJOR_SIXTH,
            RomanDegree::VII => Interval::MAJOR_SEVENTH,
        };
        
        // Apply the accidental
        let interval_with_accidental = match self.root.accidental {
            Accidental::Natural => degree_interval,
            Accidental::Flat => match degree_interval {
                Interval::MAJOR_SECOND => Interval::MINOR_SECOND,
                Interval::MAJOR_THIRD => Interval::MINOR_THIRD,
                Interval::PERFECT_FOURTH => Interval::DIMINISHED_FOURTH,
                Interval::PERFECT_FIFTH => Interval::DIMINISHED_FIFTH,
                Interval::MAJOR_SIXTH => Interval::MINOR_SIXTH,
                Interval::MAJOR_SEVENTH => Interval::MINOR_SEVENTH,
                _ => degree_interval, // PERFECT_UNISON stays the same
            },
            Accidental::Sharp => match degree_interval {
                Interval::MAJOR_SECOND => Interval::AUGMENTED_SECOND,
                Interval::MAJOR_THIRD => Interval::AUGMENTED_THIRD,
                Interval::PERFECT_FOURTH => Interval::AUGMENTED_FOURTH,
                Interval::PERFECT_FIFTH => Interval::AUGMENTED_FIFTH,
                Interval::MAJOR_SIXTH => Interval::AUGMENTED_SIXTH,
                Interval::MAJOR_SEVENTH => Interval::AUGMENTED_SEVENTH,
                _ => degree_interval,
            },
            // TODO: Implement double accidentals
            _ => degree_interval,
        };
        
        // Calculate the root note
        let root_note = key_root + interval_with_accidental;
        
        // Create the chord with the calculated root note
        Chord::new(root_note, self.intervals.clone())
    }
    
    /// Get the chord quality from the intervals
    pub fn quality(&self) -> Option<ChordQuality> {
        // Use the existing chord quality detection
        let temp_chord = Chord::new(
            super::NoteName::new(super::Letter::C, Accidental::Natural), 
            self.intervals.clone()
        );
        temp_chord.quality()
    }
}

impl Display for RomanChord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Show the roman numeral with quality-based case and any extensions
        let quality = self.quality().unwrap_or(ChordQuality::Major);
        let mut result = self.root.display_for_quality(quality);
        
        // Check for common extensions
        let has_minor_7th = self.intervals.contains(&Interval::MINOR_SEVENTH);
        let has_major_7th = self.intervals.contains(&Interval::MAJOR_SEVENTH);
        
        if has_minor_7th {
            result.push('7');
        } else if has_major_7th {
            result.push_str("maj7");
        }
        
        write!(f, "{}", result)
    }
}

impl From<super::ScaleDegree> for RomanNumeral {
    fn from(scale_degree: super::ScaleDegree) -> Self {
        let degree = RomanDegree::from_number(scale_degree.step)
            .expect("ScaleDegree should always have a valid step (1-7)");
        
        let accidental = scale_degree.alteration.unwrap_or(Accidental::Natural);
        
        RomanNumeral::new(degree, accidental)
    }
}

impl From<Interval> for RomanNumeral {
    fn from(interval: Interval) -> Self {
        let scale_degree = super::ScaleDegree::from(interval);
        RomanNumeral::from(scale_degree)
    }
}
