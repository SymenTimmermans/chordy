use super::{NoteName, ScaleType};

/// A musical key (combination of tonic and mode)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Key {
    tonic: NoteName,
    mode: Mode,  // Usually just Major or Minor
}

impl Key {
    pub fn new(tonic: NoteName, mode: Mode) -> Self {
        Key { tonic, mode }
    }
}

/// The mode of a key (Major, Minor, etc.)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Major,
    Minor,
    // etc.
}
