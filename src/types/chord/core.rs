//! Core chord structure and data types
//!
//! This module contains the fundamental chord representation including:
//! - [`Chord`] struct definition
//! - [`BassType`] enum for bass note classification
//! - Core trait implementations

use std::fmt::Display;

use super::{
    naming::{ChordAnalyzer, ChordName, ChordRenderer, ChordRoot},
};
use crate::{
    traits::{HasIntervals, HasRoot, Invertible},
    types::{Interval, IntervalSet, NoteName, Pitch},
};

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

    /// Convert this chord to a ChordName using the new naming system
    ///
    /// # Examples
    ///
    /// ```rust
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
    /// ```rust
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
    /// ```rust
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
    /// ```rust
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
    /// ```rust
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

    /// Returns the pitches of this chord at the specified octave
    ///
    /// This method ensures proper voice leading by placing each pitch at or above
    /// the previous pitch. The root note is always at the specified octave, and
    /// subsequent notes are placed in ascending order, crossing into higher octaves
    /// as needed.
    ///
    /// # Examples
    ///
    /// ```rust
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
    pub fn pitches(&self, octave: i8) -> Vec<Pitch> {
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