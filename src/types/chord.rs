use std::fmt;

use crate::error::ParseError;
use super::{NoteName, ChordExtension};

/// A chord with a root note and quality
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chord {
    root: NoteName,
    quality: ChordQuality,
    extensions: Vec<ChordExtension>,
}

impl Chord {
    pub fn new(root: NoteName, quality: ChordQuality, extensions: Vec<ChordExtension>) -> Self {
        Chord { root, quality, extensions }
    }
}

/// The quality/type of a chord (major, minor, etc.)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChordQuality {
    Major,
    Minor,
    Diminished,
    Augmented,
    Sus2,
    Sus4,
    // etc.
}
