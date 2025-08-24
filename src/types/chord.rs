//! Chord representation with support for inversions and slash chords
//!
//! This module provides comprehensive support for chord representation including:
//! - Root position chords
//! - Classical inversions (chord tones in bass)
//! - Slash chords (arbitrary bass notes)
//!
//! # Bass Note Concepts
//!
//! ## Root Position
//! A chord in root position has its root note as the lowest-sounding note (bass).
//! This is the default state for all chords.
//!
//! ```rust
//! use chordy::prelude::*;
//!
//! let c_major = Chord::major(note!("C"));
//! assert_eq!(c_major.bass_note(), note!("C"));  // Root is in bass
//! assert!(!c_major.is_inverted());
//! assert!(!c_major.is_slash_chord());
//! ```
//!
//! ## Classical Inversions
//! An inversion occurs when a chord tone other than the root is in the bass.
//! This creates different harmonic colors and voice leading possibilities.
//!
//! - **First Inversion**: Third in bass (e.g., C major = C/E)
//! - **Second Inversion**: Fifth in bass (e.g., C major = C/G)
//! - **Third Inversion**: Seventh in bass (e.g., C7 = C7/Bb)
//!
//! ```rust
//! use chordy::prelude::*;
//!
//! let c_major = Chord::major(note!("C"));
//!
//! // First inversion - third (E) in bass
//! let first_inv = c_major.with_inversion(1);
//! assert_eq!(first_inv.bass_note(), note!("E"));
//! assert!(first_inv.is_inverted());
//! assert_eq!(first_inv.inversion_number(), Some(1));
//!
//! // Second inversion - fifth (G) in bass  
//! let second_inv = c_major.with_inversion(2);
//! assert_eq!(second_inv.bass_note(), note!("G"));
//! assert_eq!(second_inv.inversion_number(), Some(2));
//! ```
//!
//! ## Slash Chords
//! Slash chords have an arbitrary bass note that may or may not be a chord tone.
//! They're common in popular music and jazz for creating specific bass lines
//! or harmonic colors.
//!
//! ```rust
//! use chordy::prelude::*;
//!
//! let c_major = Chord::major(note!("C"));
//!
//! // C/F - C major with F in bass (not a chord tone)
//! let c_slash_f = c_major.with_slash_bass(note!("F"));
//! assert_eq!(c_slash_f.bass_note(), note!("F"));
//! assert!(c_slash_f.is_slash_chord());
//! assert!(!c_slash_f.is_inverted());
//!
//! // C/E - Could be written as slash chord or first inversion
//! let c_slash_e = c_major.with_slash_bass(note!("E"));
//! assert!(c_slash_e.is_slash_chord());  // Explicitly marked as slash
//! ```
//!
//! ## BassType Distinction
//! The [`BassType`] enum distinguishes between inversions and slash chords:
//!
//! - [`BassType::Inversion(n)`]: Classical inversion with chord tone in bass
//! - [`BassType::Slash`]: Arbitrary bass note (may or may not be chord tone)
//!
//! This distinction is important for:
//! - Music analysis and theory
//! - Voice leading algorithms
//! - Chord progression analysis
//! - Notation and display formatting
//!
//! ## Practical Usage
//!
//! ### Creating Bass Chords
//! ```rust
//! use chordy::prelude::*;
//!
//! let chord = Chord::major(note!("C"));
//!
//! // Method chaining for fluent interface
//! let complex_chord = chord
//!     .with_interval(Interval::MINOR_SEVENTH)  // Add 7th
//!     .with_inversion(1);                      // First inversion
//!
//! // Convenience methods
//! let first_inv = chord.in_first_inversion();
//! let second_inv = chord.in_second_inversion();
//! ```
//!
//! ### Bass Note Preservation
//! Bass notes are preserved during interval mutations:
//!
//! ```rust
//! use chordy::prelude::*;
//!
//! let mut chord = Chord::major(note!("C")).with_slash_bass(note!("F"));
//! let original_bass = chord.bass_note();
//!
//! // Modify intervals - bass is preserved
//! chord.add_interval(Interval::MINOR_SEVENTH);
//! chord.remove_interval(Interval::PERFECT_FIFTH);
//!
//! assert_eq!(chord.bass_note(), original_bass);  // Still F in bass
//! ```

use std::{fmt::Display, str::FromStr};

use super::{scale::ScaleDegree, Interval, IntervalSet, NoteName};
use crate::{
    error::ParseError,
    note,
    traits::{HasIntervals, HasRoot, Invertible},
};

mod quality;
pub use quality::ChordQuality;

pub mod naming;
pub use naming::*;

/// Type of bass note in a chord
///
/// This enum is crucial for distinguishing between two different musical concepts:
/// classical inversions and slash chords. While both result in a note other than
/// the root being in the bass, they have different theoretical implications and
/// are used in different musical contexts.
///
/// # Music Theory Background
///
/// ## Classical Inversions
/// In traditional harmony, an inversion occurs when a chord tone other than the
/// root is placed in the bass. This creates different harmonic functions and
/// voice leading possibilities:
///
/// - **Root Position**: Most stable, strong harmonic foundation
/// - **First Inversion**: Softer, more melodic bass line, less stable
/// - **Second Inversion**: Often requires resolution, creates tension
/// - **Third Inversion**: Common in jazz, creates smooth voice leading
///
/// ## Slash Chords  
/// Slash chords (also called "polychords" or "bass alterations") place any note
/// in the bass, whether it's a chord tone or not. They're widely used in:
///
/// - Popular music for creating memorable bass lines
/// - Jazz for reharmonization and sophisticated colors
/// - Contemporary classical music for extended harmony
///
/// # Practical Implications
///
/// The distinction between [`BassType::Inversion`] and [`BassType::Slash`] affects:
///
/// - **Analysis**: Different Roman numeral notation (e.g., I⁶ vs I/♭VI)
/// - **Voice Leading**: Inversions follow classical rules, slash chords are freer
/// - **Harmonic Function**: Inversions maintain function, slash chords may alter it
/// - **Resolution Tendencies**: Different bass notes create different tensions
///
/// # Examples
///
/// ```rust
/// use chordy::{Chord, note, BassType};
///
/// let c_major = Chord::major(note!("C"));
///
/// // Classical inversion - bass note is a chord tone (third = E)
/// let inversion = c_major.with_inversion(1);
/// if let Some((bass, bass_type)) = inversion.bass {
///     assert_eq!(bass, note!("E"));
///     assert!(matches!(bass_type, BassType::Inversion(1)));
/// }
///
/// // Slash chord - bass note can be any note (F is not in C major triad)
/// let slash_chord = c_major.with_slash_bass(note!("F"));
/// if let Some((bass, bass_type)) = slash_chord.bass {
///     assert_eq!(bass, note!("F"));
///     assert!(matches!(bass_type, BassType::Slash));
/// }
///
/// // Same bass note, different meanings:
/// let c_over_e_inversion = c_major.with_inversion(1);      // Theoretical: C major in first inversion
/// let c_over_e_slash = c_major.with_slash_bass(note!("E")); // Practical: C major over E bass
///
/// // Both have E in bass, but different BassType values
/// assert!(c_over_e_inversion.is_inverted());
/// assert!(c_over_e_slash.is_slash_chord());
/// ```
///
/// # When to Use Each
///
/// ## Use [`BassType::Inversion`] when:
/// - Analyzing classical or traditional music
/// - The bass note is definitively a chord tone
/// - Following voice leading rules and harmonic progressions
/// - You need theoretical precision for analysis
///
/// ## Use [`BassType::Slash`] when:
/// - Working with popular music or jazz
/// - The bass note is chosen for melodic or color reasons
/// - The harmonic function is less important than the sound
/// - Creating modern chord symbols or lead sheets
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BassType {
    /// Classical inversion where the bass note is a chord tone
    ///
    /// The number indicates which chord tone is in the bass, counting from the root:
    ///
    /// - **1st inversion** (`Inversion(1)`): Third in bass
    ///   - C major → C/E (C-E-G with E in bass)
    ///   - Creates softer, more melodic sound
    ///   - Roman numeral: I⁶
    ///
    /// - **2nd inversion** (`Inversion(2)`): Fifth in bass  
    ///   - C major → C/G (C-E-G with G in bass)
    ///   - Often unstable, may require resolution
    ///   - Roman numeral: I⁶₄
    ///
    /// - **3rd inversion** (`Inversion(3)`): Seventh in bass
    ///   - C7 → C7/B♭ (C-E-G-B♭ with B♭ in bass)
    ///   - Common in jazz, creates smooth voice leading
    ///   - Roman numeral: I⁴₃
    ///
    /// Higher inversion numbers work for extended chords (9ths, 11ths, etc.).
    Inversion(u8),

    /// Slash chord with arbitrary bass note
    ///
    /// The bass note can be any pitch, creating various harmonic effects:
    ///
    /// - **Non-chord tones**: Create color and tension
    ///   - C/F: C major over F (sus4 sound in bass)
    ///   - Am/C: A minor over C (creates Am/C or C6 sound)
    ///
    /// - **Chord tones**: Same notes as inversion, different context
    ///   - C/E: Could be slash notation instead of first inversion
    ///   - Useful when harmonic function differs from classical inversion
    ///
    /// - **Bass lines**: Chosen for melodic rather than harmonic reasons
    ///   - G/B: G major over B (common in progressions like G/B-C)
    ///   - Creates smooth bass motion
    ///
    /// Common in popular music chord charts and lead sheets.
    Slash,
}

/// A chord represented by a root note, intervals, and optional bass note specification
///
/// The [`Chord`] struct is the primary representation of harmonic structures in chordy.
/// It supports the full spectrum of chord representations from simple triads to complex
/// extended chords, with comprehensive bass note support for both classical inversions
/// and modern slash chord notation.
///
/// # Structure
///
/// A chord consists of three components:
///
/// - **Root**: The fundamental note that defines the chord's identity
/// - **Intervals**: The harmonic content defining the chord quality and extensions  
/// - **Bass**: Optional bass note specification for inversions and slash chords
///
/// # Bass Note Support
///
/// The bass field enables sophisticated representation of chord voicings:
///
/// ## Root Position (Default)
/// When `bass` is `None`, the chord is in root position with the root note in the bass.
/// This is the most stable and common chord configuration.
///
/// ## Classical Inversions  
/// When `bass` contains `BassType::Inversion(n)`, the chord represents a classical
/// inversion where the nth chord tone is in the bass. This maintains the chord's
/// harmonic function while altering its stability and voice leading characteristics.
///
/// ## Slash Chords
/// When `bass` contains `BassType::Slash`, the chord represents a slash chord with
/// an arbitrary bass note. This is common in popular music and jazz for creating
/// specific harmonic colors or bass line motion.
///
/// # Practical Usage
///
/// ## Basic Chord Creation
/// ```rust
/// use chordy::{Chord, note, Interval};
///
/// // Simple triad construction
/// let c_major = Chord::major(note!("C"));
/// let d_minor = Chord::minor(note!("D"));
/// let g_dominant7 = Chord::dominant_7th(note!("G"));
///
/// // Custom chord from intervals
/// let custom = Chord::new(note!("F"), vec![
///     Interval::PERFECT_UNISON,
///     Interval::MAJOR_THIRD,
///     Interval::AUGMENTED_FIFTH,  // F augmented
/// ]);
/// ```
///
/// ## Bass Note Operations
/// ```rust
/// use chordy::{Chord, note, BassType};
///
/// let c_major = Chord::major(note!("C"));
///
/// // Classical inversions
/// let first_inversion = c_major.with_inversion(1);  // C/E
/// let second_inversion = c_major.with_inversion(2); // C/G
///
/// // Slash chords  
/// let c_over_f = c_major.with_slash_bass(note!("F")); // C/F
/// let c_over_a = c_major.with_slash_bass(note!("A")); // C/A
///
/// // Query bass properties
/// assert_eq!(first_inversion.bass_note(), note!("E"));
/// assert!(first_inversion.is_inverted());
/// assert_eq!(first_inversion.inversion_number(), Some(1));
///
/// assert_eq!(c_over_f.bass_note(), note!("F"));
/// assert!(c_over_f.is_slash_chord());
/// assert!(!c_over_f.is_inverted());
/// ```
///
/// ## Method Chaining
/// The fluent interface allows elegant chord construction:
///
/// ```rust
/// use chordy::{Chord, note, Interval};
///
/// let complex_chord = Chord::major(note!("C"))
///     .with_interval(Interval::MINOR_SEVENTH)  // Add dominant 7th
///     .with_interval(Interval::MAJOR_NINTH)    // Add 9th extension
///     .with_inversion(1);                      // First inversion
///
/// // Result: C9/E (C dominant 9th in first inversion)
/// ```
///
/// # Thread Safety and Performance
///
/// [`Chord`] implements `Copy` and is completely immutable after creation.
/// Bass note operations return new chord instances, making the API both
/// thread-safe and functional in style. The compact representation using
/// [`IntervalSet`] ensures efficient memory usage even for complex chords.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Chord {
    /// The root note of the chord
    pub root: NoteName,
    /// The intervals from the root note that define the chord.
    ///
    /// Intervals are typically in ascending order, starting from the root, which is included as a
    /// PERFECT_UNISON.
    pub intervals: IntervalSet,
    /// The bass note for inversions and slash chords
    ///
    /// When `None`, the chord is in root position (bass note is the root).
    /// When `Some((note, bass_type))`, the chord has a different bass note:
    /// - `BassType::Inversion(n)`: Classical inversion with chord tone in bass
    /// - `BassType::Slash`: Slash chord with arbitrary bass note
    pub bass: Option<(NoteName, BassType)>,
}

impl Chord {
    /// Create a new chord from root and intervals
    pub fn new(root: NoteName, intervals: Vec<Interval>) -> Self {
        Chord {
            root,
            intervals: IntervalSet::from_slice(&intervals),
            bass: None,
        }
    }

    /// Create a new chord from root and interval set
    pub fn from_interval_set(root: NoteName, intervals: IntervalSet) -> Self {
        Chord {
            root,
            intervals,
            bass: None,
        }
    }

    /// Create a chord from a list of notes
    pub fn from_notes(notes: &[NoteName]) -> Self {
        // We could get notes in any order, so we need to determine the root
        // In order to do this, we will create interval sets from each note.
        // The interval set that contains a fifth and some third will be the root.
        let candidate: Option<NoteName> = notes.first().cloned();
        let score: i32 = i32::MIN;
        notes
            .iter()
            .fold((candidate, score), |(mut candidate, mut score), note| {
                let note_intervals = notes
                    .iter()
                    .filter(|&&n| n != *note)
                    .map(|&n| note.interval_to(n))
                    .collect::<Vec<Interval>>();
                let note_score = note_intervals.iter().fold(0, |acc, interval| {
                    if interval.is_fifth() {
                        acc + 5
                    } else if interval.is_third() {
                        acc + 3
                    } else {
                        acc
                    }
                });
                match note_score.cmp(&score) {
                    // equal score, prefer lower note
                    std::cmp::Ordering::Equal => {
                        if let Some(c) = candidate {
                            if note.base_midi_number() < c.base_midi_number() {
                                candidate = Some(*note);
                            }
                        } else {
                            candidate = Some(*note);
                        }
                    }
                    std::cmp::Ordering::Greater => {
                        candidate = Some(*note);
                        score = note_score;
                    }
                    _ => {}
                }
                (candidate, score)
            });

        // if we have a candidate, create the chord
        let root = candidate.unwrap_or(notes.first().cloned().unwrap_or(note!("C")));

        Self::from_notes_and_root(notes, root)
    }

    /// Create a chord from a list of notes and a specified root
    pub fn from_notes_and_root(notes: &[NoteName], root: NoteName) -> Chord {
        Self::new(root, notes.iter().map(|&n| root.interval_to(n)).collect())
    }

    /// Create a chord from an ordered list of notes and a specified root
    ///
    /// This method assumes the notes are in ascending order and calculates
    /// compound intervals when necessary. For example, in "G,B,D,F,Ab",
    /// the Ab will be treated as a minor 9th rather than a minor 2nd.
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Chord, note};
    ///
    /// // This creates a G7♭9 chord
    /// let notes = [note!("G"), note!("B"), note!("D"), note!("F"), note!("Ab")];
    /// let chord = Chord::from_notes_ordered(&notes, note!("G"));
    /// assert_eq!(chord.abbreviated_name(), "G7♭9");
    /// ```
    pub fn from_notes_ordered(notes: &[NoteName], root: NoteName) -> Chord {
        if notes.is_empty() {
            return Self::new(root, vec![]);
        }

        let mut intervals = Vec::new();
        let mut last_semitone_position = 0; // Position of the root

        for &note in notes {
            if note == root {
                // Root note, add perfect unison
                intervals.push(Interval::PERFECT_UNISON);
                continue;
            }

            // Calculate the basic interval from root to this note
            let base_interval = root.interval_to(note);

            // Calculate how many semitones this represents (using positive modulo)
            let base_semitones = (base_interval.fifths as i32 * 7
                + base_interval.octaves as i32 * 12)
                .rem_euclid(12);
            let mut final_semitones = base_semitones;
            let mut octaves_to_add = 0;

            // If this note would be at the same position or lower than the last note,
            // move it up octaves until it's higher
            while final_semitones <= last_semitone_position {
                final_semitones += 12;
                octaves_to_add += 1;
            }

            // Create the final interval
            let final_interval = Interval {
                fifths: base_interval.fifths,
                octaves: base_interval.octaves + octaves_to_add,
            };

            intervals.push(final_interval);
            last_semitone_position = final_semitones;
        }

        Self::new(root, intervals)
    }

    /// Create a chord from an ordered list of notes, using the first note as root
    ///
    /// This is a convenience method that uses the first note as the root and
    /// applies ordered parsing to the rest.
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Chord, note};
    ///
    /// // This creates a G7♭9 chord
    /// let notes = [note!("G"), note!("B"), note!("D"), note!("F"), note!("Ab")];
    /// let chord = Chord::from_notes_ordered_root_first(&notes);
    /// assert_eq!(chord.abbreviated_name(), "G7♭9");
    /// ```
    pub fn from_notes_ordered_root_first(notes: &[NoteName]) -> Chord {
        if notes.is_empty() {
            return Self::new(note!("C"), vec![]);
        }

        let root = notes[0];
        Self::from_notes_ordered(notes, root)
    }

    /// Create a major chord with the given root note
    pub fn major(root: NoteName) -> Self {
        Self::new(
            root,
            vec![
                Interval::PERFECT_UNISON,
                Interval::MAJOR_THIRD,
                Interval::PERFECT_FIFTH,
            ],
        )
    }

    /// Create a minor chord with the given root note
    pub fn minor(root: NoteName) -> Self {
        Self::new(
            root,
            vec![
                Interval::PERFECT_UNISON,
                Interval::MINOR_THIRD,
                Interval::PERFECT_FIFTH,
            ],
        )
    }

    /// Create a diminished chord with the given root note
    pub fn diminished(root: NoteName) -> Self {
        Self::new(
            root,
            vec![
                Interval::PERFECT_UNISON,
                Interval::MINOR_THIRD,
                Interval::DIMINISHED_FIFTH,
            ],
        )
    }

    /// Create an augmented chord with the given root note
    pub fn augmented(root: NoteName) -> Self {
        Self::new(
            root,
            vec![
                Interval::PERFECT_UNISON,
                Interval::MAJOR_THIRD,
                Interval::AUGMENTED_FIFTH,
            ],
        )
    }

    /// Create a dominant 7th chord with the given root note
    pub fn dominant_7th(root: NoteName) -> Self {
        Self::new(
            root,
            vec![
                Interval::PERFECT_UNISON,
                Interval::MAJOR_THIRD,
                Interval::PERFECT_FIFTH,
                Interval::MINOR_SEVENTH,
            ],
        )
    }

    /// Create a major 7th chord with the given root note
    pub fn major_7th(root: NoteName) -> Self {
        Self::new(
            root,
            vec![
                Interval::PERFECT_UNISON,
                Interval::MAJOR_THIRD,
                Interval::PERFECT_FIFTH,
                Interval::MAJOR_SEVENTH,
            ],
        )
    }

    /// Create a minor 7th chord with the given root note
    pub fn minor_7th(root: NoteName) -> Self {
        Self::new(
            root,
            vec![
                Interval::PERFECT_UNISON,
                Interval::MINOR_THIRD,
                Interval::PERFECT_FIFTH,
                Interval::MINOR_SEVENTH,
            ],
        )
    }

    /// Create a minor-major 7th chord with the given root note
    pub fn minor_major_7th(root: NoteName) -> Self {
        Self::new(
            root,
            vec![
                Interval::PERFECT_UNISON,
                Interval::MINOR_THIRD,
                Interval::PERFECT_FIFTH,
                Interval::MAJOR_SEVENTH,
            ],
        )
    }

    /// Create a half-diminished 7th chord (minor 7th flat 5) with the given root note
    pub fn minor_7th_flat_5(root: NoteName) -> Self {
        Self::new(
            root,
            vec![
                Interval::PERFECT_UNISON,
                Interval::MINOR_THIRD,
                Interval::DIMINISHED_FIFTH,
                Interval::MINOR_SEVENTH,
            ],
        )
    }

    // More chord constructors can be added as needed...

    /// Return a Harte representation (string) of the chord
    pub fn to_harte(&self) -> String {
        todo!()
    }

    /// Parse a Harte representation (string) of the chord
    pub fn from_harte(_harte: &str) -> Self {
        todo!()
    }

    /// Returns true if the intervals contain the major third
    pub fn is_major(&self) -> bool {
        self.intervals.contains(Interval::MAJOR_THIRD)
    }

    /// Return abbreviated name of the chord.
    ///
    /// Uses the new chord naming system for consistent and accurate chord symbol generation.
    /// This method replaces the old nested if/else logic with a clean analyzer-based approach.
    /// Uses legacy compatibility rendering to maintain backward compatibility.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chordy::{Chord, note};
    ///
    /// let a_minor = Chord::from_notes_and_root(
    ///     [note!("A"), note!("C"), note!("E")].as_ref(),
    ///     note!("A")
    /// );
    ///
    /// assert_eq!(a_minor.abbreviated_name(), "Am");
    /// ```
    pub fn abbreviated_name(&self) -> String {
        // Use the new ChordAnalyzer with legacy renderer for backward compatibility
        let chord_name = self.to_chord_name();
        ChordRenderer::legacy().render(&chord_name)
    }

    /// Returns the HTML representation of the chord.
    pub fn to_html(&self) -> String {
        // Use the new ChordAnalyzer with HTML renderer
        let chord_name = self.to_chord_name();
        ChordRenderer::html().render(&chord_name)
    }

    /// Convert this chord to a roman numeral chord in the given key
    pub fn to_roman(&self, key: &super::Key) -> Option<super::RomanChord> {
        use super::RomanNumeral;

        // Calculate the interval from the key root to this chord's root
        let interval_from_key = key.root().interval_to(self.root);

        let roman_numeral: RomanNumeral = interval_from_key.into();
        let mut roman_chord =
            super::RomanChord::new(roman_numeral, self.intervals.iter().collect());

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
        key: &super::Key,
    ) -> (super::RomanNumeral, Option<super::HarmonicFunction>) {
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
    pub fn roman_in_key(&self, key: &super::Key) -> super::RomanNumeral {
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
    pub fn function_in_key(&self, key: &super::Key) -> Option<super::HarmonicFunction> {
        key.harmonic_function(self)
    }

    /// Convert this chord to a ChordName using the new naming system
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Chord, note};
    ///
    /// let g7 = Chord::dominant_7th(note!("G"));
    /// let chord_name = g7.to_chord_name();
    /// assert_eq!(format!("{}", chord_name), "G7");
    /// ```
    pub fn to_chord_name(&self) -> ChordName {
        let mut chord_name = ChordAnalyzer::analyze(self.root, self.intervals.as_slice());

        // Add bass note if present
        if let Some((bass_note, _)) = self.bass {
            chord_name = chord_name.with_bass(ChordRoot::Note(bass_note));
        }

        chord_name
    }

    /// Get the bass note of this chord
    ///
    /// Returns the actual note that would be in the bass position when the chord
    /// is played. For root position chords, this is the same as the root note.
    /// For inversions and slash chords, this is the note specified in the bass field.
    ///
    /// This method is essential for:
    /// - Voice leading analysis
    /// - Bass line construction  
    /// - Chord voicing decisions
    /// - MIDI note generation with proper bass notes
    ///
    /// # Returns
    /// - **Root position**: Returns the root note
    /// - **Inversions**: Returns the chord tone that's in the bass
    /// - **Slash chords**: Returns the arbitrary bass note
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Chord, note};
    ///
    /// // Root position - bass note equals root note
    /// let c_major = Chord::major(note!("C"));
    /// assert_eq!(c_major.bass_note(), note!("C"));
    ///
    /// // First inversion - third in bass
    /// let c_first_inversion = c_major.with_inversion(1);
    /// assert_eq!(c_first_inversion.bass_note(), note!("E"));
    ///
    /// // Second inversion - fifth in bass
    /// let c_second_inversion = c_major.with_inversion(2);
    /// assert_eq!(c_second_inversion.bass_note(), note!("G"));
    ///
    /// // Slash chord - arbitrary bass note
    /// let c_slash_f = c_major.with_slash_bass(note!("F"));
    /// assert_eq!(c_slash_f.bass_note(), note!("F"));
    /// ```
    pub fn bass_note(&self) -> NoteName {
        match self.bass {
            Some((bass, _)) => bass,
            None => self.root,
        }
    }

    /// Check if this chord is inverted
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Chord, note};
    ///
    /// let c_major = Chord::major(note!("C"));
    /// assert!(!c_major.is_inverted());
    ///
    /// let c_first_inversion = c_major.with_inversion(1);
    /// assert!(c_first_inversion.is_inverted());
    /// ```
    pub fn is_inverted(&self) -> bool {
        matches!(self.bass, Some((_, BassType::Inversion(_))))
    }

    /// Check if this chord is a slash chord
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Chord, note};
    ///
    /// let c_major = Chord::major(note!("C"));
    /// assert!(!c_major.is_slash_chord());
    ///
    /// let c_slash_g = c_major.with_slash_bass(note!("G"));
    /// assert!(c_slash_g.is_slash_chord());
    /// ```
    pub fn is_slash_chord(&self) -> bool {
        matches!(self.bass, Some((_, BassType::Slash)))
    }

    /// Get the inversion number if this is an inverted chord
    ///
    /// Returns the inversion number (1, 2, 3, etc.) if this is an inverted chord,
    /// otherwise returns None.
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Chord, note};
    ///
    /// let c_major = Chord::major(note!("C"));
    /// assert_eq!(c_major.inversion_number(), None);
    ///
    /// let c_first_inversion = c_major.with_inversion(1);
    /// assert_eq!(c_first_inversion.inversion_number(), Some(1));
    /// ```
    pub fn inversion_number(&self) -> Option<u8> {
        match self.bass {
            Some((_, BassType::Inversion(n))) => Some(n),
            _ => None,
        }
    }

    /// Create a chord with the specified inversion
    ///
    /// Creates a classical inversion by placing the nth chord tone in the bass.
    /// The inversion number corresponds to which chord tone (counting from the root)
    /// becomes the bass note.
    ///
    /// # Parameters
    /// - `inversion`: The inversion number (0 = root position, 1 = first inversion, etc.)
    ///
    /// # Inversion Mapping
    /// - **0**: Root position (returns chord unchanged)
    /// - **1**: First inversion (third in bass)
    /// - **2**: Second inversion (fifth in bass)
    /// - **3**: Third inversion (seventh in bass)
    /// - **n**: nth interval in bass (for extended chords)
    ///
    /// # Harmonic Effects
    /// - **First inversion**: Softer, more melodic bass line
    /// - **Second inversion**: Creates tension, often requires resolution
    /// - **Third inversion**: Smooth voice leading, common in jazz
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Chord, note, BassType};
    ///
    /// let c_major = Chord::major(note!("C"));
    ///
    /// // Root position (no change)
    /// let root_pos = c_major.with_inversion(0);
    /// assert_eq!(root_pos.bass_note(), note!("C"));
    /// assert!(!root_pos.is_inverted());
    ///
    /// // First inversion - third (E) in bass
    /// let first_inv = c_major.with_inversion(1);
    /// assert!(first_inv.is_inverted());
    /// assert_eq!(first_inv.bass_note(), note!("E"));
    /// assert_eq!(first_inv.inversion_number(), Some(1));
    ///
    /// // Second inversion - fifth (G) in bass
    /// let second_inv = c_major.with_inversion(2);
    /// assert_eq!(second_inv.bass_note(), note!("G"));
    /// assert_eq!(second_inv.inversion_number(), Some(2));
    ///
    /// // Works with extended chords too
    /// let c7 = Chord::dominant_7th(note!("C"));
    /// let third_inv = c7.with_inversion(3);  // Seventh (B♭) in bass
    /// assert_eq!(third_inv.bass_note(), note!("Bb"));
    /// ```
    pub fn with_inversion(mut self, inversion: u8) -> Self {
        if inversion == 0 {
            self.bass = None;
            return self;
        }

        // Find the note for this inversion
        let sorted_intervals: Vec<Interval> = self.intervals.iter().collect();
        if let Some(&interval) = sorted_intervals.get(inversion as usize) {
            let bass_note = self.root + interval;
            self.bass = Some((bass_note, BassType::Inversion(inversion)));
        }
        self
    }

    /// Create a chord with the specified slash bass note
    ///
    /// Creates a slash chord by placing any arbitrary note in the bass position.
    /// Unlike inversions, the bass note doesn't have to be a chord tone, giving
    /// complete freedom for harmonic coloring and bass line construction.
    ///
    /// # Parameters
    /// - `bass`: Any note to place in the bass position
    ///
    /// # Common Uses
    /// - **Bass line motion**: Creating smooth bass progressions
    /// - **Harmonic color**: Adding tension or color tones in the bass
    /// - **Pedal tones**: Sustaining a bass note through chord changes
    /// - **Reharmonization**: Altering chord function through bass changes
    ///
    /// # Musical Examples
    /// - **C/F**: Creates a sus4 color with F in bass
    /// - **Am/C**: Creates C6 or Am/C sound depending on context
    /// - **G/B**: Common in progressions for smooth bass motion
    /// - **D/F♯**: Creates D major with F♯ bass (first inversion sound)
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Chord, note, BassType};
    ///
    /// let c_major = Chord::major(note!("C"));
    ///
    /// // Non-chord tone in bass - creates color
    /// let c_slash_f = c_major.with_slash_bass(note!("F"));
    /// assert!(c_slash_f.is_slash_chord());
    /// assert!(!c_slash_f.is_inverted());
    /// assert_eq!(c_slash_f.bass_note(), note!("F"));
    ///
    /// // Chord tone in bass - same notes as inversion, different meaning
    /// let c_slash_e = c_major.with_slash_bass(note!("E"));
    /// let c_first_inv = c_major.with_inversion(1);
    ///
    /// // Same bass note, different theoretical treatment
    /// assert_eq!(c_slash_e.bass_note(), c_first_inv.bass_note());
    /// assert!(c_slash_e.is_slash_chord());
    /// assert!(c_first_inv.is_inverted());
    ///
    /// // Common bass line progression
    /// let g_major = Chord::major(note!("G"));
    /// let g_over_b = g_major.with_slash_bass(note!("B"));  // G/B
    /// let c_major = Chord::major(note!("C"));              // C
    /// // Creates smooth B→C bass motion
    /// ```
    pub fn with_slash_bass(mut self, bass: NoteName) -> Self {
        // If bass note is the same as root, it's just root position, not a slash chord
        if bass == self.root {
            self.bass = None;
        } else {
            self.bass = Some((bass, BassType::Slash));
        }
        self
    }

    /// Create a chord in first inversion
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Chord, note};
    ///
    /// let c_major = Chord::major(note!("C"));
    /// let c_first_inversion = c_major.in_first_inversion();
    /// assert_eq!(c_first_inversion.bass_note(), note!("E"));
    /// ```
    pub fn in_first_inversion(self) -> Self {
        self.with_inversion(1)
    }

    /// Create a chord in second inversion
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Chord, note};
    ///
    /// let c_major = Chord::major(note!("C"));
    /// let c_second_inversion = c_major.in_second_inversion();
    /// assert_eq!(c_second_inversion.bass_note(), note!("G"));
    /// ```
    pub fn in_second_inversion(self) -> Self {
        self.with_inversion(2)
    }

    // Fluent interface methods for method chaining

    /// Add an interval to this chord (fluent interface)
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Chord, Interval, note};
    ///
    /// let chord = Chord::major(note!("C"))
    ///     .with_interval(Interval::MINOR_SEVENTH);
    /// // Creates a C7 chord
    /// ```
    pub fn with_interval(mut self, interval: Interval) -> Self {
        if !self.intervals.contains(interval) {
            self.intervals.push(interval);
            self.intervals.sort();
        }
        self
    }

    /// Remove an interval from this chord (fluent interface)
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Chord, Interval, note};
    ///
    /// let chord = Chord::major(note!("C"))
    ///     .without_interval(Interval::PERFECT_FIFTH);
    /// // Creates a C chord without the fifth (C power chord)
    /// ```
    pub fn without_interval(self, interval: Interval) -> Self {
        let mut new_intervals = IntervalSet::new();
        for i in self.intervals.iter() {
            if i != interval {
                new_intervals.push(i);
            }
        }
        Self::from_interval_set(self.root, new_intervals)
    }

    /// Transpose this chord (fluent interface)
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Chord, Interval, note};
    ///
    /// let chord = Chord::major(note!("C"))
    ///     .transposed_by(Interval::MAJOR_THIRD);
    /// // Creates an E major chord
    /// ```
    pub fn transposed_by(mut self, interval: Interval) -> Self {
        use crate::traits::Transposable;
        self.transpose(interval);
        self
    }

    /// Convert to a different chord type while keeping the root (fluent interface)
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Chord, note};
    ///
    /// let minor_chord = Chord::major(note!("C"))
    ///     .as_minor();
    /// // Converts C major to C minor
    /// ```
    pub fn as_minor(self) -> Self {
        Chord::minor(self.root)
    }

    /// Convert to major chord while keeping the root (fluent interface)
    pub fn as_major(self) -> Self {
        Chord::major(self.root)
    }

    /// Convert to diminished chord while keeping the root (fluent interface)
    pub fn as_diminished(self) -> Self {
        Chord::diminished(self.root)
    }

    /// Convert to augmented chord while keeping the root (fluent interface)
    pub fn as_augmented(self) -> Self {
        Chord::augmented(self.root)
    }

    /// Returns the pitches of this chord at the specified octave
    ///
    /// This method ensures proper voice leading by placing each pitch at or above
    /// the previous pitch. The root note is always at the specified octave, and
    /// subsequent notes are placed in ascending order, crossing into higher octaves
    /// as needed.
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Chord, note, Letter, Accidental};
    ///
    /// let c_major = Chord::major(note!("C"));
    /// let pitches = c_major.pitches(4);
    /// // Returns [C4, E4, G4]
    ///
    /// let d_minor_7 = Chord::minor_7th(note!("D"));
    /// let pitches = d_minor_7.pitches(3);
    /// // Returns [D3, F3, A3, C4] - note the C crosses to octave 4
    /// ```
    pub fn pitches(&self, octave: i8) -> Vec<super::Pitch> {
        let mut pitches = Vec::new();
        let mut last_midi = None;

        for interval in self.intervals.iter() {
            let note = self.root + interval;
            let mut pitch_octave = octave + interval.octaves();

            // Calculate MIDI number for this pitch
            let mut midi_note = note.base_midi_number() + ((pitch_octave + 2) * 12);

            // If this note would be lower than or equal to the previous note, bump it up an octave
            if let Some(last) = last_midi {
                while midi_note <= last {
                    pitch_octave += 1;
                    midi_note = note.base_midi_number() + ((pitch_octave + 2) * 12);
                }
            }

            let pitch = note.to_pitch(pitch_octave);
            pitches.push(pitch);
            last_midi = Some(midi_note);
        }

        pitches
    }

    // Voicing Methods
    //
    // These convenience methods provide easy access to the voicing system
    // without requiring users to create Voicer instances manually.

    /// Voice this chord using a custom voicing configuration
    ///
    /// This is the most flexible voicing method, allowing full control over
    /// all voicing parameters including style, range, and bass constraints.
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Chord, VoicingConfig, VoicingStyle, PitchRange, note};
    ///
    /// let chord = Chord::major(note!("C"));
    /// let config = VoicingConfig::new()
    ///     .style(VoicingStyle::Open)
    ///     .range_from("C3".parse().unwrap(), "C6".parse().unwrap());
    ///
    /// let voiced = chord.voice(&config).unwrap();
    /// assert_eq!(voiced.chord, chord);
    /// assert_eq!(voiced.info.style, VoicingStyle::Open);
    /// ```
    pub fn voice(
        &self,
        config: &super::VoicingConfig,
    ) -> Result<super::VoicedChord, super::VoicingError> {
        let voicer = super::Voicer::new(config.clone());
        voicer.voice_chord(self)
    }

    /// Voice this chord in closed position
    ///
    /// Creates a closed voicing where all notes are packed as closely as possible,
    /// typically within an octave. This is the most compact voicing style.
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Chord, note};
    ///
    /// let chord = Chord::major(note!("C"));
    /// let voiced = chord.voice_closed("C4".parse().unwrap()).unwrap();
    ///
    /// assert!(voiced.is_closed());
    /// assert_eq!(voiced.bass_pitch().unwrap().name, note!("C"));
    /// ```
    pub fn voice_closed(
        &self,
        bass_pitch: super::Pitch,
    ) -> Result<super::VoicedChord, super::VoicingError> {
        let config = super::VoicingConfig::new()
            .style(super::VoicingStyle::Closed)
            .bass_pitch(bass_pitch);
        self.voice(&config)
    }

    /// Voice this chord in open position
    ///
    /// Creates an open voicing where notes are spread across multiple octaves,
    /// providing a fuller, more spacious sound than closed voicings.
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Chord, note};
    ///
    /// let chord = Chord::major(note!("C"));
    /// let voiced = chord.voice_open("C3".parse().unwrap()).unwrap();
    ///
    /// // Open voicings typically span more than an octave
    /// assert!(voiced.span_semitones() > 12 || voiced.pitches.len() <= 2);
    /// ```
    pub fn voice_open(
        &self,
        bass_pitch: super::Pitch,
    ) -> Result<super::VoicedChord, super::VoicingError> {
        let config = super::VoicingConfig::new()
            .style(super::VoicingStyle::Open)
            .bass_pitch(bass_pitch);
        self.voice(&config)
    }

    /// Voice this chord using drop-2 voicing
    ///
    /// Drop-2 voicing takes the second-highest note from a closed voicing
    /// and drops it down an octave. This is a common jazz piano technique
    /// that creates smoother voice leading.
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Chord, note};
    ///
    /// let chord = Chord::major_7th(note!("C")); // Works best with 4+ notes  
    /// let voiced = chord.voice_drop2("C4".parse().unwrap()).unwrap();
    ///
    /// assert_eq!(voiced.info.style, chordy::VoicingStyle::Drop2);
    /// ```
    pub fn voice_drop2(
        &self,
        bass_pitch: super::Pitch,
    ) -> Result<super::VoicedChord, super::VoicingError> {
        let config = super::VoicingConfig::new()
            .style(super::VoicingStyle::Drop2)
            .bass_pitch(bass_pitch);
        self.voice(&config)
    }

    /// Voice this chord using drop-3 voicing
    ///
    /// Drop-3 voicing takes the third-highest note from a closed voicing
    /// and drops it down an octave. This creates distinctive harmonic spacing
    /// common in jazz arrangements.
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Chord, note};
    ///
    /// let chord = Chord::dominant_7th(note!("G")); // Works best with 4+ notes
    /// let voiced = chord.voice_drop3("G3".parse().unwrap()).unwrap();
    ///
    /// assert_eq!(voiced.info.style, chordy::VoicingStyle::Drop3);
    /// ```
    pub fn voice_drop3(
        &self,
        bass_pitch: super::Pitch,
    ) -> Result<super::VoicedChord, super::VoicingError> {
        let config = super::VoicingConfig::new()
            .style(super::VoicingStyle::Drop3)
            .bass_pitch(bass_pitch);
        self.voice(&config)
    }

    /// Voice this chord with custom spread constraints
    ///
    /// Creates a spread voicing where adjacent notes maintain intervals
    /// within the specified minimum and maximum range. This provides
    /// fine control over the harmonic density.
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Chord, Interval, note};
    ///
    /// let chord = Chord::major(note!("C"));
    /// let voiced = chord.voice_spread(
    ///     "C4".parse().unwrap(),
    ///     Interval::MAJOR_THIRD,  // Minimum spacing
    ///     Interval::PERFECT_FIFTH // Maximum spacing
    /// ).unwrap();
    ///
    /// // All adjacent intervals should be between major 3rd and perfect 5th
    /// let intervals = voiced.voice_intervals();
    /// for &interval in &intervals {
    ///     assert!(interval >= 4 && interval <= 7);
    /// }
    /// ```
    pub fn voice_spread(
        &self,
        bass_pitch: super::Pitch,
        min_interval: super::Interval,
        max_interval: super::Interval,
    ) -> Result<super::VoicedChord, super::VoicingError> {
        let config = super::VoicingConfig::new()
            .style(super::VoicingStyle::spread(min_interval, max_interval))
            .bass_pitch(bass_pitch);
        self.voice(&config)
    }

    /// Voice this chord for piano with default settings
    ///
    /// Uses piano-friendly voicing settings with the standard 88-key range
    /// and closed voicing style, optimized for typical piano playing.
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Chord, note};
    ///
    /// let chord = Chord::major(note!("C"));
    /// let voiced = chord.voice_for_piano().unwrap();
    ///
    /// assert_eq!(voiced.info.range, chordy::PitchRange::piano());
    /// ```
    pub fn voice_for_piano(&self) -> Result<super::VoicedChord, super::VoicingError> {
        let config = super::VoicingConfig::piano();
        self.voice(&config)
    }

    /// Voice this chord for guitar with default settings
    ///
    /// Uses guitar-friendly voicing settings with the standard guitar range
    /// and closed voicing style, optimized for typical guitar playing.
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Chord, note};
    ///
    /// let chord = Chord::major(note!("C"));
    /// let voiced = chord.voice_for_guitar().unwrap();
    ///
    /// assert_eq!(voiced.info.range, chordy::PitchRange::guitar());
    /// ```
    pub fn voice_for_guitar(&self) -> Result<super::VoicedChord, super::VoicingError> {
        let config = super::VoicingConfig::guitar();
        self.voice(&config)
    }

    /// Voice this chord for vocals with default settings
    ///
    /// Uses vocal-friendly voicing settings with a comfortable singing range
    /// and closed voicing style, suitable for choir or vocal arrangements.
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Chord, note};
    ///
    /// let chord = Chord::major(note!("C"));
    /// let voiced = chord.voice_for_vocals().unwrap();
    ///
    /// assert_eq!(voiced.info.range, chordy::PitchRange::vocal());
    /// ```
    pub fn voice_for_vocals(&self) -> Result<super::VoicedChord, super::VoicingError> {
        let config = super::VoicingConfig::vocal();
        self.voice(&config)
    }
}

impl Display for Chord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.abbreviated_name())
    }
}

impl HasRoot for Chord {
    fn root(&self) -> NoteName {
        self.root
    }

    fn root_mut(&mut self) -> &mut NoteName {
        &mut self.root
    }
}

impl HasIntervals for Chord {
    fn intervals(&self) -> &[Interval] {
        self.intervals.as_slice()
    }

    fn set_intervals(&mut self, intervals: Vec<Interval>) {
        self.intervals = intervals.into_iter().collect();
    }

    fn remove_interval(&mut self, interval: Interval) {
        let mut intervals = self.intervals.as_slice().to_vec();
        if let Some(pos) = intervals.iter().position(|&x| x == interval) {
            intervals.remove(pos);
            self.intervals = intervals.into_iter().collect();
        }
    }

    fn add_interval(&mut self, interval: Interval) {
        if !self.contains_interval(interval) {
            let mut intervals = self.intervals.as_slice().to_vec();
            intervals.push(interval);
            intervals.sort();
            intervals.dedup();
            self.intervals = intervals.into_iter().collect();
        }
    }
}

impl Invertible for Chord {
    fn inverted(&self, inversion: u8) -> Self {
        self.with_inversion(inversion)
    }
}

/// Implement FromStr for Chord to allow parsing from a string representation.
///
impl FromStr for Chord {
    type Err = ParseError;

    /// Parses a string into a Chord, currently returning an error as a placeholder.
    ///
    /// Supports only list of notes right now, where the notes are separated by comma.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chordy::prelude::*;
    ///
    /// let chord: Chord = "C,E,G".parse().unwrap();
    /// let c_major = Chord::major(note!("C"));
    ///
    /// assert_eq!(chord, c_major);
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // split the string by commas and parse each note
        let notes: Vec<NoteName> = s
            .split(',')
            .map(|note_str| note_str.trim().parse::<NoteName>())
            .collect::<Result<Vec<NoteName>, ParseError>>()?;

        if notes.is_empty() {
            return Err(ParseError::InvalidChordFormat(s.to_string()));
        }

        // Use ordered parsing with the first note as root for test compatibility
        Ok(Chord::from_notes_ordered_root_first(&notes))
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
