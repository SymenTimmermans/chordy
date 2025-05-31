//! This module defines the types used in the music theory library.
mod pitch;
pub use pitch::Pitch;

mod letter;
pub use letter::Letter;

mod note_name;
pub use note_name::NoteName;

mod chord;
pub use chord::{Chord, HarmonicFunction};

mod accidental;
pub use accidental::Accidental;

mod scale;
pub use scale::*;

mod key;
pub use key::Key;

mod interval;
pub use interval::*;
