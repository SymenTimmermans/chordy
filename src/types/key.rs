use crate::traits::HasRoot;

use super::{NoteName, scale::ScaleDegree, Chord, RomanNumeral, HarmonicFunction, Scale};

/// The mode of a key (Major, Minor, etc.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    /// Major key signature
    ///
    /// This does not imply a specific scale, just the key signature.
    Major(NoteName),
    /// Minor key signature
    ///
    /// This does not imply a specific scale, just the key signature.
    Minor(NoteName),
}

/// Represents a key signature with sharps or flats
impl Key {
    /// Number of sharps (positive) or flats (negative)
    pub fn accidentals(&self) -> i8 {
        match self {
            Key::Major(note) | Key::Minor(note) => note.fifths(),
        }
    }

    /// Returns the scale degree of a given note within this key
    ///
    /// Calculates the scale degree by finding the interval from the key's root
    /// to the given note, then converting that interval to a ScaleDegree.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chordy::{Key, note, ScaleDegree, Accidental};
    ///
    /// let c_major = Key::Major(note!("C"));
    /// 
    /// // Natural scale degrees in C major
    /// assert_eq!(c_major.degree_of(note!("C")), ScaleDegree::new(1, None));
    /// assert_eq!(c_major.degree_of(note!("D")), ScaleDegree::new(2, None));
    /// assert_eq!(c_major.degree_of(note!("E")), ScaleDegree::new(3, None));
    /// 
    /// // Altered notes
    /// assert_eq!(c_major.degree_of(note!("C#")), ScaleDegree::new(1, Some(Accidental::Sharp)));
    /// assert_eq!(c_major.degree_of(note!("F#")), ScaleDegree::new(4, Some(Accidental::Sharp)));
    /// ```
    pub fn degree_of(&self, note: NoteName) -> ScaleDegree {
        // Calculate the interval from the key root to the given note
        let interval = self.root().interval_to(note);
        
        // Convert the interval to a scale degree
        ScaleDegree::from(interval)
    }

    /// Analyze a chord in this key, returning its roman numeral representation
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Key, Chord, RomanNumeral, note};
    ///
    /// let c_major_key = Key::Major(note!("C"));
    /// let g_chord = Chord::major(note!("G"));
    /// let roman = c_major_key.analyze_chord(&g_chord);
    /// assert_eq!(roman, RomanNumeral::V());
    /// ```
    pub fn analyze_chord(&self, chord: &Chord) -> RomanNumeral {
        let root_degree = self.degree_of(chord.root);
        RomanNumeral::from(root_degree)
    }

    /// Get the harmonic function of a chord in this key
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Key, Chord, HarmonicFunction, note};
    ///
    /// let c_major_key = Key::Major(note!("C"));
    /// let g_chord = Chord::major(note!("G"));
    /// let function = c_major_key.harmonic_function(&g_chord);
    /// assert_eq!(function, Some(HarmonicFunction::Dominant));
    /// ```
    pub fn harmonic_function(&self, chord: &Chord) -> Option<HarmonicFunction> {
        // Get the scale for this key
        let scale = match self {
            Key::Major(tonic) => Scale::major(*tonic),
            Key::Minor(tonic) => Scale::minor(*tonic),
        };
        
        scale.harmonic_function(chord)
    }

    /// Get the scale associated with this key
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Key, Scale, note};
    ///
    /// let c_major_key = Key::Major(note!("C"));
    /// let scale = c_major_key.scale();
    /// assert_eq!(scale, Scale::major(note!("C")));
    /// ```
    pub fn scale(&self) -> Scale {
        match self {
            Key::Major(tonic) => Scale::major(*tonic),
            Key::Minor(tonic) => Scale::minor(*tonic),
        }
    }
}

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Key::Major(note) => write!(f, "{} Major", note),
            Key::Minor(note) => write!(f, "{} Minor", note),
        }
    }
}

impl HasRoot for Key {
    fn root(&self) -> NoteName {
        match self {
            Key::Major(note) => *note,
            Key::Minor(note) => *note,
        }
    }

    fn root_mut(&mut self) -> &mut NoteName {
        match self {
            Key::Major(note) => note,
            Key::Minor(note) => note,
        }
    }
}

