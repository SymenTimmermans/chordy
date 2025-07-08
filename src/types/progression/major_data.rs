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

const I_ADD9_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MAJOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 3);

/// Iadd9 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, major third, perfect fifth
pub static I_ADD9: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::I, Accidental::Natural),
    intervals: I_ADD9_INTERVALS_SET,
};

const I_SUS2_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MAJOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 3);

/// Isus2 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, major third, perfect fifth
pub static I_SUS2: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::I, Accidental::Natural),
    intervals: I_SUS2_INTERVALS_SET,
};

const I_SUS4_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MAJOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 3);

/// Isus4 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, major third, perfect fifth
pub static I_SUS4: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::I, Accidental::Natural),
    intervals: I_SUS4_INTERVALS_SET,
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

/// iim7 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, minor third, perfect fifth, minor seventh
pub static MINOR_II_M7: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::II, Accidental::Natural),
    intervals: MINOR_II_M7_INTERVALS_SET,
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

/// iim9 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, minor third, perfect fifth, minor seventh, major ninth
pub static MINOR_II_M9: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::II, Accidental::Natural),
    intervals: MINOR_II_M9_INTERVALS_SET,
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

const MINOR_IV_M6_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MINOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 3);

/// ivm6 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, minor third, perfect fifth
pub static MINOR_IV_M6: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::IV, Accidental::Natural),
    intervals: MINOR_IV_M6_INTERVALS_SET,
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

const V_SUS4_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MAJOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 3);

/// Vsus4 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, major third, perfect fifth
pub static V_SUS4: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::V, Accidental::Natural),
    intervals: V_SUS4_INTERVALS_SET,
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

/// vim9 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, minor third, perfect fifth, minor seventh, major ninth
pub static MINOR_VI_M9: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::VI, Accidental::Natural),
    intervals: MINOR_VI_M9_INTERVALS_SET,
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

const V_SLASH_V_7_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
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

/// V/V7 chord - creates tension, seeks resolution (secondary node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh
pub static V_SLASH_V_7: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::II, Accidental::Natural),
    intervals: V_SLASH_V_7_INTERVALS_SET,
};

const MINOR_V_SLASH_II_7_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
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

/// V/ii7 chord - creates tension, seeks resolution (secondary node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh
pub static MINOR_V_SLASH_II_7: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::VI, Accidental::Natural),
    intervals: MINOR_V_SLASH_II_7_INTERVALS_SET,
};

const MINOR_V_SLASH_VI_7_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
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

/// V/vi7 chord - creates tension, seeks resolution (secondary node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh
pub static MINOR_V_SLASH_VI_7: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::III, Accidental::Natural),
    intervals: MINOR_V_SLASH_VI_7_INTERVALS_SET,
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

/// Progression edge: ii → iii
pub static EDGE_MINOR_II_M7_TO_MINOR_III_M7: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_M7,
    to: MINOR_III_M7,
};

/// Progression edge: ii → iii
pub static EDGE_MINOR_II_M9_TO_MINOR_III_M7: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_M9,
    to: MINOR_III_M7,
};

/// Progression edge: V → iii
pub static EDGE_V_7_TO_MINOR_III_M7: ProgressionEdge = ProgressionEdge {
    from: V_7,
    to: MINOR_III_M7,
};

/// Progression edge: V → iii
pub static EDGE_V_9_TO_MINOR_III_M7: ProgressionEdge = ProgressionEdge {
    from: V_9,
    to: MINOR_III_M7,
};

/// Progression edge: V → iii
pub static EDGE_V_11_TO_MINOR_III_M7: ProgressionEdge = ProgressionEdge {
    from: V_11,
    to: MINOR_III_M7,
};

/// Progression edge: V → iii
pub static EDGE_V_13_TO_MINOR_III_M7: ProgressionEdge = ProgressionEdge {
    from: V_13,
    to: MINOR_III_M7,
};

/// Progression edge: V → iii
pub static EDGE_V_SUS4_TO_MINOR_III_M7: ProgressionEdge = ProgressionEdge {
    from: V_SUS4,
    to: MINOR_III_M7,
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
pub static EDGE_V_SUS4_TO_MINOR_VI_M7: ProgressionEdge = ProgressionEdge {
    from: V_SUS4,
    to: MINOR_VI_M7,
};

/// Progression edge: V → vi
pub static EDGE_V_SUS4_TO_MINOR_VI_M9: ProgressionEdge = ProgressionEdge {
    from: V_SUS4,
    to: MINOR_VI_M9,
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
pub static EDGE_MINOR_VI_M9_TO_IV_6: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI_M9,
    to: IV_6,
};

/// Progression edge: vi → IV
pub static EDGE_MINOR_VI_M9_TO_IV_MAJ7: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI_M9,
    to: IV_MAJ7,
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
pub static EDGE_MINOR_VI_M9_TO_MINOR_II_M7: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI_M9,
    to: MINOR_II_M7,
};

/// Progression edge: vi → ii
pub static EDGE_MINOR_VI_M9_TO_MINOR_II_M9: ProgressionEdge = ProgressionEdge {
    from: MINOR_VI_M9,
    to: MINOR_II_M9,
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
pub static EDGE_IV_MAJ7_TO_MINOR_II_M7: ProgressionEdge = ProgressionEdge {
    from: IV_MAJ7,
    to: MINOR_II_M7,
};

/// Progression edge: IV → ii
pub static EDGE_IV_MAJ7_TO_MINOR_II_M9: ProgressionEdge = ProgressionEdge {
    from: IV_MAJ7,
    to: MINOR_II_M9,
};

/// Complete registry of all progression chords for major keys
/// 
/// Contains 27 chord variants across all harmonic functions.
/// Used internally for graph traversal and chord lookup operations.
pub static ALL_NODES: &[&RomanChord] = &[
    &I_6,
    &I_MAJ7,
    &I_MAJ9,
    &I_ADD9,
    &I_SUS2,
    &I_SUS4,
    &MINOR_II_M7,
    &MINOR_II_M9,
    &MINOR_III_M7,
    &IV_6,
    &IV_MAJ7,
    &MINOR_IV_M6,
    &V_7,
    &V_9,
    &V_11,
    &V_13,
    &V_SUS4,
    &MINOR_VI_M7,
    &MINOR_VI_M9,
    &MINOR_FLAT_III_7,
    &MINOR_FLAT_III_9,
    &MINOR_FLAT_VI_7,
    &MINOR_FLAT_VI_9,
    &MINOR_FLAT_VII_9,
    &V_SLASH_V_7,
    &MINOR_V_SLASH_II_7,
    &MINOR_V_SLASH_VI_7,
];

/// Complete registry of all progression edges for major keys
/// 
/// Contains 85 harmonic connections between chord variants.
/// Each edge represents a musically valid progression with proper voice leading.
pub static ALL_EDGES: &[&ProgressionEdge] = &[
    &EDGE_MINOR_II_M7_TO_V_7,
    &EDGE_MINOR_II_M7_TO_V_9,
    &EDGE_MINOR_II_M7_TO_V_11,
    &EDGE_MINOR_II_M7_TO_V_13,
    &EDGE_MINOR_II_M7_TO_V_SUS4,
    &EDGE_MINOR_II_M9_TO_V_7,
    &EDGE_MINOR_II_M9_TO_V_9,
    &EDGE_MINOR_II_M9_TO_V_11,
    &EDGE_MINOR_II_M9_TO_V_13,
    &EDGE_MINOR_II_M9_TO_V_SUS4,
    &EDGE_V_7_TO_I_6,
    &EDGE_V_7_TO_I_MAJ7,
    &EDGE_V_7_TO_I_MAJ9,
    &EDGE_V_7_TO_I_ADD9,
    &EDGE_V_7_TO_I_SUS2,
    &EDGE_V_7_TO_I_SUS4,
    &EDGE_V_9_TO_I_6,
    &EDGE_V_9_TO_I_MAJ7,
    &EDGE_V_9_TO_I_MAJ9,
    &EDGE_V_9_TO_I_ADD9,
    &EDGE_V_9_TO_I_SUS2,
    &EDGE_V_9_TO_I_SUS4,
    &EDGE_V_11_TO_I_6,
    &EDGE_V_11_TO_I_MAJ7,
    &EDGE_V_11_TO_I_MAJ9,
    &EDGE_V_11_TO_I_ADD9,
    &EDGE_V_11_TO_I_SUS2,
    &EDGE_V_11_TO_I_SUS4,
    &EDGE_V_13_TO_I_6,
    &EDGE_V_13_TO_I_MAJ7,
    &EDGE_V_13_TO_I_MAJ9,
    &EDGE_V_13_TO_I_ADD9,
    &EDGE_V_13_TO_I_SUS2,
    &EDGE_V_13_TO_I_SUS4,
    &EDGE_V_SUS4_TO_I_6,
    &EDGE_V_SUS4_TO_I_MAJ7,
    &EDGE_V_SUS4_TO_I_MAJ9,
    &EDGE_V_SUS4_TO_I_ADD9,
    &EDGE_V_SUS4_TO_I_SUS2,
    &EDGE_V_SUS4_TO_I_SUS4,
    &EDGE_IV_6_TO_I_6,
    &EDGE_IV_6_TO_I_MAJ7,
    &EDGE_IV_6_TO_I_MAJ9,
    &EDGE_IV_6_TO_I_ADD9,
    &EDGE_IV_6_TO_I_SUS2,
    &EDGE_IV_6_TO_I_SUS4,
    &EDGE_IV_MAJ7_TO_I_6,
    &EDGE_IV_MAJ7_TO_I_MAJ7,
    &EDGE_IV_MAJ7_TO_I_MAJ9,
    &EDGE_IV_MAJ7_TO_I_ADD9,
    &EDGE_IV_MAJ7_TO_I_SUS2,
    &EDGE_IV_MAJ7_TO_I_SUS4,
    &EDGE_MINOR_II_M7_TO_MINOR_III_M7,
    &EDGE_MINOR_II_M9_TO_MINOR_III_M7,
    &EDGE_V_7_TO_MINOR_III_M7,
    &EDGE_V_9_TO_MINOR_III_M7,
    &EDGE_V_11_TO_MINOR_III_M7,
    &EDGE_V_13_TO_MINOR_III_M7,
    &EDGE_V_SUS4_TO_MINOR_III_M7,
    &EDGE_V_7_TO_MINOR_VI_M7,
    &EDGE_V_7_TO_MINOR_VI_M9,
    &EDGE_V_9_TO_MINOR_VI_M7,
    &EDGE_V_9_TO_MINOR_VI_M9,
    &EDGE_V_11_TO_MINOR_VI_M7,
    &EDGE_V_11_TO_MINOR_VI_M9,
    &EDGE_V_13_TO_MINOR_VI_M7,
    &EDGE_V_13_TO_MINOR_VI_M9,
    &EDGE_V_SUS4_TO_MINOR_VI_M7,
    &EDGE_V_SUS4_TO_MINOR_VI_M9,
    &EDGE_MINOR_III_M7_TO_MINOR_VI_M7,
    &EDGE_MINOR_III_M7_TO_MINOR_VI_M9,
    &EDGE_MINOR_III_M7_TO_IV_6,
    &EDGE_MINOR_III_M7_TO_IV_MAJ7,
    &EDGE_MINOR_VI_M7_TO_IV_6,
    &EDGE_MINOR_VI_M7_TO_IV_MAJ7,
    &EDGE_MINOR_VI_M9_TO_IV_6,
    &EDGE_MINOR_VI_M9_TO_IV_MAJ7,
    &EDGE_MINOR_VI_M7_TO_MINOR_II_M7,
    &EDGE_MINOR_VI_M7_TO_MINOR_II_M9,
    &EDGE_MINOR_VI_M9_TO_MINOR_II_M7,
    &EDGE_MINOR_VI_M9_TO_MINOR_II_M9,
    &EDGE_IV_6_TO_MINOR_II_M7,
    &EDGE_IV_6_TO_MINOR_II_M9,
    &EDGE_IV_MAJ7_TO_MINOR_II_M7,
    &EDGE_IV_MAJ7_TO_MINOR_II_M9,
];

/// NodeType mapping for all progression chords in major keys
/// 
/// Maps each chord to its harmonic role (Primary = stable, Secondary = transitional).
pub fn get_node_types() -> HashMap<&'static RomanChord, NodeType> {
    let mut map = HashMap::new();
    map.insert(&I_6, NodeType::Primary);
    map.insert(&I_MAJ7, NodeType::Primary);
    map.insert(&I_MAJ9, NodeType::Primary);
    map.insert(&I_ADD9, NodeType::Primary);
    map.insert(&I_SUS2, NodeType::Primary);
    map.insert(&I_SUS4, NodeType::Primary);
    map.insert(&MINOR_II_M7, NodeType::Primary);
    map.insert(&MINOR_II_M9, NodeType::Primary);
    map.insert(&MINOR_III_M7, NodeType::Primary);
    map.insert(&IV_6, NodeType::Primary);
    map.insert(&IV_MAJ7, NodeType::Primary);
    map.insert(&MINOR_IV_M6, NodeType::Primary);
    map.insert(&V_7, NodeType::Primary);
    map.insert(&V_9, NodeType::Primary);
    map.insert(&V_11, NodeType::Primary);
    map.insert(&V_13, NodeType::Primary);
    map.insert(&V_SUS4, NodeType::Primary);
    map.insert(&MINOR_VI_M7, NodeType::Primary);
    map.insert(&MINOR_VI_M9, NodeType::Primary);
    map.insert(&MINOR_FLAT_III_7, NodeType::Secondary);
    map.insert(&MINOR_FLAT_III_9, NodeType::Secondary);
    map.insert(&MINOR_FLAT_VI_7, NodeType::Secondary);
    map.insert(&MINOR_FLAT_VI_9, NodeType::Secondary);
    map.insert(&MINOR_FLAT_VII_9, NodeType::Secondary);
    map.insert(&V_SLASH_V_7, NodeType::Secondary);
    map.insert(&MINOR_V_SLASH_II_7, NodeType::Secondary);
    map.insert(&MINOR_V_SLASH_VI_7, NodeType::Secondary);
    map
}

/// Look up a progression chord by its display name for major keys
/// 
/// Returns the corresponding `RomanChord` for chord symbols like "I", "V7", "ii9", etc.
/// Supports 27 different chord variants.
pub fn get_node(name: &str) -> Option<&'static RomanChord> {
    match name {
        "I6" => Some(&I_6),
        "Imaj7" => Some(&I_MAJ7),
        "Imaj9" => Some(&I_MAJ9),
        "Iadd9" => Some(&I_ADD9),
        "Isus2" => Some(&I_SUS2),
        "Isus4" => Some(&I_SUS4),
        "iim7" => Some(&MINOR_II_M7),
        "iim9" => Some(&MINOR_II_M9),
        "iiim7" => Some(&MINOR_III_M7),
        "IV6" => Some(&IV_6),
        "IVmaj7" => Some(&IV_MAJ7),
        "ivm6" => Some(&MINOR_IV_M6),
        "V7" => Some(&V_7),
        "V9" => Some(&V_9),
        "V11" => Some(&V_11),
        "V13" => Some(&V_13),
        "Vsus4" => Some(&V_SUS4),
        "vim7" => Some(&MINOR_VI_M7),
        "vim9" => Some(&MINOR_VI_M9),
        "bIII7" => Some(&MINOR_FLAT_III_7),
        "bIII9" => Some(&MINOR_FLAT_III_9),
        "bVI7" => Some(&MINOR_FLAT_VI_7),
        "bVI9" => Some(&MINOR_FLAT_VI_9),
        "bVII9" => Some(&MINOR_FLAT_VII_9),
        "V/V7" => Some(&V_SLASH_V_7),
        "V/ii7" => Some(&MINOR_V_SLASH_II_7),
        "V/vi7" => Some(&MINOR_V_SLASH_VI_7),
        _ => None,
    }
}
