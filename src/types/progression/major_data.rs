//! Generated progression data for major keys from major.progression
//! Do not edit manually.

use crate::types::progression::{ProgressionEdge, NodeType};
use crate::types::{RomanChord, RomanNumeral, RomanDegree, Accidental, Interval, IntervalSet};
use crate::types::chord::BassType;
use std::collections::HashMap;

// Common interval patterns (reused across multiple chords)
/// Standard major triad intervals: root, major third, perfect fifth
const MAJOR_TRIAD_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH,
     Interval::NONE, Interval::NONE, Interval::NONE,
     Interval::NONE, Interval::NONE, Interval::NONE,
     Interval::NONE], 3);

/// Standard minor triad intervals: root, minor third, perfect fifth
const MINOR_TRIAD_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON, Interval::MINOR_THIRD, Interval::PERFECT_FIFTH,
     Interval::NONE, Interval::NONE, Interval::NONE,
     Interval::NONE, Interval::NONE, Interval::NONE,
     Interval::NONE], 3);

/// I chord - creates tension, seeks resolution (p node)
/// Intervals: perfect unison, major third, perfect fifth
pub static I: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::I, Accidental::Natural),
    intervals: MAJOR_TRIAD_SET,
    bass: None,
};

const I_6_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MAJOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::MAJOR_SIXTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 4);

/// I6 chord - creates tension, seeks resolution (p node)
/// Intervals: perfect unison, major third, perfect fifth, major sixth
pub static I_6: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::I, Accidental::Natural),
    intervals: I_6_INTERVALS_SET,
    bass: None,
};

const I_MAJ7_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MAJOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::MAJOR_SEVENTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 4);

/// Imaj7 chord - creates tension, seeks resolution (p node)
/// Intervals: perfect unison, major third, perfect fifth, major seventh
pub static I_MAJ7: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::I, Accidental::Natural),
    intervals: I_MAJ7_INTERVALS_SET,
    bass: None,
};

const I_MAJ9_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MAJOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::MAJOR_SEVENTH,
     Interval::MAJOR_NINTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 5);

/// Imaj9 chord - creates tension, seeks resolution (p node)
/// Intervals: perfect unison, major third, perfect fifth, major seventh, major ninth
pub static I_MAJ9: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::I, Accidental::Natural),
    intervals: I_MAJ9_INTERVALS_SET,
    bass: None,
};

const I_ADD9_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MAJOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::MAJOR_NINTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 4);

/// Iadd9 chord - creates tension, seeks resolution (p node)
/// Intervals: perfect unison, major third, perfect fifth, major ninth
pub static I_ADD9: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::I, Accidental::Natural),
    intervals: I_ADD9_INTERVALS_SET,
    bass: None,
};

const I_SUS2_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MAJOR_SECOND,
     Interval::PERFECT_FIFTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 3);

/// Isus2 chord - creates tension, seeks resolution (p node)
/// Intervals: perfect unison, major second, perfect fifth
pub static I_SUS2: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::I, Accidental::Natural),
    intervals: I_SUS2_INTERVALS_SET,
    bass: None,
};

const I_SUS4_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::PERFECT_FOURTH,
     Interval::PERFECT_FIFTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 3);

/// Isus4 chord - creates tension, seeks resolution (p node)
/// Intervals: perfect unison, perfect fourth, perfect fifth
pub static I_SUS4: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::I, Accidental::Natural),
    intervals: I_SUS4_INTERVALS_SET,
    bass: None,
};

/// I/5 chord - creates tension, seeks resolution (p node)
/// Intervals: perfect unison, major third, perfect fifth
pub static I_SLASH_5: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::I, Accidental::Natural),
    intervals: MAJOR_TRIAD_SET,
    bass: Some((RomanNumeral::new(RomanDegree::V, Accidental::Natural), BassType::Slash)),
};

/// I/3 chord - creates tension, seeks resolution (p node)
/// Intervals: perfect unison, major third, perfect fifth
pub static I_SLASH_3: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::I, Accidental::Natural),
    intervals: MAJOR_TRIAD_SET,
    bass: Some((RomanNumeral::new(RomanDegree::III, Accidental::Natural), BassType::Slash)),
};

/// ii chord - creates tension, seeks resolution (p node)
/// Intervals: perfect unison, minor third, perfect fifth
pub static MINOR_II: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::II, Accidental::Natural),
    intervals: MINOR_TRIAD_SET,
    bass: None,
};

const MINOR_II_M7_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MINOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::MINOR_SEVENTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 4);

/// iim7 chord - creates tension, seeks resolution (p node)
/// Intervals: perfect unison, minor third, perfect fifth, minor seventh
pub static MINOR_II_M7: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::II, Accidental::Natural),
    intervals: MINOR_II_M7_INTERVALS_SET,
    bass: None,
};

const MINOR_II_M9_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MINOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::MINOR_SEVENTH,
     Interval::MAJOR_NINTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 5);

/// iim9 chord - creates tension, seeks resolution (p node)
/// Intervals: perfect unison, minor third, perfect fifth, minor seventh, major ninth
pub static MINOR_II_M9: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::II, Accidental::Natural),
    intervals: MINOR_II_M9_INTERVALS_SET,
    bass: None,
};

/// iii chord - creates tension, seeks resolution (p node)
/// Intervals: perfect unison, minor third, perfect fifth
pub static MINOR_III: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::III, Accidental::Natural),
    intervals: MINOR_TRIAD_SET,
    bass: None,
};

const MINOR_III_M7_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MINOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::MINOR_SEVENTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 4);

/// iiim7 chord - creates tension, seeks resolution (p node)
/// Intervals: perfect unison, minor third, perfect fifth, minor seventh
pub static MINOR_III_M7: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::III, Accidental::Natural),
    intervals: MINOR_III_M7_INTERVALS_SET,
    bass: None,
};

/// IV chord - creates tension, seeks resolution (p node)
/// Intervals: perfect unison, major third, perfect fifth
pub static IV: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::IV, Accidental::Natural),
    intervals: MAJOR_TRIAD_SET,
    bass: None,
};

const IV_6_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MAJOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::MAJOR_SIXTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 4);

/// IV6 chord - creates tension, seeks resolution (p node)
/// Intervals: perfect unison, major third, perfect fifth, major sixth
pub static IV_6: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::IV, Accidental::Natural),
    intervals: IV_6_INTERVALS_SET,
    bass: None,
};

const IV_MAJ7_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MAJOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::MAJOR_SEVENTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 4);

/// IVmaj7 chord - creates tension, seeks resolution (p node)
/// Intervals: perfect unison, major third, perfect fifth, major seventh
pub static IV_MAJ7: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::IV, Accidental::Natural),
    intervals: IV_MAJ7_INTERVALS_SET,
    bass: None,
};

/// IV/1 chord - creates tension, seeks resolution (p node)
/// Intervals: perfect unison, major third, perfect fifth
pub static IV_SLASH_1: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::IV, Accidental::Natural),
    intervals: MAJOR_TRIAD_SET,
    bass: Some((RomanNumeral::new(RomanDegree::I, Accidental::Natural), BassType::Slash)),
};

/// iv chord - creates tension, seeks resolution (p node)
/// Intervals: perfect unison, minor third, perfect fifth
pub static MINOR_IV: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::IV, Accidental::Natural),
    intervals: MINOR_TRIAD_SET,
    bass: None,
};

const MINOR_IV_M6_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MINOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::MAJOR_SIXTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 4);

/// ivm6 chord - creates tension, seeks resolution (p node)
/// Intervals: perfect unison, minor third, perfect fifth, major sixth
pub static MINOR_IV_M6: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::IV, Accidental::Natural),
    intervals: MINOR_IV_M6_INTERVALS_SET,
    bass: None,
};

/// V chord - creates tension, seeks resolution (p node)
/// Intervals: perfect unison, major third, perfect fifth
pub static V: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::V, Accidental::Natural),
    intervals: MAJOR_TRIAD_SET,
    bass: None,
};

const V_7_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MAJOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::MINOR_SEVENTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 4);

/// V7 chord - creates tension, seeks resolution (p node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh
pub static V_7: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::V, Accidental::Natural),
    intervals: V_7_INTERVALS_SET,
    bass: None,
};

const V_9_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MAJOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::MINOR_SEVENTH,
     Interval::MAJOR_NINTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 5);

/// V9 chord - creates tension, seeks resolution (p node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh, major ninth
pub static V_9: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::V, Accidental::Natural),
    intervals: V_9_INTERVALS_SET,
    bass: None,
};

const V_11_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MAJOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::MINOR_SEVENTH,
     Interval::MAJOR_NINTH,
     Interval::PERFECT_ELEVENTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 6);

/// V11 chord - creates tension, seeks resolution (p node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh, major ninth, perfect eleventh
pub static V_11: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::V, Accidental::Natural),
    intervals: V_11_INTERVALS_SET,
    bass: None,
};

const V_13_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MAJOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::MINOR_SEVENTH,
     Interval::MAJOR_NINTH,
     Interval::PERFECT_ELEVENTH,
     Interval::MAJOR_THIRTEENTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 7);

/// V13 chord - creates tension, seeks resolution (p node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh, major ninth, perfect eleventh, major thirteenth
pub static V_13: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::V, Accidental::Natural),
    intervals: V_13_INTERVALS_SET,
    bass: None,
};

const V_SUS4_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::PERFECT_FOURTH,
     Interval::PERFECT_FIFTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 3);

/// Vsus4 chord - creates tension, seeks resolution (p node)
/// Intervals: perfect unison, perfect fourth, perfect fifth
pub static V_SUS4: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::V, Accidental::Natural),
    intervals: V_SUS4_INTERVALS_SET,
    bass: None,
};

/// V/1 chord - creates tension, seeks resolution (p node)
/// Intervals: perfect unison, major third, perfect fifth
pub static V_SLASH_1: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::V, Accidental::Natural),
    intervals: MAJOR_TRIAD_SET,
    bass: Some((RomanNumeral::new(RomanDegree::I, Accidental::Natural), BassType::Slash)),
};

/// vi chord - creates tension, seeks resolution (p node)
/// Intervals: perfect unison, minor third, perfect fifth
pub static MINOR_VI: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::VI, Accidental::Natural),
    intervals: MINOR_TRIAD_SET,
    bass: None,
};

const MINOR_VI_M7_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MINOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::MINOR_SEVENTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 4);

/// vim7 chord - creates tension, seeks resolution (p node)
/// Intervals: perfect unison, minor third, perfect fifth, minor seventh
pub static MINOR_VI_M7: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::VI, Accidental::Natural),
    intervals: MINOR_VI_M7_INTERVALS_SET,
    bass: None,
};

const MINOR_VI_M9_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MINOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::MINOR_SEVENTH,
     Interval::MAJOR_NINTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 5);

/// vim9 chord - creates tension, seeks resolution (p node)
/// Intervals: perfect unison, minor third, perfect fifth, minor seventh, major ninth
pub static MINOR_VI_M9: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::VI, Accidental::Natural),
    intervals: MINOR_VI_M9_INTERVALS_SET,
    bass: None,
};

const VI_7_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MAJOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::MINOR_SEVENTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 4);

/// VI7 chord - creates tension, seeks resolution (s node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh
pub static VI_7: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::VI, Accidental::Natural),
    intervals: VI_7_INTERVALS_SET,
    bass: None,
};

const VI_9_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MAJOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::MINOR_SEVENTH,
     Interval::MAJOR_NINTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 5);

/// VI9 chord - creates tension, seeks resolution (s node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh, major ninth
pub static VI_9: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::VI, Accidental::Natural),
    intervals: VI_9_INTERVALS_SET,
    bass: None,
};

const VI_FLAT_9_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MAJOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::MINOR_NINTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 4);

/// VIb9 chord - creates tension, seeks resolution (s node)
/// Intervals: perfect unison, major third, perfect fifth, minor ninth
pub static VI_FLAT_9: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::VI, Accidental::Natural),
    intervals: VI_FLAT_9_INTERVALS_SET,
    bass: None,
};

/// sIdim7 chord - creates tension, seeks resolution (s node)
/// Intervals: perfect unison, minor third, perfect fifth, minor seventh
pub static MINOR_SIDIM7: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::I, Accidental::Natural),
    intervals: MINOR_TRIAD_SET,
    bass: None,
};

/// sIVm7b5 chord - creates tension, seeks resolution (s node)
/// Intervals: perfect unison, minor third, diminished fifth, minor seventh
pub static MINOR_SIVM7FLAT_5: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::IV, Accidental::Natural),
    intervals: MINOR_TRIAD_SET,
    bass: None,
};

const II_7_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MAJOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::MINOR_SEVENTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 4);

/// II7 chord - creates tension, seeks resolution (s node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh
pub static II_7: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::II, Accidental::Natural),
    intervals: II_7_INTERVALS_SET,
    bass: None,
};

const II_9_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MAJOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::MINOR_SEVENTH,
     Interval::MAJOR_NINTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 5);

/// II9 chord - creates tension, seeks resolution (s node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh, major ninth
pub static II_9: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::II, Accidental::Natural),
    intervals: II_9_INTERVALS_SET,
    bass: None,
};

const II_FLAT_9_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MAJOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::MINOR_NINTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 4);

/// IIb9 chord - creates tension, seeks resolution (s node)
/// Intervals: perfect unison, major third, perfect fifth, minor ninth
pub static II_FLAT_9: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::II, Accidental::Natural),
    intervals: II_FLAT_9_INTERVALS_SET,
    bass: None,
};

/// VIm7b5/b3 chord - creates tension, seeks resolution (s node)
/// Intervals: perfect unison, minor third, diminished fifth, minor seventh
pub static MINOR_VIM7FLAT_5_SLASH_FLAT_3: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::VI, Accidental::Natural),
    intervals: MINOR_TRIAD_SET,
    bass: Some((RomanNumeral::new(RomanDegree::III, Accidental::Flat), BassType::Slash)),
};

/// sIIdim7 chord - creates tension, seeks resolution (s node)
/// Intervals: perfect unison, minor third, perfect fifth, minor seventh
pub static MINOR_SIIDIM7: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::II, Accidental::Natural),
    intervals: MINOR_TRIAD_SET,
    bass: None,
};

const VII_7_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MAJOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::MINOR_SEVENTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 4);

/// VII7 chord - creates tension, seeks resolution (s node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh
pub static VII_7: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::VII, Accidental::Natural),
    intervals: VII_7_INTERVALS_SET,
    bass: None,
};

const VII_9_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MAJOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::MINOR_SEVENTH,
     Interval::MAJOR_NINTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 5);

/// VII9 chord - creates tension, seeks resolution (s node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh, major ninth
pub static VII_9: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::VII, Accidental::Natural),
    intervals: VII_9_INTERVALS_SET,
    bass: None,
};

const VII_FLAT_9_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MAJOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::MINOR_NINTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 4);

/// VIIb9 chord - creates tension, seeks resolution (s node)
/// Intervals: perfect unison, major third, perfect fifth, minor ninth
pub static VII_FLAT_9: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::VII, Accidental::Natural),
    intervals: VII_FLAT_9_INTERVALS_SET,
    bass: None,
};

const MINOR_V_7_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MINOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::MINOR_SEVENTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 4);

/// v7 chord - creates tension, seeks resolution (s node)
/// Intervals: perfect unison, minor third, perfect fifth, minor seventh
pub static MINOR_V_7: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::V, Accidental::Natural),
    intervals: MINOR_V_7_INTERVALS_SET,
    bass: None,
};

const I7_9_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MAJOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::MINOR_SEVENTH,
     Interval::MAJOR_NINTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 5);

/// I79 chord - creates tension, seeks resolution (s node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh, major ninth
pub static I7_9: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::I, Accidental::Natural),
    intervals: I7_9_INTERVALS_SET,
    bass: None,
};

const I7_FLAT_9_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MAJOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::MINOR_NINTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 4);

/// I7b9 chord - creates tension, seeks resolution (s node)
/// Intervals: perfect unison, major third, perfect fifth, minor ninth
pub static I7_FLAT_9: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::I, Accidental::Natural),
    intervals: I7_FLAT_9_INTERVALS_SET,
    bass: None,
};

/// IIIm7b5 chord - creates tension, seeks resolution (s node)
/// Intervals: perfect unison, minor third, diminished fifth, minor seventh
pub static MINOR_IIIM7FLAT_5: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::III, Accidental::Natural),
    intervals: MINOR_TRIAD_SET,
    bass: None,
};

/// Im6 chord - creates tension, seeks resolution (s node)
/// Intervals: perfect unison, minor third, perfect fifth, major sixth
pub static MINOR_IM6: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::I, Accidental::Natural),
    intervals: MINOR_TRIAD_SET,
    bass: None,
};

/// V/2 chord - creates tension, seeks resolution (s node)
/// Intervals: perfect unison, major third, perfect fifth
pub static V_SLASH_2: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::V, Accidental::Natural),
    intervals: MAJOR_TRIAD_SET,
    bass: Some((RomanNumeral::new(RomanDegree::II, Accidental::Natural), BassType::Slash)),
};

const MINOR_FLAT_VII_9_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MINOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::MINOR_SEVENTH,
     Interval::MAJOR_NINTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 5);

/// bVII9 chord - creates tension, seeks resolution (s node)
/// Intervals: perfect unison, minor third, perfect fifth, minor seventh, major ninth
pub static MINOR_FLAT_VII_9: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::VII, Accidental::Natural),
    intervals: MINOR_FLAT_VII_9_INTERVALS_SET,
    bass: None,
};

/// bVII9 chord - creates tension, seeks resolution (s node)
/// Intervals: perfect unison, minor third, perfect fifth, minor seventh, major ninth
pub static MINOR_FLAT_VII9: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::VII, Accidental::Natural),
    intervals: MINOR_TRIAD_SET,
    bass: None,
};

/// bVI chord - creates tension, seeks resolution (s node)
/// Intervals: perfect unison, minor third, perfect fifth
pub static MINOR_FLAT_VI: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::VI, Accidental::Natural),
    intervals: MINOR_TRIAD_SET,
    bass: None,
};

/// bII7 chord - creates tension, seeks resolution (s node)
/// Intervals: perfect unison, minor third, perfect fifth, minor seventh
pub static MINOR_FLAT_II7: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::II, Accidental::Natural),
    intervals: MINOR_TRIAD_SET,
    bass: None,
};

/// IVm7 chord - creates tension, seeks resolution (s node)
/// Intervals: perfect unison, minor third, perfect fifth, minor seventh
pub static MINOR_IVM7: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::IV, Accidental::Natural),
    intervals: MINOR_TRIAD_SET,
    bass: None,
};

/// bVI7 chord - creates tension, seeks resolution (s node)
/// Intervals: perfect unison, minor third, perfect fifth, minor seventh
pub static MINOR_FLAT_VI7: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::VI, Accidental::Natural),
    intervals: MINOR_TRIAD_SET,
    bass: None,
};

/// Idim/b3 chord - creates tension, seeks resolution (s node)
/// Intervals: perfect unison, minor third, perfect fifth
pub static MINOR_IDIM_SLASH_FLAT_3: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::I, Accidental::Natural),
    intervals: MINOR_TRIAD_SET,
    bass: Some((RomanNumeral::new(RomanDegree::III, Accidental::Flat), BassType::Slash)),
};

const III_7_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MAJOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::MINOR_SEVENTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 4);

/// III7 chord - creates tension, seeks resolution (s node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh
pub static III_7: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::III, Accidental::Natural),
    intervals: III_7_INTERVALS_SET,
    bass: None,
};

const III_9_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MAJOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::MINOR_SEVENTH,
     Interval::MAJOR_NINTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 5);

/// III9 chord - creates tension, seeks resolution (s node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh, major ninth
pub static III_9: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::III, Accidental::Natural),
    intervals: III_9_INTERVALS_SET,
    bass: None,
};

const III_FLAT_9_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MAJOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::MINOR_NINTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 4);

/// IIIb9 chord - creates tension, seeks resolution (s node)
/// Intervals: perfect unison, major third, perfect fifth, minor ninth
pub static III_FLAT_9: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::III, Accidental::Natural),
    intervals: III_FLAT_9_INTERVALS_SET,
    bass: None,
};

/// VIIm7b5 chord - creates tension, seeks resolution (s node)
/// Intervals: perfect unison, minor third, diminished fifth, minor seventh
pub static MINOR_VIIM7FLAT_5: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::VII, Accidental::Natural),
    intervals: MINOR_TRIAD_SET,
    bass: None,
};

/// sVdim7 chord - creates tension, seeks resolution (s node)
/// Intervals: perfect unison, minor third, perfect fifth, minor seventh
pub static MINOR_SVDIM7: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::V, Accidental::Natural),
    intervals: MINOR_TRIAD_SET,
    bass: None,
};

/// Progression edge: I → IV/1
pub static EDGE_I_TO_IV_SLASH_1: ProgressionEdge = ProgressionEdge {
    from: I,
    to: IV_SLASH_1,
};

/// Progression edge: I → IV/1
pub static EDGE_I_6_TO_IV_SLASH_1: ProgressionEdge = ProgressionEdge {
    from: I_6,
    to: IV_SLASH_1,
};

/// Progression edge: I → IV/1
pub static EDGE_I_MAJ7_TO_IV_SLASH_1: ProgressionEdge = ProgressionEdge {
    from: I_MAJ7,
    to: IV_SLASH_1,
};

/// Progression edge: I → IV/1
pub static EDGE_I_MAJ9_TO_IV_SLASH_1: ProgressionEdge = ProgressionEdge {
    from: I_MAJ9,
    to: IV_SLASH_1,
};

/// Progression edge: I → IV/1
pub static EDGE_I_ADD9_TO_IV_SLASH_1: ProgressionEdge = ProgressionEdge {
    from: I_ADD9,
    to: IV_SLASH_1,
};

/// Progression edge: I → IV/1
pub static EDGE_I_SUS2_TO_IV_SLASH_1: ProgressionEdge = ProgressionEdge {
    from: I_SUS2,
    to: IV_SLASH_1,
};

/// Progression edge: I → IV/1
pub static EDGE_I_SUS4_TO_IV_SLASH_1: ProgressionEdge = ProgressionEdge {
    from: I_SUS4,
    to: IV_SLASH_1,
};

/// Progression edge: I7 → IV
pub static EDGE_I7_9_TO_IV: ProgressionEdge = ProgressionEdge {
    from: I7_9,
    to: IV,
};

/// Progression edge: I7 → IV
pub static EDGE_I7_9_TO_IV_6: ProgressionEdge = ProgressionEdge {
    from: I7_9,
    to: IV_6,
};

/// Progression edge: I7 → IV
pub static EDGE_I7_9_TO_IV_MAJ7: ProgressionEdge = ProgressionEdge {
    from: I7_9,
    to: IV_MAJ7,
};

/// Progression edge: I7 → IV
pub static EDGE_I7_FLAT_9_TO_IV: ProgressionEdge = ProgressionEdge {
    from: I7_FLAT_9,
    to: IV,
};

/// Progression edge: I7 → IV
pub static EDGE_I7_FLAT_9_TO_IV_6: ProgressionEdge = ProgressionEdge {
    from: I7_FLAT_9,
    to: IV_6,
};

/// Progression edge: I7 → IV
pub static EDGE_I7_FLAT_9_TO_IV_MAJ7: ProgressionEdge = ProgressionEdge {
    from: I7_FLAT_9,
    to: IV_MAJ7,
};

/// Progression edge: I/3 → IV
pub static EDGE_I_SLASH_3_TO_IV: ProgressionEdge = ProgressionEdge {
    from: I_SLASH_3,
    to: IV,
};

/// Progression edge: I/3 → IV
pub static EDGE_I_SLASH_3_TO_IV_6: ProgressionEdge = ProgressionEdge {
    from: I_SLASH_3,
    to: IV_6,
};

/// Progression edge: I/3 → IV
pub static EDGE_I_SLASH_3_TO_IV_MAJ7: ProgressionEdge = ProgressionEdge {
    from: I_SLASH_3,
    to: IV_MAJ7,
};

/// Progression edge: I/3 → ii
pub static EDGE_I_SLASH_3_TO_MINOR_II: ProgressionEdge = ProgressionEdge {
    from: I_SLASH_3,
    to: MINOR_II,
};

/// Progression edge: I/3 → ii
pub static EDGE_I_SLASH_3_TO_MINOR_II_M7: ProgressionEdge = ProgressionEdge {
    from: I_SLASH_3,
    to: MINOR_II_M7,
};

/// Progression edge: I/3 → ii
pub static EDGE_I_SLASH_3_TO_MINOR_II_M9: ProgressionEdge = ProgressionEdge {
    from: I_SLASH_3,
    to: MINOR_II_M9,
};

/// Progression edge: I/5 → V
pub static EDGE_I_SLASH_5_TO_V: ProgressionEdge = ProgressionEdge {
    from: I_SLASH_5,
    to: V,
};

/// Progression edge: I/5 → V
pub static EDGE_I_SLASH_5_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: I_SLASH_5,
    to: V_7,
};

/// Progression edge: I/5 → V
pub static EDGE_I_SLASH_5_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: I_SLASH_5,
    to: V_9,
};

/// Progression edge: I/5 → V
pub static EDGE_I_SLASH_5_TO_V_11: ProgressionEdge = ProgressionEdge {
    from: I_SLASH_5,
    to: V_11,
};

/// Progression edge: I/5 → V
pub static EDGE_I_SLASH_5_TO_V_13: ProgressionEdge = ProgressionEdge {
    from: I_SLASH_5,
    to: V_13,
};

/// Progression edge: I/5 → V
pub static EDGE_I_SLASH_5_TO_V_SUS4: ProgressionEdge = ProgressionEdge {
    from: I_SLASH_5,
    to: V_SUS4,
};

/// Progression edge: Idim/b3 → ii
pub static EDGE_MINOR_IDIM_SLASH_FLAT_3_TO_MINOR_II: ProgressionEdge = ProgressionEdge {
    from: MINOR_IDIM_SLASH_FLAT_3,
    to: MINOR_II,
};

/// Progression edge: Idim/b3 → ii
pub static EDGE_MINOR_IDIM_SLASH_FLAT_3_TO_MINOR_II_M7: ProgressionEdge = ProgressionEdge {
    from: MINOR_IDIM_SLASH_FLAT_3,
    to: MINOR_II_M7,
};

/// Progression edge: Idim/b3 → ii
pub static EDGE_MINOR_IDIM_SLASH_FLAT_3_TO_MINOR_II_M9: ProgressionEdge = ProgressionEdge {
    from: MINOR_IDIM_SLASH_FLAT_3,
    to: MINOR_II_M9,
};

/// Progression edge: Im6 → V/2
pub static EDGE_MINOR_IM6_TO_V_SLASH_2: ProgressionEdge = ProgressionEdge {
    from: MINOR_IM6,
    to: V_SLASH_2,
};

/// Progression edge: Im6 → II
pub static EDGE_MINOR_IM6_TO_II_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_IM6,
    to: II_7,
};

/// Progression edge: Im6 → II
pub static EDGE_MINOR_IM6_TO_II_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_IM6,
    to: II_9,
};

/// Progression edge: Im6 → II
pub static EDGE_MINOR_IM6_TO_II_FLAT_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_IM6,
    to: II_FLAT_9,
};

/// Progression edge: sIdim7 → ii
pub static EDGE_MINOR_SIDIM7_TO_MINOR_II: ProgressionEdge = ProgressionEdge {
    from: MINOR_SIDIM7,
    to: MINOR_II,
};

/// Progression edge: sIdim7 → ii
pub static EDGE_MINOR_SIDIM7_TO_MINOR_II_M7: ProgressionEdge = ProgressionEdge {
    from: MINOR_SIDIM7,
    to: MINOR_II_M7,
};

/// Progression edge: sIdim7 → ii
pub static EDGE_MINOR_SIDIM7_TO_MINOR_II_M9: ProgressionEdge = ProgressionEdge {
    from: MINOR_SIDIM7,
    to: MINOR_II_M9,
};

/// Progression edge: bII7 → I
pub static EDGE_MINOR_FLAT_II7_TO_I: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_II7,
    to: I,
};

/// Progression edge: bII7 → I
pub static EDGE_MINOR_FLAT_II7_TO_I_6: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_II7,
    to: I_6,
};

/// Progression edge: bII7 → I
pub static EDGE_MINOR_FLAT_II7_TO_I_MAJ7: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_II7,
    to: I_MAJ7,
};

/// Progression edge: bII7 → I
pub static EDGE_MINOR_FLAT_II7_TO_I_MAJ9: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_II7,
    to: I_MAJ9,
};

/// Progression edge: bII7 → I
pub static EDGE_MINOR_FLAT_II7_TO_I_ADD9: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_II7,
    to: I_ADD9,
};

/// Progression edge: bII7 → I
pub static EDGE_MINOR_FLAT_II7_TO_I_SUS2: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_II7,
    to: I_SUS2,
};

/// Progression edge: bII7 → I
pub static EDGE_MINOR_FLAT_II7_TO_I_SUS4: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_II7,
    to: I_SUS4,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_TO_V: ProgressionEdge = ProgressionEdge {
    from: MINOR_II,
    to: V,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_II,
    to: V_7,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_II,
    to: V_9,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_TO_V_11: ProgressionEdge = ProgressionEdge {
    from: MINOR_II,
    to: V_11,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_TO_V_13: ProgressionEdge = ProgressionEdge {
    from: MINOR_II,
    to: V_13,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_TO_V_SUS4: ProgressionEdge = ProgressionEdge {
    from: MINOR_II,
    to: V_SUS4,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_M7_TO_V: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_M7,
    to: V,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_M7_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_M7,
    to: V_7,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_M7_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_M7,
    to: V_9,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_M7_TO_V_11: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_M7,
    to: V_11,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_M7_TO_V_13: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_M7,
    to: V_13,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_M7_TO_V_SUS4: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_M7,
    to: V_SUS4,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_M9_TO_V: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_M9,
    to: V,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_M9_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_M9,
    to: V_7,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_M9_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_M9,
    to: V_9,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_M9_TO_V_11: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_M9,
    to: V_11,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_M9_TO_V_13: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_M9,
    to: V_13,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_M9_TO_V_SUS4: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_M9,
    to: V_SUS4,
};

/// Progression edge: ii → iii
pub static EDGE_MINOR_II_TO_MINOR_III: ProgressionEdge = ProgressionEdge {
    from: MINOR_II,
    to: MINOR_III,
};

/// Progression edge: ii → iii
pub static EDGE_MINOR_II_TO_MINOR_III_M7: ProgressionEdge = ProgressionEdge {
    from: MINOR_II,
    to: MINOR_III_M7,
};

/// Progression edge: ii → iii
pub static EDGE_MINOR_II_M7_TO_MINOR_III: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_M7,
    to: MINOR_III,
};

/// Progression edge: ii → iii
pub static EDGE_MINOR_II_M7_TO_MINOR_III_M7: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_M7,
    to: MINOR_III_M7,
};

/// Progression edge: ii → iii
pub static EDGE_MINOR_II_M9_TO_MINOR_III: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_M9,
    to: MINOR_III,
};

/// Progression edge: ii → iii
pub static EDGE_MINOR_II_M9_TO_MINOR_III_M7: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_M9,
    to: MINOR_III_M7,
};

/// Progression edge: ii → I/3
pub static EDGE_MINOR_II_TO_I_SLASH_3: ProgressionEdge = ProgressionEdge {
    from: MINOR_II,
    to: I_SLASH_3,
};

/// Progression edge: ii → I/3
pub static EDGE_MINOR_II_M7_TO_I_SLASH_3: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_M7,
    to: I_SLASH_3,
};

/// Progression edge: ii → I/3
pub static EDGE_MINOR_II_M9_TO_I_SLASH_3: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_M9,
    to: I_SLASH_3,
};

/// Progression edge: ii → I/5
pub static EDGE_MINOR_II_TO_I_SLASH_5: ProgressionEdge = ProgressionEdge {
    from: MINOR_II,
    to: I_SLASH_5,
};

/// Progression edge: ii → I/5
pub static EDGE_MINOR_II_M7_TO_I_SLASH_5: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_M7,
    to: I_SLASH_5,
};

/// Progression edge: ii → I/5
pub static EDGE_MINOR_II_M9_TO_I_SLASH_5: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_M9,
    to: I_SLASH_5,
};

/// Progression edge: ii → IVm7
pub static EDGE_MINOR_II_TO_MINOR_IVM7: ProgressionEdge = ProgressionEdge {
    from: MINOR_II,
    to: MINOR_IVM7,
};

/// Progression edge: ii → IVm7
pub static EDGE_MINOR_II_M7_TO_MINOR_IVM7: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_M7,
    to: MINOR_IVM7,
};

/// Progression edge: ii → IVm7
pub static EDGE_MINOR_II_M9_TO_MINOR_IVM7: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_M9,
    to: MINOR_IVM7,
};

/// Progression edge: ii → bII7
pub static EDGE_MINOR_II_TO_MINOR_FLAT_II7: ProgressionEdge = ProgressionEdge {
    from: MINOR_II,
    to: MINOR_FLAT_II7,
};

/// Progression edge: ii → bII7
pub static EDGE_MINOR_II_M7_TO_MINOR_FLAT_II7: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_M7,
    to: MINOR_FLAT_II7,
};

/// Progression edge: ii → bII7
pub static EDGE_MINOR_II_M9_TO_MINOR_FLAT_II7: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_M9,
    to: MINOR_FLAT_II7,
};

/// Progression edge: II → V
pub static EDGE_II_7_TO_V: ProgressionEdge = ProgressionEdge {
    from: II_7,
    to: V,
};

/// Progression edge: II → V
pub static EDGE_II_7_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: II_7,
    to: V_7,
};

/// Progression edge: II → V
pub static EDGE_II_7_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: II_7,
    to: V_9,
};

/// Progression edge: II → V
pub static EDGE_II_7_TO_V_11: ProgressionEdge = ProgressionEdge {
    from: II_7,
    to: V_11,
};

/// Progression edge: II → V
pub static EDGE_II_7_TO_V_13: ProgressionEdge = ProgressionEdge {
    from: II_7,
    to: V_13,
};

/// Progression edge: II → V
pub static EDGE_II_7_TO_V_SUS4: ProgressionEdge = ProgressionEdge {
    from: II_7,
    to: V_SUS4,
};

/// Progression edge: II → V
pub static EDGE_II_9_TO_V: ProgressionEdge = ProgressionEdge {
    from: II_9,
    to: V,
};

/// Progression edge: II → V
pub static EDGE_II_9_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: II_9,
    to: V_7,
};

/// Progression edge: II → V
pub static EDGE_II_9_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: II_9,
    to: V_9,
};

/// Progression edge: II → V
pub static EDGE_II_9_TO_V_11: ProgressionEdge = ProgressionEdge {
    from: II_9,
    to: V_11,
};

/// Progression edge: II → V
pub static EDGE_II_9_TO_V_13: ProgressionEdge = ProgressionEdge {
    from: II_9,
    to: V_13,
};

/// Progression edge: II → V
pub static EDGE_II_9_TO_V_SUS4: ProgressionEdge = ProgressionEdge {
    from: II_9,
    to: V_SUS4,
};

/// Progression edge: II → V
pub static EDGE_II_FLAT_9_TO_V: ProgressionEdge = ProgressionEdge {
    from: II_FLAT_9,
    to: V,
};

/// Progression edge: II → V
pub static EDGE_II_FLAT_9_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: II_FLAT_9,
    to: V_7,
};

/// Progression edge: II → V
pub static EDGE_II_FLAT_9_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: II_FLAT_9,
    to: V_9,
};

/// Progression edge: II → V
pub static EDGE_II_FLAT_9_TO_V_11: ProgressionEdge = ProgressionEdge {
    from: II_FLAT_9,
    to: V_11,
};

/// Progression edge: II → V
pub static EDGE_II_FLAT_9_TO_V_13: ProgressionEdge = ProgressionEdge {
    from: II_FLAT_9,
    to: V_13,
};

/// Progression edge: II → V
pub static EDGE_II_FLAT_9_TO_V_SUS4: ProgressionEdge = ProgressionEdge {
    from: II_FLAT_9,
    to: V_SUS4,
};

/// Progression edge: sIIdim7 → iii
pub static EDGE_MINOR_SIIDIM7_TO_MINOR_III: ProgressionEdge = ProgressionEdge {
    from: MINOR_SIIDIM7,
    to: MINOR_III,
};

/// Progression edge: sIIdim7 → iii
pub static EDGE_MINOR_SIIDIM7_TO_MINOR_III_M7: ProgressionEdge = ProgressionEdge {
    from: MINOR_SIIDIM7,
    to: MINOR_III_M7,
};

/// Progression edge: iii → I
pub static EDGE_MINOR_III_TO_I: ProgressionEdge = ProgressionEdge {
    from: MINOR_III,
    to: I,
};

/// Progression edge: iii → I
pub static EDGE_MINOR_III_TO_I_6: ProgressionEdge = ProgressionEdge {
    from: MINOR_III,
    to: I_6,
};

/// Progression edge: iii → I
pub static EDGE_MINOR_III_TO_I_MAJ7: ProgressionEdge = ProgressionEdge {
    from: MINOR_III,
    to: I_MAJ7,
};

/// Progression edge: iii → I
pub static EDGE_MINOR_III_TO_I_MAJ9: ProgressionEdge = ProgressionEdge {
    from: MINOR_III,
    to: I_MAJ9,
};

/// Progression edge: iii → I
pub static EDGE_MINOR_III_TO_I_ADD9: ProgressionEdge = ProgressionEdge {
    from: MINOR_III,
    to: I_ADD9,
};

/// Progression edge: iii → I
pub static EDGE_MINOR_III_TO_I_SUS2: ProgressionEdge = ProgressionEdge {
    from: MINOR_III,
    to: I_SUS2,
};

/// Progression edge: iii → I
pub static EDGE_MINOR_III_TO_I_SUS4: ProgressionEdge = ProgressionEdge {
    from: MINOR_III,
    to: I_SUS4,
};

/// Progression edge: iii → I
pub static EDGE_MINOR_III_M7_TO_I: ProgressionEdge = ProgressionEdge {
    from: MINOR_III_M7,
    to: I,
};

/// Progression edge: iii → I
pub static EDGE_MINOR_III_M7_TO_I_6: ProgressionEdge = ProgressionEdge {
    from: MINOR_III_M7,
    to: I_6,
};

/// Progression edge: iii → I
pub static EDGE_MINOR_III_M7_TO_I_MAJ7: ProgressionEdge = ProgressionEdge {
    from: MINOR_III_M7,
    to: I_MAJ7,
};

/// Progression edge: iii → I
pub static EDGE_MINOR_III_M7_TO_I_MAJ9: ProgressionEdge = ProgressionEdge {
    from: MINOR_III_M7,
    to: I_MAJ9,
};

/// Progression edge: iii → I
pub static EDGE_MINOR_III_M7_TO_I_ADD9: ProgressionEdge = ProgressionEdge {
    from: MINOR_III_M7,
    to: I_ADD9,
};

/// Progression edge: iii → I
pub static EDGE_MINOR_III_M7_TO_I_SUS2: ProgressionEdge = ProgressionEdge {
    from: MINOR_III_M7,
    to: I_SUS2,
};

/// Progression edge: iii → I
pub static EDGE_MINOR_III_M7_TO_I_SUS4: ProgressionEdge = ProgressionEdge {
    from: MINOR_III_M7,
    to: I_SUS4,
};

/// Progression edge: iii → IV
pub static EDGE_MINOR_III_TO_IV: ProgressionEdge = ProgressionEdge {
    from: MINOR_III,
    to: IV,
};

/// Progression edge: iii → IV
pub static EDGE_MINOR_III_TO_IV_6: ProgressionEdge = ProgressionEdge {
    from: MINOR_III,
    to: IV_6,
};

/// Progression edge: iii → IV
pub static EDGE_MINOR_III_TO_IV_MAJ7: ProgressionEdge = ProgressionEdge {
    from: MINOR_III,
    to: IV_MAJ7,
};

/// Progression edge: iii → IV
pub static EDGE_MINOR_III_M7_TO_IV: ProgressionEdge = ProgressionEdge {
    from: MINOR_III_M7,
    to: IV,
};

/// Progression edge: iii → IV
pub static EDGE_MINOR_III_M7_TO_IV_6: ProgressionEdge = ProgressionEdge {
    from: MINOR_III_M7,
    to: IV_6,
};

/// Progression edge: iii → IV
pub static EDGE_MINOR_III_M7_TO_IV_MAJ7: ProgressionEdge = ProgressionEdge {
    from: MINOR_III_M7,
    to: IV_MAJ7,
};

/// Progression edge: iii → vi
pub static EDGE_MINOR_III_TO_MINOR_VI: ProgressionEdge = ProgressionEdge {
    from: MINOR_III,
    to: MINOR_VI,
};

/// Progression edge: iii → vi
pub static EDGE_MINOR_III_TO_MINOR_VI_M7: ProgressionEdge = ProgressionEdge {
    from: MINOR_III,
    to: MINOR_VI_M7,
};

/// Progression edge: iii → vi
pub static EDGE_MINOR_III_TO_MINOR_VI_M9: ProgressionEdge = ProgressionEdge {
    from: MINOR_III,
    to: MINOR_VI_M9,
};

/// Progression edge: iii → vi
pub static EDGE_MINOR_III_M7_TO_MINOR_VI: ProgressionEdge = ProgressionEdge {
    from: MINOR_III_M7,
    to: MINOR_VI,
};

/// Progression edge: iii → vi
pub static EDGE_MINOR_III_M7_TO_MINOR_VI_M7: ProgressionEdge = ProgressionEdge {
    from: MINOR_III_M7,
    to: MINOR_VI_M7,
};

/// Progression edge: iii → vi
pub static EDGE_MINOR_III_M7_TO_MINOR_VI_M9: ProgressionEdge = ProgressionEdge {
    from: MINOR_III_M7,
    to: MINOR_VI_M9,
};

/// Progression edge: III → vi
pub static EDGE_III_7_TO_MINOR_VI: ProgressionEdge = ProgressionEdge {
    from: III_7,
    to: MINOR_VI,
};

/// Progression edge: III → vi
pub static EDGE_III_7_TO_MINOR_VI_M7: ProgressionEdge = ProgressionEdge {
    from: III_7,
    to: MINOR_VI_M7,
};

/// Progression edge: III → vi
pub static EDGE_III_7_TO_MINOR_VI_M9: ProgressionEdge = ProgressionEdge {
    from: III_7,
    to: MINOR_VI_M9,
};

/// Progression edge: III → vi
pub static EDGE_III_9_TO_MINOR_VI: ProgressionEdge = ProgressionEdge {
    from: III_9,
    to: MINOR_VI,
};

/// Progression edge: III → vi
pub static EDGE_III_9_TO_MINOR_VI_M7: ProgressionEdge = ProgressionEdge {
    from: III_9,
    to: MINOR_VI_M7,
};

/// Progression edge: III → vi
pub static EDGE_III_9_TO_MINOR_VI_M9: ProgressionEdge = ProgressionEdge {
    from: III_9,
    to: MINOR_VI_M9,
};

/// Progression edge: III → vi
pub static EDGE_III_FLAT_9_TO_MINOR_VI: ProgressionEdge = ProgressionEdge {
    from: III_FLAT_9,
    to: MINOR_VI,
};

/// Progression edge: III → vi
pub static EDGE_III_FLAT_9_TO_MINOR_VI_M7: ProgressionEdge = ProgressionEdge {
    from: III_FLAT_9,
    to: MINOR_VI_M7,
};

/// Progression edge: III → vi
pub static EDGE_III_FLAT_9_TO_MINOR_VI_M9: ProgressionEdge = ProgressionEdge {
    from: III_FLAT_9,
    to: MINOR_VI_M9,
};

/// Progression edge: IIIm7b5 → VI
pub static EDGE_MINOR_IIIM7FLAT_5_TO_VI_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_IIIM7FLAT_5,
    to: VI_7,
};

/// Progression edge: IIIm7b5 → VI
pub static EDGE_MINOR_IIIM7FLAT_5_TO_VI_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_IIIM7FLAT_5,
    to: VI_9,
};

/// Progression edge: IIIm7b5 → VI
pub static EDGE_MINOR_IIIM7FLAT_5_TO_VI_FLAT_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_IIIM7FLAT_5,
    to: VI_FLAT_9,
};

/// Progression edge: IIIm7b5 → IV
pub static EDGE_MINOR_IIIM7FLAT_5_TO_IV: ProgressionEdge = ProgressionEdge {
    from: MINOR_IIIM7FLAT_5,
    to: IV,
};

/// Progression edge: IIIm7b5 → IV
pub static EDGE_MINOR_IIIM7FLAT_5_TO_IV_6: ProgressionEdge = ProgressionEdge {
    from: MINOR_IIIM7FLAT_5,
    to: IV_6,
};

/// Progression edge: IIIm7b5 → IV
pub static EDGE_MINOR_IIIM7FLAT_5_TO_IV_MAJ7: ProgressionEdge = ProgressionEdge {
    from: MINOR_IIIM7FLAT_5,
    to: IV_MAJ7,
};

/// Progression edge: IV → I
pub static EDGE_IV_TO_I: ProgressionEdge = ProgressionEdge {
    from: IV,
    to: I,
};

/// Progression edge: IV → I
pub static EDGE_IV_TO_I_6: ProgressionEdge = ProgressionEdge {
    from: IV,
    to: I_6,
};

/// Progression edge: IV → I
pub static EDGE_IV_TO_I_MAJ7: ProgressionEdge = ProgressionEdge {
    from: IV,
    to: I_MAJ7,
};

/// Progression edge: IV → I
pub static EDGE_IV_TO_I_MAJ9: ProgressionEdge = ProgressionEdge {
    from: IV,
    to: I_MAJ9,
};

/// Progression edge: IV → I
pub static EDGE_IV_TO_I_ADD9: ProgressionEdge = ProgressionEdge {
    from: IV,
    to: I_ADD9,
};

/// Progression edge: IV → I
pub static EDGE_IV_TO_I_SUS2: ProgressionEdge = ProgressionEdge {
    from: IV,
    to: I_SUS2,
};

/// Progression edge: IV → I
pub static EDGE_IV_TO_I_SUS4: ProgressionEdge = ProgressionEdge {
    from: IV,
    to: I_SUS4,
};

/// Progression edge: IV → I
pub static EDGE_IV_6_TO_I: ProgressionEdge = ProgressionEdge {
    from: IV_6,
    to: I,
};

/// Progression edge: IV → I
pub static EDGE_IV_6_TO_I_6: ProgressionEdge = ProgressionEdge {
    from: IV_6,
    to: I_6,
};

/// Progression edge: IV → I
pub static EDGE_IV_6_TO_I_MAJ7: ProgressionEdge = ProgressionEdge {
    from: IV_6,
    to: I_MAJ7,
};

/// Progression edge: IV → I
pub static EDGE_IV_6_TO_I_MAJ9: ProgressionEdge = ProgressionEdge {
    from: IV_6,
    to: I_MAJ9,
};

/// Progression edge: IV → I
pub static EDGE_IV_6_TO_I_ADD9: ProgressionEdge = ProgressionEdge {
    from: IV_6,
    to: I_ADD9,
};

/// Progression edge: IV → I
pub static EDGE_IV_6_TO_I_SUS2: ProgressionEdge = ProgressionEdge {
    from: IV_6,
    to: I_SUS2,
};

/// Progression edge: IV → I
pub static EDGE_IV_6_TO_I_SUS4: ProgressionEdge = ProgressionEdge {
    from: IV_6,
    to: I_SUS4,
};

/// Progression edge: IV → I
pub static EDGE_IV_MAJ7_TO_I: ProgressionEdge = ProgressionEdge {
    from: IV_MAJ7,
    to: I,
};

/// Progression edge: IV → I
pub static EDGE_IV_MAJ7_TO_I_6: ProgressionEdge = ProgressionEdge {
    from: IV_MAJ7,
    to: I_6,
};

/// Progression edge: IV → I
pub static EDGE_IV_MAJ7_TO_I_MAJ7: ProgressionEdge = ProgressionEdge {
    from: IV_MAJ7,
    to: I_MAJ7,
};

/// Progression edge: IV → I
pub static EDGE_IV_MAJ7_TO_I_MAJ9: ProgressionEdge = ProgressionEdge {
    from: IV_MAJ7,
    to: I_MAJ9,
};

/// Progression edge: IV → I
pub static EDGE_IV_MAJ7_TO_I_ADD9: ProgressionEdge = ProgressionEdge {
    from: IV_MAJ7,
    to: I_ADD9,
};

/// Progression edge: IV → I
pub static EDGE_IV_MAJ7_TO_I_SUS2: ProgressionEdge = ProgressionEdge {
    from: IV_MAJ7,
    to: I_SUS2,
};

/// Progression edge: IV → I
pub static EDGE_IV_MAJ7_TO_I_SUS4: ProgressionEdge = ProgressionEdge {
    from: IV_MAJ7,
    to: I_SUS4,
};

/// Progression edge: IV → V
pub static EDGE_IV_TO_V: ProgressionEdge = ProgressionEdge {
    from: IV,
    to: V,
};

/// Progression edge: IV → V
pub static EDGE_IV_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: IV,
    to: V_7,
};

/// Progression edge: IV → V
pub static EDGE_IV_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: IV,
    to: V_9,
};

/// Progression edge: IV → V
pub static EDGE_IV_TO_V_11: ProgressionEdge = ProgressionEdge {
    from: IV,
    to: V_11,
};

/// Progression edge: IV → V
pub static EDGE_IV_TO_V_13: ProgressionEdge = ProgressionEdge {
    from: IV,
    to: V_13,
};

/// Progression edge: IV → V
pub static EDGE_IV_TO_V_SUS4: ProgressionEdge = ProgressionEdge {
    from: IV,
    to: V_SUS4,
};

/// Progression edge: IV → V
pub static EDGE_IV_6_TO_V: ProgressionEdge = ProgressionEdge {
    from: IV_6,
    to: V,
};

/// Progression edge: IV → V
pub static EDGE_IV_6_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: IV_6,
    to: V_7,
};

/// Progression edge: IV → V
pub static EDGE_IV_6_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: IV_6,
    to: V_9,
};

/// Progression edge: IV → V
pub static EDGE_IV_6_TO_V_11: ProgressionEdge = ProgressionEdge {
    from: IV_6,
    to: V_11,
};

/// Progression edge: IV → V
pub static EDGE_IV_6_TO_V_13: ProgressionEdge = ProgressionEdge {
    from: IV_6,
    to: V_13,
};

/// Progression edge: IV → V
pub static EDGE_IV_6_TO_V_SUS4: ProgressionEdge = ProgressionEdge {
    from: IV_6,
    to: V_SUS4,
};

/// Progression edge: IV → V
pub static EDGE_IV_MAJ7_TO_V: ProgressionEdge = ProgressionEdge {
    from: IV_MAJ7,
    to: V,
};

/// Progression edge: IV → V
pub static EDGE_IV_MAJ7_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: IV_MAJ7,
    to: V_7,
};

/// Progression edge: IV → V
pub static EDGE_IV_MAJ7_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: IV_MAJ7,
    to: V_9,
};

/// Progression edge: IV → V
pub static EDGE_IV_MAJ7_TO_V_11: ProgressionEdge = ProgressionEdge {
    from: IV_MAJ7,
    to: V_11,
};

/// Progression edge: IV → V
pub static EDGE_IV_MAJ7_TO_V_13: ProgressionEdge = ProgressionEdge {
    from: IV_MAJ7,
    to: V_13,
};

/// Progression edge: IV → V
pub static EDGE_IV_MAJ7_TO_V_SUS4: ProgressionEdge = ProgressionEdge {
    from: IV_MAJ7,
    to: V_SUS4,
};

/// Progression edge: IV → I/5
pub static EDGE_IV_TO_I_SLASH_5: ProgressionEdge = ProgressionEdge {
    from: IV,
    to: I_SLASH_5,
};

/// Progression edge: IV → I/5
pub static EDGE_IV_6_TO_I_SLASH_5: ProgressionEdge = ProgressionEdge {
    from: IV_6,
    to: I_SLASH_5,
};

/// Progression edge: IV → I/5
pub static EDGE_IV_MAJ7_TO_I_SLASH_5: ProgressionEdge = ProgressionEdge {
    from: IV_MAJ7,
    to: I_SLASH_5,
};

/// Progression edge: IV → ii
pub static EDGE_IV_TO_MINOR_II: ProgressionEdge = ProgressionEdge {
    from: IV,
    to: MINOR_II,
};

/// Progression edge: IV → ii
pub static EDGE_IV_TO_MINOR_II_M7: ProgressionEdge = ProgressionEdge {
    from: IV,
    to: MINOR_II_M7,
};

/// Progression edge: IV → ii
pub static EDGE_IV_TO_MINOR_II_M9: ProgressionEdge = ProgressionEdge {
    from: IV,
    to: MINOR_II_M9,
};

/// Progression edge: IV → ii
pub static EDGE_IV_6_TO_MINOR_II: ProgressionEdge = ProgressionEdge {
    from: IV_6,
    to: MINOR_II,
};

/// Progression edge: IV → ii
pub static EDGE_IV_6_TO_MINOR_II_M7: ProgressionEdge = ProgressionEdge {
    from: IV_6,
    to: MINOR_II_M7,
};

/// Progression edge: IV → ii
pub static EDGE_IV_6_TO_MINOR_II_M9: ProgressionEdge = ProgressionEdge {
    from: IV_6,
    to: MINOR_II_M9,
};

/// Progression edge: IV → ii
pub static EDGE_IV_MAJ7_TO_MINOR_II: ProgressionEdge = ProgressionEdge {
    from: IV_MAJ7,
    to: MINOR_II,
};

/// Progression edge: IV → ii
pub static EDGE_IV_MAJ7_TO_MINOR_II_M7: ProgressionEdge = ProgressionEdge {
    from: IV_MAJ7,
    to: MINOR_II_M7,
};

/// Progression edge: IV → ii
pub static EDGE_IV_MAJ7_TO_MINOR_II_M9: ProgressionEdge = ProgressionEdge {
    from: IV_MAJ7,
    to: MINOR_II_M9,
};

/// Progression edge: IV → I/3
pub static EDGE_IV_TO_I_SLASH_3: ProgressionEdge = ProgressionEdge {
    from: IV,
    to: I_SLASH_3,
};

/// Progression edge: IV → I/3
pub static EDGE_IV_6_TO_I_SLASH_3: ProgressionEdge = ProgressionEdge {
    from: IV_6,
    to: I_SLASH_3,
};

/// Progression edge: IV → I/3
pub static EDGE_IV_MAJ7_TO_I_SLASH_3: ProgressionEdge = ProgressionEdge {
    from: IV_MAJ7,
    to: I_SLASH_3,
};

/// Progression edge: sIVm7b5 → VII
pub static EDGE_MINOR_SIVM7FLAT_5_TO_VII_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_SIVM7FLAT_5,
    to: VII_7,
};

/// Progression edge: sIVm7b5 → VII
pub static EDGE_MINOR_SIVM7FLAT_5_TO_VII_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_SIVM7FLAT_5,
    to: VII_9,
};

/// Progression edge: sIVm7b5 → VII
pub static EDGE_MINOR_SIVM7FLAT_5_TO_VII_FLAT_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_SIVM7FLAT_5,
    to: VII_FLAT_9,
};

/// Progression edge: sIVm7b5 → V
pub static EDGE_MINOR_SIVM7FLAT_5_TO_V: ProgressionEdge = ProgressionEdge {
    from: MINOR_SIVM7FLAT_5,
    to: V,
};

/// Progression edge: sIVm7b5 → V
pub static EDGE_MINOR_SIVM7FLAT_5_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_SIVM7FLAT_5,
    to: V_7,
};

/// Progression edge: sIVm7b5 → V
pub static EDGE_MINOR_SIVM7FLAT_5_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_SIVM7FLAT_5,
    to: V_9,
};

/// Progression edge: sIVm7b5 → V
pub static EDGE_MINOR_SIVM7FLAT_5_TO_V_11: ProgressionEdge = ProgressionEdge {
    from: MINOR_SIVM7FLAT_5,
    to: V_11,
};

/// Progression edge: sIVm7b5 → V
pub static EDGE_MINOR_SIVM7FLAT_5_TO_V_13: ProgressionEdge = ProgressionEdge {
    from: MINOR_SIVM7FLAT_5,
    to: V_13,
};

/// Progression edge: sIVm7b5 → V
pub static EDGE_MINOR_SIVM7FLAT_5_TO_V_SUS4: ProgressionEdge = ProgressionEdge {
    from: MINOR_SIVM7FLAT_5,
    to: V_SUS4,
};

/// Progression edge: sIVm7b5 → I/5
pub static EDGE_MINOR_SIVM7FLAT_5_TO_I_SLASH_5: ProgressionEdge = ProgressionEdge {
    from: MINOR_SIVM7FLAT_5,
    to: I_SLASH_5,
};

/// Progression edge: V → iii
pub static EDGE_V_TO_MINOR_III: ProgressionEdge = ProgressionEdge {
    from: V,
    to: MINOR_III,
};

/// Progression edge: V → iii
pub static EDGE_V_TO_MINOR_III_M7: ProgressionEdge = ProgressionEdge {
    from: V,
    to: MINOR_III_M7,
};

/// Progression edge: V → iii
pub static EDGE_V_7_TO_MINOR_III: ProgressionEdge = ProgressionEdge {
    from: V_7,
    to: MINOR_III,
};

/// Progression edge: V → iii
pub static EDGE_V_7_TO_MINOR_III_M7: ProgressionEdge = ProgressionEdge {
    from: V_7,
    to: MINOR_III_M7,
};

/// Progression edge: V → iii
pub static EDGE_V_9_TO_MINOR_III: ProgressionEdge = ProgressionEdge {
    from: V_9,
    to: MINOR_III,
};

/// Progression edge: V → iii
pub static EDGE_V_9_TO_MINOR_III_M7: ProgressionEdge = ProgressionEdge {
    from: V_9,
    to: MINOR_III_M7,
};

/// Progression edge: V → iii
pub static EDGE_V_11_TO_MINOR_III: ProgressionEdge = ProgressionEdge {
    from: V_11,
    to: MINOR_III,
};

/// Progression edge: V → iii
pub static EDGE_V_11_TO_MINOR_III_M7: ProgressionEdge = ProgressionEdge {
    from: V_11,
    to: MINOR_III_M7,
};

/// Progression edge: V → iii
pub static EDGE_V_13_TO_MINOR_III: ProgressionEdge = ProgressionEdge {
    from: V_13,
    to: MINOR_III,
};

/// Progression edge: V → iii
pub static EDGE_V_13_TO_MINOR_III_M7: ProgressionEdge = ProgressionEdge {
    from: V_13,
    to: MINOR_III_M7,
};

/// Progression edge: V → iii
pub static EDGE_V_SUS4_TO_MINOR_III: ProgressionEdge = ProgressionEdge {
    from: V_SUS4,
    to: MINOR_III,
};

/// Progression edge: V → iii
pub static EDGE_V_SUS4_TO_MINOR_III_M7: ProgressionEdge = ProgressionEdge {
    from: V_SUS4,
    to: MINOR_III_M7,
};

/// Progression edge: V → vi
pub static EDGE_V_TO_MINOR_VI: ProgressionEdge = ProgressionEdge {
    from: V,
    to: MINOR_VI,
};

/// Progression edge: V → vi
pub static EDGE_V_TO_MINOR_VI_M7: ProgressionEdge = ProgressionEdge {
    from: V,
    to: MINOR_VI_M7,
};

/// Progression edge: V → vi
pub static EDGE_V_TO_MINOR_VI_M9: ProgressionEdge = ProgressionEdge {
    from: V,
    to: MINOR_VI_M9,
};

/// Progression edge: V → vi
pub static EDGE_V_7_TO_MINOR_VI: ProgressionEdge = ProgressionEdge {
    from: V_7,
    to: MINOR_VI,
};

/// Progression edge: V → vi
pub static EDGE_V_7_TO_MINOR_VI_M7: ProgressionEdge = ProgressionEdge {
    from: V_7,
    to: MINOR_VI_M7,
};

/// Progression edge: V → vi
pub static EDGE_V_7_TO_MINOR_VI_M9: ProgressionEdge = ProgressionEdge {
    from: V_7,
    to: MINOR_VI_M9,
};

/// Progression edge: V → vi
pub static EDGE_V_9_TO_MINOR_VI: ProgressionEdge = ProgressionEdge {
    from: V_9,
    to: MINOR_VI,
};

/// Progression edge: V → vi
pub static EDGE_V_9_TO_MINOR_VI_M7: ProgressionEdge = ProgressionEdge {
    from: V_9,
    to: MINOR_VI_M7,
};

/// Progression edge: V → vi
pub static EDGE_V_9_TO_MINOR_VI_M9: ProgressionEdge = ProgressionEdge {
    from: V_9,
    to: MINOR_VI_M9,
};

/// Progression edge: V → vi
pub static EDGE_V_11_TO_MINOR_VI: ProgressionEdge = ProgressionEdge {
    from: V_11,
    to: MINOR_VI,
};

/// Progression edge: V → vi
pub static EDGE_V_11_TO_MINOR_VI_M7: ProgressionEdge = ProgressionEdge {
    from: V_11,
    to: MINOR_VI_M7,
};

/// Progression edge: V → vi
pub static EDGE_V_11_TO_MINOR_VI_M9: ProgressionEdge = ProgressionEdge {
    from: V_11,
    to: MINOR_VI_M9,
};

/// Progression edge: V → vi
pub static EDGE_V_13_TO_MINOR_VI: ProgressionEdge = ProgressionEdge {
    from: V_13,
    to: MINOR_VI,
};

/// Progression edge: V → vi
pub static EDGE_V_13_TO_MINOR_VI_M7: ProgressionEdge = ProgressionEdge {
    from: V_13,
    to: MINOR_VI_M7,
};

/// Progression edge: V → vi
pub static EDGE_V_13_TO_MINOR_VI_M9: ProgressionEdge = ProgressionEdge {
    from: V_13,
    to: MINOR_VI_M9,
};

/// Progression edge: V → vi
pub static EDGE_V_SUS4_TO_MINOR_VI: ProgressionEdge = ProgressionEdge {
    from: V_SUS4,
    to: MINOR_VI,
};

/// Progression edge: V → vi
pub static EDGE_V_SUS4_TO_MINOR_VI_M7: ProgressionEdge = ProgressionEdge {
    from: V_SUS4,
    to: MINOR_VI_M7,
};

/// Progression edge: V → vi
pub static EDGE_V_SUS4_TO_MINOR_VI_M9: ProgressionEdge = ProgressionEdge {
    from: V_SUS4,
    to: MINOR_VI_M9,
};

/// Progression edge: V → I
pub static EDGE_V_TO_I: ProgressionEdge = ProgressionEdge {
    from: V,
    to: I,
};

/// Progression edge: V → I
pub static EDGE_V_TO_I_6: ProgressionEdge = ProgressionEdge {
    from: V,
    to: I_6,
};

/// Progression edge: V → I
pub static EDGE_V_TO_I_MAJ7: ProgressionEdge = ProgressionEdge {
    from: V,
    to: I_MAJ7,
};

/// Progression edge: V → I
pub static EDGE_V_TO_I_MAJ9: ProgressionEdge = ProgressionEdge {
    from: V,
    to: I_MAJ9,
};

/// Progression edge: V → I
pub static EDGE_V_TO_I_ADD9: ProgressionEdge = ProgressionEdge {
    from: V,
    to: I_ADD9,
};

/// Progression edge: V → I
pub static EDGE_V_TO_I_SUS2: ProgressionEdge = ProgressionEdge {
    from: V,
    to: I_SUS2,
};

/// Progression edge: V → I
pub static EDGE_V_TO_I_SUS4: ProgressionEdge = ProgressionEdge {
    from: V,
    to: I_SUS4,
};

/// Progression edge: V → I
pub static EDGE_V_7_TO_I: ProgressionEdge = ProgressionEdge {
    from: V_7,
    to: I,
};

/// Progression edge: V → I
pub static EDGE_V_7_TO_I_6: ProgressionEdge = ProgressionEdge {
    from: V_7,
    to: I_6,
};

/// Progression edge: V → I
pub static EDGE_V_7_TO_I_MAJ7: ProgressionEdge = ProgressionEdge {
    from: V_7,
    to: I_MAJ7,
};

/// Progression edge: V → I
pub static EDGE_V_7_TO_I_MAJ9: ProgressionEdge = ProgressionEdge {
    from: V_7,
    to: I_MAJ9,
};

/// Progression edge: V → I
pub static EDGE_V_7_TO_I_ADD9: ProgressionEdge = ProgressionEdge {
    from: V_7,
    to: I_ADD9,
};

/// Progression edge: V → I
pub static EDGE_V_7_TO_I_SUS2: ProgressionEdge = ProgressionEdge {
    from: V_7,
    to: I_SUS2,
};

/// Progression edge: V → I
pub static EDGE_V_7_TO_I_SUS4: ProgressionEdge = ProgressionEdge {
    from: V_7,
    to: I_SUS4,
};

/// Progression edge: V → I
pub static EDGE_V_9_TO_I: ProgressionEdge = ProgressionEdge {
    from: V_9,
    to: I,
};

/// Progression edge: V → I
pub static EDGE_V_9_TO_I_6: ProgressionEdge = ProgressionEdge {
    from: V_9,
    to: I_6,
};

/// Progression edge: V → I
pub static EDGE_V_9_TO_I_MAJ7: ProgressionEdge = ProgressionEdge {
    from: V_9,
    to: I_MAJ7,
};

/// Progression edge: V → I
pub static EDGE_V_9_TO_I_MAJ9: ProgressionEdge = ProgressionEdge {
    from: V_9,
    to: I_MAJ9,
};

/// Progression edge: V → I
pub static EDGE_V_9_TO_I_ADD9: ProgressionEdge = ProgressionEdge {
    from: V_9,
    to: I_ADD9,
};

/// Progression edge: V → I
pub static EDGE_V_9_TO_I_SUS2: ProgressionEdge = ProgressionEdge {
    from: V_9,
    to: I_SUS2,
};

/// Progression edge: V → I
pub static EDGE_V_9_TO_I_SUS4: ProgressionEdge = ProgressionEdge {
    from: V_9,
    to: I_SUS4,
};

/// Progression edge: V → I
pub static EDGE_V_11_TO_I: ProgressionEdge = ProgressionEdge {
    from: V_11,
    to: I,
};

/// Progression edge: V → I
pub static EDGE_V_11_TO_I_6: ProgressionEdge = ProgressionEdge {
    from: V_11,
    to: I_6,
};

/// Progression edge: V → I
pub static EDGE_V_11_TO_I_MAJ7: ProgressionEdge = ProgressionEdge {
    from: V_11,
    to: I_MAJ7,
};

/// Progression edge: V → I
pub static EDGE_V_11_TO_I_MAJ9: ProgressionEdge = ProgressionEdge {
    from: V_11,
    to: I_MAJ9,
};

/// Progression edge: V → I
pub static EDGE_V_11_TO_I_ADD9: ProgressionEdge = ProgressionEdge {
    from: V_11,
    to: I_ADD9,
};

/// Progression edge: V → I
pub static EDGE_V_11_TO_I_SUS2: ProgressionEdge = ProgressionEdge {
    from: V_11,
    to: I_SUS2,
};

/// Progression edge: V → I
pub static EDGE_V_11_TO_I_SUS4: ProgressionEdge = ProgressionEdge {
    from: V_11,
    to: I_SUS4,
};

/// Progression edge: V → I
pub static EDGE_V_13_TO_I: ProgressionEdge = ProgressionEdge {
    from: V_13,
    to: I,
};

/// Progression edge: V → I
pub static EDGE_V_13_TO_I_6: ProgressionEdge = ProgressionEdge {
    from: V_13,
    to: I_6,
};

/// Progression edge: V → I
pub static EDGE_V_13_TO_I_MAJ7: ProgressionEdge = ProgressionEdge {
    from: V_13,
    to: I_MAJ7,
};

/// Progression edge: V → I
pub static EDGE_V_13_TO_I_MAJ9: ProgressionEdge = ProgressionEdge {
    from: V_13,
    to: I_MAJ9,
};

/// Progression edge: V → I
pub static EDGE_V_13_TO_I_ADD9: ProgressionEdge = ProgressionEdge {
    from: V_13,
    to: I_ADD9,
};

/// Progression edge: V → I
pub static EDGE_V_13_TO_I_SUS2: ProgressionEdge = ProgressionEdge {
    from: V_13,
    to: I_SUS2,
};

/// Progression edge: V → I
pub static EDGE_V_13_TO_I_SUS4: ProgressionEdge = ProgressionEdge {
    from: V_13,
    to: I_SUS4,
};

/// Progression edge: V → I
pub static EDGE_V_SUS4_TO_I: ProgressionEdge = ProgressionEdge {
    from: V_SUS4,
    to: I,
};

/// Progression edge: V → I
pub static EDGE_V_SUS4_TO_I_6: ProgressionEdge = ProgressionEdge {
    from: V_SUS4,
    to: I_6,
};

/// Progression edge: V → I
pub static EDGE_V_SUS4_TO_I_MAJ7: ProgressionEdge = ProgressionEdge {
    from: V_SUS4,
    to: I_MAJ7,
};

/// Progression edge: V → I
pub static EDGE_V_SUS4_TO_I_MAJ9: ProgressionEdge = ProgressionEdge {
    from: V_SUS4,
    to: I_MAJ9,
};

/// Progression edge: V → I
pub static EDGE_V_SUS4_TO_I_ADD9: ProgressionEdge = ProgressionEdge {
    from: V_SUS4,
    to: I_ADD9,
};

/// Progression edge: V → I
pub static EDGE_V_SUS4_TO_I_SUS2: ProgressionEdge = ProgressionEdge {
    from: V_SUS4,
    to: I_SUS2,
};

/// Progression edge: V → I
pub static EDGE_V_SUS4_TO_I_SUS4: ProgressionEdge = ProgressionEdge {
    from: V_SUS4,
    to: I_SUS4,
};

/// Progression edge: V/1 → I
pub static EDGE_V_SLASH_1_TO_I: ProgressionEdge = ProgressionEdge {
    from: V_SLASH_1,
    to: I,
};

/// Progression edge: V/1 → I
pub static EDGE_V_SLASH_1_TO_I_6: ProgressionEdge = ProgressionEdge {
    from: V_SLASH_1,
    to: I_6,
};

/// Progression edge: V/1 → I
pub static EDGE_V_SLASH_1_TO_I_MAJ7: ProgressionEdge = ProgressionEdge {
    from: V_SLASH_1,
    to: I_MAJ7,
};

/// Progression edge: V/1 → I
pub static EDGE_V_SLASH_1_TO_I_MAJ9: ProgressionEdge = ProgressionEdge {
    from: V_SLASH_1,
    to: I_MAJ9,
};

/// Progression edge: V/1 → I
pub static EDGE_V_SLASH_1_TO_I_ADD9: ProgressionEdge = ProgressionEdge {
    from: V_SLASH_1,
    to: I_ADD9,
};

/// Progression edge: V/1 → I
pub static EDGE_V_SLASH_1_TO_I_SUS2: ProgressionEdge = ProgressionEdge {
    from: V_SLASH_1,
    to: I_SUS2,
};

/// Progression edge: V/1 → I
pub static EDGE_V_SLASH_1_TO_I_SUS4: ProgressionEdge = ProgressionEdge {
    from: V_SLASH_1,
    to: I_SUS4,
};

/// Progression edge: V/2 → II
pub static EDGE_V_SLASH_2_TO_II_7: ProgressionEdge = ProgressionEdge {
    from: V_SLASH_2,
    to: II_7,
};

/// Progression edge: V/2 → II
pub static EDGE_V_SLASH_2_TO_II_9: ProgressionEdge = ProgressionEdge {
    from: V_SLASH_2,
    to: II_9,
};

/// Progression edge: V/2 → II
pub static EDGE_V_SLASH_2_TO_II_FLAT_9: ProgressionEdge = ProgressionEdge {
    from: V_SLASH_2,
    to: II_FLAT_9,
};

/// Progression edge: v → I
pub static EDGE_MINOR_V_7_TO_I: ProgressionEdge = ProgressionEdge {
    from: MINOR_V_7,
    to: I,
};

/// Progression edge: v → I
pub static EDGE_MINOR_V_7_TO_I_6: ProgressionEdge = ProgressionEdge {
    from: MINOR_V_7,
    to: I_6,
};

/// Progression edge: v → I
pub static EDGE_MINOR_V_7_TO_I_MAJ7: ProgressionEdge = ProgressionEdge {
    from: MINOR_V_7,
    to: I_MAJ7,
};

/// Progression edge: v → I
pub static EDGE_MINOR_V_7_TO_I_MAJ9: ProgressionEdge = ProgressionEdge {
    from: MINOR_V_7,
    to: I_MAJ9,
};

/// Progression edge: v → I
pub static EDGE_MINOR_V_7_TO_I_ADD9: ProgressionEdge = ProgressionEdge {
    from: MINOR_V_7,
    to: I_ADD9,
};

/// Progression edge: v → I
pub static EDGE_MINOR_V_7_TO_I_SUS2: ProgressionEdge = ProgressionEdge {
    from: MINOR_V_7,
    to: I_SUS2,
};

/// Progression edge: v → I
pub static EDGE_MINOR_V_7_TO_I_SUS4: ProgressionEdge = ProgressionEdge {
    from: MINOR_V_7,
    to: I_SUS4,
};

/// Progression edge: sVdim7 → vi
pub static EDGE_MINOR_SVDIM7_TO_MINOR_VI: ProgressionEdge = ProgressionEdge {
    from: MINOR_SVDIM7,
    to: MINOR_VI,
};

/// Progression edge: sVdim7 → vi
pub static EDGE_MINOR_SVDIM7_TO_MINOR_VI_M7: ProgressionEdge = ProgressionEdge {
    from: MINOR_SVDIM7,
    to: MINOR_VI_M7,
};

/// Progression edge: sVdim7 → vi
pub static EDGE_MINOR_SVDIM7_TO_MINOR_VI_M9: ProgressionEdge = ProgressionEdge {
    from: MINOR_SVDIM7,
    to: MINOR_VI_M9,
};

/// Progression edge: bVI → bVII
pub static EDGE_MINOR_FLAT_VI_TO_MINOR_FLAT_VII_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VI,
    to: MINOR_FLAT_VII_9,
};

/// Progression edge: bVI7 → I/5
pub static EDGE_MINOR_FLAT_VI7_TO_I_SLASH_5: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VI7,
    to: I_SLASH_5,
};

/// Progression edge: vi → ii
pub static EDGE_MINOR_VI_TO_MINOR_II: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI,
    to: MINOR_II,
};

/// Progression edge: vi → ii
pub static EDGE_MINOR_VI_TO_MINOR_II_M7: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI,
    to: MINOR_II_M7,
};

/// Progression edge: vi → ii
pub static EDGE_MINOR_VI_TO_MINOR_II_M9: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI,
    to: MINOR_II_M9,
};

/// Progression edge: vi → ii
pub static EDGE_MINOR_VI_M7_TO_MINOR_II: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI_M7,
    to: MINOR_II,
};

/// Progression edge: vi → ii
pub static EDGE_MINOR_VI_M7_TO_MINOR_II_M7: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI_M7,
    to: MINOR_II_M7,
};

/// Progression edge: vi → ii
pub static EDGE_MINOR_VI_M7_TO_MINOR_II_M9: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI_M7,
    to: MINOR_II_M9,
};

/// Progression edge: vi → ii
pub static EDGE_MINOR_VI_M9_TO_MINOR_II: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI_M9,
    to: MINOR_II,
};

/// Progression edge: vi → ii
pub static EDGE_MINOR_VI_M9_TO_MINOR_II_M7: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI_M9,
    to: MINOR_II_M7,
};

/// Progression edge: vi → ii
pub static EDGE_MINOR_VI_M9_TO_MINOR_II_M9: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI_M9,
    to: MINOR_II_M9,
};

/// Progression edge: vi → IV
pub static EDGE_MINOR_VI_TO_IV: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI,
    to: IV,
};

/// Progression edge: vi → IV
pub static EDGE_MINOR_VI_TO_IV_6: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI,
    to: IV_6,
};

/// Progression edge: vi → IV
pub static EDGE_MINOR_VI_TO_IV_MAJ7: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI,
    to: IV_MAJ7,
};

/// Progression edge: vi → IV
pub static EDGE_MINOR_VI_M7_TO_IV: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI_M7,
    to: IV,
};

/// Progression edge: vi → IV
pub static EDGE_MINOR_VI_M7_TO_IV_6: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI_M7,
    to: IV_6,
};

/// Progression edge: vi → IV
pub static EDGE_MINOR_VI_M7_TO_IV_MAJ7: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI_M7,
    to: IV_MAJ7,
};

/// Progression edge: vi → IV
pub static EDGE_MINOR_VI_M9_TO_IV: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI_M9,
    to: IV,
};

/// Progression edge: vi → IV
pub static EDGE_MINOR_VI_M9_TO_IV_6: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI_M9,
    to: IV_6,
};

/// Progression edge: vi → IV
pub static EDGE_MINOR_VI_M9_TO_IV_MAJ7: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI_M9,
    to: IV_MAJ7,
};

/// Progression edge: VI → ii
pub static EDGE_VI_7_TO_MINOR_II: ProgressionEdge = ProgressionEdge {
    from: VI_7,
    to: MINOR_II,
};

/// Progression edge: VI → ii
pub static EDGE_VI_7_TO_MINOR_II_M7: ProgressionEdge = ProgressionEdge {
    from: VI_7,
    to: MINOR_II_M7,
};

/// Progression edge: VI → ii
pub static EDGE_VI_7_TO_MINOR_II_M9: ProgressionEdge = ProgressionEdge {
    from: VI_7,
    to: MINOR_II_M9,
};

/// Progression edge: VI → ii
pub static EDGE_VI_9_TO_MINOR_II: ProgressionEdge = ProgressionEdge {
    from: VI_9,
    to: MINOR_II,
};

/// Progression edge: VI → ii
pub static EDGE_VI_9_TO_MINOR_II_M7: ProgressionEdge = ProgressionEdge {
    from: VI_9,
    to: MINOR_II_M7,
};

/// Progression edge: VI → ii
pub static EDGE_VI_9_TO_MINOR_II_M9: ProgressionEdge = ProgressionEdge {
    from: VI_9,
    to: MINOR_II_M9,
};

/// Progression edge: VI → ii
pub static EDGE_VI_FLAT_9_TO_MINOR_II: ProgressionEdge = ProgressionEdge {
    from: VI_FLAT_9,
    to: MINOR_II,
};

/// Progression edge: VI → ii
pub static EDGE_VI_FLAT_9_TO_MINOR_II_M7: ProgressionEdge = ProgressionEdge {
    from: VI_FLAT_9,
    to: MINOR_II_M7,
};

/// Progression edge: VI → ii
pub static EDGE_VI_FLAT_9_TO_MINOR_II_M9: ProgressionEdge = ProgressionEdge {
    from: VI_FLAT_9,
    to: MINOR_II_M9,
};

/// Progression edge: VIm7b5/b3 → II
pub static EDGE_MINOR_VIM7FLAT_5_SLASH_FLAT_3_TO_II_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_VIM7FLAT_5_SLASH_FLAT_3,
    to: II_7,
};

/// Progression edge: VIm7b5/b3 → II
pub static EDGE_MINOR_VIM7FLAT_5_SLASH_FLAT_3_TO_II_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_VIM7FLAT_5_SLASH_FLAT_3,
    to: II_9,
};

/// Progression edge: VIm7b5/b3 → II
pub static EDGE_MINOR_VIM7FLAT_5_SLASH_FLAT_3_TO_II_FLAT_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_VIM7FLAT_5_SLASH_FLAT_3,
    to: II_FLAT_9,
};

/// Progression edge: bVII → I
pub static EDGE_MINOR_FLAT_VII_9_TO_I: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VII_9,
    to: I,
};

/// Progression edge: bVII → I
pub static EDGE_MINOR_FLAT_VII_9_TO_I_6: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VII_9,
    to: I_6,
};

/// Progression edge: bVII → I
pub static EDGE_MINOR_FLAT_VII_9_TO_I_MAJ7: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VII_9,
    to: I_MAJ7,
};

/// Progression edge: bVII → I
pub static EDGE_MINOR_FLAT_VII_9_TO_I_MAJ9: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VII_9,
    to: I_MAJ9,
};

/// Progression edge: bVII → I
pub static EDGE_MINOR_FLAT_VII_9_TO_I_ADD9: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VII_9,
    to: I_ADD9,
};

/// Progression edge: bVII → I
pub static EDGE_MINOR_FLAT_VII_9_TO_I_SUS2: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VII_9,
    to: I_SUS2,
};

/// Progression edge: bVII → I
pub static EDGE_MINOR_FLAT_VII_9_TO_I_SUS4: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VII_9,
    to: I_SUS4,
};

/// Progression edge: bVII9 → I/5
pub static EDGE_MINOR_FLAT_VII9_TO_I_SLASH_5: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VII9,
    to: I_SLASH_5,
};

/// Progression edge: VII → iii
pub static EDGE_VII_7_TO_MINOR_III: ProgressionEdge = ProgressionEdge {
    from: VII_7,
    to: MINOR_III,
};

/// Progression edge: VII → iii
pub static EDGE_VII_7_TO_MINOR_III_M7: ProgressionEdge = ProgressionEdge {
    from: VII_7,
    to: MINOR_III_M7,
};

/// Progression edge: VII → iii
pub static EDGE_VII_9_TO_MINOR_III: ProgressionEdge = ProgressionEdge {
    from: VII_9,
    to: MINOR_III,
};

/// Progression edge: VII → iii
pub static EDGE_VII_9_TO_MINOR_III_M7: ProgressionEdge = ProgressionEdge {
    from: VII_9,
    to: MINOR_III_M7,
};

/// Progression edge: VII → iii
pub static EDGE_VII_FLAT_9_TO_MINOR_III: ProgressionEdge = ProgressionEdge {
    from: VII_FLAT_9,
    to: MINOR_III,
};

/// Progression edge: VII → iii
pub static EDGE_VII_FLAT_9_TO_MINOR_III_M7: ProgressionEdge = ProgressionEdge {
    from: VII_FLAT_9,
    to: MINOR_III_M7,
};

/// Progression edge: VIIm7b5 → III
pub static EDGE_MINOR_VIIM7FLAT_5_TO_III_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_VIIM7FLAT_5,
    to: III_7,
};

/// Progression edge: VIIm7b5 → III
pub static EDGE_MINOR_VIIM7FLAT_5_TO_III_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_VIIM7FLAT_5,
    to: III_9,
};

/// Progression edge: VIIm7b5 → III
pub static EDGE_MINOR_VIIM7FLAT_5_TO_III_FLAT_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_VIIM7FLAT_5,
    to: III_FLAT_9,
};

/// Complete registry of all progression chords for major keys
/// 
/// Contains 61 chord variants across all harmonic functions.
/// Used internally for graph traversal and chord lookup operations.
pub static ALL_NODES: &[&RomanChord] = &[
    &I,
    &I_6,
    &I_MAJ7,
    &I_MAJ9,
    &I_ADD9,
    &I_SUS2,
    &I_SUS4,
    &I_SLASH_5,
    &I_SLASH_3,
    &MINOR_II,
    &MINOR_II_M7,
    &MINOR_II_M9,
    &MINOR_III,
    &MINOR_III_M7,
    &IV,
    &IV_6,
    &IV_MAJ7,
    &IV_SLASH_1,
    &MINOR_IV,
    &MINOR_IV_M6,
    &V,
    &V_7,
    &V_9,
    &V_11,
    &V_13,
    &V_SUS4,
    &V_SLASH_1,
    &MINOR_VI,
    &MINOR_VI_M7,
    &MINOR_VI_M9,
    &VI_7,
    &VI_9,
    &VI_FLAT_9,
    &MINOR_SIDIM7,
    &MINOR_SIVM7FLAT_5,
    &II_7,
    &II_9,
    &II_FLAT_9,
    &MINOR_VIM7FLAT_5_SLASH_FLAT_3,
    &MINOR_SIIDIM7,
    &VII_7,
    &VII_9,
    &VII_FLAT_9,
    &MINOR_V_7,
    &I7_9,
    &I7_FLAT_9,
    &MINOR_IIIM7FLAT_5,
    &MINOR_IM6,
    &V_SLASH_2,
    &MINOR_FLAT_VII_9,
    &MINOR_FLAT_VII9,
    &MINOR_FLAT_VI,
    &MINOR_FLAT_II7,
    &MINOR_IVM7,
    &MINOR_FLAT_VI7,
    &MINOR_IDIM_SLASH_FLAT_3,
    &III_7,
    &III_9,
    &III_FLAT_9,
    &MINOR_VIIM7FLAT_5,
    &MINOR_SVDIM7,
];

/// Complete registry of all progression edges for major keys
/// 
/// Contains 344 harmonic connections between chord variants.
/// Each edge represents a musically valid progression with proper voice leading.
pub static ALL_EDGES: &[&ProgressionEdge] = &[
    &EDGE_I_TO_IV_SLASH_1,
    &EDGE_I_6_TO_IV_SLASH_1,
    &EDGE_I_MAJ7_TO_IV_SLASH_1,
    &EDGE_I_MAJ9_TO_IV_SLASH_1,
    &EDGE_I_ADD9_TO_IV_SLASH_1,
    &EDGE_I_SUS2_TO_IV_SLASH_1,
    &EDGE_I_SUS4_TO_IV_SLASH_1,
    &EDGE_I7_9_TO_IV,
    &EDGE_I7_9_TO_IV_6,
    &EDGE_I7_9_TO_IV_MAJ7,
    &EDGE_I7_FLAT_9_TO_IV,
    &EDGE_I7_FLAT_9_TO_IV_6,
    &EDGE_I7_FLAT_9_TO_IV_MAJ7,
    &EDGE_I_SLASH_3_TO_IV,
    &EDGE_I_SLASH_3_TO_IV_6,
    &EDGE_I_SLASH_3_TO_IV_MAJ7,
    &EDGE_I_SLASH_3_TO_MINOR_II,
    &EDGE_I_SLASH_3_TO_MINOR_II_M7,
    &EDGE_I_SLASH_3_TO_MINOR_II_M9,
    &EDGE_I_SLASH_5_TO_V,
    &EDGE_I_SLASH_5_TO_V_7,
    &EDGE_I_SLASH_5_TO_V_9,
    &EDGE_I_SLASH_5_TO_V_11,
    &EDGE_I_SLASH_5_TO_V_13,
    &EDGE_I_SLASH_5_TO_V_SUS4,
    &EDGE_MINOR_IDIM_SLASH_FLAT_3_TO_MINOR_II,
    &EDGE_MINOR_IDIM_SLASH_FLAT_3_TO_MINOR_II_M7,
    &EDGE_MINOR_IDIM_SLASH_FLAT_3_TO_MINOR_II_M9,
    &EDGE_MINOR_IM6_TO_V_SLASH_2,
    &EDGE_MINOR_IM6_TO_II_7,
    &EDGE_MINOR_IM6_TO_II_9,
    &EDGE_MINOR_IM6_TO_II_FLAT_9,
    &EDGE_MINOR_SIDIM7_TO_MINOR_II,
    &EDGE_MINOR_SIDIM7_TO_MINOR_II_M7,
    &EDGE_MINOR_SIDIM7_TO_MINOR_II_M9,
    &EDGE_MINOR_FLAT_II7_TO_I,
    &EDGE_MINOR_FLAT_II7_TO_I_6,
    &EDGE_MINOR_FLAT_II7_TO_I_MAJ7,
    &EDGE_MINOR_FLAT_II7_TO_I_MAJ9,
    &EDGE_MINOR_FLAT_II7_TO_I_ADD9,
    &EDGE_MINOR_FLAT_II7_TO_I_SUS2,
    &EDGE_MINOR_FLAT_II7_TO_I_SUS4,
    &EDGE_MINOR_II_TO_V,
    &EDGE_MINOR_II_TO_V_7,
    &EDGE_MINOR_II_TO_V_9,
    &EDGE_MINOR_II_TO_V_11,
    &EDGE_MINOR_II_TO_V_13,
    &EDGE_MINOR_II_TO_V_SUS4,
    &EDGE_MINOR_II_M7_TO_V,
    &EDGE_MINOR_II_M7_TO_V_7,
    &EDGE_MINOR_II_M7_TO_V_9,
    &EDGE_MINOR_II_M7_TO_V_11,
    &EDGE_MINOR_II_M7_TO_V_13,
    &EDGE_MINOR_II_M7_TO_V_SUS4,
    &EDGE_MINOR_II_M9_TO_V,
    &EDGE_MINOR_II_M9_TO_V_7,
    &EDGE_MINOR_II_M9_TO_V_9,
    &EDGE_MINOR_II_M9_TO_V_11,
    &EDGE_MINOR_II_M9_TO_V_13,
    &EDGE_MINOR_II_M9_TO_V_SUS4,
    &EDGE_MINOR_II_TO_MINOR_III,
    &EDGE_MINOR_II_TO_MINOR_III_M7,
    &EDGE_MINOR_II_M7_TO_MINOR_III,
    &EDGE_MINOR_II_M7_TO_MINOR_III_M7,
    &EDGE_MINOR_II_M9_TO_MINOR_III,
    &EDGE_MINOR_II_M9_TO_MINOR_III_M7,
    &EDGE_MINOR_II_TO_I_SLASH_3,
    &EDGE_MINOR_II_M7_TO_I_SLASH_3,
    &EDGE_MINOR_II_M9_TO_I_SLASH_3,
    &EDGE_MINOR_II_TO_I_SLASH_5,
    &EDGE_MINOR_II_M7_TO_I_SLASH_5,
    &EDGE_MINOR_II_M9_TO_I_SLASH_5,
    &EDGE_MINOR_II_TO_MINOR_IVM7,
    &EDGE_MINOR_II_M7_TO_MINOR_IVM7,
    &EDGE_MINOR_II_M9_TO_MINOR_IVM7,
    &EDGE_MINOR_II_TO_MINOR_FLAT_II7,
    &EDGE_MINOR_II_M7_TO_MINOR_FLAT_II7,
    &EDGE_MINOR_II_M9_TO_MINOR_FLAT_II7,
    &EDGE_II_7_TO_V,
    &EDGE_II_7_TO_V_7,
    &EDGE_II_7_TO_V_9,
    &EDGE_II_7_TO_V_11,
    &EDGE_II_7_TO_V_13,
    &EDGE_II_7_TO_V_SUS4,
    &EDGE_II_9_TO_V,
    &EDGE_II_9_TO_V_7,
    &EDGE_II_9_TO_V_9,
    &EDGE_II_9_TO_V_11,
    &EDGE_II_9_TO_V_13,
    &EDGE_II_9_TO_V_SUS4,
    &EDGE_II_FLAT_9_TO_V,
    &EDGE_II_FLAT_9_TO_V_7,
    &EDGE_II_FLAT_9_TO_V_9,
    &EDGE_II_FLAT_9_TO_V_11,
    &EDGE_II_FLAT_9_TO_V_13,
    &EDGE_II_FLAT_9_TO_V_SUS4,
    &EDGE_MINOR_SIIDIM7_TO_MINOR_III,
    &EDGE_MINOR_SIIDIM7_TO_MINOR_III_M7,
    &EDGE_MINOR_III_TO_I,
    &EDGE_MINOR_III_TO_I_6,
    &EDGE_MINOR_III_TO_I_MAJ7,
    &EDGE_MINOR_III_TO_I_MAJ9,
    &EDGE_MINOR_III_TO_I_ADD9,
    &EDGE_MINOR_III_TO_I_SUS2,
    &EDGE_MINOR_III_TO_I_SUS4,
    &EDGE_MINOR_III_M7_TO_I,
    &EDGE_MINOR_III_M7_TO_I_6,
    &EDGE_MINOR_III_M7_TO_I_MAJ7,
    &EDGE_MINOR_III_M7_TO_I_MAJ9,
    &EDGE_MINOR_III_M7_TO_I_ADD9,
    &EDGE_MINOR_III_M7_TO_I_SUS2,
    &EDGE_MINOR_III_M7_TO_I_SUS4,
    &EDGE_MINOR_III_TO_IV,
    &EDGE_MINOR_III_TO_IV_6,
    &EDGE_MINOR_III_TO_IV_MAJ7,
    &EDGE_MINOR_III_M7_TO_IV,
    &EDGE_MINOR_III_M7_TO_IV_6,
    &EDGE_MINOR_III_M7_TO_IV_MAJ7,
    &EDGE_MINOR_III_TO_MINOR_VI,
    &EDGE_MINOR_III_TO_MINOR_VI_M7,
    &EDGE_MINOR_III_TO_MINOR_VI_M9,
    &EDGE_MINOR_III_M7_TO_MINOR_VI,
    &EDGE_MINOR_III_M7_TO_MINOR_VI_M7,
    &EDGE_MINOR_III_M7_TO_MINOR_VI_M9,
    &EDGE_III_7_TO_MINOR_VI,
    &EDGE_III_7_TO_MINOR_VI_M7,
    &EDGE_III_7_TO_MINOR_VI_M9,
    &EDGE_III_9_TO_MINOR_VI,
    &EDGE_III_9_TO_MINOR_VI_M7,
    &EDGE_III_9_TO_MINOR_VI_M9,
    &EDGE_III_FLAT_9_TO_MINOR_VI,
    &EDGE_III_FLAT_9_TO_MINOR_VI_M7,
    &EDGE_III_FLAT_9_TO_MINOR_VI_M9,
    &EDGE_MINOR_IIIM7FLAT_5_TO_VI_7,
    &EDGE_MINOR_IIIM7FLAT_5_TO_VI_9,
    &EDGE_MINOR_IIIM7FLAT_5_TO_VI_FLAT_9,
    &EDGE_MINOR_IIIM7FLAT_5_TO_IV,
    &EDGE_MINOR_IIIM7FLAT_5_TO_IV_6,
    &EDGE_MINOR_IIIM7FLAT_5_TO_IV_MAJ7,
    &EDGE_IV_TO_I,
    &EDGE_IV_TO_I_6,
    &EDGE_IV_TO_I_MAJ7,
    &EDGE_IV_TO_I_MAJ9,
    &EDGE_IV_TO_I_ADD9,
    &EDGE_IV_TO_I_SUS2,
    &EDGE_IV_TO_I_SUS4,
    &EDGE_IV_6_TO_I,
    &EDGE_IV_6_TO_I_6,
    &EDGE_IV_6_TO_I_MAJ7,
    &EDGE_IV_6_TO_I_MAJ9,
    &EDGE_IV_6_TO_I_ADD9,
    &EDGE_IV_6_TO_I_SUS2,
    &EDGE_IV_6_TO_I_SUS4,
    &EDGE_IV_MAJ7_TO_I,
    &EDGE_IV_MAJ7_TO_I_6,
    &EDGE_IV_MAJ7_TO_I_MAJ7,
    &EDGE_IV_MAJ7_TO_I_MAJ9,
    &EDGE_IV_MAJ7_TO_I_ADD9,
    &EDGE_IV_MAJ7_TO_I_SUS2,
    &EDGE_IV_MAJ7_TO_I_SUS4,
    &EDGE_IV_TO_V,
    &EDGE_IV_TO_V_7,
    &EDGE_IV_TO_V_9,
    &EDGE_IV_TO_V_11,
    &EDGE_IV_TO_V_13,
    &EDGE_IV_TO_V_SUS4,
    &EDGE_IV_6_TO_V,
    &EDGE_IV_6_TO_V_7,
    &EDGE_IV_6_TO_V_9,
    &EDGE_IV_6_TO_V_11,
    &EDGE_IV_6_TO_V_13,
    &EDGE_IV_6_TO_V_SUS4,
    &EDGE_IV_MAJ7_TO_V,
    &EDGE_IV_MAJ7_TO_V_7,
    &EDGE_IV_MAJ7_TO_V_9,
    &EDGE_IV_MAJ7_TO_V_11,
    &EDGE_IV_MAJ7_TO_V_13,
    &EDGE_IV_MAJ7_TO_V_SUS4,
    &EDGE_IV_TO_I_SLASH_5,
    &EDGE_IV_6_TO_I_SLASH_5,
    &EDGE_IV_MAJ7_TO_I_SLASH_5,
    &EDGE_IV_TO_MINOR_II,
    &EDGE_IV_TO_MINOR_II_M7,
    &EDGE_IV_TO_MINOR_II_M9,
    &EDGE_IV_6_TO_MINOR_II,
    &EDGE_IV_6_TO_MINOR_II_M7,
    &EDGE_IV_6_TO_MINOR_II_M9,
    &EDGE_IV_MAJ7_TO_MINOR_II,
    &EDGE_IV_MAJ7_TO_MINOR_II_M7,
    &EDGE_IV_MAJ7_TO_MINOR_II_M9,
    &EDGE_IV_TO_I_SLASH_3,
    &EDGE_IV_6_TO_I_SLASH_3,
    &EDGE_IV_MAJ7_TO_I_SLASH_3,
    &EDGE_MINOR_SIVM7FLAT_5_TO_VII_7,
    &EDGE_MINOR_SIVM7FLAT_5_TO_VII_9,
    &EDGE_MINOR_SIVM7FLAT_5_TO_VII_FLAT_9,
    &EDGE_MINOR_SIVM7FLAT_5_TO_V,
    &EDGE_MINOR_SIVM7FLAT_5_TO_V_7,
    &EDGE_MINOR_SIVM7FLAT_5_TO_V_9,
    &EDGE_MINOR_SIVM7FLAT_5_TO_V_11,
    &EDGE_MINOR_SIVM7FLAT_5_TO_V_13,
    &EDGE_MINOR_SIVM7FLAT_5_TO_V_SUS4,
    &EDGE_MINOR_SIVM7FLAT_5_TO_I_SLASH_5,
    &EDGE_V_TO_MINOR_III,
    &EDGE_V_TO_MINOR_III_M7,
    &EDGE_V_7_TO_MINOR_III,
    &EDGE_V_7_TO_MINOR_III_M7,
    &EDGE_V_9_TO_MINOR_III,
    &EDGE_V_9_TO_MINOR_III_M7,
    &EDGE_V_11_TO_MINOR_III,
    &EDGE_V_11_TO_MINOR_III_M7,
    &EDGE_V_13_TO_MINOR_III,
    &EDGE_V_13_TO_MINOR_III_M7,
    &EDGE_V_SUS4_TO_MINOR_III,
    &EDGE_V_SUS4_TO_MINOR_III_M7,
    &EDGE_V_TO_MINOR_VI,
    &EDGE_V_TO_MINOR_VI_M7,
    &EDGE_V_TO_MINOR_VI_M9,
    &EDGE_V_7_TO_MINOR_VI,
    &EDGE_V_7_TO_MINOR_VI_M7,
    &EDGE_V_7_TO_MINOR_VI_M9,
    &EDGE_V_9_TO_MINOR_VI,
    &EDGE_V_9_TO_MINOR_VI_M7,
    &EDGE_V_9_TO_MINOR_VI_M9,
    &EDGE_V_11_TO_MINOR_VI,
    &EDGE_V_11_TO_MINOR_VI_M7,
    &EDGE_V_11_TO_MINOR_VI_M9,
    &EDGE_V_13_TO_MINOR_VI,
    &EDGE_V_13_TO_MINOR_VI_M7,
    &EDGE_V_13_TO_MINOR_VI_M9,
    &EDGE_V_SUS4_TO_MINOR_VI,
    &EDGE_V_SUS4_TO_MINOR_VI_M7,
    &EDGE_V_SUS4_TO_MINOR_VI_M9,
    &EDGE_V_TO_I,
    &EDGE_V_TO_I_6,
    &EDGE_V_TO_I_MAJ7,
    &EDGE_V_TO_I_MAJ9,
    &EDGE_V_TO_I_ADD9,
    &EDGE_V_TO_I_SUS2,
    &EDGE_V_TO_I_SUS4,
    &EDGE_V_7_TO_I,
    &EDGE_V_7_TO_I_6,
    &EDGE_V_7_TO_I_MAJ7,
    &EDGE_V_7_TO_I_MAJ9,
    &EDGE_V_7_TO_I_ADD9,
    &EDGE_V_7_TO_I_SUS2,
    &EDGE_V_7_TO_I_SUS4,
    &EDGE_V_9_TO_I,
    &EDGE_V_9_TO_I_6,
    &EDGE_V_9_TO_I_MAJ7,
    &EDGE_V_9_TO_I_MAJ9,
    &EDGE_V_9_TO_I_ADD9,
    &EDGE_V_9_TO_I_SUS2,
    &EDGE_V_9_TO_I_SUS4,
    &EDGE_V_11_TO_I,
    &EDGE_V_11_TO_I_6,
    &EDGE_V_11_TO_I_MAJ7,
    &EDGE_V_11_TO_I_MAJ9,
    &EDGE_V_11_TO_I_ADD9,
    &EDGE_V_11_TO_I_SUS2,
    &EDGE_V_11_TO_I_SUS4,
    &EDGE_V_13_TO_I,
    &EDGE_V_13_TO_I_6,
    &EDGE_V_13_TO_I_MAJ7,
    &EDGE_V_13_TO_I_MAJ9,
    &EDGE_V_13_TO_I_ADD9,
    &EDGE_V_13_TO_I_SUS2,
    &EDGE_V_13_TO_I_SUS4,
    &EDGE_V_SUS4_TO_I,
    &EDGE_V_SUS4_TO_I_6,
    &EDGE_V_SUS4_TO_I_MAJ7,
    &EDGE_V_SUS4_TO_I_MAJ9,
    &EDGE_V_SUS4_TO_I_ADD9,
    &EDGE_V_SUS4_TO_I_SUS2,
    &EDGE_V_SUS4_TO_I_SUS4,
    &EDGE_V_SLASH_1_TO_I,
    &EDGE_V_SLASH_1_TO_I_6,
    &EDGE_V_SLASH_1_TO_I_MAJ7,
    &EDGE_V_SLASH_1_TO_I_MAJ9,
    &EDGE_V_SLASH_1_TO_I_ADD9,
    &EDGE_V_SLASH_1_TO_I_SUS2,
    &EDGE_V_SLASH_1_TO_I_SUS4,
    &EDGE_V_SLASH_2_TO_II_7,
    &EDGE_V_SLASH_2_TO_II_9,
    &EDGE_V_SLASH_2_TO_II_FLAT_9,
    &EDGE_MINOR_V_7_TO_I,
    &EDGE_MINOR_V_7_TO_I_6,
    &EDGE_MINOR_V_7_TO_I_MAJ7,
    &EDGE_MINOR_V_7_TO_I_MAJ9,
    &EDGE_MINOR_V_7_TO_I_ADD9,
    &EDGE_MINOR_V_7_TO_I_SUS2,
    &EDGE_MINOR_V_7_TO_I_SUS4,
    &EDGE_MINOR_SVDIM7_TO_MINOR_VI,
    &EDGE_MINOR_SVDIM7_TO_MINOR_VI_M7,
    &EDGE_MINOR_SVDIM7_TO_MINOR_VI_M9,
    &EDGE_MINOR_FLAT_VI_TO_MINOR_FLAT_VII_9,
    &EDGE_MINOR_FLAT_VI7_TO_I_SLASH_5,
    &EDGE_MINOR_VI_TO_MINOR_II,
    &EDGE_MINOR_VI_TO_MINOR_II_M7,
    &EDGE_MINOR_VI_TO_MINOR_II_M9,
    &EDGE_MINOR_VI_M7_TO_MINOR_II,
    &EDGE_MINOR_VI_M7_TO_MINOR_II_M7,
    &EDGE_MINOR_VI_M7_TO_MINOR_II_M9,
    &EDGE_MINOR_VI_M9_TO_MINOR_II,
    &EDGE_MINOR_VI_M9_TO_MINOR_II_M7,
    &EDGE_MINOR_VI_M9_TO_MINOR_II_M9,
    &EDGE_MINOR_VI_TO_IV,
    &EDGE_MINOR_VI_TO_IV_6,
    &EDGE_MINOR_VI_TO_IV_MAJ7,
    &EDGE_MINOR_VI_M7_TO_IV,
    &EDGE_MINOR_VI_M7_TO_IV_6,
    &EDGE_MINOR_VI_M7_TO_IV_MAJ7,
    &EDGE_MINOR_VI_M9_TO_IV,
    &EDGE_MINOR_VI_M9_TO_IV_6,
    &EDGE_MINOR_VI_M9_TO_IV_MAJ7,
    &EDGE_VI_7_TO_MINOR_II,
    &EDGE_VI_7_TO_MINOR_II_M7,
    &EDGE_VI_7_TO_MINOR_II_M9,
    &EDGE_VI_9_TO_MINOR_II,
    &EDGE_VI_9_TO_MINOR_II_M7,
    &EDGE_VI_9_TO_MINOR_II_M9,
    &EDGE_VI_FLAT_9_TO_MINOR_II,
    &EDGE_VI_FLAT_9_TO_MINOR_II_M7,
    &EDGE_VI_FLAT_9_TO_MINOR_II_M9,
    &EDGE_MINOR_VIM7FLAT_5_SLASH_FLAT_3_TO_II_7,
    &EDGE_MINOR_VIM7FLAT_5_SLASH_FLAT_3_TO_II_9,
    &EDGE_MINOR_VIM7FLAT_5_SLASH_FLAT_3_TO_II_FLAT_9,
    &EDGE_MINOR_FLAT_VII_9_TO_I,
    &EDGE_MINOR_FLAT_VII_9_TO_I_6,
    &EDGE_MINOR_FLAT_VII_9_TO_I_MAJ7,
    &EDGE_MINOR_FLAT_VII_9_TO_I_MAJ9,
    &EDGE_MINOR_FLAT_VII_9_TO_I_ADD9,
    &EDGE_MINOR_FLAT_VII_9_TO_I_SUS2,
    &EDGE_MINOR_FLAT_VII_9_TO_I_SUS4,
    &EDGE_MINOR_FLAT_VII9_TO_I_SLASH_5,
    &EDGE_VII_7_TO_MINOR_III,
    &EDGE_VII_7_TO_MINOR_III_M7,
    &EDGE_VII_9_TO_MINOR_III,
    &EDGE_VII_9_TO_MINOR_III_M7,
    &EDGE_VII_FLAT_9_TO_MINOR_III,
    &EDGE_VII_FLAT_9_TO_MINOR_III_M7,
    &EDGE_MINOR_VIIM7FLAT_5_TO_III_7,
    &EDGE_MINOR_VIIM7FLAT_5_TO_III_9,
    &EDGE_MINOR_VIIM7FLAT_5_TO_III_FLAT_9,
];

/// NodeType mapping for all progression chords in major keys
/// 
/// Maps each chord to its harmonic role (Primary = stable, Secondary = transitional).
pub fn get_node_types() -> HashMap<&'static RomanChord, NodeType> {
    let mut map = HashMap::new();
    map.insert(&I, NodeType::Primary);
    map.insert(&I_6, NodeType::Primary);
    map.insert(&I_MAJ7, NodeType::Primary);
    map.insert(&I_MAJ9, NodeType::Primary);
    map.insert(&I_ADD9, NodeType::Primary);
    map.insert(&I_SUS2, NodeType::Primary);
    map.insert(&I_SUS4, NodeType::Primary);
    map.insert(&I_SLASH_5, NodeType::Primary);
    map.insert(&I_SLASH_3, NodeType::Primary);
    map.insert(&MINOR_II, NodeType::Primary);
    map.insert(&MINOR_II_M7, NodeType::Primary);
    map.insert(&MINOR_II_M9, NodeType::Primary);
    map.insert(&MINOR_III, NodeType::Primary);
    map.insert(&MINOR_III_M7, NodeType::Primary);
    map.insert(&IV, NodeType::Primary);
    map.insert(&IV_6, NodeType::Primary);
    map.insert(&IV_MAJ7, NodeType::Primary);
    map.insert(&IV_SLASH_1, NodeType::Primary);
    map.insert(&MINOR_IV, NodeType::Primary);
    map.insert(&MINOR_IV_M6, NodeType::Primary);
    map.insert(&V, NodeType::Primary);
    map.insert(&V_7, NodeType::Primary);
    map.insert(&V_9, NodeType::Primary);
    map.insert(&V_11, NodeType::Primary);
    map.insert(&V_13, NodeType::Primary);
    map.insert(&V_SUS4, NodeType::Primary);
    map.insert(&V_SLASH_1, NodeType::Primary);
    map.insert(&MINOR_VI, NodeType::Primary);
    map.insert(&MINOR_VI_M7, NodeType::Primary);
    map.insert(&MINOR_VI_M9, NodeType::Primary);
    map.insert(&VI_7, NodeType::Secondary);
    map.insert(&VI_9, NodeType::Secondary);
    map.insert(&VI_FLAT_9, NodeType::Secondary);
    map.insert(&MINOR_SIDIM7, NodeType::Secondary);
    map.insert(&MINOR_SIVM7FLAT_5, NodeType::Secondary);
    map.insert(&II_7, NodeType::Secondary);
    map.insert(&II_9, NodeType::Secondary);
    map.insert(&II_FLAT_9, NodeType::Secondary);
    map.insert(&MINOR_VIM7FLAT_5_SLASH_FLAT_3, NodeType::Secondary);
    map.insert(&MINOR_SIIDIM7, NodeType::Secondary);
    map.insert(&VII_7, NodeType::Secondary);
    map.insert(&VII_9, NodeType::Secondary);
    map.insert(&VII_FLAT_9, NodeType::Secondary);
    map.insert(&MINOR_V_7, NodeType::Secondary);
    map.insert(&I7_9, NodeType::Secondary);
    map.insert(&I7_FLAT_9, NodeType::Secondary);
    map.insert(&MINOR_IIIM7FLAT_5, NodeType::Secondary);
    map.insert(&MINOR_IM6, NodeType::Secondary);
    map.insert(&V_SLASH_2, NodeType::Secondary);
    map.insert(&MINOR_FLAT_VII_9, NodeType::Secondary);
    map.insert(&MINOR_FLAT_VII9, NodeType::Secondary);
    map.insert(&MINOR_FLAT_VI, NodeType::Secondary);
    map.insert(&MINOR_FLAT_II7, NodeType::Secondary);
    map.insert(&MINOR_IVM7, NodeType::Secondary);
    map.insert(&MINOR_FLAT_VI7, NodeType::Secondary);
    map.insert(&MINOR_IDIM_SLASH_FLAT_3, NodeType::Secondary);
    map.insert(&III_7, NodeType::Secondary);
    map.insert(&III_9, NodeType::Secondary);
    map.insert(&III_FLAT_9, NodeType::Secondary);
    map.insert(&MINOR_VIIM7FLAT_5, NodeType::Secondary);
    map.insert(&MINOR_SVDIM7, NodeType::Secondary);
    map
}

