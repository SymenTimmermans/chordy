#![warn(missing_docs)]
#![doc = include_str!("../README.md")]
//! Music theory library for Rust
//!
//! ## Getting Started
//! 
//! ```rust
//! use chordy::prelude::*;
//!
//! // Create musical elements
//! let chord = Chord::major(note!("C"));
//! let scale = Scale::new(note!("C"), scales::IONIAN);
//!
//! // Use trait methods (all available via prelude)
//! let notes = chord.notes(); // From ChordLike
//! let triads = scale.triads(); // From ChordLike 
//! let transposed = chord.transposed(Interval::MAJOR_THIRD); // From Transposable
//! ```
//!
//! ## Core Concepts
//! - All musical types implement relevant traits (chords are `Transposable`, scales are `ChordLike`)
//! - Traits are automatically in scope when using `prelude::*`
//! - Macros provide compile-time safety for note creation
//!
//! # Macros
//!
//! The `pitch!` and `note!` macros provide compile-time pitch and note creation:
//! ```
//! use chordy::{pitch, note};
//!
//! // Creates a Pitch at compile time (validated during compilation)
//! let my_pitch = pitch!("C#4");
//! assert_eq!(my_pitch.to_string(), "C♯4");
//!
//! // Creates a NoteName at compile time
//! let my_note = note!("Ab");
//! assert_eq!(my_note.to_string(), "A♭");
//!
//! // Supports double accidentals
//! let double_flat = note!("Bbb");
//! assert_eq!(double_flat.to_string(), "B𝄫");
//! let double_sharp = note!("F##");
//! assert_eq!(double_sharp.to_string(), "F𝄪");
//!
//! // The following would fail to compile:
//! // let invalid_pitch = pitch!("H4");
//! // let invalid_note = note!("H#");
//! ```

pub mod error;
pub mod symbols;
pub mod traits;
pub mod transformation;
pub mod transposition;
pub mod types;

/// The chordy prelude
pub mod prelude;
pub use types::*;

/// Makes it easy to create a `NoteName` at compile time.
///
/// # Examples
///
/// ```rust
/// let note = chordy::note!("C#");
/// let c_sharp = chordy::NoteName::new(chordy::Letter::C, chordy::Accidental::Sharp);
///
/// assert_eq!(note, c_sharp);
/// ```
#[macro_export]
macro_rules! note {
    ($s:literal) => {{
        // Only do compile-time validation in non-test contexts
        #[cfg(not(test))]
        const _VALIDATE: () = {
            if !$crate::is_valid_note($s, false) {
                panic!(concat!(
                    "Invalid note string '", $s, "'. ",
                    "Must be a letter (A-G) followed by optional accidental (b, #, n, bb, ##, ♭, ♯, 𝄫, 𝄪)"
                ));
            }
        };
        $s.parse::<$crate::NoteName>().unwrap()
    }};
}

/// Makes it easy to create a `Pitch` at compile time.
///
/// # Examples
///
/// ```rust
/// use chordy::{Pitch, Accidental, Letter};
///
/// let pitch = chordy::pitch!("C#4");
/// let c_sharp_4 = Pitch::new(
///     Letter::C,
///     Accidental::Sharp,
///     4
/// );
/// assert_eq!(pitch, c_sharp_4);
///
/// ```
#[macro_export]
macro_rules! pitch {
    ($s:literal) => {{
        // Only do compile-time validation in non-test contexts
        #[cfg(not(test))]
        const _VALIDATE: () = {
            if !$crate::is_valid_note($s, true) {
                panic!(concat!(
                    "Invalid pitch string '",
                    $s,
                    "'. ",
                    "Must be a note (A-G with optional accidental) followed by octave number"
                ));
            }
        };
        $s.parse::<$crate::Pitch>().unwrap()
    }};
}

/// Helper function for note/pitch validation
#[doc(hidden)]
pub const fn is_valid_note(s: &str, check_octave: bool) -> bool {
    let bytes = s.as_bytes();
    if bytes.is_empty() {
        return false;
    }

    // Validate letter
    let valid_letter = matches!(
        bytes[0] as char,
        'C' | 'c' | 'D' | 'd' | 'E' | 'e' | 'F' | 'f' | 'G' | 'g' | 'A' | 'a' | 'B' | 'b'
    );

    if !valid_letter {
        return false;
    }

    // Find where note part ends and octave begins
    let mut note_end = 1;
    while note_end < bytes.len() {
        let c = bytes[note_end] as char;
        if c.is_ascii_digit() || c == '-' {
            break;
        }
        note_end += 1;
    }

    // Validate accidental if present
    if note_end > 1 {
        match bytes[1] as char {
            // ASCII accidentals
            'b' | '#' | 'n' => {
                // Check for double accidentals
                if note_end > 2 && bytes[1] == bytes[2] {
                    if !(bytes[1] == b'b' || bytes[1] == b'#') || note_end != 3 {
                        return false;
                    }
                } else if note_end != 2 {
                    return false;
                }
            }
            // Unicode accidentals
            '♭' | '♯' => {
                // Check for double accidentals (either single char or two identical)
                if note_end > 2 {
                    let next_char = bytes[2] as char;
                    if !((next_char == bytes[1] as char && note_end == 3) ||  // Two identical singles
                        (next_char == '𝄫' || next_char == '𝄪') && note_end == 4)
                    {
                        // Single double
                        return false;
                    }
                } else if note_end != 2 {
                    return false;
                }
            }
            '♮' => {
                if note_end != 2 {
                    return false;
                }
            }
            '𝄫' | '𝄪' => {
                if note_end != 3 {
                    return false;
                }
            }
            _ => return false,
        }
    } else if check_octave {
        return false; // Must have accidental if checking octave
    }

    // If checking octave, validate the remaining part is a valid number
    if check_octave && note_end < bytes.len() {
        let mut pos = note_end;

        // Check for negative
        if bytes[pos] == b'-' {
            pos += 1;
            if pos >= bytes.len() {
                return false;
            }
        }

        // Check digits
        while pos < bytes.len() {
            if !bytes[pos].is_ascii_digit() {
                return false;
            }
            pos += 1;
        }
    }

    true
}
