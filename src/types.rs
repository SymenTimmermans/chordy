use std::{fmt, str::FromStr};
use std::ops::AddAssign;
use std::ops::Add;

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
    letter: Letter,
    accidental: Accidental,
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
    ///
    pub fn base_midi_number(&self) -> i8 {
        self.letter.base_midi_number() + self.accidental.semitone_offset()
    }

    /// Checks if two note names are enharmonically equivalent
    pub fn is_enharmonic_with(&self, other: &Self) -> bool {
        // Notes are enharmonically equivalent if they represent the same pitch
        self.base_midi_number() % 12 == other.base_midi_number() % 12
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

/// A chord with a root note and quality
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chord {
    root: NoteName,
    quality: ChordQuality,
    extensions: Vec<ChordExtension>,
}

/// A scale with a tonic and mode
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scale {
    tonic: NoteName,
    mode: ScaleType,
}

impl Scale {
    pub fn new(tonic: NoteName, mode: ScaleType) -> Self {
        Scale { tonic, mode }
    }

    pub fn notes(&self) -> Vec<NoteName> {
        // Generate notes based on tonic and mode
        // This is a placeholder implementation
        vec![self.tonic]
    }
}

/// A musical key (combination of tonic and mode)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Key {
    tonic: NoteName,
    mode: Mode,  // Usually just Major or Minor
}

/// Musical letter names A through G, with numeric backing
/// representing their position in the chromatic scale.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i8)]
pub enum Letter {
    C = 0,
    D = 2,
    E = 4,
    F = 5,
    G = 7,
    A = 9,
    B = 11,
}

impl fmt::Display for Letter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::symbols::*;
        
        match self {
            Letter::C => write!(f, "{}", C),
            Letter::D => write!(f, "{}", D),
            Letter::E => write!(f, "{}", E),
            Letter::F => write!(f, "{}", F),
            Letter::G => write!(f, "{}", G),
            Letter::A => write!(f, "{}", A),
            Letter::B => write!(f, "{}", B),
        }
    }
}

impl Letter {
    /// Returns the base MIDI note number for this letter in octave 0
    pub fn base_midi_number(&self) -> i8 {
        *self as i8
    }
    
    /// Gets the next letter in the sequence (wrapping from G to A)
    pub fn next(&self) -> Self {
        match self {
            Letter::A => Letter::B,
            Letter::B => Letter::C,
            Letter::C => Letter::D,
            Letter::D => Letter::E,
            Letter::E => Letter::F,
            Letter::F => Letter::G,
            Letter::G => Letter::A,
        }
    }
    
    /// Gets the previous letter in the sequence (wrapping from A to G)
    pub fn _prev(&self) -> Self {
        match self {
            Letter::A => Letter::G,
            Letter::B => Letter::A,
            Letter::C => Letter::B,
            Letter::D => Letter::C,
            Letter::E => Letter::D,
            Letter::F => Letter::E,
            Letter::G => Letter::F,
        }
    }
}

/// Accidentals that modify the pitch of a note,
/// with numeric backing representing semitone shifts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i8)]
pub enum Accidental {
    DoubleFlat = -2,
    Flat = -1,
    Natural = 0,
    Sharp = 1,
    DoubleSharp = 2,
}

impl Accidental {
    /// Returns the semitone offset for this accidental
    pub fn semitone_offset(&self) -> i8 {
        *self as i8
    }
}

impl fmt::Display for Accidental {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::symbols::*;
        
        match self {
            Accidental::Flat => write!(f, "{}", FLAT),
            Accidental::Sharp => write!(f, "{}", SHARP),
            Accidental::Natural => write!(f, "{}", NATURAL),
            Accidental::DoubleFlat => write!(f, "{}", DOUBLE_FLAT),
            Accidental::DoubleSharp => write!(f, "{}", DOUBLE_SHARP),
        }
    }
}

impl FromStr for Accidental {
    type Err = ParseError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "b" | "♭" => Ok(Accidental::Flat),
            "#" | "♯" => Ok(Accidental::Sharp),
            "n" | "♮" => Ok(Accidental::Natural),
            "bb" | "𝄫" => Ok(Accidental::DoubleFlat),
            "##" | "𝄪" => Ok(Accidental::DoubleSharp),
            _ => Err(ParseError::InvalidAccidental(s.to_string())),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChordQuality {
    Major,
    Minor,
    Diminished,
    Augmented,
    Sus2,
    Sus4,
    // etc.
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScaleType {
    Major,
    NaturalMinor,
    HarmonicMinor,
    MelodicMinor,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Locrian,
    // etc.
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Major,
    Minor,
    // etc.
}
/// Extensions and alterations that can be added to basic chord triads
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChordExtension {
    /// 7th chords (dominant 7, major 7, etc.)
    Seventh(SeventhType),
    
    /// 9th extension (adds 9th above root)
    Ninth(NinthType),
    
    /// 11th extension (adds 11th above root)
    Eleventh(EleventhType),
    
    /// 13th extension (adds 13th above root)
    Thirteenth(ThirteenthType),
    
    /// Added notes that aren't standard extensions (add2, add4, etc.)
    Add(AddedNote),
    
    /// Suspended notes (sus2, sus4)
    Sus(SuspendedType),
    
    /// Altered fifth (e.g., ♭5, ♯5)
    AlteredFifth(AlteredFifthType),
    
    /// Altered ninth (e.g., ♭9, ♯9)
    AlteredNinth(AlteredNinthType),
    
    /// Omitted notes (e.g., no3, no5)
    Omit(OmittedNote),
}

/// Types of seventh chords
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SeventhType {
    /// Dominant seventh (♭7)
    Dominant,
    
    /// Major seventh (major triad with major 7th)
    Major,
    
    /// Minor seventh (minor triad with minor 7th)
    Minor,
    
    /// Half-diminished seventh (diminished triad with minor 7th)
    HalfDiminished,
    
    /// Diminished seventh (diminished triad with diminished 7th)
    Diminished,
}

/// Types of ninth extensions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NinthType {
    /// Standard ninth (major 9th)
    Natural,
    
    /// Flat ninth (♭9)
    Flat,
    
    /// Sharp ninth (♯9)
    Sharp,
}

/// Types of eleventh extensions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EleventhType {
    /// Standard eleventh (perfect 11th)
    Natural,
    
    /// Sharp eleventh (♯11)
    Sharp,
}

/// Types of thirteenth extensions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThirteenthType {
    /// Standard thirteenth (major 13th)
    Natural,
    
    /// Flat thirteenth (♭13)
    Flat,
}

/// Added notes not part of standard extensions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AddedNote {
    /// Added 2nd/9th without 7th
    Add2,
    
    /// Added 4th/11th without 7th and 9th
    Add4,
    
    /// Added 6th
    Add6,
    
    /// Added ♭6th
    AddFlat6,
}

/// Suspended chord types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SuspendedType {
    /// Suspended 2nd (replaces 3rd with 2nd)
    Sus2,
    
    /// Suspended 4th (replaces 3rd with 4th)
    Sus4,
}

/// Altered fifth variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AlteredFifthType {
    /// Flat fifth (♭5)
    Flat,
    
    /// Sharp fifth (♯5)
    Sharp,
}

/// Altered ninth variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AlteredNinthType {
    /// Flat ninth (♭9)
    Flat,
    
    /// Sharp ninth (♯9)
    Sharp,
}

/// Notes that can be omitted from chords
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OmittedNote {
    /// Omitted 3rd
    No3,
    
    /// Omitted 5th
    No5,
}
