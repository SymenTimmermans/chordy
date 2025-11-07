//! Music analysis methods for chords
//!
//! This module contains methods for analyzing chords in musical contexts:
//! - Roman numeral analysis
//! - Harmonic function analysis
//! - Key-relative chord analysis

use crate::{
    traits::HasRoot,
    types::{scale::ScaleDegree, Chord, Key, RomanChord, RomanNumeral},
};

impl Chord {
    /// Convert this chord to a roman numeral chord in the given key
    pub fn to_roman(&self, key: &Key) -> Option<RomanChord> {
        use RomanNumeral;

        // Calculate the interval from the key root to this chord's root
        let interval_from_key = key.root().interval_to(self.root);

        let roman_numeral: RomanNumeral = interval_from_key.into();
        let mut roman_chord =
            RomanChord::new(roman_numeral, self.intervals.iter().collect());

        // Preserve bass information if present
        if let Some((bass_note, bass_type)) = self.bass {
            // Convert bass note to roman numeral relative to the key
            let bass_interval = key.root().interval_to(bass_note);
            let bass_roman: RomanNumeral = bass_interval.into();
            roman_chord.bass = Some((bass_roman, bass_type));
        }

        Some(roman_chord)
    }

    /// Analyze this chord in the given key, returning both roman numeral and harmonic function
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Chord, Key, note};
    ///
    /// let g_chord = Chord::major(note!("G"));
    /// let c_major_key = Key::Major(note!("C"));
    /// let (roman, function) = g_chord.analyze_in_key(&c_major_key);
    /// // roman would be V, function would be Some(Dominant)
    /// ```
    pub fn analyze_in_key(
        &self,
        key: &Key,
    ) -> (RomanNumeral, Option<HarmonicFunction>) {
        let roman = key.analyze_chord(self);
        let function = key.harmonic_function(self);
        (roman, function)
    }

    /// Get the roman numeral for this chord in the given key
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Chord, Key, RomanNumeral, note};
    ///
    /// let g_chord = Chord::major(note!("G"));
    /// let c_major_key = Key::Major(note!("C"));
    /// let roman = g_chord.roman_in_key(&c_major_key);
    /// assert_eq!(roman, RomanNumeral::V());
    /// ```
    pub fn roman_in_key(&self, key: &Key) -> RomanNumeral {
        key.analyze_chord(self)
    }

    /// Get the harmonic function of this chord in the given key
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Chord, Key, HarmonicFunction, note};
    ///
    /// let g_chord = Chord::major(note!("G"));
    /// let c_major_key = Key::Major(note!("C"));
    /// let function = g_chord.function_in_key(&c_major_key);
    /// assert_eq!(function, Some(HarmonicFunction::Dominant));
    /// ```
    pub fn function_in_key(&self, key: &Key) -> Option<HarmonicFunction> {
        key.harmonic_function(self)
    }
}

/// Harmonic functions represent the roles chords play in a key
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum HarmonicFunction {
    Tonic,
    Subdominant,
    Dominant,
}

impl HarmonicFunction {
    fn all() -> Vec<HarmonicFunction> {
        vec![
            HarmonicFunction::Tonic,
            HarmonicFunction::Subdominant,
            HarmonicFunction::Dominant,
        ]
    }

    fn triggers(&self) -> Vec<u8> {
        match self {
            HarmonicFunction::Tonic => vec![1, 3],
            HarmonicFunction::Subdominant => vec![4, 6],
            HarmonicFunction::Dominant => vec![5, 7],
        }
    }

    fn associates(&self) -> Vec<u8> {
        match self {
            HarmonicFunction::Tonic => vec![5, 6],
            HarmonicFunction::Subdominant => vec![1, 2],
            HarmonicFunction::Dominant => vec![2],
        }
    }

    fn dissonances(&self) -> Vec<(u8, Option<u8>)> {
        match self {
            HarmonicFunction::Tonic => vec![(5, Some(6)), (7, None)],
            HarmonicFunction::Subdominant => vec![(1, Some(2)), (3, None)],
            HarmonicFunction::Dominant => vec![(4, None), (6, None)],
        }
    }

    /// Detects the harmonic function of a chord based on its scale degrees
    pub fn detect_by_scale_degrees(scale_degrees: &[ScaleDegree]) -> Option<Self> {
        let all = Self::all();
        let simple_degrees = scale_degrees.iter().map(|sd| sd.step).collect::<Vec<u8>>();
        let scores = all.iter().map(|hf| {
            let mut score = 0;
            for degree in simple_degrees.iter() {
                if hf.triggers().contains(degree) {
                    score += 8;
                } else if hf.associates().contains(degree) {
                    score += 4;
                } else if hf.dissonances().iter().any(|(d, i)| {
                    d == degree && (i.is_none() || simple_degrees.contains(&i.unwrap()))
                }) {
                    score += 1;
                }
            }
            (hf, score)
        });

        // if no harmonic function has a positive score, return None
        if scores.clone().all(|(_, score)| score <= 0) {
            return None;
        }

        // return the harmonic function with the highest score
        scores.max_by_key(|(_, score)| *score).map(|(hf, _)| *hf)
    }
}