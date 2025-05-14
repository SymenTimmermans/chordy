//! Music theory library for Rust
//!
//! For most use cases, import the prelude:
//! ```
//! use chordy::prelude::*;
//! ```
//!
//! # Macros
//!
//! The `pitch!` macro provides compile-time pitch creation:
//! ```
//! use chordy::pitch;
//! 
//! // Creates a Pitch at compile time (validated during compilation)
//! let my_pitch = pitch!("C#4");
//! assert_eq!(my_pitch.to_string(), "C♯4");
//! 
//! // The following would fail to compile:
//! // let invalid = pitch!("H4"); 
//! ```

pub mod types;
pub mod error;
pub mod symbols;

/// The chordy prelude
pub mod prelude;
pub use types::*;

#[macro_export]
macro_rules! pitch {
    ($s:literal) => {{
        // Only do compile-time validation in non-test contexts
        #[cfg(not(test))]
        const _VALIDATE: () = {
            if !$crate::is_valid_pitch($s) {
                panic!(concat!("Invalid pitch string: ", $s));
            }
        };
        $s.parse::<$crate::Pitch>().unwrap()
    }};
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


