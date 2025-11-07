//! Bass note operations for chords
//!
//! This module contains methods for working with chord inversions and slash chords:
//! - Classical inversion operations
//! - Slash chord operations
//! - Bass note queries and analysis

use crate::types::{BassType, Chord, Interval, NoteName};

impl Chord {
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
}