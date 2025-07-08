//! Generated progression data for major keys from major.progression
//! Do not edit manually.

use crate::types::progression::{ProgressionEdge, NodeType};
use crate::types::{RomanChord, RomanNumeral, RomanDegree, Accidental, Interval, IntervalSet};
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

/// I chord - stable harmonic center (primary node)
/// Intervals: perfect unison, major third, perfect fifth
pub static I: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::I, Accidental::Natural),
    intervals: MAJOR_TRIAD_SET,
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

/// I6 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, major third, perfect fifth, major sixth
pub static I_6: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::I, Accidental::Natural),
    intervals: I_6_INTERVALS_SET,
};

const I_7_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
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

/// I7 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh
pub static I_7: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::I, Accidental::Natural),
    intervals: I_7_INTERVALS_SET,
};

const I_9_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
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

/// I9 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh, major ninth
pub static I_9: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::I, Accidental::Natural),
    intervals: I_9_INTERVALS_SET,
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

/// Imaj7 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, major third, perfect fifth, major seventh
pub static I_MAJ7: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::I, Accidental::Natural),
    intervals: I_MAJ7_INTERVALS_SET,
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

/// Imaj9 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, major third, perfect fifth, major seventh, major ninth
pub static I_MAJ9: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::I, Accidental::Natural),
    intervals: I_MAJ9_INTERVALS_SET,
};

/// ii chord - stable harmonic center (primary node)
/// Intervals: perfect unison, minor third, perfect fifth
pub static MINOR_II: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::II, Accidental::Natural),
    intervals: MINOR_TRIAD_SET,
};

const MINOR_II_7_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
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

/// ii7 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, minor third, perfect fifth, minor seventh
pub static MINOR_II_7: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::II, Accidental::Natural),
    intervals: MINOR_II_7_INTERVALS_SET,
};

const MINOR_II_9_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
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

/// ii9 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, minor third, perfect fifth, minor seventh, major ninth
pub static MINOR_II_9: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::II, Accidental::Natural),
    intervals: MINOR_II_9_INTERVALS_SET,
};

const MINOR_II_11_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MINOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::MINOR_SEVENTH,
     Interval::MAJOR_NINTH,
     Interval::PERFECT_ELEVENTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 6);

/// ii11 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, minor third, perfect fifth, minor seventh, major ninth, perfect eleventh
pub static MINOR_II_11: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::II, Accidental::Natural),
    intervals: MINOR_II_11_INTERVALS_SET,
};

const MINOR_II_7_PLUS_FLAT_9_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MINOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::MINOR_SEVENTH,
     Interval::MINOR_NINTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 5);

/// ii7+b9 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, minor third, perfect fifth, minor seventh, minor ninth
pub static MINOR_II_7_PLUS_FLAT_9: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::II, Accidental::Natural),
    intervals: MINOR_II_7_PLUS_FLAT_9_INTERVALS_SET,
};

/// iii chord - stable harmonic center (primary node)
/// Intervals: perfect unison, minor third, perfect fifth
pub static MINOR_III: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::III, Accidental::Natural),
    intervals: MINOR_TRIAD_SET,
};

const MINOR_III_7_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
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

/// iii7 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, minor third, perfect fifth, minor seventh
pub static MINOR_III_7: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::III, Accidental::Natural),
    intervals: MINOR_III_7_INTERVALS_SET,
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

/// iiim7 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, minor third, perfect fifth, minor seventh
pub static MINOR_III_M7: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::III, Accidental::Natural),
    intervals: MINOR_III_M7_INTERVALS_SET,
};

/// IV chord - stable harmonic center (primary node)
/// Intervals: perfect unison, major third, perfect fifth
pub static IV: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::IV, Accidental::Natural),
    intervals: MAJOR_TRIAD_SET,
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

/// IV6 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, major third, perfect fifth, major sixth
pub static IV_6: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::IV, Accidental::Natural),
    intervals: IV_6_INTERVALS_SET,
};

const IV_7_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
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

/// IV7 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh
pub static IV_7: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::IV, Accidental::Natural),
    intervals: IV_7_INTERVALS_SET,
};

const IV_9_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
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

/// IV9 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh, major ninth
pub static IV_9: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::IV, Accidental::Natural),
    intervals: IV_9_INTERVALS_SET,
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

/// IVmaj7 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, major third, perfect fifth, major seventh
pub static IV_MAJ7: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::IV, Accidental::Natural),
    intervals: IV_MAJ7_INTERVALS_SET,
};

const IV_SHARP_11_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MAJOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::MAJOR_SEVENTH,
     Interval::MAJOR_NINTH,
     Interval::AUGMENTED_ELEVENTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 6);

/// IV#11 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, major third, perfect fifth, major seventh, major ninth, augmented eleventh
pub static IV_SHARP_11: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::IV, Accidental::Natural),
    intervals: IV_SHARP_11_INTERVALS_SET,
};

/// V chord - stable harmonic center (primary node)
/// Intervals: perfect unison, major third, perfect fifth
pub static V: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::V, Accidental::Natural),
    intervals: MAJOR_TRIAD_SET,
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

/// V7 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh
pub static V_7: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::V, Accidental::Natural),
    intervals: V_7_INTERVALS_SET,
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

/// V9 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh, major ninth
pub static V_9: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::V, Accidental::Natural),
    intervals: V_9_INTERVALS_SET,
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

/// V11 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh, major ninth, perfect eleventh
pub static V_11: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::V, Accidental::Natural),
    intervals: V_11_INTERVALS_SET,
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

/// V13 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh, major ninth, perfect eleventh, major thirteenth
pub static V_13: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::V, Accidental::Natural),
    intervals: V_13_INTERVALS_SET,
};

const V_7_PLUS_FLAT_9_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MAJOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::MINOR_SEVENTH,
     Interval::MINOR_NINTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 5);

/// V7+b9 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh, minor ninth
pub static V_7_PLUS_FLAT_9: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::V, Accidental::Natural),
    intervals: V_7_PLUS_FLAT_9_INTERVALS_SET,
};

const V_7_PLUS_SHARP_9_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MAJOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::MINOR_SEVENTH,
     Interval::AUGMENTED_NINTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 5);

/// V7+#9 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh, augmented ninth
pub static V_7_PLUS_SHARP_9: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::V, Accidental::Natural),
    intervals: V_7_PLUS_SHARP_9_INTERVALS_SET,
};

/// vi chord - stable harmonic center (primary node)
/// Intervals: perfect unison, minor third, perfect fifth
pub static MINOR_VI: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::VI, Accidental::Natural),
    intervals: MINOR_TRIAD_SET,
};

const MINOR_VI_7_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
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

/// vi7 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, minor third, perfect fifth, minor seventh
pub static MINOR_VI_7: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::VI, Accidental::Natural),
    intervals: MINOR_VI_7_INTERVALS_SET,
};

const MINOR_VI_9_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
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

/// vi9 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, minor third, perfect fifth, minor seventh, major ninth
pub static MINOR_VI_9: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::VI, Accidental::Natural),
    intervals: MINOR_VI_9_INTERVALS_SET,
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

/// vim7 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, minor third, perfect fifth, minor seventh
pub static MINOR_VI_M7: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::VI, Accidental::Natural),
    intervals: MINOR_VI_M7_INTERVALS_SET,
};

/// vii chord - stable harmonic center (primary node)
/// Intervals: perfect unison, minor third, perfect fifth
pub static MINOR_VII: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::VII, Accidental::Natural),
    intervals: MINOR_TRIAD_SET,
};

const MINOR_VII_FLAT_5_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MINOR_THIRD,
     Interval::DIMINISHED_FIFTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 3);

/// viib5 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, minor third, diminished fifth
pub static MINOR_VII_FLAT_5: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::VII, Accidental::Natural),
    intervals: MINOR_VII_FLAT_5_INTERVALS_SET,
};

const MINOR_VII_M7_FLAT_5_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MINOR_THIRD,
     Interval::DIMINISHED_FIFTH,
     Interval::MINOR_SEVENTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 4);

/// viim7b5 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, minor third, diminished fifth, minor seventh
pub static MINOR_VII_M7_FLAT_5: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::VII, Accidental::Natural),
    intervals: MINOR_VII_M7_FLAT_5_INTERVALS_SET,
};

const MINOR_FLAT_III_7_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
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

/// bIII7 chord - creates tension, seeks resolution (secondary node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh
pub static MINOR_FLAT_III_7: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::III, Accidental::Natural),
    intervals: MINOR_FLAT_III_7_INTERVALS_SET,
};

const MINOR_FLAT_III_9_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
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

/// bIII9 chord - creates tension, seeks resolution (secondary node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh, major ninth
pub static MINOR_FLAT_III_9: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::III, Accidental::Natural),
    intervals: MINOR_FLAT_III_9_INTERVALS_SET,
};

const MINOR_FLAT_VI_7_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
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

/// bVI7 chord - creates tension, seeks resolution (secondary node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh
pub static MINOR_FLAT_VI_7: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::VI, Accidental::Natural),
    intervals: MINOR_FLAT_VI_7_INTERVALS_SET,
};

const MINOR_FLAT_VI_9_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
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

/// bVI9 chord - creates tension, seeks resolution (secondary node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh, major ninth
pub static MINOR_FLAT_VI_9: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::VI, Accidental::Natural),
    intervals: MINOR_FLAT_VI_9_INTERVALS_SET,
};

const MINOR_FLAT_VII_7_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
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

/// bVII7 chord - creates tension, seeks resolution (secondary node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh
pub static MINOR_FLAT_VII_7: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::VII, Accidental::Natural),
    intervals: MINOR_FLAT_VII_7_INTERVALS_SET,
};

const MINOR_FLAT_VII_9_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
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

/// bVII9 chord - creates tension, seeks resolution (secondary node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh, major ninth
pub static MINOR_FLAT_VII_9: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::VII, Accidental::Natural),
    intervals: MINOR_FLAT_VII_9_INTERVALS_SET,
};

/// Progression edge: I → IV
pub static EDGE_I_TO_IV: ProgressionEdge = ProgressionEdge {
    from: I,
    to: IV,
};

/// Progression edge: I → IV
pub static EDGE_I_TO_IV_6: ProgressionEdge = ProgressionEdge {
    from: I,
    to: IV_6,
};

/// Progression edge: I → IV
pub static EDGE_I_TO_IV_7: ProgressionEdge = ProgressionEdge {
    from: I,
    to: IV_7,
};

/// Progression edge: I → IV
pub static EDGE_I_TO_IV_9: ProgressionEdge = ProgressionEdge {
    from: I,
    to: IV_9,
};

/// Progression edge: I → IV
pub static EDGE_I_TO_IV_MAJ7: ProgressionEdge = ProgressionEdge {
    from: I,
    to: IV_MAJ7,
};

/// Progression edge: I → IV
pub static EDGE_I_TO_IV_SHARP_11: ProgressionEdge = ProgressionEdge {
    from: I,
    to: IV_SHARP_11,
};

/// Progression edge: I → IV
pub static EDGE_I_6_TO_IV: ProgressionEdge = ProgressionEdge {
    from: I_6,
    to: IV,
};

/// Progression edge: I → IV
pub static EDGE_I_6_TO_IV_6: ProgressionEdge = ProgressionEdge {
    from: I_6,
    to: IV_6,
};

/// Progression edge: I → IV
pub static EDGE_I_6_TO_IV_7: ProgressionEdge = ProgressionEdge {
    from: I_6,
    to: IV_7,
};

/// Progression edge: I → IV
pub static EDGE_I_6_TO_IV_9: ProgressionEdge = ProgressionEdge {
    from: I_6,
    to: IV_9,
};

/// Progression edge: I → IV
pub static EDGE_I_6_TO_IV_MAJ7: ProgressionEdge = ProgressionEdge {
    from: I_6,
    to: IV_MAJ7,
};

/// Progression edge: I → IV
pub static EDGE_I_6_TO_IV_SHARP_11: ProgressionEdge = ProgressionEdge {
    from: I_6,
    to: IV_SHARP_11,
};

/// Progression edge: I → IV
pub static EDGE_I_7_TO_IV: ProgressionEdge = ProgressionEdge {
    from: I_7,
    to: IV,
};

/// Progression edge: I → IV
pub static EDGE_I_7_TO_IV_6: ProgressionEdge = ProgressionEdge {
    from: I_7,
    to: IV_6,
};

/// Progression edge: I → IV
pub static EDGE_I_7_TO_IV_7: ProgressionEdge = ProgressionEdge {
    from: I_7,
    to: IV_7,
};

/// Progression edge: I → IV
pub static EDGE_I_7_TO_IV_9: ProgressionEdge = ProgressionEdge {
    from: I_7,
    to: IV_9,
};

/// Progression edge: I → IV
pub static EDGE_I_7_TO_IV_MAJ7: ProgressionEdge = ProgressionEdge {
    from: I_7,
    to: IV_MAJ7,
};

/// Progression edge: I → IV
pub static EDGE_I_7_TO_IV_SHARP_11: ProgressionEdge = ProgressionEdge {
    from: I_7,
    to: IV_SHARP_11,
};

/// Progression edge: I → IV
pub static EDGE_I_9_TO_IV: ProgressionEdge = ProgressionEdge {
    from: I_9,
    to: IV,
};

/// Progression edge: I → IV
pub static EDGE_I_9_TO_IV_6: ProgressionEdge = ProgressionEdge {
    from: I_9,
    to: IV_6,
};

/// Progression edge: I → IV
pub static EDGE_I_9_TO_IV_7: ProgressionEdge = ProgressionEdge {
    from: I_9,
    to: IV_7,
};

/// Progression edge: I → IV
pub static EDGE_I_9_TO_IV_9: ProgressionEdge = ProgressionEdge {
    from: I_9,
    to: IV_9,
};

/// Progression edge: I → IV
pub static EDGE_I_9_TO_IV_MAJ7: ProgressionEdge = ProgressionEdge {
    from: I_9,
    to: IV_MAJ7,
};

/// Progression edge: I → IV
pub static EDGE_I_9_TO_IV_SHARP_11: ProgressionEdge = ProgressionEdge {
    from: I_9,
    to: IV_SHARP_11,
};

/// Progression edge: I → IV
pub static EDGE_I_MAJ7_TO_IV: ProgressionEdge = ProgressionEdge {
    from: I_MAJ7,
    to: IV,
};

/// Progression edge: I → IV
pub static EDGE_I_MAJ7_TO_IV_6: ProgressionEdge = ProgressionEdge {
    from: I_MAJ7,
    to: IV_6,
};

/// Progression edge: I → IV
pub static EDGE_I_MAJ7_TO_IV_7: ProgressionEdge = ProgressionEdge {
    from: I_MAJ7,
    to: IV_7,
};

/// Progression edge: I → IV
pub static EDGE_I_MAJ7_TO_IV_9: ProgressionEdge = ProgressionEdge {
    from: I_MAJ7,
    to: IV_9,
};

/// Progression edge: I → IV
pub static EDGE_I_MAJ7_TO_IV_MAJ7: ProgressionEdge = ProgressionEdge {
    from: I_MAJ7,
    to: IV_MAJ7,
};

/// Progression edge: I → IV
pub static EDGE_I_MAJ7_TO_IV_SHARP_11: ProgressionEdge = ProgressionEdge {
    from: I_MAJ7,
    to: IV_SHARP_11,
};

/// Progression edge: I → IV
pub static EDGE_I_MAJ9_TO_IV: ProgressionEdge = ProgressionEdge {
    from: I_MAJ9,
    to: IV,
};

/// Progression edge: I → IV
pub static EDGE_I_MAJ9_TO_IV_6: ProgressionEdge = ProgressionEdge {
    from: I_MAJ9,
    to: IV_6,
};

/// Progression edge: I → IV
pub static EDGE_I_MAJ9_TO_IV_7: ProgressionEdge = ProgressionEdge {
    from: I_MAJ9,
    to: IV_7,
};

/// Progression edge: I → IV
pub static EDGE_I_MAJ9_TO_IV_9: ProgressionEdge = ProgressionEdge {
    from: I_MAJ9,
    to: IV_9,
};

/// Progression edge: I → IV
pub static EDGE_I_MAJ9_TO_IV_MAJ7: ProgressionEdge = ProgressionEdge {
    from: I_MAJ9,
    to: IV_MAJ7,
};

/// Progression edge: I → IV
pub static EDGE_I_MAJ9_TO_IV_SHARP_11: ProgressionEdge = ProgressionEdge {
    from: I_MAJ9,
    to: IV_SHARP_11,
};

/// Progression edge: I → V
pub static EDGE_I_TO_V: ProgressionEdge = ProgressionEdge {
    from: I,
    to: V,
};

/// Progression edge: I → V
pub static EDGE_I_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: I,
    to: V_7,
};

/// Progression edge: I → V
pub static EDGE_I_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: I,
    to: V_9,
};

/// Progression edge: I → V
pub static EDGE_I_TO_V_11: ProgressionEdge = ProgressionEdge {
    from: I,
    to: V_11,
};

/// Progression edge: I → V
pub static EDGE_I_TO_V_13: ProgressionEdge = ProgressionEdge {
    from: I,
    to: V_13,
};

/// Progression edge: I → V
pub static EDGE_I_TO_V_7_PLUS_FLAT_9: ProgressionEdge = ProgressionEdge {
    from: I,
    to: V_7_PLUS_FLAT_9,
};

/// Progression edge: I → V
pub static EDGE_I_TO_V_7_PLUS_SHARP_9: ProgressionEdge = ProgressionEdge {
    from: I,
    to: V_7_PLUS_SHARP_9,
};

/// Progression edge: I → V
pub static EDGE_I_6_TO_V: ProgressionEdge = ProgressionEdge {
    from: I_6,
    to: V,
};

/// Progression edge: I → V
pub static EDGE_I_6_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: I_6,
    to: V_7,
};

/// Progression edge: I → V
pub static EDGE_I_6_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: I_6,
    to: V_9,
};

/// Progression edge: I → V
pub static EDGE_I_6_TO_V_11: ProgressionEdge = ProgressionEdge {
    from: I_6,
    to: V_11,
};

/// Progression edge: I → V
pub static EDGE_I_6_TO_V_13: ProgressionEdge = ProgressionEdge {
    from: I_6,
    to: V_13,
};

/// Progression edge: I → V
pub static EDGE_I_6_TO_V_7_PLUS_FLAT_9: ProgressionEdge = ProgressionEdge {
    from: I_6,
    to: V_7_PLUS_FLAT_9,
};

/// Progression edge: I → V
pub static EDGE_I_6_TO_V_7_PLUS_SHARP_9: ProgressionEdge = ProgressionEdge {
    from: I_6,
    to: V_7_PLUS_SHARP_9,
};

/// Progression edge: I → V
pub static EDGE_I_7_TO_V: ProgressionEdge = ProgressionEdge {
    from: I_7,
    to: V,
};

/// Progression edge: I → V
pub static EDGE_I_7_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: I_7,
    to: V_7,
};

/// Progression edge: I → V
pub static EDGE_I_7_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: I_7,
    to: V_9,
};

/// Progression edge: I → V
pub static EDGE_I_7_TO_V_11: ProgressionEdge = ProgressionEdge {
    from: I_7,
    to: V_11,
};

/// Progression edge: I → V
pub static EDGE_I_7_TO_V_13: ProgressionEdge = ProgressionEdge {
    from: I_7,
    to: V_13,
};

/// Progression edge: I → V
pub static EDGE_I_7_TO_V_7_PLUS_FLAT_9: ProgressionEdge = ProgressionEdge {
    from: I_7,
    to: V_7_PLUS_FLAT_9,
};

/// Progression edge: I → V
pub static EDGE_I_7_TO_V_7_PLUS_SHARP_9: ProgressionEdge = ProgressionEdge {
    from: I_7,
    to: V_7_PLUS_SHARP_9,
};

/// Progression edge: I → V
pub static EDGE_I_9_TO_V: ProgressionEdge = ProgressionEdge {
    from: I_9,
    to: V,
};

/// Progression edge: I → V
pub static EDGE_I_9_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: I_9,
    to: V_7,
};

/// Progression edge: I → V
pub static EDGE_I_9_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: I_9,
    to: V_9,
};

/// Progression edge: I → V
pub static EDGE_I_9_TO_V_11: ProgressionEdge = ProgressionEdge {
    from: I_9,
    to: V_11,
};

/// Progression edge: I → V
pub static EDGE_I_9_TO_V_13: ProgressionEdge = ProgressionEdge {
    from: I_9,
    to: V_13,
};

/// Progression edge: I → V
pub static EDGE_I_9_TO_V_7_PLUS_FLAT_9: ProgressionEdge = ProgressionEdge {
    from: I_9,
    to: V_7_PLUS_FLAT_9,
};

/// Progression edge: I → V
pub static EDGE_I_9_TO_V_7_PLUS_SHARP_9: ProgressionEdge = ProgressionEdge {
    from: I_9,
    to: V_7_PLUS_SHARP_9,
};

/// Progression edge: I → V
pub static EDGE_I_MAJ7_TO_V: ProgressionEdge = ProgressionEdge {
    from: I_MAJ7,
    to: V,
};

/// Progression edge: I → V
pub static EDGE_I_MAJ7_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: I_MAJ7,
    to: V_7,
};

/// Progression edge: I → V
pub static EDGE_I_MAJ7_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: I_MAJ7,
    to: V_9,
};

/// Progression edge: I → V
pub static EDGE_I_MAJ7_TO_V_11: ProgressionEdge = ProgressionEdge {
    from: I_MAJ7,
    to: V_11,
};

/// Progression edge: I → V
pub static EDGE_I_MAJ7_TO_V_13: ProgressionEdge = ProgressionEdge {
    from: I_MAJ7,
    to: V_13,
};

/// Progression edge: I → V
pub static EDGE_I_MAJ7_TO_V_7_PLUS_FLAT_9: ProgressionEdge = ProgressionEdge {
    from: I_MAJ7,
    to: V_7_PLUS_FLAT_9,
};

/// Progression edge: I → V
pub static EDGE_I_MAJ7_TO_V_7_PLUS_SHARP_9: ProgressionEdge = ProgressionEdge {
    from: I_MAJ7,
    to: V_7_PLUS_SHARP_9,
};

/// Progression edge: I → V
pub static EDGE_I_MAJ9_TO_V: ProgressionEdge = ProgressionEdge {
    from: I_MAJ9,
    to: V,
};

/// Progression edge: I → V
pub static EDGE_I_MAJ9_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: I_MAJ9,
    to: V_7,
};

/// Progression edge: I → V
pub static EDGE_I_MAJ9_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: I_MAJ9,
    to: V_9,
};

/// Progression edge: I → V
pub static EDGE_I_MAJ9_TO_V_11: ProgressionEdge = ProgressionEdge {
    from: I_MAJ9,
    to: V_11,
};

/// Progression edge: I → V
pub static EDGE_I_MAJ9_TO_V_13: ProgressionEdge = ProgressionEdge {
    from: I_MAJ9,
    to: V_13,
};

/// Progression edge: I → V
pub static EDGE_I_MAJ9_TO_V_7_PLUS_FLAT_9: ProgressionEdge = ProgressionEdge {
    from: I_MAJ9,
    to: V_7_PLUS_FLAT_9,
};

/// Progression edge: I → V
pub static EDGE_I_MAJ9_TO_V_7_PLUS_SHARP_9: ProgressionEdge = ProgressionEdge {
    from: I_MAJ9,
    to: V_7_PLUS_SHARP_9,
};

/// Progression edge: I → vi
pub static EDGE_I_TO_MINOR_VI: ProgressionEdge = ProgressionEdge {
    from: I,
    to: MINOR_VI,
};

/// Progression edge: I → vi
pub static EDGE_I_TO_MINOR_VI_7: ProgressionEdge = ProgressionEdge {
    from: I,
    to: MINOR_VI_7,
};

/// Progression edge: I → vi
pub static EDGE_I_TO_MINOR_VI_9: ProgressionEdge = ProgressionEdge {
    from: I,
    to: MINOR_VI_9,
};

/// Progression edge: I → vi
pub static EDGE_I_TO_MINOR_VI_M7: ProgressionEdge = ProgressionEdge {
    from: I,
    to: MINOR_VI_M7,
};

/// Progression edge: I → vi
pub static EDGE_I_6_TO_MINOR_VI: ProgressionEdge = ProgressionEdge {
    from: I_6,
    to: MINOR_VI,
};

/// Progression edge: I → vi
pub static EDGE_I_6_TO_MINOR_VI_7: ProgressionEdge = ProgressionEdge {
    from: I_6,
    to: MINOR_VI_7,
};

/// Progression edge: I → vi
pub static EDGE_I_6_TO_MINOR_VI_9: ProgressionEdge = ProgressionEdge {
    from: I_6,
    to: MINOR_VI_9,
};

/// Progression edge: I → vi
pub static EDGE_I_6_TO_MINOR_VI_M7: ProgressionEdge = ProgressionEdge {
    from: I_6,
    to: MINOR_VI_M7,
};

/// Progression edge: I → vi
pub static EDGE_I_7_TO_MINOR_VI: ProgressionEdge = ProgressionEdge {
    from: I_7,
    to: MINOR_VI,
};

/// Progression edge: I → vi
pub static EDGE_I_7_TO_MINOR_VI_7: ProgressionEdge = ProgressionEdge {
    from: I_7,
    to: MINOR_VI_7,
};

/// Progression edge: I → vi
pub static EDGE_I_7_TO_MINOR_VI_9: ProgressionEdge = ProgressionEdge {
    from: I_7,
    to: MINOR_VI_9,
};

/// Progression edge: I → vi
pub static EDGE_I_7_TO_MINOR_VI_M7: ProgressionEdge = ProgressionEdge {
    from: I_7,
    to: MINOR_VI_M7,
};

/// Progression edge: I → vi
pub static EDGE_I_9_TO_MINOR_VI: ProgressionEdge = ProgressionEdge {
    from: I_9,
    to: MINOR_VI,
};

/// Progression edge: I → vi
pub static EDGE_I_9_TO_MINOR_VI_7: ProgressionEdge = ProgressionEdge {
    from: I_9,
    to: MINOR_VI_7,
};

/// Progression edge: I → vi
pub static EDGE_I_9_TO_MINOR_VI_9: ProgressionEdge = ProgressionEdge {
    from: I_9,
    to: MINOR_VI_9,
};

/// Progression edge: I → vi
pub static EDGE_I_9_TO_MINOR_VI_M7: ProgressionEdge = ProgressionEdge {
    from: I_9,
    to: MINOR_VI_M7,
};

/// Progression edge: I → vi
pub static EDGE_I_MAJ7_TO_MINOR_VI: ProgressionEdge = ProgressionEdge {
    from: I_MAJ7,
    to: MINOR_VI,
};

/// Progression edge: I → vi
pub static EDGE_I_MAJ7_TO_MINOR_VI_7: ProgressionEdge = ProgressionEdge {
    from: I_MAJ7,
    to: MINOR_VI_7,
};

/// Progression edge: I → vi
pub static EDGE_I_MAJ7_TO_MINOR_VI_9: ProgressionEdge = ProgressionEdge {
    from: I_MAJ7,
    to: MINOR_VI_9,
};

/// Progression edge: I → vi
pub static EDGE_I_MAJ7_TO_MINOR_VI_M7: ProgressionEdge = ProgressionEdge {
    from: I_MAJ7,
    to: MINOR_VI_M7,
};

/// Progression edge: I → vi
pub static EDGE_I_MAJ9_TO_MINOR_VI: ProgressionEdge = ProgressionEdge {
    from: I_MAJ9,
    to: MINOR_VI,
};

/// Progression edge: I → vi
pub static EDGE_I_MAJ9_TO_MINOR_VI_7: ProgressionEdge = ProgressionEdge {
    from: I_MAJ9,
    to: MINOR_VI_7,
};

/// Progression edge: I → vi
pub static EDGE_I_MAJ9_TO_MINOR_VI_9: ProgressionEdge = ProgressionEdge {
    from: I_MAJ9,
    to: MINOR_VI_9,
};

/// Progression edge: I → vi
pub static EDGE_I_MAJ9_TO_MINOR_VI_M7: ProgressionEdge = ProgressionEdge {
    from: I_MAJ9,
    to: MINOR_VI_M7,
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
pub static EDGE_MINOR_II_TO_V_7_PLUS_FLAT_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_II,
    to: V_7_PLUS_FLAT_9,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_TO_V_7_PLUS_SHARP_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_II,
    to: V_7_PLUS_SHARP_9,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_7_TO_V: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_7,
    to: V,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_7_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_7,
    to: V_7,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_7_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_7,
    to: V_9,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_7_TO_V_11: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_7,
    to: V_11,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_7_TO_V_13: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_7,
    to: V_13,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_7_TO_V_7_PLUS_FLAT_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_7,
    to: V_7_PLUS_FLAT_9,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_7_TO_V_7_PLUS_SHARP_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_7,
    to: V_7_PLUS_SHARP_9,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_9_TO_V: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_9,
    to: V,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_9_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_9,
    to: V_7,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_9_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_9,
    to: V_9,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_9_TO_V_11: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_9,
    to: V_11,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_9_TO_V_13: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_9,
    to: V_13,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_9_TO_V_7_PLUS_FLAT_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_9,
    to: V_7_PLUS_FLAT_9,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_9_TO_V_7_PLUS_SHARP_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_9,
    to: V_7_PLUS_SHARP_9,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_11_TO_V: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_11,
    to: V,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_11_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_11,
    to: V_7,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_11_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_11,
    to: V_9,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_11_TO_V_11: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_11,
    to: V_11,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_11_TO_V_13: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_11,
    to: V_13,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_11_TO_V_7_PLUS_FLAT_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_11,
    to: V_7_PLUS_FLAT_9,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_11_TO_V_7_PLUS_SHARP_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_11,
    to: V_7_PLUS_SHARP_9,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_7_PLUS_FLAT_9_TO_V: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_7_PLUS_FLAT_9,
    to: V,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_7_PLUS_FLAT_9_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_7_PLUS_FLAT_9,
    to: V_7,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_7_PLUS_FLAT_9_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_7_PLUS_FLAT_9,
    to: V_9,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_7_PLUS_FLAT_9_TO_V_11: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_7_PLUS_FLAT_9,
    to: V_11,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_7_PLUS_FLAT_9_TO_V_13: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_7_PLUS_FLAT_9,
    to: V_13,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_7_PLUS_FLAT_9_TO_V_7_PLUS_FLAT_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_7_PLUS_FLAT_9,
    to: V_7_PLUS_FLAT_9,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_7_PLUS_FLAT_9_TO_V_7_PLUS_SHARP_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_7_PLUS_FLAT_9,
    to: V_7_PLUS_SHARP_9,
};

/// Progression edge: ii → vii
pub static EDGE_MINOR_II_TO_MINOR_VII: ProgressionEdge = ProgressionEdge {
    from: MINOR_II,
    to: MINOR_VII,
};

/// Progression edge: ii → vii
pub static EDGE_MINOR_II_TO_MINOR_VII_FLAT_5: ProgressionEdge = ProgressionEdge {
    from: MINOR_II,
    to: MINOR_VII_FLAT_5,
};

/// Progression edge: ii → vii
pub static EDGE_MINOR_II_TO_MINOR_VII_M7_FLAT_5: ProgressionEdge = ProgressionEdge {
    from: MINOR_II,
    to: MINOR_VII_M7_FLAT_5,
};

/// Progression edge: ii → vii
pub static EDGE_MINOR_II_7_TO_MINOR_VII: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_7,
    to: MINOR_VII,
};

/// Progression edge: ii → vii
pub static EDGE_MINOR_II_7_TO_MINOR_VII_FLAT_5: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_7,
    to: MINOR_VII_FLAT_5,
};

/// Progression edge: ii → vii
pub static EDGE_MINOR_II_7_TO_MINOR_VII_M7_FLAT_5: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_7,
    to: MINOR_VII_M7_FLAT_5,
};

/// Progression edge: ii → vii
pub static EDGE_MINOR_II_9_TO_MINOR_VII: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_9,
    to: MINOR_VII,
};

/// Progression edge: ii → vii
pub static EDGE_MINOR_II_9_TO_MINOR_VII_FLAT_5: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_9,
    to: MINOR_VII_FLAT_5,
};

/// Progression edge: ii → vii
pub static EDGE_MINOR_II_9_TO_MINOR_VII_M7_FLAT_5: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_9,
    to: MINOR_VII_M7_FLAT_5,
};

/// Progression edge: ii → vii
pub static EDGE_MINOR_II_11_TO_MINOR_VII: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_11,
    to: MINOR_VII,
};

/// Progression edge: ii → vii
pub static EDGE_MINOR_II_11_TO_MINOR_VII_FLAT_5: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_11,
    to: MINOR_VII_FLAT_5,
};

/// Progression edge: ii → vii
pub static EDGE_MINOR_II_11_TO_MINOR_VII_M7_FLAT_5: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_11,
    to: MINOR_VII_M7_FLAT_5,
};

/// Progression edge: ii → vii
pub static EDGE_MINOR_II_7_PLUS_FLAT_9_TO_MINOR_VII: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_7_PLUS_FLAT_9,
    to: MINOR_VII,
};

/// Progression edge: ii → vii
pub static EDGE_MINOR_II_7_PLUS_FLAT_9_TO_MINOR_VII_FLAT_5: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_7_PLUS_FLAT_9,
    to: MINOR_VII_FLAT_5,
};

/// Progression edge: ii → vii
pub static EDGE_MINOR_II_7_PLUS_FLAT_9_TO_MINOR_VII_M7_FLAT_5: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_7_PLUS_FLAT_9,
    to: MINOR_VII_M7_FLAT_5,
};

/// Progression edge: iii → vi
pub static EDGE_MINOR_III_TO_MINOR_VI: ProgressionEdge = ProgressionEdge {
    from: MINOR_III,
    to: MINOR_VI,
};

/// Progression edge: iii → vi
pub static EDGE_MINOR_III_TO_MINOR_VI_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_III,
    to: MINOR_VI_7,
};

/// Progression edge: iii → vi
pub static EDGE_MINOR_III_TO_MINOR_VI_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_III,
    to: MINOR_VI_9,
};

/// Progression edge: iii → vi
pub static EDGE_MINOR_III_TO_MINOR_VI_M7: ProgressionEdge = ProgressionEdge {
    from: MINOR_III,
    to: MINOR_VI_M7,
};

/// Progression edge: iii → vi
pub static EDGE_MINOR_III_7_TO_MINOR_VI: ProgressionEdge = ProgressionEdge {
    from: MINOR_III_7,
    to: MINOR_VI,
};

/// Progression edge: iii → vi
pub static EDGE_MINOR_III_7_TO_MINOR_VI_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_III_7,
    to: MINOR_VI_7,
};

/// Progression edge: iii → vi
pub static EDGE_MINOR_III_7_TO_MINOR_VI_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_III_7,
    to: MINOR_VI_9,
};

/// Progression edge: iii → vi
pub static EDGE_MINOR_III_7_TO_MINOR_VI_M7: ProgressionEdge = ProgressionEdge {
    from: MINOR_III_7,
    to: MINOR_VI_M7,
};

/// Progression edge: iii → vi
pub static EDGE_MINOR_III_M7_TO_MINOR_VI: ProgressionEdge = ProgressionEdge {
    from: MINOR_III_M7,
    to: MINOR_VI,
};

/// Progression edge: iii → vi
pub static EDGE_MINOR_III_M7_TO_MINOR_VI_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_III_M7,
    to: MINOR_VI_7,
};

/// Progression edge: iii → vi
pub static EDGE_MINOR_III_M7_TO_MINOR_VI_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_III_M7,
    to: MINOR_VI_9,
};

/// Progression edge: iii → vi
pub static EDGE_MINOR_III_M7_TO_MINOR_VI_M7: ProgressionEdge = ProgressionEdge {
    from: MINOR_III_M7,
    to: MINOR_VI_M7,
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
pub static EDGE_MINOR_III_TO_IV_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_III,
    to: IV_7,
};

/// Progression edge: iii → IV
pub static EDGE_MINOR_III_TO_IV_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_III,
    to: IV_9,
};

/// Progression edge: iii → IV
pub static EDGE_MINOR_III_TO_IV_MAJ7: ProgressionEdge = ProgressionEdge {
    from: MINOR_III,
    to: IV_MAJ7,
};

/// Progression edge: iii → IV
pub static EDGE_MINOR_III_TO_IV_SHARP_11: ProgressionEdge = ProgressionEdge {
    from: MINOR_III,
    to: IV_SHARP_11,
};

/// Progression edge: iii → IV
pub static EDGE_MINOR_III_7_TO_IV: ProgressionEdge = ProgressionEdge {
    from: MINOR_III_7,
    to: IV,
};

/// Progression edge: iii → IV
pub static EDGE_MINOR_III_7_TO_IV_6: ProgressionEdge = ProgressionEdge {
    from: MINOR_III_7,
    to: IV_6,
};

/// Progression edge: iii → IV
pub static EDGE_MINOR_III_7_TO_IV_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_III_7,
    to: IV_7,
};

/// Progression edge: iii → IV
pub static EDGE_MINOR_III_7_TO_IV_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_III_7,
    to: IV_9,
};

/// Progression edge: iii → IV
pub static EDGE_MINOR_III_7_TO_IV_MAJ7: ProgressionEdge = ProgressionEdge {
    from: MINOR_III_7,
    to: IV_MAJ7,
};

/// Progression edge: iii → IV
pub static EDGE_MINOR_III_7_TO_IV_SHARP_11: ProgressionEdge = ProgressionEdge {
    from: MINOR_III_7,
    to: IV_SHARP_11,
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
pub static EDGE_MINOR_III_M7_TO_IV_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_III_M7,
    to: IV_7,
};

/// Progression edge: iii → IV
pub static EDGE_MINOR_III_M7_TO_IV_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_III_M7,
    to: IV_9,
};

/// Progression edge: iii → IV
pub static EDGE_MINOR_III_M7_TO_IV_MAJ7: ProgressionEdge = ProgressionEdge {
    from: MINOR_III_M7,
    to: IV_MAJ7,
};

/// Progression edge: iii → IV
pub static EDGE_MINOR_III_M7_TO_IV_SHARP_11: ProgressionEdge = ProgressionEdge {
    from: MINOR_III_M7,
    to: IV_SHARP_11,
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
pub static EDGE_IV_TO_I_7: ProgressionEdge = ProgressionEdge {
    from: IV,
    to: I_7,
};

/// Progression edge: IV → I
pub static EDGE_IV_TO_I_9: ProgressionEdge = ProgressionEdge {
    from: IV,
    to: I_9,
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
pub static EDGE_IV_6_TO_I_7: ProgressionEdge = ProgressionEdge {
    from: IV_6,
    to: I_7,
};

/// Progression edge: IV → I
pub static EDGE_IV_6_TO_I_9: ProgressionEdge = ProgressionEdge {
    from: IV_6,
    to: I_9,
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
pub static EDGE_IV_7_TO_I: ProgressionEdge = ProgressionEdge {
    from: IV_7,
    to: I,
};

/// Progression edge: IV → I
pub static EDGE_IV_7_TO_I_6: ProgressionEdge = ProgressionEdge {
    from: IV_7,
    to: I_6,
};

/// Progression edge: IV → I
pub static EDGE_IV_7_TO_I_7: ProgressionEdge = ProgressionEdge {
    from: IV_7,
    to: I_7,
};

/// Progression edge: IV → I
pub static EDGE_IV_7_TO_I_9: ProgressionEdge = ProgressionEdge {
    from: IV_7,
    to: I_9,
};

/// Progression edge: IV → I
pub static EDGE_IV_7_TO_I_MAJ7: ProgressionEdge = ProgressionEdge {
    from: IV_7,
    to: I_MAJ7,
};

/// Progression edge: IV → I
pub static EDGE_IV_7_TO_I_MAJ9: ProgressionEdge = ProgressionEdge {
    from: IV_7,
    to: I_MAJ9,
};

/// Progression edge: IV → I
pub static EDGE_IV_9_TO_I: ProgressionEdge = ProgressionEdge {
    from: IV_9,
    to: I,
};

/// Progression edge: IV → I
pub static EDGE_IV_9_TO_I_6: ProgressionEdge = ProgressionEdge {
    from: IV_9,
    to: I_6,
};

/// Progression edge: IV → I
pub static EDGE_IV_9_TO_I_7: ProgressionEdge = ProgressionEdge {
    from: IV_9,
    to: I_7,
};

/// Progression edge: IV → I
pub static EDGE_IV_9_TO_I_9: ProgressionEdge = ProgressionEdge {
    from: IV_9,
    to: I_9,
};

/// Progression edge: IV → I
pub static EDGE_IV_9_TO_I_MAJ7: ProgressionEdge = ProgressionEdge {
    from: IV_9,
    to: I_MAJ7,
};

/// Progression edge: IV → I
pub static EDGE_IV_9_TO_I_MAJ9: ProgressionEdge = ProgressionEdge {
    from: IV_9,
    to: I_MAJ9,
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
pub static EDGE_IV_MAJ7_TO_I_7: ProgressionEdge = ProgressionEdge {
    from: IV_MAJ7,
    to: I_7,
};

/// Progression edge: IV → I
pub static EDGE_IV_MAJ7_TO_I_9: ProgressionEdge = ProgressionEdge {
    from: IV_MAJ7,
    to: I_9,
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
pub static EDGE_IV_SHARP_11_TO_I: ProgressionEdge = ProgressionEdge {
    from: IV_SHARP_11,
    to: I,
};

/// Progression edge: IV → I
pub static EDGE_IV_SHARP_11_TO_I_6: ProgressionEdge = ProgressionEdge {
    from: IV_SHARP_11,
    to: I_6,
};

/// Progression edge: IV → I
pub static EDGE_IV_SHARP_11_TO_I_7: ProgressionEdge = ProgressionEdge {
    from: IV_SHARP_11,
    to: I_7,
};

/// Progression edge: IV → I
pub static EDGE_IV_SHARP_11_TO_I_9: ProgressionEdge = ProgressionEdge {
    from: IV_SHARP_11,
    to: I_9,
};

/// Progression edge: IV → I
pub static EDGE_IV_SHARP_11_TO_I_MAJ7: ProgressionEdge = ProgressionEdge {
    from: IV_SHARP_11,
    to: I_MAJ7,
};

/// Progression edge: IV → I
pub static EDGE_IV_SHARP_11_TO_I_MAJ9: ProgressionEdge = ProgressionEdge {
    from: IV_SHARP_11,
    to: I_MAJ9,
};

/// Progression edge: IV → ii
pub static EDGE_IV_TO_MINOR_II: ProgressionEdge = ProgressionEdge {
    from: IV,
    to: MINOR_II,
};

/// Progression edge: IV → ii
pub static EDGE_IV_TO_MINOR_II_7: ProgressionEdge = ProgressionEdge {
    from: IV,
    to: MINOR_II_7,
};

/// Progression edge: IV → ii
pub static EDGE_IV_TO_MINOR_II_9: ProgressionEdge = ProgressionEdge {
    from: IV,
    to: MINOR_II_9,
};

/// Progression edge: IV → ii
pub static EDGE_IV_TO_MINOR_II_11: ProgressionEdge = ProgressionEdge {
    from: IV,
    to: MINOR_II_11,
};

/// Progression edge: IV → ii
pub static EDGE_IV_TO_MINOR_II_7_PLUS_FLAT_9: ProgressionEdge = ProgressionEdge {
    from: IV,
    to: MINOR_II_7_PLUS_FLAT_9,
};

/// Progression edge: IV → ii
pub static EDGE_IV_6_TO_MINOR_II: ProgressionEdge = ProgressionEdge {
    from: IV_6,
    to: MINOR_II,
};

/// Progression edge: IV → ii
pub static EDGE_IV_6_TO_MINOR_II_7: ProgressionEdge = ProgressionEdge {
    from: IV_6,
    to: MINOR_II_7,
};

/// Progression edge: IV → ii
pub static EDGE_IV_6_TO_MINOR_II_9: ProgressionEdge = ProgressionEdge {
    from: IV_6,
    to: MINOR_II_9,
};

/// Progression edge: IV → ii
pub static EDGE_IV_6_TO_MINOR_II_11: ProgressionEdge = ProgressionEdge {
    from: IV_6,
    to: MINOR_II_11,
};

/// Progression edge: IV → ii
pub static EDGE_IV_6_TO_MINOR_II_7_PLUS_FLAT_9: ProgressionEdge = ProgressionEdge {
    from: IV_6,
    to: MINOR_II_7_PLUS_FLAT_9,
};

/// Progression edge: IV → ii
pub static EDGE_IV_7_TO_MINOR_II: ProgressionEdge = ProgressionEdge {
    from: IV_7,
    to: MINOR_II,
};

/// Progression edge: IV → ii
pub static EDGE_IV_7_TO_MINOR_II_7: ProgressionEdge = ProgressionEdge {
    from: IV_7,
    to: MINOR_II_7,
};

/// Progression edge: IV → ii
pub static EDGE_IV_7_TO_MINOR_II_9: ProgressionEdge = ProgressionEdge {
    from: IV_7,
    to: MINOR_II_9,
};

/// Progression edge: IV → ii
pub static EDGE_IV_7_TO_MINOR_II_11: ProgressionEdge = ProgressionEdge {
    from: IV_7,
    to: MINOR_II_11,
};

/// Progression edge: IV → ii
pub static EDGE_IV_7_TO_MINOR_II_7_PLUS_FLAT_9: ProgressionEdge = ProgressionEdge {
    from: IV_7,
    to: MINOR_II_7_PLUS_FLAT_9,
};

/// Progression edge: IV → ii
pub static EDGE_IV_9_TO_MINOR_II: ProgressionEdge = ProgressionEdge {
    from: IV_9,
    to: MINOR_II,
};

/// Progression edge: IV → ii
pub static EDGE_IV_9_TO_MINOR_II_7: ProgressionEdge = ProgressionEdge {
    from: IV_9,
    to: MINOR_II_7,
};

/// Progression edge: IV → ii
pub static EDGE_IV_9_TO_MINOR_II_9: ProgressionEdge = ProgressionEdge {
    from: IV_9,
    to: MINOR_II_9,
};

/// Progression edge: IV → ii
pub static EDGE_IV_9_TO_MINOR_II_11: ProgressionEdge = ProgressionEdge {
    from: IV_9,
    to: MINOR_II_11,
};

/// Progression edge: IV → ii
pub static EDGE_IV_9_TO_MINOR_II_7_PLUS_FLAT_9: ProgressionEdge = ProgressionEdge {
    from: IV_9,
    to: MINOR_II_7_PLUS_FLAT_9,
};

/// Progression edge: IV → ii
pub static EDGE_IV_MAJ7_TO_MINOR_II: ProgressionEdge = ProgressionEdge {
    from: IV_MAJ7,
    to: MINOR_II,
};

/// Progression edge: IV → ii
pub static EDGE_IV_MAJ7_TO_MINOR_II_7: ProgressionEdge = ProgressionEdge {
    from: IV_MAJ7,
    to: MINOR_II_7,
};

/// Progression edge: IV → ii
pub static EDGE_IV_MAJ7_TO_MINOR_II_9: ProgressionEdge = ProgressionEdge {
    from: IV_MAJ7,
    to: MINOR_II_9,
};

/// Progression edge: IV → ii
pub static EDGE_IV_MAJ7_TO_MINOR_II_11: ProgressionEdge = ProgressionEdge {
    from: IV_MAJ7,
    to: MINOR_II_11,
};

/// Progression edge: IV → ii
pub static EDGE_IV_MAJ7_TO_MINOR_II_7_PLUS_FLAT_9: ProgressionEdge = ProgressionEdge {
    from: IV_MAJ7,
    to: MINOR_II_7_PLUS_FLAT_9,
};

/// Progression edge: IV → ii
pub static EDGE_IV_SHARP_11_TO_MINOR_II: ProgressionEdge = ProgressionEdge {
    from: IV_SHARP_11,
    to: MINOR_II,
};

/// Progression edge: IV → ii
pub static EDGE_IV_SHARP_11_TO_MINOR_II_7: ProgressionEdge = ProgressionEdge {
    from: IV_SHARP_11,
    to: MINOR_II_7,
};

/// Progression edge: IV → ii
pub static EDGE_IV_SHARP_11_TO_MINOR_II_9: ProgressionEdge = ProgressionEdge {
    from: IV_SHARP_11,
    to: MINOR_II_9,
};

/// Progression edge: IV → ii
pub static EDGE_IV_SHARP_11_TO_MINOR_II_11: ProgressionEdge = ProgressionEdge {
    from: IV_SHARP_11,
    to: MINOR_II_11,
};

/// Progression edge: IV → ii
pub static EDGE_IV_SHARP_11_TO_MINOR_II_7_PLUS_FLAT_9: ProgressionEdge = ProgressionEdge {
    from: IV_SHARP_11,
    to: MINOR_II_7_PLUS_FLAT_9,
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
pub static EDGE_IV_TO_V_7_PLUS_FLAT_9: ProgressionEdge = ProgressionEdge {
    from: IV,
    to: V_7_PLUS_FLAT_9,
};

/// Progression edge: IV → V
pub static EDGE_IV_TO_V_7_PLUS_SHARP_9: ProgressionEdge = ProgressionEdge {
    from: IV,
    to: V_7_PLUS_SHARP_9,
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
pub static EDGE_IV_6_TO_V_7_PLUS_FLAT_9: ProgressionEdge = ProgressionEdge {
    from: IV_6,
    to: V_7_PLUS_FLAT_9,
};

/// Progression edge: IV → V
pub static EDGE_IV_6_TO_V_7_PLUS_SHARP_9: ProgressionEdge = ProgressionEdge {
    from: IV_6,
    to: V_7_PLUS_SHARP_9,
};

/// Progression edge: IV → V
pub static EDGE_IV_7_TO_V: ProgressionEdge = ProgressionEdge {
    from: IV_7,
    to: V,
};

/// Progression edge: IV → V
pub static EDGE_IV_7_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: IV_7,
    to: V_7,
};

/// Progression edge: IV → V
pub static EDGE_IV_7_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: IV_7,
    to: V_9,
};

/// Progression edge: IV → V
pub static EDGE_IV_7_TO_V_11: ProgressionEdge = ProgressionEdge {
    from: IV_7,
    to: V_11,
};

/// Progression edge: IV → V
pub static EDGE_IV_7_TO_V_13: ProgressionEdge = ProgressionEdge {
    from: IV_7,
    to: V_13,
};

/// Progression edge: IV → V
pub static EDGE_IV_7_TO_V_7_PLUS_FLAT_9: ProgressionEdge = ProgressionEdge {
    from: IV_7,
    to: V_7_PLUS_FLAT_9,
};

/// Progression edge: IV → V
pub static EDGE_IV_7_TO_V_7_PLUS_SHARP_9: ProgressionEdge = ProgressionEdge {
    from: IV_7,
    to: V_7_PLUS_SHARP_9,
};

/// Progression edge: IV → V
pub static EDGE_IV_9_TO_V: ProgressionEdge = ProgressionEdge {
    from: IV_9,
    to: V,
};

/// Progression edge: IV → V
pub static EDGE_IV_9_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: IV_9,
    to: V_7,
};

/// Progression edge: IV → V
pub static EDGE_IV_9_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: IV_9,
    to: V_9,
};

/// Progression edge: IV → V
pub static EDGE_IV_9_TO_V_11: ProgressionEdge = ProgressionEdge {
    from: IV_9,
    to: V_11,
};

/// Progression edge: IV → V
pub static EDGE_IV_9_TO_V_13: ProgressionEdge = ProgressionEdge {
    from: IV_9,
    to: V_13,
};

/// Progression edge: IV → V
pub static EDGE_IV_9_TO_V_7_PLUS_FLAT_9: ProgressionEdge = ProgressionEdge {
    from: IV_9,
    to: V_7_PLUS_FLAT_9,
};

/// Progression edge: IV → V
pub static EDGE_IV_9_TO_V_7_PLUS_SHARP_9: ProgressionEdge = ProgressionEdge {
    from: IV_9,
    to: V_7_PLUS_SHARP_9,
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
pub static EDGE_IV_MAJ7_TO_V_7_PLUS_FLAT_9: ProgressionEdge = ProgressionEdge {
    from: IV_MAJ7,
    to: V_7_PLUS_FLAT_9,
};

/// Progression edge: IV → V
pub static EDGE_IV_MAJ7_TO_V_7_PLUS_SHARP_9: ProgressionEdge = ProgressionEdge {
    from: IV_MAJ7,
    to: V_7_PLUS_SHARP_9,
};

/// Progression edge: IV → V
pub static EDGE_IV_SHARP_11_TO_V: ProgressionEdge = ProgressionEdge {
    from: IV_SHARP_11,
    to: V,
};

/// Progression edge: IV → V
pub static EDGE_IV_SHARP_11_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: IV_SHARP_11,
    to: V_7,
};

/// Progression edge: IV → V
pub static EDGE_IV_SHARP_11_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: IV_SHARP_11,
    to: V_9,
};

/// Progression edge: IV → V
pub static EDGE_IV_SHARP_11_TO_V_11: ProgressionEdge = ProgressionEdge {
    from: IV_SHARP_11,
    to: V_11,
};

/// Progression edge: IV → V
pub static EDGE_IV_SHARP_11_TO_V_13: ProgressionEdge = ProgressionEdge {
    from: IV_SHARP_11,
    to: V_13,
};

/// Progression edge: IV → V
pub static EDGE_IV_SHARP_11_TO_V_7_PLUS_FLAT_9: ProgressionEdge = ProgressionEdge {
    from: IV_SHARP_11,
    to: V_7_PLUS_FLAT_9,
};

/// Progression edge: IV → V
pub static EDGE_IV_SHARP_11_TO_V_7_PLUS_SHARP_9: ProgressionEdge = ProgressionEdge {
    from: IV_SHARP_11,
    to: V_7_PLUS_SHARP_9,
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
pub static EDGE_V_TO_I_7: ProgressionEdge = ProgressionEdge {
    from: V,
    to: I_7,
};

/// Progression edge: V → I
pub static EDGE_V_TO_I_9: ProgressionEdge = ProgressionEdge {
    from: V,
    to: I_9,
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
pub static EDGE_V_7_TO_I_7: ProgressionEdge = ProgressionEdge {
    from: V_7,
    to: I_7,
};

/// Progression edge: V → I
pub static EDGE_V_7_TO_I_9: ProgressionEdge = ProgressionEdge {
    from: V_7,
    to: I_9,
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
pub static EDGE_V_9_TO_I_7: ProgressionEdge = ProgressionEdge {
    from: V_9,
    to: I_7,
};

/// Progression edge: V → I
pub static EDGE_V_9_TO_I_9: ProgressionEdge = ProgressionEdge {
    from: V_9,
    to: I_9,
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
pub static EDGE_V_11_TO_I_7: ProgressionEdge = ProgressionEdge {
    from: V_11,
    to: I_7,
};

/// Progression edge: V → I
pub static EDGE_V_11_TO_I_9: ProgressionEdge = ProgressionEdge {
    from: V_11,
    to: I_9,
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
pub static EDGE_V_13_TO_I_7: ProgressionEdge = ProgressionEdge {
    from: V_13,
    to: I_7,
};

/// Progression edge: V → I
pub static EDGE_V_13_TO_I_9: ProgressionEdge = ProgressionEdge {
    from: V_13,
    to: I_9,
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
pub static EDGE_V_7_PLUS_FLAT_9_TO_I: ProgressionEdge = ProgressionEdge {
    from: V_7_PLUS_FLAT_9,
    to: I,
};

/// Progression edge: V → I
pub static EDGE_V_7_PLUS_FLAT_9_TO_I_6: ProgressionEdge = ProgressionEdge {
    from: V_7_PLUS_FLAT_9,
    to: I_6,
};

/// Progression edge: V → I
pub static EDGE_V_7_PLUS_FLAT_9_TO_I_7: ProgressionEdge = ProgressionEdge {
    from: V_7_PLUS_FLAT_9,
    to: I_7,
};

/// Progression edge: V → I
pub static EDGE_V_7_PLUS_FLAT_9_TO_I_9: ProgressionEdge = ProgressionEdge {
    from: V_7_PLUS_FLAT_9,
    to: I_9,
};

/// Progression edge: V → I
pub static EDGE_V_7_PLUS_FLAT_9_TO_I_MAJ7: ProgressionEdge = ProgressionEdge {
    from: V_7_PLUS_FLAT_9,
    to: I_MAJ7,
};

/// Progression edge: V → I
pub static EDGE_V_7_PLUS_FLAT_9_TO_I_MAJ9: ProgressionEdge = ProgressionEdge {
    from: V_7_PLUS_FLAT_9,
    to: I_MAJ9,
};

/// Progression edge: V → I
pub static EDGE_V_7_PLUS_SHARP_9_TO_I: ProgressionEdge = ProgressionEdge {
    from: V_7_PLUS_SHARP_9,
    to: I,
};

/// Progression edge: V → I
pub static EDGE_V_7_PLUS_SHARP_9_TO_I_6: ProgressionEdge = ProgressionEdge {
    from: V_7_PLUS_SHARP_9,
    to: I_6,
};

/// Progression edge: V → I
pub static EDGE_V_7_PLUS_SHARP_9_TO_I_7: ProgressionEdge = ProgressionEdge {
    from: V_7_PLUS_SHARP_9,
    to: I_7,
};

/// Progression edge: V → I
pub static EDGE_V_7_PLUS_SHARP_9_TO_I_9: ProgressionEdge = ProgressionEdge {
    from: V_7_PLUS_SHARP_9,
    to: I_9,
};

/// Progression edge: V → I
pub static EDGE_V_7_PLUS_SHARP_9_TO_I_MAJ7: ProgressionEdge = ProgressionEdge {
    from: V_7_PLUS_SHARP_9,
    to: I_MAJ7,
};

/// Progression edge: V → I
pub static EDGE_V_7_PLUS_SHARP_9_TO_I_MAJ9: ProgressionEdge = ProgressionEdge {
    from: V_7_PLUS_SHARP_9,
    to: I_MAJ9,
};

/// Progression edge: V → vi
pub static EDGE_V_TO_MINOR_VI: ProgressionEdge = ProgressionEdge {
    from: V,
    to: MINOR_VI,
};

/// Progression edge: V → vi
pub static EDGE_V_TO_MINOR_VI_7: ProgressionEdge = ProgressionEdge {
    from: V,
    to: MINOR_VI_7,
};

/// Progression edge: V → vi
pub static EDGE_V_TO_MINOR_VI_9: ProgressionEdge = ProgressionEdge {
    from: V,
    to: MINOR_VI_9,
};

/// Progression edge: V → vi
pub static EDGE_V_TO_MINOR_VI_M7: ProgressionEdge = ProgressionEdge {
    from: V,
    to: MINOR_VI_M7,
};

/// Progression edge: V → vi
pub static EDGE_V_7_TO_MINOR_VI: ProgressionEdge = ProgressionEdge {
    from: V_7,
    to: MINOR_VI,
};

/// Progression edge: V → vi
pub static EDGE_V_7_TO_MINOR_VI_7: ProgressionEdge = ProgressionEdge {
    from: V_7,
    to: MINOR_VI_7,
};

/// Progression edge: V → vi
pub static EDGE_V_7_TO_MINOR_VI_9: ProgressionEdge = ProgressionEdge {
    from: V_7,
    to: MINOR_VI_9,
};

/// Progression edge: V → vi
pub static EDGE_V_7_TO_MINOR_VI_M7: ProgressionEdge = ProgressionEdge {
    from: V_7,
    to: MINOR_VI_M7,
};

/// Progression edge: V → vi
pub static EDGE_V_9_TO_MINOR_VI: ProgressionEdge = ProgressionEdge {
    from: V_9,
    to: MINOR_VI,
};

/// Progression edge: V → vi
pub static EDGE_V_9_TO_MINOR_VI_7: ProgressionEdge = ProgressionEdge {
    from: V_9,
    to: MINOR_VI_7,
};

/// Progression edge: V → vi
pub static EDGE_V_9_TO_MINOR_VI_9: ProgressionEdge = ProgressionEdge {
    from: V_9,
    to: MINOR_VI_9,
};

/// Progression edge: V → vi
pub static EDGE_V_9_TO_MINOR_VI_M7: ProgressionEdge = ProgressionEdge {
    from: V_9,
    to: MINOR_VI_M7,
};

/// Progression edge: V → vi
pub static EDGE_V_11_TO_MINOR_VI: ProgressionEdge = ProgressionEdge {
    from: V_11,
    to: MINOR_VI,
};

/// Progression edge: V → vi
pub static EDGE_V_11_TO_MINOR_VI_7: ProgressionEdge = ProgressionEdge {
    from: V_11,
    to: MINOR_VI_7,
};

/// Progression edge: V → vi
pub static EDGE_V_11_TO_MINOR_VI_9: ProgressionEdge = ProgressionEdge {
    from: V_11,
    to: MINOR_VI_9,
};

/// Progression edge: V → vi
pub static EDGE_V_11_TO_MINOR_VI_M7: ProgressionEdge = ProgressionEdge {
    from: V_11,
    to: MINOR_VI_M7,
};

/// Progression edge: V → vi
pub static EDGE_V_13_TO_MINOR_VI: ProgressionEdge = ProgressionEdge {
    from: V_13,
    to: MINOR_VI,
};

/// Progression edge: V → vi
pub static EDGE_V_13_TO_MINOR_VI_7: ProgressionEdge = ProgressionEdge {
    from: V_13,
    to: MINOR_VI_7,
};

/// Progression edge: V → vi
pub static EDGE_V_13_TO_MINOR_VI_9: ProgressionEdge = ProgressionEdge {
    from: V_13,
    to: MINOR_VI_9,
};

/// Progression edge: V → vi
pub static EDGE_V_13_TO_MINOR_VI_M7: ProgressionEdge = ProgressionEdge {
    from: V_13,
    to: MINOR_VI_M7,
};

/// Progression edge: V → vi
pub static EDGE_V_7_PLUS_FLAT_9_TO_MINOR_VI: ProgressionEdge = ProgressionEdge {
    from: V_7_PLUS_FLAT_9,
    to: MINOR_VI,
};

/// Progression edge: V → vi
pub static EDGE_V_7_PLUS_FLAT_9_TO_MINOR_VI_7: ProgressionEdge = ProgressionEdge {
    from: V_7_PLUS_FLAT_9,
    to: MINOR_VI_7,
};

/// Progression edge: V → vi
pub static EDGE_V_7_PLUS_FLAT_9_TO_MINOR_VI_9: ProgressionEdge = ProgressionEdge {
    from: V_7_PLUS_FLAT_9,
    to: MINOR_VI_9,
};

/// Progression edge: V → vi
pub static EDGE_V_7_PLUS_FLAT_9_TO_MINOR_VI_M7: ProgressionEdge = ProgressionEdge {
    from: V_7_PLUS_FLAT_9,
    to: MINOR_VI_M7,
};

/// Progression edge: V → vi
pub static EDGE_V_7_PLUS_SHARP_9_TO_MINOR_VI: ProgressionEdge = ProgressionEdge {
    from: V_7_PLUS_SHARP_9,
    to: MINOR_VI,
};

/// Progression edge: V → vi
pub static EDGE_V_7_PLUS_SHARP_9_TO_MINOR_VI_7: ProgressionEdge = ProgressionEdge {
    from: V_7_PLUS_SHARP_9,
    to: MINOR_VI_7,
};

/// Progression edge: V → vi
pub static EDGE_V_7_PLUS_SHARP_9_TO_MINOR_VI_9: ProgressionEdge = ProgressionEdge {
    from: V_7_PLUS_SHARP_9,
    to: MINOR_VI_9,
};

/// Progression edge: V → vi
pub static EDGE_V_7_PLUS_SHARP_9_TO_MINOR_VI_M7: ProgressionEdge = ProgressionEdge {
    from: V_7_PLUS_SHARP_9,
    to: MINOR_VI_M7,
};

/// Progression edge: vi → ii
pub static EDGE_MINOR_VI_TO_MINOR_II: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI,
    to: MINOR_II,
};

/// Progression edge: vi → ii
pub static EDGE_MINOR_VI_TO_MINOR_II_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI,
    to: MINOR_II_7,
};

/// Progression edge: vi → ii
pub static EDGE_MINOR_VI_TO_MINOR_II_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI,
    to: MINOR_II_9,
};

/// Progression edge: vi → ii
pub static EDGE_MINOR_VI_TO_MINOR_II_11: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI,
    to: MINOR_II_11,
};

/// Progression edge: vi → ii
pub static EDGE_MINOR_VI_TO_MINOR_II_7_PLUS_FLAT_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI,
    to: MINOR_II_7_PLUS_FLAT_9,
};

/// Progression edge: vi → ii
pub static EDGE_MINOR_VI_7_TO_MINOR_II: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI_7,
    to: MINOR_II,
};

/// Progression edge: vi → ii
pub static EDGE_MINOR_VI_7_TO_MINOR_II_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI_7,
    to: MINOR_II_7,
};

/// Progression edge: vi → ii
pub static EDGE_MINOR_VI_7_TO_MINOR_II_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI_7,
    to: MINOR_II_9,
};

/// Progression edge: vi → ii
pub static EDGE_MINOR_VI_7_TO_MINOR_II_11: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI_7,
    to: MINOR_II_11,
};

/// Progression edge: vi → ii
pub static EDGE_MINOR_VI_7_TO_MINOR_II_7_PLUS_FLAT_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI_7,
    to: MINOR_II_7_PLUS_FLAT_9,
};

/// Progression edge: vi → ii
pub static EDGE_MINOR_VI_9_TO_MINOR_II: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI_9,
    to: MINOR_II,
};

/// Progression edge: vi → ii
pub static EDGE_MINOR_VI_9_TO_MINOR_II_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI_9,
    to: MINOR_II_7,
};

/// Progression edge: vi → ii
pub static EDGE_MINOR_VI_9_TO_MINOR_II_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI_9,
    to: MINOR_II_9,
};

/// Progression edge: vi → ii
pub static EDGE_MINOR_VI_9_TO_MINOR_II_11: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI_9,
    to: MINOR_II_11,
};

/// Progression edge: vi → ii
pub static EDGE_MINOR_VI_9_TO_MINOR_II_7_PLUS_FLAT_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI_9,
    to: MINOR_II_7_PLUS_FLAT_9,
};

/// Progression edge: vi → ii
pub static EDGE_MINOR_VI_M7_TO_MINOR_II: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI_M7,
    to: MINOR_II,
};

/// Progression edge: vi → ii
pub static EDGE_MINOR_VI_M7_TO_MINOR_II_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI_M7,
    to: MINOR_II_7,
};

/// Progression edge: vi → ii
pub static EDGE_MINOR_VI_M7_TO_MINOR_II_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI_M7,
    to: MINOR_II_9,
};

/// Progression edge: vi → ii
pub static EDGE_MINOR_VI_M7_TO_MINOR_II_11: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI_M7,
    to: MINOR_II_11,
};

/// Progression edge: vi → ii
pub static EDGE_MINOR_VI_M7_TO_MINOR_II_7_PLUS_FLAT_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI_M7,
    to: MINOR_II_7_PLUS_FLAT_9,
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
pub static EDGE_MINOR_VI_TO_IV_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI,
    to: IV_7,
};

/// Progression edge: vi → IV
pub static EDGE_MINOR_VI_TO_IV_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI,
    to: IV_9,
};

/// Progression edge: vi → IV
pub static EDGE_MINOR_VI_TO_IV_MAJ7: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI,
    to: IV_MAJ7,
};

/// Progression edge: vi → IV
pub static EDGE_MINOR_VI_TO_IV_SHARP_11: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI,
    to: IV_SHARP_11,
};

/// Progression edge: vi → IV
pub static EDGE_MINOR_VI_7_TO_IV: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI_7,
    to: IV,
};

/// Progression edge: vi → IV
pub static EDGE_MINOR_VI_7_TO_IV_6: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI_7,
    to: IV_6,
};

/// Progression edge: vi → IV
pub static EDGE_MINOR_VI_7_TO_IV_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI_7,
    to: IV_7,
};

/// Progression edge: vi → IV
pub static EDGE_MINOR_VI_7_TO_IV_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI_7,
    to: IV_9,
};

/// Progression edge: vi → IV
pub static EDGE_MINOR_VI_7_TO_IV_MAJ7: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI_7,
    to: IV_MAJ7,
};

/// Progression edge: vi → IV
pub static EDGE_MINOR_VI_7_TO_IV_SHARP_11: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI_7,
    to: IV_SHARP_11,
};

/// Progression edge: vi → IV
pub static EDGE_MINOR_VI_9_TO_IV: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI_9,
    to: IV,
};

/// Progression edge: vi → IV
pub static EDGE_MINOR_VI_9_TO_IV_6: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI_9,
    to: IV_6,
};

/// Progression edge: vi → IV
pub static EDGE_MINOR_VI_9_TO_IV_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI_9,
    to: IV_7,
};

/// Progression edge: vi → IV
pub static EDGE_MINOR_VI_9_TO_IV_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI_9,
    to: IV_9,
};

/// Progression edge: vi → IV
pub static EDGE_MINOR_VI_9_TO_IV_MAJ7: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI_9,
    to: IV_MAJ7,
};

/// Progression edge: vi → IV
pub static EDGE_MINOR_VI_9_TO_IV_SHARP_11: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI_9,
    to: IV_SHARP_11,
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
pub static EDGE_MINOR_VI_M7_TO_IV_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI_M7,
    to: IV_7,
};

/// Progression edge: vi → IV
pub static EDGE_MINOR_VI_M7_TO_IV_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI_M7,
    to: IV_9,
};

/// Progression edge: vi → IV
pub static EDGE_MINOR_VI_M7_TO_IV_MAJ7: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI_M7,
    to: IV_MAJ7,
};

/// Progression edge: vi → IV
pub static EDGE_MINOR_VI_M7_TO_IV_SHARP_11: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI_M7,
    to: IV_SHARP_11,
};

/// Progression edge: vii → I
pub static EDGE_MINOR_VII_TO_I: ProgressionEdge = ProgressionEdge {
    from: MINOR_VII,
    to: I,
};

/// Progression edge: vii → I
pub static EDGE_MINOR_VII_TO_I_6: ProgressionEdge = ProgressionEdge {
    from: MINOR_VII,
    to: I_6,
};

/// Progression edge: vii → I
pub static EDGE_MINOR_VII_TO_I_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_VII,
    to: I_7,
};

/// Progression edge: vii → I
pub static EDGE_MINOR_VII_TO_I_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_VII,
    to: I_9,
};

/// Progression edge: vii → I
pub static EDGE_MINOR_VII_TO_I_MAJ7: ProgressionEdge = ProgressionEdge {
    from: MINOR_VII,
    to: I_MAJ7,
};

/// Progression edge: vii → I
pub static EDGE_MINOR_VII_TO_I_MAJ9: ProgressionEdge = ProgressionEdge {
    from: MINOR_VII,
    to: I_MAJ9,
};

/// Progression edge: vii → I
pub static EDGE_MINOR_VII_FLAT_5_TO_I: ProgressionEdge = ProgressionEdge {
    from: MINOR_VII_FLAT_5,
    to: I,
};

/// Progression edge: vii → I
pub static EDGE_MINOR_VII_FLAT_5_TO_I_6: ProgressionEdge = ProgressionEdge {
    from: MINOR_VII_FLAT_5,
    to: I_6,
};

/// Progression edge: vii → I
pub static EDGE_MINOR_VII_FLAT_5_TO_I_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_VII_FLAT_5,
    to: I_7,
};

/// Progression edge: vii → I
pub static EDGE_MINOR_VII_FLAT_5_TO_I_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_VII_FLAT_5,
    to: I_9,
};

/// Progression edge: vii → I
pub static EDGE_MINOR_VII_FLAT_5_TO_I_MAJ7: ProgressionEdge = ProgressionEdge {
    from: MINOR_VII_FLAT_5,
    to: I_MAJ7,
};

/// Progression edge: vii → I
pub static EDGE_MINOR_VII_FLAT_5_TO_I_MAJ9: ProgressionEdge = ProgressionEdge {
    from: MINOR_VII_FLAT_5,
    to: I_MAJ9,
};

/// Progression edge: vii → I
pub static EDGE_MINOR_VII_M7_FLAT_5_TO_I: ProgressionEdge = ProgressionEdge {
    from: MINOR_VII_M7_FLAT_5,
    to: I,
};

/// Progression edge: vii → I
pub static EDGE_MINOR_VII_M7_FLAT_5_TO_I_6: ProgressionEdge = ProgressionEdge {
    from: MINOR_VII_M7_FLAT_5,
    to: I_6,
};

/// Progression edge: vii → I
pub static EDGE_MINOR_VII_M7_FLAT_5_TO_I_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_VII_M7_FLAT_5,
    to: I_7,
};

/// Progression edge: vii → I
pub static EDGE_MINOR_VII_M7_FLAT_5_TO_I_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_VII_M7_FLAT_5,
    to: I_9,
};

/// Progression edge: vii → I
pub static EDGE_MINOR_VII_M7_FLAT_5_TO_I_MAJ7: ProgressionEdge = ProgressionEdge {
    from: MINOR_VII_M7_FLAT_5,
    to: I_MAJ7,
};

/// Progression edge: vii → I
pub static EDGE_MINOR_VII_M7_FLAT_5_TO_I_MAJ9: ProgressionEdge = ProgressionEdge {
    from: MINOR_VII_M7_FLAT_5,
    to: I_MAJ9,
};

/// Progression edge: bIII → IV
pub static EDGE_MINOR_FLAT_III_7_TO_IV: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_III_7,
    to: IV,
};

/// Progression edge: bIII → IV
pub static EDGE_MINOR_FLAT_III_7_TO_IV_6: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_III_7,
    to: IV_6,
};

/// Progression edge: bIII → IV
pub static EDGE_MINOR_FLAT_III_7_TO_IV_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_III_7,
    to: IV_7,
};

/// Progression edge: bIII → IV
pub static EDGE_MINOR_FLAT_III_7_TO_IV_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_III_7,
    to: IV_9,
};

/// Progression edge: bIII → IV
pub static EDGE_MINOR_FLAT_III_7_TO_IV_MAJ7: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_III_7,
    to: IV_MAJ7,
};

/// Progression edge: bIII → IV
pub static EDGE_MINOR_FLAT_III_7_TO_IV_SHARP_11: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_III_7,
    to: IV_SHARP_11,
};

/// Progression edge: bIII → IV
pub static EDGE_MINOR_FLAT_III_9_TO_IV: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_III_9,
    to: IV,
};

/// Progression edge: bIII → IV
pub static EDGE_MINOR_FLAT_III_9_TO_IV_6: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_III_9,
    to: IV_6,
};

/// Progression edge: bIII → IV
pub static EDGE_MINOR_FLAT_III_9_TO_IV_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_III_9,
    to: IV_7,
};

/// Progression edge: bIII → IV
pub static EDGE_MINOR_FLAT_III_9_TO_IV_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_III_9,
    to: IV_9,
};

/// Progression edge: bIII → IV
pub static EDGE_MINOR_FLAT_III_9_TO_IV_MAJ7: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_III_9,
    to: IV_MAJ7,
};

/// Progression edge: bIII → IV
pub static EDGE_MINOR_FLAT_III_9_TO_IV_SHARP_11: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_III_9,
    to: IV_SHARP_11,
};

/// Progression edge: bVI → bVII
pub static EDGE_MINOR_FLAT_VI_7_TO_MINOR_FLAT_VII_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VI_7,
    to: MINOR_FLAT_VII_7,
};

/// Progression edge: bVI → bVII
pub static EDGE_MINOR_FLAT_VI_7_TO_MINOR_FLAT_VII_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VI_7,
    to: MINOR_FLAT_VII_9,
};

/// Progression edge: bVI → bVII
pub static EDGE_MINOR_FLAT_VI_9_TO_MINOR_FLAT_VII_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VI_9,
    to: MINOR_FLAT_VII_7,
};

/// Progression edge: bVI → bVII
pub static EDGE_MINOR_FLAT_VI_9_TO_MINOR_FLAT_VII_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VI_9,
    to: MINOR_FLAT_VII_9,
};

/// Progression edge: bVII → I
pub static EDGE_MINOR_FLAT_VII_7_TO_I: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VII_7,
    to: I,
};

/// Progression edge: bVII → I
pub static EDGE_MINOR_FLAT_VII_7_TO_I_6: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VII_7,
    to: I_6,
};

/// Progression edge: bVII → I
pub static EDGE_MINOR_FLAT_VII_7_TO_I_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VII_7,
    to: I_7,
};

/// Progression edge: bVII → I
pub static EDGE_MINOR_FLAT_VII_7_TO_I_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VII_7,
    to: I_9,
};

/// Progression edge: bVII → I
pub static EDGE_MINOR_FLAT_VII_7_TO_I_MAJ7: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VII_7,
    to: I_MAJ7,
};

/// Progression edge: bVII → I
pub static EDGE_MINOR_FLAT_VII_7_TO_I_MAJ9: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VII_7,
    to: I_MAJ9,
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
pub static EDGE_MINOR_FLAT_VII_9_TO_I_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VII_9,
    to: I_7,
};

/// Progression edge: bVII → I
pub static EDGE_MINOR_FLAT_VII_9_TO_I_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VII_9,
    to: I_9,
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

/// Complete registry of all progression chords for major keys
/// 
/// Contains 40 chord variants across all harmonic functions.
/// Used internally for graph traversal and chord lookup operations.
pub static ALL_NODES: &[&RomanChord] = &[
    &I,
    &I_6,
    &I_7,
    &I_9,
    &I_MAJ7,
    &I_MAJ9,
    &MINOR_II,
    &MINOR_II_7,
    &MINOR_II_9,
    &MINOR_II_11,
    &MINOR_II_7_PLUS_FLAT_9,
    &MINOR_III,
    &MINOR_III_7,
    &MINOR_III_M7,
    &IV,
    &IV_6,
    &IV_7,
    &IV_9,
    &IV_MAJ7,
    &IV_SHARP_11,
    &V,
    &V_7,
    &V_9,
    &V_11,
    &V_13,
    &V_7_PLUS_FLAT_9,
    &V_7_PLUS_SHARP_9,
    &MINOR_VI,
    &MINOR_VI_7,
    &MINOR_VI_9,
    &MINOR_VI_M7,
    &MINOR_VII,
    &MINOR_VII_FLAT_5,
    &MINOR_VII_M7_FLAT_5,
    &MINOR_FLAT_III_7,
    &MINOR_FLAT_III_9,
    &MINOR_FLAT_VI_7,
    &MINOR_FLAT_VI_9,
    &MINOR_FLAT_VII_7,
    &MINOR_FLAT_VII_9,
];

/// Complete registry of all progression edges for major keys
/// 
/// Contains 450 harmonic connections between chord variants.
/// Each edge represents a musically valid progression with proper voice leading.
pub static ALL_EDGES: &[&ProgressionEdge] = &[
    &EDGE_I_TO_IV,
    &EDGE_I_TO_IV_6,
    &EDGE_I_TO_IV_7,
    &EDGE_I_TO_IV_9,
    &EDGE_I_TO_IV_MAJ7,
    &EDGE_I_TO_IV_SHARP_11,
    &EDGE_I_6_TO_IV,
    &EDGE_I_6_TO_IV_6,
    &EDGE_I_6_TO_IV_7,
    &EDGE_I_6_TO_IV_9,
    &EDGE_I_6_TO_IV_MAJ7,
    &EDGE_I_6_TO_IV_SHARP_11,
    &EDGE_I_7_TO_IV,
    &EDGE_I_7_TO_IV_6,
    &EDGE_I_7_TO_IV_7,
    &EDGE_I_7_TO_IV_9,
    &EDGE_I_7_TO_IV_MAJ7,
    &EDGE_I_7_TO_IV_SHARP_11,
    &EDGE_I_9_TO_IV,
    &EDGE_I_9_TO_IV_6,
    &EDGE_I_9_TO_IV_7,
    &EDGE_I_9_TO_IV_9,
    &EDGE_I_9_TO_IV_MAJ7,
    &EDGE_I_9_TO_IV_SHARP_11,
    &EDGE_I_MAJ7_TO_IV,
    &EDGE_I_MAJ7_TO_IV_6,
    &EDGE_I_MAJ7_TO_IV_7,
    &EDGE_I_MAJ7_TO_IV_9,
    &EDGE_I_MAJ7_TO_IV_MAJ7,
    &EDGE_I_MAJ7_TO_IV_SHARP_11,
    &EDGE_I_MAJ9_TO_IV,
    &EDGE_I_MAJ9_TO_IV_6,
    &EDGE_I_MAJ9_TO_IV_7,
    &EDGE_I_MAJ9_TO_IV_9,
    &EDGE_I_MAJ9_TO_IV_MAJ7,
    &EDGE_I_MAJ9_TO_IV_SHARP_11,
    &EDGE_I_TO_V,
    &EDGE_I_TO_V_7,
    &EDGE_I_TO_V_9,
    &EDGE_I_TO_V_11,
    &EDGE_I_TO_V_13,
    &EDGE_I_TO_V_7_PLUS_FLAT_9,
    &EDGE_I_TO_V_7_PLUS_SHARP_9,
    &EDGE_I_6_TO_V,
    &EDGE_I_6_TO_V_7,
    &EDGE_I_6_TO_V_9,
    &EDGE_I_6_TO_V_11,
    &EDGE_I_6_TO_V_13,
    &EDGE_I_6_TO_V_7_PLUS_FLAT_9,
    &EDGE_I_6_TO_V_7_PLUS_SHARP_9,
    &EDGE_I_7_TO_V,
    &EDGE_I_7_TO_V_7,
    &EDGE_I_7_TO_V_9,
    &EDGE_I_7_TO_V_11,
    &EDGE_I_7_TO_V_13,
    &EDGE_I_7_TO_V_7_PLUS_FLAT_9,
    &EDGE_I_7_TO_V_7_PLUS_SHARP_9,
    &EDGE_I_9_TO_V,
    &EDGE_I_9_TO_V_7,
    &EDGE_I_9_TO_V_9,
    &EDGE_I_9_TO_V_11,
    &EDGE_I_9_TO_V_13,
    &EDGE_I_9_TO_V_7_PLUS_FLAT_9,
    &EDGE_I_9_TO_V_7_PLUS_SHARP_9,
    &EDGE_I_MAJ7_TO_V,
    &EDGE_I_MAJ7_TO_V_7,
    &EDGE_I_MAJ7_TO_V_9,
    &EDGE_I_MAJ7_TO_V_11,
    &EDGE_I_MAJ7_TO_V_13,
    &EDGE_I_MAJ7_TO_V_7_PLUS_FLAT_9,
    &EDGE_I_MAJ7_TO_V_7_PLUS_SHARP_9,
    &EDGE_I_MAJ9_TO_V,
    &EDGE_I_MAJ9_TO_V_7,
    &EDGE_I_MAJ9_TO_V_9,
    &EDGE_I_MAJ9_TO_V_11,
    &EDGE_I_MAJ9_TO_V_13,
    &EDGE_I_MAJ9_TO_V_7_PLUS_FLAT_9,
    &EDGE_I_MAJ9_TO_V_7_PLUS_SHARP_9,
    &EDGE_I_TO_MINOR_VI,
    &EDGE_I_TO_MINOR_VI_7,
    &EDGE_I_TO_MINOR_VI_9,
    &EDGE_I_TO_MINOR_VI_M7,
    &EDGE_I_6_TO_MINOR_VI,
    &EDGE_I_6_TO_MINOR_VI_7,
    &EDGE_I_6_TO_MINOR_VI_9,
    &EDGE_I_6_TO_MINOR_VI_M7,
    &EDGE_I_7_TO_MINOR_VI,
    &EDGE_I_7_TO_MINOR_VI_7,
    &EDGE_I_7_TO_MINOR_VI_9,
    &EDGE_I_7_TO_MINOR_VI_M7,
    &EDGE_I_9_TO_MINOR_VI,
    &EDGE_I_9_TO_MINOR_VI_7,
    &EDGE_I_9_TO_MINOR_VI_9,
    &EDGE_I_9_TO_MINOR_VI_M7,
    &EDGE_I_MAJ7_TO_MINOR_VI,
    &EDGE_I_MAJ7_TO_MINOR_VI_7,
    &EDGE_I_MAJ7_TO_MINOR_VI_9,
    &EDGE_I_MAJ7_TO_MINOR_VI_M7,
    &EDGE_I_MAJ9_TO_MINOR_VI,
    &EDGE_I_MAJ9_TO_MINOR_VI_7,
    &EDGE_I_MAJ9_TO_MINOR_VI_9,
    &EDGE_I_MAJ9_TO_MINOR_VI_M7,
    &EDGE_MINOR_II_TO_V,
    &EDGE_MINOR_II_TO_V_7,
    &EDGE_MINOR_II_TO_V_9,
    &EDGE_MINOR_II_TO_V_11,
    &EDGE_MINOR_II_TO_V_13,
    &EDGE_MINOR_II_TO_V_7_PLUS_FLAT_9,
    &EDGE_MINOR_II_TO_V_7_PLUS_SHARP_9,
    &EDGE_MINOR_II_7_TO_V,
    &EDGE_MINOR_II_7_TO_V_7,
    &EDGE_MINOR_II_7_TO_V_9,
    &EDGE_MINOR_II_7_TO_V_11,
    &EDGE_MINOR_II_7_TO_V_13,
    &EDGE_MINOR_II_7_TO_V_7_PLUS_FLAT_9,
    &EDGE_MINOR_II_7_TO_V_7_PLUS_SHARP_9,
    &EDGE_MINOR_II_9_TO_V,
    &EDGE_MINOR_II_9_TO_V_7,
    &EDGE_MINOR_II_9_TO_V_9,
    &EDGE_MINOR_II_9_TO_V_11,
    &EDGE_MINOR_II_9_TO_V_13,
    &EDGE_MINOR_II_9_TO_V_7_PLUS_FLAT_9,
    &EDGE_MINOR_II_9_TO_V_7_PLUS_SHARP_9,
    &EDGE_MINOR_II_11_TO_V,
    &EDGE_MINOR_II_11_TO_V_7,
    &EDGE_MINOR_II_11_TO_V_9,
    &EDGE_MINOR_II_11_TO_V_11,
    &EDGE_MINOR_II_11_TO_V_13,
    &EDGE_MINOR_II_11_TO_V_7_PLUS_FLAT_9,
    &EDGE_MINOR_II_11_TO_V_7_PLUS_SHARP_9,
    &EDGE_MINOR_II_7_PLUS_FLAT_9_TO_V,
    &EDGE_MINOR_II_7_PLUS_FLAT_9_TO_V_7,
    &EDGE_MINOR_II_7_PLUS_FLAT_9_TO_V_9,
    &EDGE_MINOR_II_7_PLUS_FLAT_9_TO_V_11,
    &EDGE_MINOR_II_7_PLUS_FLAT_9_TO_V_13,
    &EDGE_MINOR_II_7_PLUS_FLAT_9_TO_V_7_PLUS_FLAT_9,
    &EDGE_MINOR_II_7_PLUS_FLAT_9_TO_V_7_PLUS_SHARP_9,
    &EDGE_MINOR_II_TO_MINOR_VII,
    &EDGE_MINOR_II_TO_MINOR_VII_FLAT_5,
    &EDGE_MINOR_II_TO_MINOR_VII_M7_FLAT_5,
    &EDGE_MINOR_II_7_TO_MINOR_VII,
    &EDGE_MINOR_II_7_TO_MINOR_VII_FLAT_5,
    &EDGE_MINOR_II_7_TO_MINOR_VII_M7_FLAT_5,
    &EDGE_MINOR_II_9_TO_MINOR_VII,
    &EDGE_MINOR_II_9_TO_MINOR_VII_FLAT_5,
    &EDGE_MINOR_II_9_TO_MINOR_VII_M7_FLAT_5,
    &EDGE_MINOR_II_11_TO_MINOR_VII,
    &EDGE_MINOR_II_11_TO_MINOR_VII_FLAT_5,
    &EDGE_MINOR_II_11_TO_MINOR_VII_M7_FLAT_5,
    &EDGE_MINOR_II_7_PLUS_FLAT_9_TO_MINOR_VII,
    &EDGE_MINOR_II_7_PLUS_FLAT_9_TO_MINOR_VII_FLAT_5,
    &EDGE_MINOR_II_7_PLUS_FLAT_9_TO_MINOR_VII_M7_FLAT_5,
    &EDGE_MINOR_III_TO_MINOR_VI,
    &EDGE_MINOR_III_TO_MINOR_VI_7,
    &EDGE_MINOR_III_TO_MINOR_VI_9,
    &EDGE_MINOR_III_TO_MINOR_VI_M7,
    &EDGE_MINOR_III_7_TO_MINOR_VI,
    &EDGE_MINOR_III_7_TO_MINOR_VI_7,
    &EDGE_MINOR_III_7_TO_MINOR_VI_9,
    &EDGE_MINOR_III_7_TO_MINOR_VI_M7,
    &EDGE_MINOR_III_M7_TO_MINOR_VI,
    &EDGE_MINOR_III_M7_TO_MINOR_VI_7,
    &EDGE_MINOR_III_M7_TO_MINOR_VI_9,
    &EDGE_MINOR_III_M7_TO_MINOR_VI_M7,
    &EDGE_MINOR_III_TO_IV,
    &EDGE_MINOR_III_TO_IV_6,
    &EDGE_MINOR_III_TO_IV_7,
    &EDGE_MINOR_III_TO_IV_9,
    &EDGE_MINOR_III_TO_IV_MAJ7,
    &EDGE_MINOR_III_TO_IV_SHARP_11,
    &EDGE_MINOR_III_7_TO_IV,
    &EDGE_MINOR_III_7_TO_IV_6,
    &EDGE_MINOR_III_7_TO_IV_7,
    &EDGE_MINOR_III_7_TO_IV_9,
    &EDGE_MINOR_III_7_TO_IV_MAJ7,
    &EDGE_MINOR_III_7_TO_IV_SHARP_11,
    &EDGE_MINOR_III_M7_TO_IV,
    &EDGE_MINOR_III_M7_TO_IV_6,
    &EDGE_MINOR_III_M7_TO_IV_7,
    &EDGE_MINOR_III_M7_TO_IV_9,
    &EDGE_MINOR_III_M7_TO_IV_MAJ7,
    &EDGE_MINOR_III_M7_TO_IV_SHARP_11,
    &EDGE_IV_TO_I,
    &EDGE_IV_TO_I_6,
    &EDGE_IV_TO_I_7,
    &EDGE_IV_TO_I_9,
    &EDGE_IV_TO_I_MAJ7,
    &EDGE_IV_TO_I_MAJ9,
    &EDGE_IV_6_TO_I,
    &EDGE_IV_6_TO_I_6,
    &EDGE_IV_6_TO_I_7,
    &EDGE_IV_6_TO_I_9,
    &EDGE_IV_6_TO_I_MAJ7,
    &EDGE_IV_6_TO_I_MAJ9,
    &EDGE_IV_7_TO_I,
    &EDGE_IV_7_TO_I_6,
    &EDGE_IV_7_TO_I_7,
    &EDGE_IV_7_TO_I_9,
    &EDGE_IV_7_TO_I_MAJ7,
    &EDGE_IV_7_TO_I_MAJ9,
    &EDGE_IV_9_TO_I,
    &EDGE_IV_9_TO_I_6,
    &EDGE_IV_9_TO_I_7,
    &EDGE_IV_9_TO_I_9,
    &EDGE_IV_9_TO_I_MAJ7,
    &EDGE_IV_9_TO_I_MAJ9,
    &EDGE_IV_MAJ7_TO_I,
    &EDGE_IV_MAJ7_TO_I_6,
    &EDGE_IV_MAJ7_TO_I_7,
    &EDGE_IV_MAJ7_TO_I_9,
    &EDGE_IV_MAJ7_TO_I_MAJ7,
    &EDGE_IV_MAJ7_TO_I_MAJ9,
    &EDGE_IV_SHARP_11_TO_I,
    &EDGE_IV_SHARP_11_TO_I_6,
    &EDGE_IV_SHARP_11_TO_I_7,
    &EDGE_IV_SHARP_11_TO_I_9,
    &EDGE_IV_SHARP_11_TO_I_MAJ7,
    &EDGE_IV_SHARP_11_TO_I_MAJ9,
    &EDGE_IV_TO_MINOR_II,
    &EDGE_IV_TO_MINOR_II_7,
    &EDGE_IV_TO_MINOR_II_9,
    &EDGE_IV_TO_MINOR_II_11,
    &EDGE_IV_TO_MINOR_II_7_PLUS_FLAT_9,
    &EDGE_IV_6_TO_MINOR_II,
    &EDGE_IV_6_TO_MINOR_II_7,
    &EDGE_IV_6_TO_MINOR_II_9,
    &EDGE_IV_6_TO_MINOR_II_11,
    &EDGE_IV_6_TO_MINOR_II_7_PLUS_FLAT_9,
    &EDGE_IV_7_TO_MINOR_II,
    &EDGE_IV_7_TO_MINOR_II_7,
    &EDGE_IV_7_TO_MINOR_II_9,
    &EDGE_IV_7_TO_MINOR_II_11,
    &EDGE_IV_7_TO_MINOR_II_7_PLUS_FLAT_9,
    &EDGE_IV_9_TO_MINOR_II,
    &EDGE_IV_9_TO_MINOR_II_7,
    &EDGE_IV_9_TO_MINOR_II_9,
    &EDGE_IV_9_TO_MINOR_II_11,
    &EDGE_IV_9_TO_MINOR_II_7_PLUS_FLAT_9,
    &EDGE_IV_MAJ7_TO_MINOR_II,
    &EDGE_IV_MAJ7_TO_MINOR_II_7,
    &EDGE_IV_MAJ7_TO_MINOR_II_9,
    &EDGE_IV_MAJ7_TO_MINOR_II_11,
    &EDGE_IV_MAJ7_TO_MINOR_II_7_PLUS_FLAT_9,
    &EDGE_IV_SHARP_11_TO_MINOR_II,
    &EDGE_IV_SHARP_11_TO_MINOR_II_7,
    &EDGE_IV_SHARP_11_TO_MINOR_II_9,
    &EDGE_IV_SHARP_11_TO_MINOR_II_11,
    &EDGE_IV_SHARP_11_TO_MINOR_II_7_PLUS_FLAT_9,
    &EDGE_IV_TO_V,
    &EDGE_IV_TO_V_7,
    &EDGE_IV_TO_V_9,
    &EDGE_IV_TO_V_11,
    &EDGE_IV_TO_V_13,
    &EDGE_IV_TO_V_7_PLUS_FLAT_9,
    &EDGE_IV_TO_V_7_PLUS_SHARP_9,
    &EDGE_IV_6_TO_V,
    &EDGE_IV_6_TO_V_7,
    &EDGE_IV_6_TO_V_9,
    &EDGE_IV_6_TO_V_11,
    &EDGE_IV_6_TO_V_13,
    &EDGE_IV_6_TO_V_7_PLUS_FLAT_9,
    &EDGE_IV_6_TO_V_7_PLUS_SHARP_9,
    &EDGE_IV_7_TO_V,
    &EDGE_IV_7_TO_V_7,
    &EDGE_IV_7_TO_V_9,
    &EDGE_IV_7_TO_V_11,
    &EDGE_IV_7_TO_V_13,
    &EDGE_IV_7_TO_V_7_PLUS_FLAT_9,
    &EDGE_IV_7_TO_V_7_PLUS_SHARP_9,
    &EDGE_IV_9_TO_V,
    &EDGE_IV_9_TO_V_7,
    &EDGE_IV_9_TO_V_9,
    &EDGE_IV_9_TO_V_11,
    &EDGE_IV_9_TO_V_13,
    &EDGE_IV_9_TO_V_7_PLUS_FLAT_9,
    &EDGE_IV_9_TO_V_7_PLUS_SHARP_9,
    &EDGE_IV_MAJ7_TO_V,
    &EDGE_IV_MAJ7_TO_V_7,
    &EDGE_IV_MAJ7_TO_V_9,
    &EDGE_IV_MAJ7_TO_V_11,
    &EDGE_IV_MAJ7_TO_V_13,
    &EDGE_IV_MAJ7_TO_V_7_PLUS_FLAT_9,
    &EDGE_IV_MAJ7_TO_V_7_PLUS_SHARP_9,
    &EDGE_IV_SHARP_11_TO_V,
    &EDGE_IV_SHARP_11_TO_V_7,
    &EDGE_IV_SHARP_11_TO_V_9,
    &EDGE_IV_SHARP_11_TO_V_11,
    &EDGE_IV_SHARP_11_TO_V_13,
    &EDGE_IV_SHARP_11_TO_V_7_PLUS_FLAT_9,
    &EDGE_IV_SHARP_11_TO_V_7_PLUS_SHARP_9,
    &EDGE_V_TO_I,
    &EDGE_V_TO_I_6,
    &EDGE_V_TO_I_7,
    &EDGE_V_TO_I_9,
    &EDGE_V_TO_I_MAJ7,
    &EDGE_V_TO_I_MAJ9,
    &EDGE_V_7_TO_I,
    &EDGE_V_7_TO_I_6,
    &EDGE_V_7_TO_I_7,
    &EDGE_V_7_TO_I_9,
    &EDGE_V_7_TO_I_MAJ7,
    &EDGE_V_7_TO_I_MAJ9,
    &EDGE_V_9_TO_I,
    &EDGE_V_9_TO_I_6,
    &EDGE_V_9_TO_I_7,
    &EDGE_V_9_TO_I_9,
    &EDGE_V_9_TO_I_MAJ7,
    &EDGE_V_9_TO_I_MAJ9,
    &EDGE_V_11_TO_I,
    &EDGE_V_11_TO_I_6,
    &EDGE_V_11_TO_I_7,
    &EDGE_V_11_TO_I_9,
    &EDGE_V_11_TO_I_MAJ7,
    &EDGE_V_11_TO_I_MAJ9,
    &EDGE_V_13_TO_I,
    &EDGE_V_13_TO_I_6,
    &EDGE_V_13_TO_I_7,
    &EDGE_V_13_TO_I_9,
    &EDGE_V_13_TO_I_MAJ7,
    &EDGE_V_13_TO_I_MAJ9,
    &EDGE_V_7_PLUS_FLAT_9_TO_I,
    &EDGE_V_7_PLUS_FLAT_9_TO_I_6,
    &EDGE_V_7_PLUS_FLAT_9_TO_I_7,
    &EDGE_V_7_PLUS_FLAT_9_TO_I_9,
    &EDGE_V_7_PLUS_FLAT_9_TO_I_MAJ7,
    &EDGE_V_7_PLUS_FLAT_9_TO_I_MAJ9,
    &EDGE_V_7_PLUS_SHARP_9_TO_I,
    &EDGE_V_7_PLUS_SHARP_9_TO_I_6,
    &EDGE_V_7_PLUS_SHARP_9_TO_I_7,
    &EDGE_V_7_PLUS_SHARP_9_TO_I_9,
    &EDGE_V_7_PLUS_SHARP_9_TO_I_MAJ7,
    &EDGE_V_7_PLUS_SHARP_9_TO_I_MAJ9,
    &EDGE_V_TO_MINOR_VI,
    &EDGE_V_TO_MINOR_VI_7,
    &EDGE_V_TO_MINOR_VI_9,
    &EDGE_V_TO_MINOR_VI_M7,
    &EDGE_V_7_TO_MINOR_VI,
    &EDGE_V_7_TO_MINOR_VI_7,
    &EDGE_V_7_TO_MINOR_VI_9,
    &EDGE_V_7_TO_MINOR_VI_M7,
    &EDGE_V_9_TO_MINOR_VI,
    &EDGE_V_9_TO_MINOR_VI_7,
    &EDGE_V_9_TO_MINOR_VI_9,
    &EDGE_V_9_TO_MINOR_VI_M7,
    &EDGE_V_11_TO_MINOR_VI,
    &EDGE_V_11_TO_MINOR_VI_7,
    &EDGE_V_11_TO_MINOR_VI_9,
    &EDGE_V_11_TO_MINOR_VI_M7,
    &EDGE_V_13_TO_MINOR_VI,
    &EDGE_V_13_TO_MINOR_VI_7,
    &EDGE_V_13_TO_MINOR_VI_9,
    &EDGE_V_13_TO_MINOR_VI_M7,
    &EDGE_V_7_PLUS_FLAT_9_TO_MINOR_VI,
    &EDGE_V_7_PLUS_FLAT_9_TO_MINOR_VI_7,
    &EDGE_V_7_PLUS_FLAT_9_TO_MINOR_VI_9,
    &EDGE_V_7_PLUS_FLAT_9_TO_MINOR_VI_M7,
    &EDGE_V_7_PLUS_SHARP_9_TO_MINOR_VI,
    &EDGE_V_7_PLUS_SHARP_9_TO_MINOR_VI_7,
    &EDGE_V_7_PLUS_SHARP_9_TO_MINOR_VI_9,
    &EDGE_V_7_PLUS_SHARP_9_TO_MINOR_VI_M7,
    &EDGE_MINOR_VI_TO_MINOR_II,
    &EDGE_MINOR_VI_TO_MINOR_II_7,
    &EDGE_MINOR_VI_TO_MINOR_II_9,
    &EDGE_MINOR_VI_TO_MINOR_II_11,
    &EDGE_MINOR_VI_TO_MINOR_II_7_PLUS_FLAT_9,
    &EDGE_MINOR_VI_7_TO_MINOR_II,
    &EDGE_MINOR_VI_7_TO_MINOR_II_7,
    &EDGE_MINOR_VI_7_TO_MINOR_II_9,
    &EDGE_MINOR_VI_7_TO_MINOR_II_11,
    &EDGE_MINOR_VI_7_TO_MINOR_II_7_PLUS_FLAT_9,
    &EDGE_MINOR_VI_9_TO_MINOR_II,
    &EDGE_MINOR_VI_9_TO_MINOR_II_7,
    &EDGE_MINOR_VI_9_TO_MINOR_II_9,
    &EDGE_MINOR_VI_9_TO_MINOR_II_11,
    &EDGE_MINOR_VI_9_TO_MINOR_II_7_PLUS_FLAT_9,
    &EDGE_MINOR_VI_M7_TO_MINOR_II,
    &EDGE_MINOR_VI_M7_TO_MINOR_II_7,
    &EDGE_MINOR_VI_M7_TO_MINOR_II_9,
    &EDGE_MINOR_VI_M7_TO_MINOR_II_11,
    &EDGE_MINOR_VI_M7_TO_MINOR_II_7_PLUS_FLAT_9,
    &EDGE_MINOR_VI_TO_IV,
    &EDGE_MINOR_VI_TO_IV_6,
    &EDGE_MINOR_VI_TO_IV_7,
    &EDGE_MINOR_VI_TO_IV_9,
    &EDGE_MINOR_VI_TO_IV_MAJ7,
    &EDGE_MINOR_VI_TO_IV_SHARP_11,
    &EDGE_MINOR_VI_7_TO_IV,
    &EDGE_MINOR_VI_7_TO_IV_6,
    &EDGE_MINOR_VI_7_TO_IV_7,
    &EDGE_MINOR_VI_7_TO_IV_9,
    &EDGE_MINOR_VI_7_TO_IV_MAJ7,
    &EDGE_MINOR_VI_7_TO_IV_SHARP_11,
    &EDGE_MINOR_VI_9_TO_IV,
    &EDGE_MINOR_VI_9_TO_IV_6,
    &EDGE_MINOR_VI_9_TO_IV_7,
    &EDGE_MINOR_VI_9_TO_IV_9,
    &EDGE_MINOR_VI_9_TO_IV_MAJ7,
    &EDGE_MINOR_VI_9_TO_IV_SHARP_11,
    &EDGE_MINOR_VI_M7_TO_IV,
    &EDGE_MINOR_VI_M7_TO_IV_6,
    &EDGE_MINOR_VI_M7_TO_IV_7,
    &EDGE_MINOR_VI_M7_TO_IV_9,
    &EDGE_MINOR_VI_M7_TO_IV_MAJ7,
    &EDGE_MINOR_VI_M7_TO_IV_SHARP_11,
    &EDGE_MINOR_VII_TO_I,
    &EDGE_MINOR_VII_TO_I_6,
    &EDGE_MINOR_VII_TO_I_7,
    &EDGE_MINOR_VII_TO_I_9,
    &EDGE_MINOR_VII_TO_I_MAJ7,
    &EDGE_MINOR_VII_TO_I_MAJ9,
    &EDGE_MINOR_VII_FLAT_5_TO_I,
    &EDGE_MINOR_VII_FLAT_5_TO_I_6,
    &EDGE_MINOR_VII_FLAT_5_TO_I_7,
    &EDGE_MINOR_VII_FLAT_5_TO_I_9,
    &EDGE_MINOR_VII_FLAT_5_TO_I_MAJ7,
    &EDGE_MINOR_VII_FLAT_5_TO_I_MAJ9,
    &EDGE_MINOR_VII_M7_FLAT_5_TO_I,
    &EDGE_MINOR_VII_M7_FLAT_5_TO_I_6,
    &EDGE_MINOR_VII_M7_FLAT_5_TO_I_7,
    &EDGE_MINOR_VII_M7_FLAT_5_TO_I_9,
    &EDGE_MINOR_VII_M7_FLAT_5_TO_I_MAJ7,
    &EDGE_MINOR_VII_M7_FLAT_5_TO_I_MAJ9,
    &EDGE_MINOR_FLAT_III_7_TO_IV,
    &EDGE_MINOR_FLAT_III_7_TO_IV_6,
    &EDGE_MINOR_FLAT_III_7_TO_IV_7,
    &EDGE_MINOR_FLAT_III_7_TO_IV_9,
    &EDGE_MINOR_FLAT_III_7_TO_IV_MAJ7,
    &EDGE_MINOR_FLAT_III_7_TO_IV_SHARP_11,
    &EDGE_MINOR_FLAT_III_9_TO_IV,
    &EDGE_MINOR_FLAT_III_9_TO_IV_6,
    &EDGE_MINOR_FLAT_III_9_TO_IV_7,
    &EDGE_MINOR_FLAT_III_9_TO_IV_9,
    &EDGE_MINOR_FLAT_III_9_TO_IV_MAJ7,
    &EDGE_MINOR_FLAT_III_9_TO_IV_SHARP_11,
    &EDGE_MINOR_FLAT_VI_7_TO_MINOR_FLAT_VII_7,
    &EDGE_MINOR_FLAT_VI_7_TO_MINOR_FLAT_VII_9,
    &EDGE_MINOR_FLAT_VI_9_TO_MINOR_FLAT_VII_7,
    &EDGE_MINOR_FLAT_VI_9_TO_MINOR_FLAT_VII_9,
    &EDGE_MINOR_FLAT_VII_7_TO_I,
    &EDGE_MINOR_FLAT_VII_7_TO_I_6,
    &EDGE_MINOR_FLAT_VII_7_TO_I_7,
    &EDGE_MINOR_FLAT_VII_7_TO_I_9,
    &EDGE_MINOR_FLAT_VII_7_TO_I_MAJ7,
    &EDGE_MINOR_FLAT_VII_7_TO_I_MAJ9,
    &EDGE_MINOR_FLAT_VII_9_TO_I,
    &EDGE_MINOR_FLAT_VII_9_TO_I_6,
    &EDGE_MINOR_FLAT_VII_9_TO_I_7,
    &EDGE_MINOR_FLAT_VII_9_TO_I_9,
    &EDGE_MINOR_FLAT_VII_9_TO_I_MAJ7,
    &EDGE_MINOR_FLAT_VII_9_TO_I_MAJ9,
];

/// NodeType mapping for all progression chords in major keys
/// 
/// Maps each chord to its harmonic role (Primary = stable, Secondary = transitional).
pub fn get_node_types() -> HashMap<&'static RomanChord, NodeType> {
    let mut map = HashMap::new();
    map.insert(&I, NodeType::Primary);
    map.insert(&I_6, NodeType::Primary);
    map.insert(&I_7, NodeType::Primary);
    map.insert(&I_9, NodeType::Primary);
    map.insert(&I_MAJ7, NodeType::Primary);
    map.insert(&I_MAJ9, NodeType::Primary);
    map.insert(&MINOR_II, NodeType::Primary);
    map.insert(&MINOR_II_7, NodeType::Primary);
    map.insert(&MINOR_II_9, NodeType::Primary);
    map.insert(&MINOR_II_11, NodeType::Primary);
    map.insert(&MINOR_II_7_PLUS_FLAT_9, NodeType::Primary);
    map.insert(&MINOR_III, NodeType::Primary);
    map.insert(&MINOR_III_7, NodeType::Primary);
    map.insert(&MINOR_III_M7, NodeType::Primary);
    map.insert(&IV, NodeType::Primary);
    map.insert(&IV_6, NodeType::Primary);
    map.insert(&IV_7, NodeType::Primary);
    map.insert(&IV_9, NodeType::Primary);
    map.insert(&IV_MAJ7, NodeType::Primary);
    map.insert(&IV_SHARP_11, NodeType::Primary);
    map.insert(&V, NodeType::Primary);
    map.insert(&V_7, NodeType::Primary);
    map.insert(&V_9, NodeType::Primary);
    map.insert(&V_11, NodeType::Primary);
    map.insert(&V_13, NodeType::Primary);
    map.insert(&V_7_PLUS_FLAT_9, NodeType::Primary);
    map.insert(&V_7_PLUS_SHARP_9, NodeType::Primary);
    map.insert(&MINOR_VI, NodeType::Primary);
    map.insert(&MINOR_VI_7, NodeType::Primary);
    map.insert(&MINOR_VI_9, NodeType::Primary);
    map.insert(&MINOR_VI_M7, NodeType::Primary);
    map.insert(&MINOR_VII, NodeType::Primary);
    map.insert(&MINOR_VII_FLAT_5, NodeType::Primary);
    map.insert(&MINOR_VII_M7_FLAT_5, NodeType::Primary);
    map.insert(&MINOR_FLAT_III_7, NodeType::Secondary);
    map.insert(&MINOR_FLAT_III_9, NodeType::Secondary);
    map.insert(&MINOR_FLAT_VI_7, NodeType::Secondary);
    map.insert(&MINOR_FLAT_VI_9, NodeType::Secondary);
    map.insert(&MINOR_FLAT_VII_7, NodeType::Secondary);
    map.insert(&MINOR_FLAT_VII_9, NodeType::Secondary);
    map
}

/// Look up a progression chord by its display name for major keys
/// 
/// Returns the corresponding `RomanChord` for chord symbols like "I", "V7", "ii9", etc.
/// Supports 40 different chord variants.
pub fn get_node(name: &str) -> Option<&'static RomanChord> {
    match name {
        "I" => Some(&I),
        "I6" => Some(&I_6),
        "I7" => Some(&I_7),
        "I9" => Some(&I_9),
        "Imaj7" => Some(&I_MAJ7),
        "Imaj9" => Some(&I_MAJ9),
        "ii" => Some(&MINOR_II),
        "ii7" => Some(&MINOR_II_7),
        "ii9" => Some(&MINOR_II_9),
        "ii11" => Some(&MINOR_II_11),
        "ii7+b9" => Some(&MINOR_II_7_PLUS_FLAT_9),
        "iii" => Some(&MINOR_III),
        "iii7" => Some(&MINOR_III_7),
        "iiim7" => Some(&MINOR_III_M7),
        "IV" => Some(&IV),
        "IV6" => Some(&IV_6),
        "IV7" => Some(&IV_7),
        "IV9" => Some(&IV_9),
        "IVmaj7" => Some(&IV_MAJ7),
        "IV#11" => Some(&IV_SHARP_11),
        "V" => Some(&V),
        "V7" => Some(&V_7),
        "V9" => Some(&V_9),
        "V11" => Some(&V_11),
        "V13" => Some(&V_13),
        "V7+b9" => Some(&V_7_PLUS_FLAT_9),
        "V7+#9" => Some(&V_7_PLUS_SHARP_9),
        "vi" => Some(&MINOR_VI),
        "vi7" => Some(&MINOR_VI_7),
        "vi9" => Some(&MINOR_VI_9),
        "vim7" => Some(&MINOR_VI_M7),
        "vii" => Some(&MINOR_VII),
        "viib5" => Some(&MINOR_VII_FLAT_5),
        "viim7b5" => Some(&MINOR_VII_M7_FLAT_5),
        "bIII7" => Some(&MINOR_FLAT_III_7),
        "bIII9" => Some(&MINOR_FLAT_III_9),
        "bVI7" => Some(&MINOR_FLAT_VI_7),
        "bVI9" => Some(&MINOR_FLAT_VI_9),
        "bVII7" => Some(&MINOR_FLAT_VII_7),
        "bVII9" => Some(&MINOR_FLAT_VII_9),
        _ => None,
    }
}
