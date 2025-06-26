use std::{fmt::Display, str::FromStr};
use super::{Interval, Key, Chord, Accidental, ChordQuality};
use crate::{error::ParseError, traits::{HasIntervals, HasRoot}};

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
    pub const fn new(degree: RomanDegree, accidental: Accidental) -> Self {
        RomanNumeral { degree, accidental }
    }

    // Convenience constructors for natural degrees
    /// I - Tonic
    #[allow(non_snake_case)]
    pub const fn I() -> Self { Self::new(RomanDegree::I, Accidental::Natural) }
    /// II - Supertonic  
    #[allow(non_snake_case)]
    pub const fn II() -> Self { Self::new(RomanDegree::II, Accidental::Natural) }
    /// III - Mediant
    #[allow(non_snake_case)]
    pub const fn III() -> Self { Self::new(RomanDegree::III, Accidental::Natural) }
    /// IV - Subdominant
    #[allow(non_snake_case)]
    pub const fn IV() -> Self { Self::new(RomanDegree::IV, Accidental::Natural) }
    /// V - Dominant
    #[allow(non_snake_case)]
    pub const fn V() -> Self { Self::new(RomanDegree::V, Accidental::Natural) }
    /// VI - Submediant
    #[allow(non_snake_case)]
    pub const fn VI() -> Self { Self::new(RomanDegree::VI, Accidental::Natural) }
    /// VII - Leading tone
    #[allow(non_snake_case)]
    pub const fn VII() -> Self { Self::new(RomanDegree::VII, Accidental::Natural) }

    // Convenience constructors for flat degrees
    /// ♭II - Flat supertonic
    #[allow(non_snake_case)]
    pub const fn flat_II() -> Self { Self::new(RomanDegree::II, Accidental::Flat) }
    /// ♭III - Flat mediant
    #[allow(non_snake_case)]
    pub const fn flat_III() -> Self { Self::new(RomanDegree::III, Accidental::Flat) }
    /// ♭VI - Flat submediant
    #[allow(non_snake_case)]
    pub const fn flat_VI() -> Self { Self::new(RomanDegree::VI, Accidental::Flat) }
    /// ♭VII - Flat subtonic
    #[allow(non_snake_case)]
    pub const fn flat_VII() -> Self { Self::new(RomanDegree::VII, Accidental::Flat) }

    // Convenience constructors for sharp degrees
    /// ♯I - Sharp tonic
    #[allow(non_snake_case)]
    pub const fn sharp_I() -> Self { Self::new(RomanDegree::I, Accidental::Sharp) }
    /// ♯IV - Sharp subdominant
    #[allow(non_snake_case)]
    pub const fn sharp_IV() -> Self { Self::new(RomanDegree::IV, Accidental::Sharp) }
    /// ♯V - Sharp dominant
    #[allow(non_snake_case)]
    pub const fn sharp_V() -> Self { Self::new(RomanDegree::V, Accidental::Sharp) }
    /// ♯VII - Sharp leading tone
    #[allow(non_snake_case)]
    pub const fn sharp_VII() -> Self { Self::new(RomanDegree::VII, Accidental::Sharp) }
    
    /// Get the degree
    pub fn degree(self) -> RomanDegree {
        self.degree
    }
    
    /// Get the accidental
    pub fn accidental(self) -> Accidental {
        self.accidental
    }
    
    /// Convert this roman numeral to its corresponding interval from the tonic
    ///
    /// This delegates to the underlying ScaleDegree conversion, providing a convenient
    /// way to get the interval representation of a roman numeral.
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{RomanNumeral, RomanDegree, Accidental, Interval};
    ///
    /// let v = RomanNumeral::new(RomanDegree::V, Accidental::Natural);
    /// assert_eq!(v.to_interval(), Interval::PERFECT_FIFTH);
    ///
    /// let flat_ii = RomanNumeral::new(RomanDegree::II, Accidental::Flat);
    /// assert_eq!(flat_ii.to_interval(), Interval::MINOR_SECOND);
    ///
    /// let sharp_iv = RomanNumeral::new(RomanDegree::IV, Accidental::Sharp);
    /// assert_eq!(sharp_iv.to_interval(), Interval::AUGMENTED_FOURTH);
    /// ```
    pub fn to_interval(self) -> Interval {
        // Convert RomanNumeral to ScaleDegree manually since From trait doesn't exist in reverse
        let scale_degree = super::scale::ScaleDegree::new(
            self.degree.to_number(), 
            self.accidental.as_alteration()
        );
        scale_degree.to_interval()
    }
}

impl Display for RomanNumeral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Default display without quality context - use uppercase
        let base = self.degree.base_string();
        
        write!(f, "{}{}", self.accidental.component_str(), base)
    }
}

impl FromStr for RomanNumeral {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(ParseError::InvalidRomanNumeral(s.to_string()));
        }
        
        // Parse accidental prefix using centralized Accidental::FromStr
        // Try different prefix lengths to find the longest match
        let mut accidental = Accidental::Natural;
        let mut degree_start = 0;
        
        // Try double accidentals first (longer strings)
        for prefix_len in (1..=s.len()).rev() {
            if let Some(prefix) = s.get(0..prefix_len) {
                if let Ok(parsed_accidental) = Accidental::from_str(prefix) {
                    accidental = parsed_accidental;
                    degree_start = prefix_len;
                    break;
                }
            }
        }
        
        // Parse the roman numeral part
        let roman_part = &s[degree_start..];
        if roman_part.is_empty() {
            return Err(ParseError::InvalidRomanNumeral(s.to_string()));
        }
        
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
    
    /// Create a simple roman chord with basic quality (triad intervals)
    pub fn simple(root: RomanNumeral, quality: ChordQuality) -> Self {
        let intervals = match quality {
            ChordQuality::Major => vec![
                Interval::PERFECT_UNISON,
                Interval::MAJOR_THIRD,
                Interval::PERFECT_FIFTH,
            ],
            ChordQuality::Minor => vec![
                Interval::PERFECT_UNISON,
                Interval::MINOR_THIRD,
                Interval::PERFECT_FIFTH,
            ],
            ChordQuality::Diminished => vec![
                Interval::PERFECT_UNISON,
                Interval::MINOR_THIRD,
                Interval::DIMINISHED_FIFTH,
            ],
            ChordQuality::Augmented => vec![
                Interval::PERFECT_UNISON,
                Interval::MAJOR_THIRD,
                Interval::AUGMENTED_FIFTH,
            ],
        };
        Self::new(root, intervals)
    }

    /// Get the root roman numeral
    pub fn root(&self) -> RomanNumeral {
        self.root
    }

    /// Get the intervals
    pub fn intervals(&self) -> &[Interval] {
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
        self.of(key)
    }

    /// Creates a concrete `Chord` from this `RomanChord` relative to the root of another musical structure.
    ///
    /// This allows for music theory analysis operations like secondary dominants.
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{RomanChord, RomanNumeral, RomanDegree, Chord, NoteName, Letter, Accidental, Key};
    /// use chordy::traits::{HasRoot, HasIntervals};
    ///
    /// let c_major_key = Key::Major(NoteName::new(Letter::C, Accidental::Natural));
    /// let roman_five = RomanChord::simple(RomanNumeral::V(), ChordQuality::Major);
    ///
    /// // Get the dominant chord of C major (G major)
    /// let g_major_chord = roman_five.of(&c_major_key);
    /// assert_eq!(g_major_chord.root(), NoteName::new(Letter::G, Accidental::Natural));
    /// assert_eq!(g_major_chord.intervals.len(), 3);
    ///
    /// // You can also use it with a Chord directly if it implements HasRoot
    /// let c_major_chord = Chord::major(NoteName::new(Letter::C, Accidental::Natural));
    /// let g_major_from_chord = roman_five.of(&c_major_chord);
    /// assert_eq!(g_major_from_chord.root(), NoteName::new(Letter::G, Accidental::Natural));
    /// ```
    pub fn of<T: HasRoot>(&self, c: &T) -> Chord {
        let base_root = c.root();
        let interval_from_base = self.root.to_interval();
        let actual_root = base_root + interval_from_base;
        Chord::new(actual_root, self.intervals.clone())
    }
    
    /// Convert this roman chord to a ChordName using the new naming system
    pub fn to_chord_name(&self) -> super::chord::ChordName {
        use super::chord::{ChordRoot, ChordAnalyzer};
        let chord_root = ChordRoot::Roman(self.root);
        ChordAnalyzer::analyze_with_root(chord_root, &self.intervals)
    }
}

impl Display for RomanChord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Use the new chord naming system for consistent and complete notation
        use super::chord::{ChordRenderer, NamingConvention};
        
        let chord_name = self.to_chord_name();
        
        // Use Roman numeral specific renderer
        let renderer = ChordRenderer::new(
            super::chord::ChordFormat::Unicode, 
            NamingConvention::Jazz
        );
        
        write!(f, "{}", renderer.render(&chord_name))
    }
}

impl From<super::scale::ScaleDegree> for RomanNumeral {
    fn from(scale_degree: super::scale::ScaleDegree) -> Self {
        let degree = RomanDegree::from_number(scale_degree.step)
            .expect("ScaleDegree should always have a valid step (1-7)");
        
        let accidental = scale_degree.alteration.unwrap_or(Accidental::Natural);
        
        RomanNumeral::new(degree, accidental)
    }
}

impl From<Interval> for RomanNumeral {
    fn from(interval: Interval) -> Self {
        let scale_degree = super::scale::ScaleDegree::from(interval);
        RomanNumeral::from(scale_degree)
    }
}

impl From<u8> for RomanNumeral {
    /// Create a natural roman numeral from a degree number (1-7)
    /// 
    /// # Panics
    /// Panics if the number is not in the range 1-7
    fn from(degree_num: u8) -> Self {
        let degree = RomanDegree::from_number(degree_num)
            .expect("Degree number must be in range 1-7");
        RomanNumeral::new(degree, Accidental::Natural)
    }
}

impl HasIntervals for RomanChord {
    fn intervals(&self) -> &[Interval] {
        &self.intervals
    }

    fn intervals_mut(&mut self) -> &mut Vec<Interval> {
        &mut self.intervals
    }
}
