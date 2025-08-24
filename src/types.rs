//! This module defines the types used in the music theory library.
mod pitch;
pub use pitch::Pitch;

mod letter;
pub use letter::Letter;

mod note_name;
pub use note_name::NoteName;

mod chord;
pub use chord::{BassType, Chord, ChordQuality, HarmonicFunction};

mod accidental;
pub use accidental::Accidental;

mod scale;
pub use scale::*;

mod key;
pub use key::Key;

mod interval;
pub use interval::*;

mod interval_set;
pub use interval_set::IntervalSet;

mod roman_numeral;
pub use roman_numeral::{RomanChord, RomanDegree, RomanNumeral};

pub mod progression;
pub use progression::{
    ChordProgressionOptions, ChordRef, NodeType, ProgressionEdge, ProgressionGraph,
    ProgressionOptions, ProgressionStrength,
};

mod voicing;
pub use voicing::{
    GuitarFingering, GuitarTuning, IntervalFirstGuitarFinder, PitchRange, StringState, VoicedChord,
    Voicer, VoicingConfig, VoicingDetails, VoicingError, VoicingInfo, VoicingStyle,
};

pub mod guitar_shapes;
