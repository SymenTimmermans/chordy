//! Music theory library for Rust
//!
//! For most use cases, import the prelude:
//! ```
//! use chordy::prelude::*;
//! ```
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

pub mod types;
pub mod error;
pub mod symbols;
pub mod transposition;

/// The chordy prelude
pub mod prelude;
pub use types::*;

#[macro_export]
macro_rules! note {
    ($s:literal) => {{
        // Only do compile-time validation in non-test contexts
        #[cfg(not(test))]
        const _VALIDATE: () = {
            if !$crate::is_valid_note($s) {
                panic!(concat!(
                    "Invalid note string '", $s, "'. ",
                    "Must be a letter (A-G) followed by optional accidental (b, #, n, bb, ##)"
                ));
            }
        };
        $s.parse::<$crate::NoteName>().unwrap()
    }};
}

#[macro_export]
macro_rules! pitch {
    ($s:literal) => {{
        // Only do compile-time validation in non-test contexts
        #[cfg(not(test))]
        const _VALIDATE: () = {
            if !$crate::is_valid_pitch($s) {
                panic!(concat!(
                    "Invalid pitch string '", $s, "'. ",
                    "Must be a note (A-G with optional accidental) followed by octave number"
                ));
            }
        };
        $s.parse::<$crate::Pitch>().unwrap()
    }};
}

/// Helper function for note name validation
#[doc(hidden)]
pub const fn is_valid_note(s: &str) -> bool {
    let bytes = s.as_bytes();
    if bytes.is_empty() {
        return false;
    }

    // Validate letter
    let valid_letter = matches!(bytes[0] as char, 'C' | 'c' | 'D' | 'd' | 'E' | 'e' |
        'F' | 'f' | 'G' | 'g' | 'A' | 'a' | 'B' | 'b');

    if !valid_letter {
        return false;
    }

    // Validate accidental if present
    if bytes.len() > 1 {
        match bytes[1] as char {
            // ASCII accidentals
            'b' | '#' | 'n' => {
                // Check for double accidentals
                if bytes.len() > 2 && bytes[1] == bytes[2] {
                    matches!(bytes[1] as char, 'b' | '#') && bytes.len() == 3
                } else {
                    bytes.len() == 2
                }
            }
            // Unicode accidentals
            '♭' | '♯' => {
                // Check for double accidentals (either single char or two identical)
                if bytes.len() > 2 {
                    let next_char = bytes[2] as char;
                    (next_char == bytes[1] as char && bytes.len() == 3) ||  // Two identical singles
                    (next_char == '𝄫' || next_char == '𝄪') && bytes.len() == 4  // Single double
                } else {
                    bytes.len() == 2
                }
            }
            '♮' => bytes.len() == 2,  // Natural can't be doubled
            '𝄫' | '𝄪' => bytes.len() == 3,  // Double flat/sharp as single Unicode chars
            _ => false
        }
    } else {
        true // Natural note with no accidental
    }
}

/// Helper function for pitch string validation
#[doc(hidden)]
pub const fn is_valid_pitch(s: &str) -> bool {
    // Removed unused import
    
    let bytes = s.as_bytes();
    if bytes.is_empty() {
        return false;
    }

    // Validate letter (using direct match since from_char isn't fully const)
    let valid_letter = matches!(bytes[0] as char, 'C' | 'c' | 'D' | 'd' | 'E' | 'e' |
        'F' | 'f' | 'G' | 'g' | 'A' | 'a' | 'B' | 'b');

    if !valid_letter {
        return false;
    }

    // Validate accidental and octave
    let mut pos = 1;
    let len = bytes.len();
    
    // Check for accidental
    if pos < len {
        match bytes[pos] as char {
            'b' | '#' | 'n' => pos += 1,
            _ => {}
        }
    }

    // Check for octave
    if pos >= len {
        return false;
    }

    // Check for negative octave
    if bytes[pos] == b'-' {
        pos += 1;
        if pos >= len {
            return false;
        }
    }

    // Check remaining digits
    while pos < len {
        if !bytes[pos].is_ascii_digit() {
            return false;
        }
        pos += 1;
    }

    true
}


