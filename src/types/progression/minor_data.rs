//! Generated progression data for minor keys from minor.progression
//! Do not edit manually.

use crate::types::chord::BassType;
use crate::types::progression::{NodeType, ProgressionEdge};
use crate::types::{Accidental, Interval, IntervalSet, RomanChord, RomanDegree, RomanNumeral};
use std::collections::HashMap;

// Common interval patterns (reused across multiple chords)
/// Standard major triad intervals: root, major third, perfect fifth
const MAJOR_TRIAD_SET: IntervalSet = IntervalSet::const_from_array(
    [
        Interval::PERFECT_UNISON,
        Interval::MAJOR_THIRD,
        Interval::PERFECT_FIFTH,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
    ],
    3,
);

/// Standard minor triad intervals: root, minor third, perfect fifth
const MINOR_TRIAD_SET: IntervalSet = IntervalSet::const_from_array(
    [
        Interval::PERFECT_UNISON,
        Interval::MINOR_THIRD,
        Interval::PERFECT_FIFTH,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
    ],
    3,
);

/// iidim chord - creates tension, seeks resolution (p node)
/// Intervals: perfect unison, minor third, perfect fifth
pub static MINOR_IIDIM: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::II, Accidental::Natural),
    intervals: MINOR_TRIAD_SET,
    bass: None,
};

/// viidim/2 chord - creates tension, seeks resolution (p node)
/// Intervals: perfect unison, minor third, perfect fifth
pub static MINOR_VIIDIM_SLASH_2: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::VII, Accidental::Natural),
    intervals: MINOR_TRIAD_SET,
    bass: Some((
        RomanNumeral::new(RomanDegree::II, Accidental::Natural),
        BassType::Slash,
    )),
};

/// V chord - creates tension, seeks resolution (p node)
/// Intervals: perfect unison, major third, perfect fifth
pub static V: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::V, Accidental::Natural),
    intervals: MAJOR_TRIAD_SET,
    bass: None,
};

const V_7_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [
        Interval::PERFECT_UNISON,
        Interval::MAJOR_THIRD,
        Interval::PERFECT_FIFTH,
        Interval::MINOR_SEVENTH,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
    ],
    4,
);

/// V7 chord - creates tension, seeks resolution (p node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh
pub static V_7: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::V, Accidental::Natural),
    intervals: V_7_INTERVALS_SET,
    bass: None,
};

const V_FLAT_9_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [
        Interval::PERFECT_UNISON,
        Interval::MAJOR_THIRD,
        Interval::PERFECT_FIFTH,
        Interval::MINOR_NINTH,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
    ],
    4,
);

/// Vb9 chord - creates tension, seeks resolution (p node)
/// Intervals: perfect unison, major third, perfect fifth, minor ninth
pub static V_FLAT_9: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::V, Accidental::Natural),
    intervals: V_FLAT_9_INTERVALS_SET,
    bass: None,
};

const V_FLAT_13_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [
        Interval::PERFECT_UNISON,
        Interval::MAJOR_THIRD,
        Interval::PERFECT_FIFTH,
        Interval::MINOR_THIRTEENTH,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
    ],
    4,
);

/// Vb13 chord - creates tension, seeks resolution (p node)
/// Intervals: perfect unison, major third, perfect fifth, minor thirteenth
pub static V_FLAT_13: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::V, Accidental::Natural),
    intervals: V_FLAT_13_INTERVALS_SET,
    bass: None,
};

const V_SUS4_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [
        Interval::PERFECT_UNISON,
        Interval::PERFECT_FOURTH,
        Interval::PERFECT_FIFTH,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
    ],
    3,
);

/// Vsus4 chord - creates tension, seeks resolution (p node)
/// Intervals: perfect unison, perfect fourth, perfect fifth
pub static V_SUS4: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::V, Accidental::Natural),
    intervals: V_SUS4_INTERVALS_SET,
    bass: None,
};

/// bIIIaug chord - creates tension, seeks resolution (p node)
/// Intervals: perfect unison, minor third, perfect fifth
pub static MINOR_FLAT_IIIAUG: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::III, Accidental::Natural),
    intervals: MINOR_TRIAD_SET,
    bass: None,
};

/// i/b3 chord - creates tension, seeks resolution (p node)
/// Intervals: perfect unison, minor third, perfect fifth
pub static MINOR_I_SLASH_FLAT_3: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::I, Accidental::Natural),
    intervals: MINOR_TRIAD_SET,
    bass: Some((
        RomanNumeral::new(RomanDegree::III, Accidental::Flat),
        BassType::Slash,
    )),
};

/// bVI chord - creates tension, seeks resolution (p node)
/// Intervals: perfect unison, minor third, perfect fifth
pub static MINOR_FLAT_VI: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::VI, Accidental::Natural),
    intervals: MINOR_TRIAD_SET,
    bass: None,
};

const MINOR_FLAT_VI_6_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [
        Interval::PERFECT_UNISON,
        Interval::MINOR_THIRD,
        Interval::PERFECT_FIFTH,
        Interval::MAJOR_SIXTH,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
    ],
    4,
);

/// bVI6 chord - creates tension, seeks resolution (p node)
/// Intervals: perfect unison, minor third, perfect fifth, major sixth
pub static MINOR_FLAT_VI_6: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::VI, Accidental::Natural),
    intervals: MINOR_FLAT_VI_6_INTERVALS_SET,
    bass: None,
};

const MINOR_FLAT_VI_MAJ7_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [
        Interval::PERFECT_UNISON,
        Interval::MINOR_THIRD,
        Interval::PERFECT_FIFTH,
        Interval::MAJOR_SEVENTH,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
    ],
    4,
);

/// bVImaj7 chord - creates tension, seeks resolution (p node)
/// Intervals: perfect unison, minor third, perfect fifth, major seventh
pub static MINOR_FLAT_VI_MAJ7: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::VI, Accidental::Natural),
    intervals: MINOR_FLAT_VI_MAJ7_INTERVALS_SET,
    bass: None,
};

/// iv/b6 chord - creates tension, seeks resolution (p node)
/// Intervals: perfect unison, minor third, perfect fifth, major sixth
pub static MINOR_IV_SLASH_FLAT_6: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::IV, Accidental::Natural),
    intervals: MINOR_TRIAD_SET,
    bass: Some((
        RomanNumeral::new(RomanDegree::VI, Accidental::Flat),
        BassType::Slash,
    )),
};

const MINOR_IV_SLASH_FLAT_6_M6_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [
        Interval::PERFECT_UNISON,
        Interval::MINOR_THIRD,
        Interval::PERFECT_FIFTH,
        Interval::MAJOR_SIXTH,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
    ],
    4,
);

/// iv/b6m6 chord - creates tension, seeks resolution (p node)
/// Intervals: perfect unison, minor third, perfect fifth, major sixth
pub static MINOR_IV_SLASH_FLAT_6_M6: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::IV, Accidental::Natural),
    intervals: MINOR_IV_SLASH_FLAT_6_M6_INTERVALS_SET,
    bass: Some((
        RomanNumeral::new(RomanDegree::VI, Accidental::Flat),
        BassType::Slash,
    )),
};

const MINOR_IV_SLASH_FLAT_6_M7_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [
        Interval::PERFECT_UNISON,
        Interval::MINOR_THIRD,
        Interval::PERFECT_FIFTH,
        Interval::MINOR_SEVENTH,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
    ],
    4,
);

/// iv/b6m7 chord - creates tension, seeks resolution (p node)
/// Intervals: perfect unison, minor third, perfect fifth, minor seventh
pub static MINOR_IV_SLASH_FLAT_6_M7: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::IV, Accidental::Natural),
    intervals: MINOR_IV_SLASH_FLAT_6_M7_INTERVALS_SET,
    bass: Some((
        RomanNumeral::new(RomanDegree::VI, Accidental::Flat),
        BassType::Slash,
    )),
};

/// iv chord - creates tension, seeks resolution (p node)
/// Intervals: perfect unison, minor third, perfect fifth
pub static MINOR_IV: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::IV, Accidental::Natural),
    intervals: MINOR_TRIAD_SET,
    bass: None,
};

const MINOR_IV_M6_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [
        Interval::PERFECT_UNISON,
        Interval::MINOR_THIRD,
        Interval::PERFECT_FIFTH,
        Interval::MAJOR_SIXTH,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
    ],
    4,
);

/// ivm6 chord - creates tension, seeks resolution (p node)
/// Intervals: perfect unison, minor third, perfect fifth, major sixth
pub static MINOR_IV_M6: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::IV, Accidental::Natural),
    intervals: MINOR_IV_M6_INTERVALS_SET,
    bass: None,
};

const MINOR_IV_M7_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [
        Interval::PERFECT_UNISON,
        Interval::MINOR_THIRD,
        Interval::PERFECT_FIFTH,
        Interval::MINOR_SEVENTH,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
    ],
    4,
);

/// ivm7 chord - creates tension, seeks resolution (p node)
/// Intervals: perfect unison, minor third, perfect fifth, minor seventh
pub static MINOR_IV_M7: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::IV, Accidental::Natural),
    intervals: MINOR_IV_M7_INTERVALS_SET,
    bass: None,
};

const MINOR_IV_M9_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [
        Interval::PERFECT_UNISON,
        Interval::MINOR_THIRD,
        Interval::PERFECT_FIFTH,
        Interval::MINOR_SEVENTH,
        Interval::MAJOR_NINTH,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
    ],
    5,
);

/// ivm9 chord - creates tension, seeks resolution (p node)
/// Intervals: perfect unison, minor third, perfect fifth, minor seventh, major ninth
pub static MINOR_IV_M9: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::IV, Accidental::Natural),
    intervals: MINOR_IV_M9_INTERVALS_SET,
    bass: None,
};

/// bII/4 chord - creates tension, seeks resolution (p node)
/// Intervals: perfect unison, minor third, perfect fifth
pub static MINOR_FLAT_II_SLASH_4: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::II, Accidental::Natural),
    intervals: MINOR_TRIAD_SET,
    bass: Some((
        RomanNumeral::new(RomanDegree::IV, Accidental::Natural),
        BassType::Slash,
    )),
};

/// i/5 chord - creates tension, seeks resolution (p node)
/// Intervals: perfect unison, minor third, perfect fifth
pub static MINOR_I_SLASH_5: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::I, Accidental::Natural),
    intervals: MINOR_TRIAD_SET,
    bass: Some((
        RomanNumeral::new(RomanDegree::V, Accidental::Natural),
        BassType::Slash,
    )),
};

/// i chord - creates tension, seeks resolution (p node)
/// Intervals: perfect unison, minor third, perfect fifth
pub static MINOR_I: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::I, Accidental::Natural),
    intervals: MINOR_TRIAD_SET,
    bass: None,
};

const MINOR_I_SUS2_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [
        Interval::PERFECT_UNISON,
        Interval::MAJOR_SECOND,
        Interval::PERFECT_FIFTH,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
    ],
    3,
);

/// isus2 chord - creates tension, seeks resolution (p node)
/// Intervals: perfect unison, major second, perfect fifth
pub static MINOR_I_SUS2: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::I, Accidental::Natural),
    intervals: MINOR_I_SUS2_INTERVALS_SET,
    bass: None,
};

const MINOR_I_SUS4_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [
        Interval::PERFECT_UNISON,
        Interval::PERFECT_FOURTH,
        Interval::PERFECT_FIFTH,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
    ],
    3,
);

/// isus4 chord - creates tension, seeks resolution (p node)
/// Intervals: perfect unison, perfect fourth, perfect fifth
pub static MINOR_I_SUS4: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::I, Accidental::Natural),
    intervals: MINOR_I_SUS4_INTERVALS_SET,
    bass: None,
};

/// iv/1 chord - creates tension, seeks resolution (p node)
/// Intervals: perfect unison, minor third, perfect fifth
pub static MINOR_IV_SLASH_1: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::IV, Accidental::Natural),
    intervals: MINOR_TRIAD_SET,
    bass: Some((
        RomanNumeral::new(RomanDegree::I, Accidental::Natural),
        BassType::Slash,
    )),
};

/// V/1 chord - creates tension, seeks resolution (p node)
/// Intervals: perfect unison, major third, perfect fifth
pub static V_SLASH_1: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::V, Accidental::Natural),
    intervals: MAJOR_TRIAD_SET,
    bass: Some((
        RomanNumeral::new(RomanDegree::I, Accidental::Natural),
        BassType::Slash,
    )),
};

/// iidim7 chord - creates tension, seeks resolution (s node)
/// Intervals: perfect unison, minor third, perfect fifth, minor seventh
pub static MINOR_IIDIM7: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::II, Accidental::Natural),
    intervals: MINOR_TRIAD_SET,
    bass: None,
};

/// V7 chord - creates tension, seeks resolution (s node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh
pub static V7: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::V, Accidental::Natural),
    intervals: MAJOR_TRIAD_SET,
    bass: None,
};

const V7_FLAT_9_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [
        Interval::PERFECT_UNISON,
        Interval::MAJOR_THIRD,
        Interval::PERFECT_FIFTH,
        Interval::MINOR_NINTH,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
    ],
    4,
);

/// V7b9 chord - creates tension, seeks resolution (s node)
/// Intervals: perfect unison, major third, perfect fifth, minor ninth
pub static V7_FLAT_9: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::V, Accidental::Natural),
    intervals: V7_FLAT_9_INTERVALS_SET,
    bass: None,
};

/// iiidim7 chord - creates tension, seeks resolution (s node)
/// Intervals: perfect unison, minor third, perfect fifth, minor seventh
pub static MINOR_IIIDIM7: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::III, Accidental::Natural),
    intervals: MINOR_TRIAD_SET,
    bass: None,
};

/// Vdim7 chord - creates tension, seeks resolution (s node)
/// Intervals: perfect unison, minor third, perfect fifth, minor seventh
pub static MINOR_VDIM7: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::V, Accidental::Natural),
    intervals: MINOR_TRIAD_SET,
    bass: None,
};

/// Vm7b5 chord - creates tension, seeks resolution (s node)
/// Intervals: perfect unison, minor third, diminished fifth, minor seventh
pub static MINOR_VM7FLAT_5: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::V, Accidental::Natural),
    intervals: MINOR_TRIAD_SET,
    bass: None,
};

/// I chord - creates tension, seeks resolution (s node)
/// Intervals: perfect unison, major third, perfect fifth
pub static I: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::I, Accidental::Natural),
    intervals: MAJOR_TRIAD_SET,
    bass: None,
};

const I_7_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [
        Interval::PERFECT_UNISON,
        Interval::MAJOR_THIRD,
        Interval::PERFECT_FIFTH,
        Interval::MINOR_SEVENTH,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
    ],
    4,
);

/// I7 chord - creates tension, seeks resolution (s node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh
pub static I_7: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::I, Accidental::Natural),
    intervals: I_7_INTERVALS_SET,
    bass: None,
};

const I_FLAT_9_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [
        Interval::PERFECT_UNISON,
        Interval::MAJOR_THIRD,
        Interval::PERFECT_FIFTH,
        Interval::MINOR_NINTH,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
    ],
    4,
);

/// Ib9 chord - creates tension, seeks resolution (s node)
/// Intervals: perfect unison, major third, perfect fifth, minor ninth
pub static I_FLAT_9: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::I, Accidental::Natural),
    intervals: I_FLAT_9_INTERVALS_SET,
    bass: None,
};

/// vim7b5 chord - creates tension, seeks resolution (s node)
/// Intervals: perfect unison, minor third, diminished fifth, minor seventh
pub static MINOR_VIM7FLAT_5: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::VI, Accidental::Natural),
    intervals: MINOR_TRIAD_SET,
    bass: None,
};

/// II chord - creates tension, seeks resolution (s node)
/// Intervals: perfect unison, major third, perfect fifth
pub static II: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::II, Accidental::Natural),
    intervals: MAJOR_TRIAD_SET,
    bass: None,
};

const II_7_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [
        Interval::PERFECT_UNISON,
        Interval::MAJOR_THIRD,
        Interval::PERFECT_FIFTH,
        Interval::MINOR_SEVENTH,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
    ],
    4,
);

/// II7 chord - creates tension, seeks resolution (s node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh
pub static II_7: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::II, Accidental::Natural),
    intervals: II_7_INTERVALS_SET,
    bass: None,
};

const II_FLAT_9_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [
        Interval::PERFECT_UNISON,
        Interval::MAJOR_THIRD,
        Interval::PERFECT_FIFTH,
        Interval::MINOR_NINTH,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
    ],
    4,
);

/// IIb9 chord - creates tension, seeks resolution (s node)
/// Intervals: perfect unison, major third, perfect fifth, minor ninth
pub static II_FLAT_9: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::II, Accidental::Natural),
    intervals: II_FLAT_9_INTERVALS_SET,
    bass: None,
};

/// sivdim7 chord - creates tension, seeks resolution (s node)
/// Intervals: perfect unison, minor third, perfect fifth, minor seventh
pub static MINOR_SIVDIM7: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::IV, Accidental::Natural),
    intervals: MINOR_TRIAD_SET,
    bass: None,
};

/// bVII chord - creates tension, seeks resolution (s node)
/// Intervals: perfect unison, minor third, perfect fifth
pub static MINOR_FLAT_VII: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::VII, Accidental::Natural),
    intervals: MINOR_TRIAD_SET,
    bass: None,
};

const MINOR_FLAT_VII_9_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [
        Interval::PERFECT_UNISON,
        Interval::MINOR_THIRD,
        Interval::PERFECT_FIFTH,
        Interval::MINOR_SEVENTH,
        Interval::MAJOR_NINTH,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
    ],
    5,
);

/// bVII9 chord - creates tension, seeks resolution (s node)
/// Intervals: perfect unison, minor third, perfect fifth, minor seventh, major ninth
pub static MINOR_FLAT_VII_9: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::VII, Accidental::Natural),
    intervals: MINOR_FLAT_VII_9_INTERVALS_SET,
    bass: None,
};

/// bIII chord - creates tension, seeks resolution (s node)
/// Intervals: perfect unison, minor third, perfect fifth
pub static MINOR_FLAT_III: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::III, Accidental::Natural),
    intervals: MINOR_TRIAD_SET,
    bass: None,
};

const MINOR_FLAT_III_7_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [
        Interval::PERFECT_UNISON,
        Interval::MINOR_THIRD,
        Interval::PERFECT_FIFTH,
        Interval::MINOR_SEVENTH,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
    ],
    4,
);

/// bIII7 chord - creates tension, seeks resolution (s node)
/// Intervals: perfect unison, minor third, perfect fifth, minor seventh
pub static MINOR_FLAT_III_7: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::III, Accidental::Natural),
    intervals: MINOR_FLAT_III_7_INTERVALS_SET,
    bass: None,
};

const MINOR_FLAT_III_9_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [
        Interval::PERFECT_UNISON,
        Interval::MINOR_THIRD,
        Interval::PERFECT_FIFTH,
        Interval::MINOR_SEVENTH,
        Interval::MAJOR_NINTH,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
    ],
    5,
);

/// bIII9 chord - creates tension, seeks resolution (s node)
/// Intervals: perfect unison, minor third, perfect fifth, minor seventh, major ninth
pub static MINOR_FLAT_III_9: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::III, Accidental::Natural),
    intervals: MINOR_FLAT_III_9_INTERVALS_SET,
    bass: None,
};

const MINOR_FLAT_III_FLAT_9_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [
        Interval::PERFECT_UNISON,
        Interval::MINOR_THIRD,
        Interval::PERFECT_FIFTH,
        Interval::MINOR_NINTH,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
        Interval::NONE,
    ],
    4,
);

/// bIIIb9 chord - creates tension, seeks resolution (s node)
/// Intervals: perfect unison, minor third, perfect fifth, minor ninth
pub static MINOR_FLAT_III_FLAT_9: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::III, Accidental::Natural),
    intervals: MINOR_FLAT_III_FLAT_9_INTERVALS_SET,
    bass: None,
};

/// bviim7b5 chord - creates tension, seeks resolution (s node)
/// Intervals: perfect unison, minor third, diminished fifth, minor seventh
pub static MINOR_FLAT_VIIM7FLAT_5: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::VII, Accidental::Natural),
    intervals: MINOR_TRIAD_SET,
    bass: None,
};

/// ivm7 chord - creates tension, seeks resolution (s node)
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

/// Progression edge: iidim → viidim/2
pub static EDGE_MINOR_IIDIM_TO_MINOR_VIIDIM_SLASH_2: ProgressionEdge = ProgressionEdge {
    from: MINOR_IIDIM,
    to: MINOR_VIIDIM_SLASH_2,
};

/// Progression edge: iidim → V
pub static EDGE_MINOR_IIDIM_TO_V: ProgressionEdge = ProgressionEdge {
    from: MINOR_IIDIM,
    to: V,
};

/// Progression edge: iidim → V
pub static EDGE_MINOR_IIDIM_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_IIDIM,
    to: V_7,
};

/// Progression edge: iidim → V
pub static EDGE_MINOR_IIDIM_TO_V_FLAT_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_IIDIM,
    to: V_FLAT_9,
};

/// Progression edge: iidim → V
pub static EDGE_MINOR_IIDIM_TO_V_FLAT_13: ProgressionEdge = ProgressionEdge {
    from: MINOR_IIDIM,
    to: V_FLAT_13,
};

/// Progression edge: iidim → V
pub static EDGE_MINOR_IIDIM_TO_V_SUS4: ProgressionEdge = ProgressionEdge {
    from: MINOR_IIDIM,
    to: V_SUS4,
};

/// Progression edge: iidim → bIIIaug
pub static EDGE_MINOR_IIDIM_TO_MINOR_FLAT_IIIAUG: ProgressionEdge = ProgressionEdge {
    from: MINOR_IIDIM,
    to: MINOR_FLAT_IIIAUG,
};

/// Progression edge: iidim → i/b3
pub static EDGE_MINOR_IIDIM_TO_MINOR_I_SLASH_FLAT_3: ProgressionEdge = ProgressionEdge {
    from: MINOR_IIDIM,
    to: MINOR_I_SLASH_FLAT_3,
};

/// Progression edge: viidim/2 → iidim
pub static EDGE_MINOR_VIIDIM_SLASH_2_TO_MINOR_IIDIM: ProgressionEdge = ProgressionEdge {
    from: MINOR_VIIDIM_SLASH_2,
    to: MINOR_IIDIM,
};

/// Progression edge: viidim/2 → V
pub static EDGE_MINOR_VIIDIM_SLASH_2_TO_V: ProgressionEdge = ProgressionEdge {
    from: MINOR_VIIDIM_SLASH_2,
    to: V,
};

/// Progression edge: viidim/2 → V
pub static EDGE_MINOR_VIIDIM_SLASH_2_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_VIIDIM_SLASH_2,
    to: V_7,
};

/// Progression edge: viidim/2 → V
pub static EDGE_MINOR_VIIDIM_SLASH_2_TO_V_FLAT_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_VIIDIM_SLASH_2,
    to: V_FLAT_9,
};

/// Progression edge: viidim/2 → V
pub static EDGE_MINOR_VIIDIM_SLASH_2_TO_V_FLAT_13: ProgressionEdge = ProgressionEdge {
    from: MINOR_VIIDIM_SLASH_2,
    to: V_FLAT_13,
};

/// Progression edge: viidim/2 → V
pub static EDGE_MINOR_VIIDIM_SLASH_2_TO_V_SUS4: ProgressionEdge = ProgressionEdge {
    from: MINOR_VIIDIM_SLASH_2,
    to: V_SUS4,
};

/// Progression edge: viidim/2 → bIIIaug
pub static EDGE_MINOR_VIIDIM_SLASH_2_TO_MINOR_FLAT_IIIAUG: ProgressionEdge = ProgressionEdge {
    from: MINOR_VIIDIM_SLASH_2,
    to: MINOR_FLAT_IIIAUG,
};

/// Progression edge: viidim/2 → i/b3
pub static EDGE_MINOR_VIIDIM_SLASH_2_TO_MINOR_I_SLASH_FLAT_3: ProgressionEdge = ProgressionEdge {
    from: MINOR_VIIDIM_SLASH_2,
    to: MINOR_I_SLASH_FLAT_3,
};

/// Progression edge: V → bIIIaug
pub static EDGE_V_TO_MINOR_FLAT_IIIAUG: ProgressionEdge = ProgressionEdge {
    from: V,
    to: MINOR_FLAT_IIIAUG,
};

/// Progression edge: V → bIIIaug
pub static EDGE_V_7_TO_MINOR_FLAT_IIIAUG: ProgressionEdge = ProgressionEdge {
    from: V_7,
    to: MINOR_FLAT_IIIAUG,
};

/// Progression edge: V → bIIIaug
pub static EDGE_V_FLAT_9_TO_MINOR_FLAT_IIIAUG: ProgressionEdge = ProgressionEdge {
    from: V_FLAT_9,
    to: MINOR_FLAT_IIIAUG,
};

/// Progression edge: V → bIIIaug
pub static EDGE_V_FLAT_13_TO_MINOR_FLAT_IIIAUG: ProgressionEdge = ProgressionEdge {
    from: V_FLAT_13,
    to: MINOR_FLAT_IIIAUG,
};

/// Progression edge: V → bIIIaug
pub static EDGE_V_SUS4_TO_MINOR_FLAT_IIIAUG: ProgressionEdge = ProgressionEdge {
    from: V_SUS4,
    to: MINOR_FLAT_IIIAUG,
};

/// Progression edge: V → i/b3
pub static EDGE_V_TO_MINOR_I_SLASH_FLAT_3: ProgressionEdge = ProgressionEdge {
    from: V,
    to: MINOR_I_SLASH_FLAT_3,
};

/// Progression edge: V → i/b3
pub static EDGE_V_7_TO_MINOR_I_SLASH_FLAT_3: ProgressionEdge = ProgressionEdge {
    from: V_7,
    to: MINOR_I_SLASH_FLAT_3,
};

/// Progression edge: V → i/b3
pub static EDGE_V_FLAT_9_TO_MINOR_I_SLASH_FLAT_3: ProgressionEdge = ProgressionEdge {
    from: V_FLAT_9,
    to: MINOR_I_SLASH_FLAT_3,
};

/// Progression edge: V → i/b3
pub static EDGE_V_FLAT_13_TO_MINOR_I_SLASH_FLAT_3: ProgressionEdge = ProgressionEdge {
    from: V_FLAT_13,
    to: MINOR_I_SLASH_FLAT_3,
};

/// Progression edge: V → i/b3
pub static EDGE_V_SUS4_TO_MINOR_I_SLASH_FLAT_3: ProgressionEdge = ProgressionEdge {
    from: V_SUS4,
    to: MINOR_I_SLASH_FLAT_3,
};

/// Progression edge: V → bVI
pub static EDGE_V_TO_MINOR_FLAT_VI: ProgressionEdge = ProgressionEdge {
    from: V,
    to: MINOR_FLAT_VI,
};

/// Progression edge: V → bVI
pub static EDGE_V_TO_MINOR_FLAT_VI_6: ProgressionEdge = ProgressionEdge {
    from: V,
    to: MINOR_FLAT_VI_6,
};

/// Progression edge: V → bVI
pub static EDGE_V_TO_MINOR_FLAT_VI_MAJ7: ProgressionEdge = ProgressionEdge {
    from: V,
    to: MINOR_FLAT_VI_MAJ7,
};

/// Progression edge: V → bVI
pub static EDGE_V_7_TO_MINOR_FLAT_VI: ProgressionEdge = ProgressionEdge {
    from: V_7,
    to: MINOR_FLAT_VI,
};

/// Progression edge: V → bVI
pub static EDGE_V_7_TO_MINOR_FLAT_VI_6: ProgressionEdge = ProgressionEdge {
    from: V_7,
    to: MINOR_FLAT_VI_6,
};

/// Progression edge: V → bVI
pub static EDGE_V_7_TO_MINOR_FLAT_VI_MAJ7: ProgressionEdge = ProgressionEdge {
    from: V_7,
    to: MINOR_FLAT_VI_MAJ7,
};

/// Progression edge: V → bVI
pub static EDGE_V_FLAT_9_TO_MINOR_FLAT_VI: ProgressionEdge = ProgressionEdge {
    from: V_FLAT_9,
    to: MINOR_FLAT_VI,
};

/// Progression edge: V → bVI
pub static EDGE_V_FLAT_9_TO_MINOR_FLAT_VI_6: ProgressionEdge = ProgressionEdge {
    from: V_FLAT_9,
    to: MINOR_FLAT_VI_6,
};

/// Progression edge: V → bVI
pub static EDGE_V_FLAT_9_TO_MINOR_FLAT_VI_MAJ7: ProgressionEdge = ProgressionEdge {
    from: V_FLAT_9,
    to: MINOR_FLAT_VI_MAJ7,
};

/// Progression edge: V → bVI
pub static EDGE_V_FLAT_13_TO_MINOR_FLAT_VI: ProgressionEdge = ProgressionEdge {
    from: V_FLAT_13,
    to: MINOR_FLAT_VI,
};

/// Progression edge: V → bVI
pub static EDGE_V_FLAT_13_TO_MINOR_FLAT_VI_6: ProgressionEdge = ProgressionEdge {
    from: V_FLAT_13,
    to: MINOR_FLAT_VI_6,
};

/// Progression edge: V → bVI
pub static EDGE_V_FLAT_13_TO_MINOR_FLAT_VI_MAJ7: ProgressionEdge = ProgressionEdge {
    from: V_FLAT_13,
    to: MINOR_FLAT_VI_MAJ7,
};

/// Progression edge: V → bVI
pub static EDGE_V_SUS4_TO_MINOR_FLAT_VI: ProgressionEdge = ProgressionEdge {
    from: V_SUS4,
    to: MINOR_FLAT_VI,
};

/// Progression edge: V → bVI
pub static EDGE_V_SUS4_TO_MINOR_FLAT_VI_6: ProgressionEdge = ProgressionEdge {
    from: V_SUS4,
    to: MINOR_FLAT_VI_6,
};

/// Progression edge: V → bVI
pub static EDGE_V_SUS4_TO_MINOR_FLAT_VI_MAJ7: ProgressionEdge = ProgressionEdge {
    from: V_SUS4,
    to: MINOR_FLAT_VI_MAJ7,
};

/// Progression edge: V → iv/b6
pub static EDGE_V_TO_MINOR_IV_SLASH_FLAT_6: ProgressionEdge = ProgressionEdge {
    from: V,
    to: MINOR_IV_SLASH_FLAT_6,
};

/// Progression edge: V → iv/b6
pub static EDGE_V_TO_MINOR_IV_SLASH_FLAT_6_M6: ProgressionEdge = ProgressionEdge {
    from: V,
    to: MINOR_IV_SLASH_FLAT_6_M6,
};

/// Progression edge: V → iv/b6
pub static EDGE_V_TO_MINOR_IV_SLASH_FLAT_6_M7: ProgressionEdge = ProgressionEdge {
    from: V,
    to: MINOR_IV_SLASH_FLAT_6_M7,
};

/// Progression edge: V → iv/b6
pub static EDGE_V_7_TO_MINOR_IV_SLASH_FLAT_6: ProgressionEdge = ProgressionEdge {
    from: V_7,
    to: MINOR_IV_SLASH_FLAT_6,
};

/// Progression edge: V → iv/b6
pub static EDGE_V_7_TO_MINOR_IV_SLASH_FLAT_6_M6: ProgressionEdge = ProgressionEdge {
    from: V_7,
    to: MINOR_IV_SLASH_FLAT_6_M6,
};

/// Progression edge: V → iv/b6
pub static EDGE_V_7_TO_MINOR_IV_SLASH_FLAT_6_M7: ProgressionEdge = ProgressionEdge {
    from: V_7,
    to: MINOR_IV_SLASH_FLAT_6_M7,
};

/// Progression edge: V → iv/b6
pub static EDGE_V_FLAT_9_TO_MINOR_IV_SLASH_FLAT_6: ProgressionEdge = ProgressionEdge {
    from: V_FLAT_9,
    to: MINOR_IV_SLASH_FLAT_6,
};

/// Progression edge: V → iv/b6
pub static EDGE_V_FLAT_9_TO_MINOR_IV_SLASH_FLAT_6_M6: ProgressionEdge = ProgressionEdge {
    from: V_FLAT_9,
    to: MINOR_IV_SLASH_FLAT_6_M6,
};

/// Progression edge: V → iv/b6
pub static EDGE_V_FLAT_9_TO_MINOR_IV_SLASH_FLAT_6_M7: ProgressionEdge = ProgressionEdge {
    from: V_FLAT_9,
    to: MINOR_IV_SLASH_FLAT_6_M7,
};

/// Progression edge: V → iv/b6
pub static EDGE_V_FLAT_13_TO_MINOR_IV_SLASH_FLAT_6: ProgressionEdge = ProgressionEdge {
    from: V_FLAT_13,
    to: MINOR_IV_SLASH_FLAT_6,
};

/// Progression edge: V → iv/b6
pub static EDGE_V_FLAT_13_TO_MINOR_IV_SLASH_FLAT_6_M6: ProgressionEdge = ProgressionEdge {
    from: V_FLAT_13,
    to: MINOR_IV_SLASH_FLAT_6_M6,
};

/// Progression edge: V → iv/b6
pub static EDGE_V_FLAT_13_TO_MINOR_IV_SLASH_FLAT_6_M7: ProgressionEdge = ProgressionEdge {
    from: V_FLAT_13,
    to: MINOR_IV_SLASH_FLAT_6_M7,
};

/// Progression edge: V → iv/b6
pub static EDGE_V_SUS4_TO_MINOR_IV_SLASH_FLAT_6: ProgressionEdge = ProgressionEdge {
    from: V_SUS4,
    to: MINOR_IV_SLASH_FLAT_6,
};

/// Progression edge: V → iv/b6
pub static EDGE_V_SUS4_TO_MINOR_IV_SLASH_FLAT_6_M6: ProgressionEdge = ProgressionEdge {
    from: V_SUS4,
    to: MINOR_IV_SLASH_FLAT_6_M6,
};

/// Progression edge: V → iv/b6
pub static EDGE_V_SUS4_TO_MINOR_IV_SLASH_FLAT_6_M7: ProgressionEdge = ProgressionEdge {
    from: V_SUS4,
    to: MINOR_IV_SLASH_FLAT_6_M7,
};

/// Progression edge: bIIIaug → i/b3
pub static EDGE_MINOR_FLAT_IIIAUG_TO_MINOR_I_SLASH_FLAT_3: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_IIIAUG,
    to: MINOR_I_SLASH_FLAT_3,
};

/// Progression edge: bIIIaug → i
pub static EDGE_MINOR_FLAT_IIIAUG_TO_MINOR_I: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_IIIAUG,
    to: MINOR_I,
};

/// Progression edge: bIIIaug → i
pub static EDGE_MINOR_FLAT_IIIAUG_TO_MINOR_I_SUS2: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_IIIAUG,
    to: MINOR_I_SUS2,
};

/// Progression edge: bIIIaug → i
pub static EDGE_MINOR_FLAT_IIIAUG_TO_MINOR_I_SUS4: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_IIIAUG,
    to: MINOR_I_SUS4,
};

/// Progression edge: bIIIaug → iv
pub static EDGE_MINOR_FLAT_IIIAUG_TO_MINOR_IV: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_IIIAUG,
    to: MINOR_IV,
};

/// Progression edge: bIIIaug → iv
pub static EDGE_MINOR_FLAT_IIIAUG_TO_MINOR_IV_M6: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_IIIAUG,
    to: MINOR_IV_M6,
};

/// Progression edge: bIIIaug → iv
pub static EDGE_MINOR_FLAT_IIIAUG_TO_MINOR_IV_M7: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_IIIAUG,
    to: MINOR_IV_M7,
};

/// Progression edge: bIIIaug → iv
pub static EDGE_MINOR_FLAT_IIIAUG_TO_MINOR_IV_M9: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_IIIAUG,
    to: MINOR_IV_M9,
};

/// Progression edge: bIIIaug → bII/4
pub static EDGE_MINOR_FLAT_IIIAUG_TO_MINOR_FLAT_II_SLASH_4: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_IIIAUG,
    to: MINOR_FLAT_II_SLASH_4,
};

/// Progression edge: bIIIaug → bVI
pub static EDGE_MINOR_FLAT_IIIAUG_TO_MINOR_FLAT_VI: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_IIIAUG,
    to: MINOR_FLAT_VI,
};

/// Progression edge: bIIIaug → bVI
pub static EDGE_MINOR_FLAT_IIIAUG_TO_MINOR_FLAT_VI_6: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_IIIAUG,
    to: MINOR_FLAT_VI_6,
};

/// Progression edge: bIIIaug → bVI
pub static EDGE_MINOR_FLAT_IIIAUG_TO_MINOR_FLAT_VI_MAJ7: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_IIIAUG,
    to: MINOR_FLAT_VI_MAJ7,
};

/// Progression edge: bIIIaug → iv/b6
pub static EDGE_MINOR_FLAT_IIIAUG_TO_MINOR_IV_SLASH_FLAT_6: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_IIIAUG,
    to: MINOR_IV_SLASH_FLAT_6,
};

/// Progression edge: bIIIaug → iv/b6
pub static EDGE_MINOR_FLAT_IIIAUG_TO_MINOR_IV_SLASH_FLAT_6_M6: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_IIIAUG,
    to: MINOR_IV_SLASH_FLAT_6_M6,
};

/// Progression edge: bIIIaug → iv/b6
pub static EDGE_MINOR_FLAT_IIIAUG_TO_MINOR_IV_SLASH_FLAT_6_M7: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_IIIAUG,
    to: MINOR_IV_SLASH_FLAT_6_M7,
};

/// Progression edge: i/b3 → bIIIaug
pub static EDGE_MINOR_I_SLASH_FLAT_3_TO_MINOR_FLAT_IIIAUG: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_SLASH_FLAT_3,
    to: MINOR_FLAT_IIIAUG,
};

/// Progression edge: i/b3 → i
pub static EDGE_MINOR_I_SLASH_FLAT_3_TO_MINOR_I: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_SLASH_FLAT_3,
    to: MINOR_I,
};

/// Progression edge: i/b3 → i
pub static EDGE_MINOR_I_SLASH_FLAT_3_TO_MINOR_I_SUS2: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_SLASH_FLAT_3,
    to: MINOR_I_SUS2,
};

/// Progression edge: i/b3 → i
pub static EDGE_MINOR_I_SLASH_FLAT_3_TO_MINOR_I_SUS4: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_SLASH_FLAT_3,
    to: MINOR_I_SUS4,
};

/// Progression edge: i/b3 → iv
pub static EDGE_MINOR_I_SLASH_FLAT_3_TO_MINOR_IV: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_SLASH_FLAT_3,
    to: MINOR_IV,
};

/// Progression edge: i/b3 → iv
pub static EDGE_MINOR_I_SLASH_FLAT_3_TO_MINOR_IV_M6: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_SLASH_FLAT_3,
    to: MINOR_IV_M6,
};

/// Progression edge: i/b3 → iv
pub static EDGE_MINOR_I_SLASH_FLAT_3_TO_MINOR_IV_M7: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_SLASH_FLAT_3,
    to: MINOR_IV_M7,
};

/// Progression edge: i/b3 → iv
pub static EDGE_MINOR_I_SLASH_FLAT_3_TO_MINOR_IV_M9: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_SLASH_FLAT_3,
    to: MINOR_IV_M9,
};

/// Progression edge: i/b3 → bII/4
pub static EDGE_MINOR_I_SLASH_FLAT_3_TO_MINOR_FLAT_II_SLASH_4: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_SLASH_FLAT_3,
    to: MINOR_FLAT_II_SLASH_4,
};

/// Progression edge: i/b3 → bVI
pub static EDGE_MINOR_I_SLASH_FLAT_3_TO_MINOR_FLAT_VI: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_SLASH_FLAT_3,
    to: MINOR_FLAT_VI,
};

/// Progression edge: i/b3 → bVI
pub static EDGE_MINOR_I_SLASH_FLAT_3_TO_MINOR_FLAT_VI_6: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_SLASH_FLAT_3,
    to: MINOR_FLAT_VI_6,
};

/// Progression edge: i/b3 → bVI
pub static EDGE_MINOR_I_SLASH_FLAT_3_TO_MINOR_FLAT_VI_MAJ7: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_SLASH_FLAT_3,
    to: MINOR_FLAT_VI_MAJ7,
};

/// Progression edge: i/b3 → iv/b6
pub static EDGE_MINOR_I_SLASH_FLAT_3_TO_MINOR_IV_SLASH_FLAT_6: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_SLASH_FLAT_3,
    to: MINOR_IV_SLASH_FLAT_6,
};

/// Progression edge: i/b3 → iv/b6
pub static EDGE_MINOR_I_SLASH_FLAT_3_TO_MINOR_IV_SLASH_FLAT_6_M6: ProgressionEdge =
    ProgressionEdge {
        from: MINOR_I_SLASH_FLAT_3,
        to: MINOR_IV_SLASH_FLAT_6_M6,
    };

/// Progression edge: i/b3 → iv/b6
pub static EDGE_MINOR_I_SLASH_FLAT_3_TO_MINOR_IV_SLASH_FLAT_6_M7: ProgressionEdge =
    ProgressionEdge {
        from: MINOR_I_SLASH_FLAT_3,
        to: MINOR_IV_SLASH_FLAT_6_M7,
    };

/// Progression edge: bVI → iv/b6
pub static EDGE_MINOR_FLAT_VI_TO_MINOR_IV_SLASH_FLAT_6: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VI,
    to: MINOR_IV_SLASH_FLAT_6,
};

/// Progression edge: bVI → iv/b6
pub static EDGE_MINOR_FLAT_VI_TO_MINOR_IV_SLASH_FLAT_6_M6: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VI,
    to: MINOR_IV_SLASH_FLAT_6_M6,
};

/// Progression edge: bVI → iv/b6
pub static EDGE_MINOR_FLAT_VI_TO_MINOR_IV_SLASH_FLAT_6_M7: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VI,
    to: MINOR_IV_SLASH_FLAT_6_M7,
};

/// Progression edge: bVI → iv/b6
pub static EDGE_MINOR_FLAT_VI_6_TO_MINOR_IV_SLASH_FLAT_6: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VI_6,
    to: MINOR_IV_SLASH_FLAT_6,
};

/// Progression edge: bVI → iv/b6
pub static EDGE_MINOR_FLAT_VI_6_TO_MINOR_IV_SLASH_FLAT_6_M6: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VI_6,
    to: MINOR_IV_SLASH_FLAT_6_M6,
};

/// Progression edge: bVI → iv/b6
pub static EDGE_MINOR_FLAT_VI_6_TO_MINOR_IV_SLASH_FLAT_6_M7: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VI_6,
    to: MINOR_IV_SLASH_FLAT_6_M7,
};

/// Progression edge: bVI → iv/b6
pub static EDGE_MINOR_FLAT_VI_MAJ7_TO_MINOR_IV_SLASH_FLAT_6: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VI_MAJ7,
    to: MINOR_IV_SLASH_FLAT_6,
};

/// Progression edge: bVI → iv/b6
pub static EDGE_MINOR_FLAT_VI_MAJ7_TO_MINOR_IV_SLASH_FLAT_6_M6: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VI_MAJ7,
    to: MINOR_IV_SLASH_FLAT_6_M6,
};

/// Progression edge: bVI → iv/b6
pub static EDGE_MINOR_FLAT_VI_MAJ7_TO_MINOR_IV_SLASH_FLAT_6_M7: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VI_MAJ7,
    to: MINOR_IV_SLASH_FLAT_6_M7,
};

/// Progression edge: bVI → iv
pub static EDGE_MINOR_FLAT_VI_TO_MINOR_IV: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VI,
    to: MINOR_IV,
};

/// Progression edge: bVI → iv
pub static EDGE_MINOR_FLAT_VI_TO_MINOR_IV_M6: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VI,
    to: MINOR_IV_M6,
};

/// Progression edge: bVI → iv
pub static EDGE_MINOR_FLAT_VI_TO_MINOR_IV_M7: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VI,
    to: MINOR_IV_M7,
};

/// Progression edge: bVI → iv
pub static EDGE_MINOR_FLAT_VI_TO_MINOR_IV_M9: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VI,
    to: MINOR_IV_M9,
};

/// Progression edge: bVI → iv
pub static EDGE_MINOR_FLAT_VI_6_TO_MINOR_IV: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VI_6,
    to: MINOR_IV,
};

/// Progression edge: bVI → iv
pub static EDGE_MINOR_FLAT_VI_6_TO_MINOR_IV_M6: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VI_6,
    to: MINOR_IV_M6,
};

/// Progression edge: bVI → iv
pub static EDGE_MINOR_FLAT_VI_6_TO_MINOR_IV_M7: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VI_6,
    to: MINOR_IV_M7,
};

/// Progression edge: bVI → iv
pub static EDGE_MINOR_FLAT_VI_6_TO_MINOR_IV_M9: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VI_6,
    to: MINOR_IV_M9,
};

/// Progression edge: bVI → iv
pub static EDGE_MINOR_FLAT_VI_MAJ7_TO_MINOR_IV: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VI_MAJ7,
    to: MINOR_IV,
};

/// Progression edge: bVI → iv
pub static EDGE_MINOR_FLAT_VI_MAJ7_TO_MINOR_IV_M6: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VI_MAJ7,
    to: MINOR_IV_M6,
};

/// Progression edge: bVI → iv
pub static EDGE_MINOR_FLAT_VI_MAJ7_TO_MINOR_IV_M7: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VI_MAJ7,
    to: MINOR_IV_M7,
};

/// Progression edge: bVI → iv
pub static EDGE_MINOR_FLAT_VI_MAJ7_TO_MINOR_IV_M9: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VI_MAJ7,
    to: MINOR_IV_M9,
};

/// Progression edge: bVI → bII/4
pub static EDGE_MINOR_FLAT_VI_TO_MINOR_FLAT_II_SLASH_4: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VI,
    to: MINOR_FLAT_II_SLASH_4,
};

/// Progression edge: bVI → bII/4
pub static EDGE_MINOR_FLAT_VI_6_TO_MINOR_FLAT_II_SLASH_4: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VI_6,
    to: MINOR_FLAT_II_SLASH_4,
};

/// Progression edge: bVI → bII/4
pub static EDGE_MINOR_FLAT_VI_MAJ7_TO_MINOR_FLAT_II_SLASH_4: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VI_MAJ7,
    to: MINOR_FLAT_II_SLASH_4,
};

/// Progression edge: bVI → iidim
pub static EDGE_MINOR_FLAT_VI_TO_MINOR_IIDIM: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VI,
    to: MINOR_IIDIM,
};

/// Progression edge: bVI → iidim
pub static EDGE_MINOR_FLAT_VI_6_TO_MINOR_IIDIM: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VI_6,
    to: MINOR_IIDIM,
};

/// Progression edge: bVI → iidim
pub static EDGE_MINOR_FLAT_VI_MAJ7_TO_MINOR_IIDIM: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VI_MAJ7,
    to: MINOR_IIDIM,
};

/// Progression edge: bVI → viidim/2
pub static EDGE_MINOR_FLAT_VI_TO_MINOR_VIIDIM_SLASH_2: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VI,
    to: MINOR_VIIDIM_SLASH_2,
};

/// Progression edge: bVI → viidim/2
pub static EDGE_MINOR_FLAT_VI_6_TO_MINOR_VIIDIM_SLASH_2: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VI_6,
    to: MINOR_VIIDIM_SLASH_2,
};

/// Progression edge: bVI → viidim/2
pub static EDGE_MINOR_FLAT_VI_MAJ7_TO_MINOR_VIIDIM_SLASH_2: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VI_MAJ7,
    to: MINOR_VIIDIM_SLASH_2,
};

/// Progression edge: iv/b6 → bVI
pub static EDGE_MINOR_IV_SLASH_FLAT_6_TO_MINOR_FLAT_VI: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_SLASH_FLAT_6,
    to: MINOR_FLAT_VI,
};

/// Progression edge: iv/b6 → bVI
pub static EDGE_MINOR_IV_SLASH_FLAT_6_TO_MINOR_FLAT_VI_6: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_SLASH_FLAT_6,
    to: MINOR_FLAT_VI_6,
};

/// Progression edge: iv/b6 → bVI
pub static EDGE_MINOR_IV_SLASH_FLAT_6_TO_MINOR_FLAT_VI_MAJ7: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_SLASH_FLAT_6,
    to: MINOR_FLAT_VI_MAJ7,
};

/// Progression edge: iv/b6 → bVI
pub static EDGE_MINOR_IV_SLASH_FLAT_6_M6_TO_MINOR_FLAT_VI: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_SLASH_FLAT_6_M6,
    to: MINOR_FLAT_VI,
};

/// Progression edge: iv/b6 → bVI
pub static EDGE_MINOR_IV_SLASH_FLAT_6_M6_TO_MINOR_FLAT_VI_6: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_SLASH_FLAT_6_M6,
    to: MINOR_FLAT_VI_6,
};

/// Progression edge: iv/b6 → bVI
pub static EDGE_MINOR_IV_SLASH_FLAT_6_M6_TO_MINOR_FLAT_VI_MAJ7: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_SLASH_FLAT_6_M6,
    to: MINOR_FLAT_VI_MAJ7,
};

/// Progression edge: iv/b6 → bVI
pub static EDGE_MINOR_IV_SLASH_FLAT_6_M7_TO_MINOR_FLAT_VI: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_SLASH_FLAT_6_M7,
    to: MINOR_FLAT_VI,
};

/// Progression edge: iv/b6 → bVI
pub static EDGE_MINOR_IV_SLASH_FLAT_6_M7_TO_MINOR_FLAT_VI_6: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_SLASH_FLAT_6_M7,
    to: MINOR_FLAT_VI_6,
};

/// Progression edge: iv/b6 → bVI
pub static EDGE_MINOR_IV_SLASH_FLAT_6_M7_TO_MINOR_FLAT_VI_MAJ7: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_SLASH_FLAT_6_M7,
    to: MINOR_FLAT_VI_MAJ7,
};

/// Progression edge: iv/b6 → iv
pub static EDGE_MINOR_IV_SLASH_FLAT_6_TO_MINOR_IV: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_SLASH_FLAT_6,
    to: MINOR_IV,
};

/// Progression edge: iv/b6 → iv
pub static EDGE_MINOR_IV_SLASH_FLAT_6_TO_MINOR_IV_M6: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_SLASH_FLAT_6,
    to: MINOR_IV_M6,
};

/// Progression edge: iv/b6 → iv
pub static EDGE_MINOR_IV_SLASH_FLAT_6_TO_MINOR_IV_M7: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_SLASH_FLAT_6,
    to: MINOR_IV_M7,
};

/// Progression edge: iv/b6 → iv
pub static EDGE_MINOR_IV_SLASH_FLAT_6_TO_MINOR_IV_M9: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_SLASH_FLAT_6,
    to: MINOR_IV_M9,
};

/// Progression edge: iv/b6 → iv
pub static EDGE_MINOR_IV_SLASH_FLAT_6_M6_TO_MINOR_IV: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_SLASH_FLAT_6_M6,
    to: MINOR_IV,
};

/// Progression edge: iv/b6 → iv
pub static EDGE_MINOR_IV_SLASH_FLAT_6_M6_TO_MINOR_IV_M6: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_SLASH_FLAT_6_M6,
    to: MINOR_IV_M6,
};

/// Progression edge: iv/b6 → iv
pub static EDGE_MINOR_IV_SLASH_FLAT_6_M6_TO_MINOR_IV_M7: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_SLASH_FLAT_6_M6,
    to: MINOR_IV_M7,
};

/// Progression edge: iv/b6 → iv
pub static EDGE_MINOR_IV_SLASH_FLAT_6_M6_TO_MINOR_IV_M9: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_SLASH_FLAT_6_M6,
    to: MINOR_IV_M9,
};

/// Progression edge: iv/b6 → iv
pub static EDGE_MINOR_IV_SLASH_FLAT_6_M7_TO_MINOR_IV: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_SLASH_FLAT_6_M7,
    to: MINOR_IV,
};

/// Progression edge: iv/b6 → iv
pub static EDGE_MINOR_IV_SLASH_FLAT_6_M7_TO_MINOR_IV_M6: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_SLASH_FLAT_6_M7,
    to: MINOR_IV_M6,
};

/// Progression edge: iv/b6 → iv
pub static EDGE_MINOR_IV_SLASH_FLAT_6_M7_TO_MINOR_IV_M7: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_SLASH_FLAT_6_M7,
    to: MINOR_IV_M7,
};

/// Progression edge: iv/b6 → iv
pub static EDGE_MINOR_IV_SLASH_FLAT_6_M7_TO_MINOR_IV_M9: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_SLASH_FLAT_6_M7,
    to: MINOR_IV_M9,
};

/// Progression edge: iv/b6 → bII/4
pub static EDGE_MINOR_IV_SLASH_FLAT_6_TO_MINOR_FLAT_II_SLASH_4: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_SLASH_FLAT_6,
    to: MINOR_FLAT_II_SLASH_4,
};

/// Progression edge: iv/b6 → bII/4
pub static EDGE_MINOR_IV_SLASH_FLAT_6_M6_TO_MINOR_FLAT_II_SLASH_4: ProgressionEdge =
    ProgressionEdge {
        from: MINOR_IV_SLASH_FLAT_6_M6,
        to: MINOR_FLAT_II_SLASH_4,
    };

/// Progression edge: iv/b6 → bII/4
pub static EDGE_MINOR_IV_SLASH_FLAT_6_M7_TO_MINOR_FLAT_II_SLASH_4: ProgressionEdge =
    ProgressionEdge {
        from: MINOR_IV_SLASH_FLAT_6_M7,
        to: MINOR_FLAT_II_SLASH_4,
    };

/// Progression edge: iv/b6 → iidim
pub static EDGE_MINOR_IV_SLASH_FLAT_6_TO_MINOR_IIDIM: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_SLASH_FLAT_6,
    to: MINOR_IIDIM,
};

/// Progression edge: iv/b6 → iidim
pub static EDGE_MINOR_IV_SLASH_FLAT_6_M6_TO_MINOR_IIDIM: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_SLASH_FLAT_6_M6,
    to: MINOR_IIDIM,
};

/// Progression edge: iv/b6 → iidim
pub static EDGE_MINOR_IV_SLASH_FLAT_6_M7_TO_MINOR_IIDIM: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_SLASH_FLAT_6_M7,
    to: MINOR_IIDIM,
};

/// Progression edge: iv/b6 → viidim/2
pub static EDGE_MINOR_IV_SLASH_FLAT_6_TO_MINOR_VIIDIM_SLASH_2: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_SLASH_FLAT_6,
    to: MINOR_VIIDIM_SLASH_2,
};

/// Progression edge: iv/b6 → viidim/2
pub static EDGE_MINOR_IV_SLASH_FLAT_6_M6_TO_MINOR_VIIDIM_SLASH_2: ProgressionEdge =
    ProgressionEdge {
        from: MINOR_IV_SLASH_FLAT_6_M6,
        to: MINOR_VIIDIM_SLASH_2,
    };

/// Progression edge: iv/b6 → viidim/2
pub static EDGE_MINOR_IV_SLASH_FLAT_6_M7_TO_MINOR_VIIDIM_SLASH_2: ProgressionEdge =
    ProgressionEdge {
        from: MINOR_IV_SLASH_FLAT_6_M7,
        to: MINOR_VIIDIM_SLASH_2,
    };

/// Progression edge: iv → bII/4
pub static EDGE_MINOR_IV_TO_MINOR_FLAT_II_SLASH_4: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV,
    to: MINOR_FLAT_II_SLASH_4,
};

/// Progression edge: iv → bII/4
pub static EDGE_MINOR_IV_M6_TO_MINOR_FLAT_II_SLASH_4: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_M6,
    to: MINOR_FLAT_II_SLASH_4,
};

/// Progression edge: iv → bII/4
pub static EDGE_MINOR_IV_M7_TO_MINOR_FLAT_II_SLASH_4: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_M7,
    to: MINOR_FLAT_II_SLASH_4,
};

/// Progression edge: iv → bII/4
pub static EDGE_MINOR_IV_M9_TO_MINOR_FLAT_II_SLASH_4: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_M9,
    to: MINOR_FLAT_II_SLASH_4,
};

/// Progression edge: iv → i
pub static EDGE_MINOR_IV_TO_MINOR_I: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV,
    to: MINOR_I,
};

/// Progression edge: iv → i
pub static EDGE_MINOR_IV_TO_MINOR_I_SUS2: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV,
    to: MINOR_I_SUS2,
};

/// Progression edge: iv → i
pub static EDGE_MINOR_IV_TO_MINOR_I_SUS4: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV,
    to: MINOR_I_SUS4,
};

/// Progression edge: iv → i
pub static EDGE_MINOR_IV_M6_TO_MINOR_I: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_M6,
    to: MINOR_I,
};

/// Progression edge: iv → i
pub static EDGE_MINOR_IV_M6_TO_MINOR_I_SUS2: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_M6,
    to: MINOR_I_SUS2,
};

/// Progression edge: iv → i
pub static EDGE_MINOR_IV_M6_TO_MINOR_I_SUS4: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_M6,
    to: MINOR_I_SUS4,
};

/// Progression edge: iv → i
pub static EDGE_MINOR_IV_M7_TO_MINOR_I: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_M7,
    to: MINOR_I,
};

/// Progression edge: iv → i
pub static EDGE_MINOR_IV_M7_TO_MINOR_I_SUS2: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_M7,
    to: MINOR_I_SUS2,
};

/// Progression edge: iv → i
pub static EDGE_MINOR_IV_M7_TO_MINOR_I_SUS4: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_M7,
    to: MINOR_I_SUS4,
};

/// Progression edge: iv → i
pub static EDGE_MINOR_IV_M9_TO_MINOR_I: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_M9,
    to: MINOR_I,
};

/// Progression edge: iv → i
pub static EDGE_MINOR_IV_M9_TO_MINOR_I_SUS2: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_M9,
    to: MINOR_I_SUS2,
};

/// Progression edge: iv → i
pub static EDGE_MINOR_IV_M9_TO_MINOR_I_SUS4: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_M9,
    to: MINOR_I_SUS4,
};

/// Progression edge: iv → V
pub static EDGE_MINOR_IV_TO_V: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV,
    to: V,
};

/// Progression edge: iv → V
pub static EDGE_MINOR_IV_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV,
    to: V_7,
};

/// Progression edge: iv → V
pub static EDGE_MINOR_IV_TO_V_FLAT_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV,
    to: V_FLAT_9,
};

/// Progression edge: iv → V
pub static EDGE_MINOR_IV_TO_V_FLAT_13: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV,
    to: V_FLAT_13,
};

/// Progression edge: iv → V
pub static EDGE_MINOR_IV_TO_V_SUS4: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV,
    to: V_SUS4,
};

/// Progression edge: iv → V
pub static EDGE_MINOR_IV_M6_TO_V: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_M6,
    to: V,
};

/// Progression edge: iv → V
pub static EDGE_MINOR_IV_M6_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_M6,
    to: V_7,
};

/// Progression edge: iv → V
pub static EDGE_MINOR_IV_M6_TO_V_FLAT_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_M6,
    to: V_FLAT_9,
};

/// Progression edge: iv → V
pub static EDGE_MINOR_IV_M6_TO_V_FLAT_13: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_M6,
    to: V_FLAT_13,
};

/// Progression edge: iv → V
pub static EDGE_MINOR_IV_M6_TO_V_SUS4: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_M6,
    to: V_SUS4,
};

/// Progression edge: iv → V
pub static EDGE_MINOR_IV_M7_TO_V: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_M7,
    to: V,
};

/// Progression edge: iv → V
pub static EDGE_MINOR_IV_M7_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_M7,
    to: V_7,
};

/// Progression edge: iv → V
pub static EDGE_MINOR_IV_M7_TO_V_FLAT_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_M7,
    to: V_FLAT_9,
};

/// Progression edge: iv → V
pub static EDGE_MINOR_IV_M7_TO_V_FLAT_13: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_M7,
    to: V_FLAT_13,
};

/// Progression edge: iv → V
pub static EDGE_MINOR_IV_M7_TO_V_SUS4: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_M7,
    to: V_SUS4,
};

/// Progression edge: iv → V
pub static EDGE_MINOR_IV_M9_TO_V: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_M9,
    to: V,
};

/// Progression edge: iv → V
pub static EDGE_MINOR_IV_M9_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_M9,
    to: V_7,
};

/// Progression edge: iv → V
pub static EDGE_MINOR_IV_M9_TO_V_FLAT_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_M9,
    to: V_FLAT_9,
};

/// Progression edge: iv → V
pub static EDGE_MINOR_IV_M9_TO_V_FLAT_13: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_M9,
    to: V_FLAT_13,
};

/// Progression edge: iv → V
pub static EDGE_MINOR_IV_M9_TO_V_SUS4: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_M9,
    to: V_SUS4,
};

/// Progression edge: iv → i/5
pub static EDGE_MINOR_IV_TO_MINOR_I_SLASH_5: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV,
    to: MINOR_I_SLASH_5,
};

/// Progression edge: iv → i/5
pub static EDGE_MINOR_IV_M6_TO_MINOR_I_SLASH_5: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_M6,
    to: MINOR_I_SLASH_5,
};

/// Progression edge: iv → i/5
pub static EDGE_MINOR_IV_M7_TO_MINOR_I_SLASH_5: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_M7,
    to: MINOR_I_SLASH_5,
};

/// Progression edge: iv → i/5
pub static EDGE_MINOR_IV_M9_TO_MINOR_I_SLASH_5: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_M9,
    to: MINOR_I_SLASH_5,
};

/// Progression edge: iv → iidim
pub static EDGE_MINOR_IV_TO_MINOR_IIDIM: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV,
    to: MINOR_IIDIM,
};

/// Progression edge: iv → iidim
pub static EDGE_MINOR_IV_M6_TO_MINOR_IIDIM: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_M6,
    to: MINOR_IIDIM,
};

/// Progression edge: iv → iidim
pub static EDGE_MINOR_IV_M7_TO_MINOR_IIDIM: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_M7,
    to: MINOR_IIDIM,
};

/// Progression edge: iv → iidim
pub static EDGE_MINOR_IV_M9_TO_MINOR_IIDIM: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_M9,
    to: MINOR_IIDIM,
};

/// Progression edge: iv → viidim/2
pub static EDGE_MINOR_IV_TO_MINOR_VIIDIM_SLASH_2: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV,
    to: MINOR_VIIDIM_SLASH_2,
};

/// Progression edge: iv → viidim/2
pub static EDGE_MINOR_IV_M6_TO_MINOR_VIIDIM_SLASH_2: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_M6,
    to: MINOR_VIIDIM_SLASH_2,
};

/// Progression edge: iv → viidim/2
pub static EDGE_MINOR_IV_M7_TO_MINOR_VIIDIM_SLASH_2: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_M7,
    to: MINOR_VIIDIM_SLASH_2,
};

/// Progression edge: iv → viidim/2
pub static EDGE_MINOR_IV_M9_TO_MINOR_VIIDIM_SLASH_2: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_M9,
    to: MINOR_VIIDIM_SLASH_2,
};

/// Progression edge: iv → i/b3
pub static EDGE_MINOR_IV_TO_MINOR_I_SLASH_FLAT_3: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV,
    to: MINOR_I_SLASH_FLAT_3,
};

/// Progression edge: iv → i/b3
pub static EDGE_MINOR_IV_M6_TO_MINOR_I_SLASH_FLAT_3: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_M6,
    to: MINOR_I_SLASH_FLAT_3,
};

/// Progression edge: iv → i/b3
pub static EDGE_MINOR_IV_M7_TO_MINOR_I_SLASH_FLAT_3: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_M7,
    to: MINOR_I_SLASH_FLAT_3,
};

/// Progression edge: iv → i/b3
pub static EDGE_MINOR_IV_M9_TO_MINOR_I_SLASH_FLAT_3: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_M9,
    to: MINOR_I_SLASH_FLAT_3,
};

/// Progression edge: bII/4 → iv
pub static EDGE_MINOR_FLAT_II_SLASH_4_TO_MINOR_IV: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_II_SLASH_4,
    to: MINOR_IV,
};

/// Progression edge: bII/4 → iv
pub static EDGE_MINOR_FLAT_II_SLASH_4_TO_MINOR_IV_M6: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_II_SLASH_4,
    to: MINOR_IV_M6,
};

/// Progression edge: bII/4 → iv
pub static EDGE_MINOR_FLAT_II_SLASH_4_TO_MINOR_IV_M7: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_II_SLASH_4,
    to: MINOR_IV_M7,
};

/// Progression edge: bII/4 → iv
pub static EDGE_MINOR_FLAT_II_SLASH_4_TO_MINOR_IV_M9: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_II_SLASH_4,
    to: MINOR_IV_M9,
};

/// Progression edge: bII/4 → i
pub static EDGE_MINOR_FLAT_II_SLASH_4_TO_MINOR_I: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_II_SLASH_4,
    to: MINOR_I,
};

/// Progression edge: bII/4 → i
pub static EDGE_MINOR_FLAT_II_SLASH_4_TO_MINOR_I_SUS2: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_II_SLASH_4,
    to: MINOR_I_SUS2,
};

/// Progression edge: bII/4 → i
pub static EDGE_MINOR_FLAT_II_SLASH_4_TO_MINOR_I_SUS4: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_II_SLASH_4,
    to: MINOR_I_SUS4,
};

/// Progression edge: bII/4 → V
pub static EDGE_MINOR_FLAT_II_SLASH_4_TO_V: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_II_SLASH_4,
    to: V,
};

/// Progression edge: bII/4 → V
pub static EDGE_MINOR_FLAT_II_SLASH_4_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_II_SLASH_4,
    to: V_7,
};

/// Progression edge: bII/4 → V
pub static EDGE_MINOR_FLAT_II_SLASH_4_TO_V_FLAT_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_II_SLASH_4,
    to: V_FLAT_9,
};

/// Progression edge: bII/4 → V
pub static EDGE_MINOR_FLAT_II_SLASH_4_TO_V_FLAT_13: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_II_SLASH_4,
    to: V_FLAT_13,
};

/// Progression edge: bII/4 → V
pub static EDGE_MINOR_FLAT_II_SLASH_4_TO_V_SUS4: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_II_SLASH_4,
    to: V_SUS4,
};

/// Progression edge: bII/4 → i/5
pub static EDGE_MINOR_FLAT_II_SLASH_4_TO_MINOR_I_SLASH_5: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_II_SLASH_4,
    to: MINOR_I_SLASH_5,
};

/// Progression edge: bII/4 → iidim
pub static EDGE_MINOR_FLAT_II_SLASH_4_TO_MINOR_IIDIM: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_II_SLASH_4,
    to: MINOR_IIDIM,
};

/// Progression edge: bII/4 → viidim/2
pub static EDGE_MINOR_FLAT_II_SLASH_4_TO_MINOR_VIIDIM_SLASH_2: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_II_SLASH_4,
    to: MINOR_VIIDIM_SLASH_2,
};

/// Progression edge: bII/4 → i/b3
pub static EDGE_MINOR_FLAT_II_SLASH_4_TO_MINOR_I_SLASH_FLAT_3: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_II_SLASH_4,
    to: MINOR_I_SLASH_FLAT_3,
};

/// Progression edge: i/b3 → iidim
pub static EDGE_MINOR_I_SLASH_FLAT_3_TO_MINOR_IIDIM: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_SLASH_FLAT_3,
    to: MINOR_IIDIM,
};

/// Progression edge: i/b3 → viidim/2
pub static EDGE_MINOR_I_SLASH_FLAT_3_TO_MINOR_VIIDIM_SLASH_2: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_SLASH_FLAT_3,
    to: MINOR_VIIDIM_SLASH_2,
};

/// Progression edge: iidim → i/5
pub static EDGE_MINOR_IIDIM_TO_MINOR_I_SLASH_5: ProgressionEdge = ProgressionEdge {
    from: MINOR_IIDIM,
    to: MINOR_I_SLASH_5,
};

/// Progression edge: viidim/2 → i/5
pub static EDGE_MINOR_VIIDIM_SLASH_2_TO_MINOR_I_SLASH_5: ProgressionEdge = ProgressionEdge {
    from: MINOR_VIIDIM_SLASH_2,
    to: MINOR_I_SLASH_5,
};

/// Progression edge: V → i
pub static EDGE_V_TO_MINOR_I: ProgressionEdge = ProgressionEdge {
    from: V,
    to: MINOR_I,
};

/// Progression edge: V → i
pub static EDGE_V_TO_MINOR_I_SUS2: ProgressionEdge = ProgressionEdge {
    from: V,
    to: MINOR_I_SUS2,
};

/// Progression edge: V → i
pub static EDGE_V_TO_MINOR_I_SUS4: ProgressionEdge = ProgressionEdge {
    from: V,
    to: MINOR_I_SUS4,
};

/// Progression edge: V → i
pub static EDGE_V_7_TO_MINOR_I: ProgressionEdge = ProgressionEdge {
    from: V_7,
    to: MINOR_I,
};

/// Progression edge: V → i
pub static EDGE_V_7_TO_MINOR_I_SUS2: ProgressionEdge = ProgressionEdge {
    from: V_7,
    to: MINOR_I_SUS2,
};

/// Progression edge: V → i
pub static EDGE_V_7_TO_MINOR_I_SUS4: ProgressionEdge = ProgressionEdge {
    from: V_7,
    to: MINOR_I_SUS4,
};

/// Progression edge: V → i
pub static EDGE_V_FLAT_9_TO_MINOR_I: ProgressionEdge = ProgressionEdge {
    from: V_FLAT_9,
    to: MINOR_I,
};

/// Progression edge: V → i
pub static EDGE_V_FLAT_9_TO_MINOR_I_SUS2: ProgressionEdge = ProgressionEdge {
    from: V_FLAT_9,
    to: MINOR_I_SUS2,
};

/// Progression edge: V → i
pub static EDGE_V_FLAT_9_TO_MINOR_I_SUS4: ProgressionEdge = ProgressionEdge {
    from: V_FLAT_9,
    to: MINOR_I_SUS4,
};

/// Progression edge: V → i
pub static EDGE_V_FLAT_13_TO_MINOR_I: ProgressionEdge = ProgressionEdge {
    from: V_FLAT_13,
    to: MINOR_I,
};

/// Progression edge: V → i
pub static EDGE_V_FLAT_13_TO_MINOR_I_SUS2: ProgressionEdge = ProgressionEdge {
    from: V_FLAT_13,
    to: MINOR_I_SUS2,
};

/// Progression edge: V → i
pub static EDGE_V_FLAT_13_TO_MINOR_I_SUS4: ProgressionEdge = ProgressionEdge {
    from: V_FLAT_13,
    to: MINOR_I_SUS4,
};

/// Progression edge: V → i
pub static EDGE_V_SUS4_TO_MINOR_I: ProgressionEdge = ProgressionEdge {
    from: V_SUS4,
    to: MINOR_I,
};

/// Progression edge: V → i
pub static EDGE_V_SUS4_TO_MINOR_I_SUS2: ProgressionEdge = ProgressionEdge {
    from: V_SUS4,
    to: MINOR_I_SUS2,
};

/// Progression edge: V → i
pub static EDGE_V_SUS4_TO_MINOR_I_SUS4: ProgressionEdge = ProgressionEdge {
    from: V_SUS4,
    to: MINOR_I_SUS4,
};

/// Progression edge: i → V/1
pub static EDGE_MINOR_I_TO_V_SLASH_1: ProgressionEdge = ProgressionEdge {
    from: MINOR_I,
    to: V_SLASH_1,
};

/// Progression edge: i → V/1
pub static EDGE_MINOR_I_SUS2_TO_V_SLASH_1: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_SUS2,
    to: V_SLASH_1,
};

/// Progression edge: i → V/1
pub static EDGE_MINOR_I_SUS4_TO_V_SLASH_1: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_SUS4,
    to: V_SLASH_1,
};

/// Progression edge: i → iv/1
pub static EDGE_MINOR_I_TO_MINOR_IV_SLASH_1: ProgressionEdge = ProgressionEdge {
    from: MINOR_I,
    to: MINOR_IV_SLASH_1,
};

/// Progression edge: i → iv/1
pub static EDGE_MINOR_I_SUS2_TO_MINOR_IV_SLASH_1: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_SUS2,
    to: MINOR_IV_SLASH_1,
};

/// Progression edge: i → iv/1
pub static EDGE_MINOR_I_SUS4_TO_MINOR_IV_SLASH_1: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_SUS4,
    to: MINOR_IV_SLASH_1,
};

/// Progression edge: V/1 → i
pub static EDGE_V_SLASH_1_TO_MINOR_I: ProgressionEdge = ProgressionEdge {
    from: V_SLASH_1,
    to: MINOR_I,
};

/// Progression edge: V/1 → i
pub static EDGE_V_SLASH_1_TO_MINOR_I_SUS2: ProgressionEdge = ProgressionEdge {
    from: V_SLASH_1,
    to: MINOR_I_SUS2,
};

/// Progression edge: V/1 → i
pub static EDGE_V_SLASH_1_TO_MINOR_I_SUS4: ProgressionEdge = ProgressionEdge {
    from: V_SLASH_1,
    to: MINOR_I_SUS4,
};

/// Progression edge: V/1 → iv/1
pub static EDGE_V_SLASH_1_TO_MINOR_IV_SLASH_1: ProgressionEdge = ProgressionEdge {
    from: V_SLASH_1,
    to: MINOR_IV_SLASH_1,
};

/// Progression edge: iv/1 → i
pub static EDGE_MINOR_IV_SLASH_1_TO_MINOR_I: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_SLASH_1,
    to: MINOR_I,
};

/// Progression edge: iv/1 → i
pub static EDGE_MINOR_IV_SLASH_1_TO_MINOR_I_SUS2: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_SLASH_1,
    to: MINOR_I_SUS2,
};

/// Progression edge: iv/1 → i
pub static EDGE_MINOR_IV_SLASH_1_TO_MINOR_I_SUS4: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_SLASH_1,
    to: MINOR_I_SUS4,
};

/// Progression edge: iv/1 → V/1
pub static EDGE_MINOR_IV_SLASH_1_TO_V_SLASH_1: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_SLASH_1,
    to: V_SLASH_1,
};

/// Progression edge: iidim7 → bIIIaug
pub static EDGE_MINOR_IIDIM7_TO_MINOR_FLAT_IIIAUG: ProgressionEdge = ProgressionEdge {
    from: MINOR_IIDIM7,
    to: MINOR_FLAT_IIIAUG,
};

/// Progression edge: iidim7 → i/b3
pub static EDGE_MINOR_IIDIM7_TO_MINOR_I_SLASH_FLAT_3: ProgressionEdge = ProgressionEdge {
    from: MINOR_IIDIM7,
    to: MINOR_I_SLASH_FLAT_3,
};

/// Progression edge: V7 → bIIIaug
pub static EDGE_V7_TO_MINOR_FLAT_IIIAUG: ProgressionEdge = ProgressionEdge {
    from: V7,
    to: MINOR_FLAT_IIIAUG,
};

/// Progression edge: V7 → bIIIaug
pub static EDGE_V7_FLAT_9_TO_MINOR_FLAT_IIIAUG: ProgressionEdge = ProgressionEdge {
    from: V7_FLAT_9,
    to: MINOR_FLAT_IIIAUG,
};

/// Progression edge: V7 → i/b3
pub static EDGE_V7_TO_MINOR_I_SLASH_FLAT_3: ProgressionEdge = ProgressionEdge {
    from: V7,
    to: MINOR_I_SLASH_FLAT_3,
};

/// Progression edge: V7 → i/b3
pub static EDGE_V7_FLAT_9_TO_MINOR_I_SLASH_FLAT_3: ProgressionEdge = ProgressionEdge {
    from: V7_FLAT_9,
    to: MINOR_I_SLASH_FLAT_3,
};

/// Progression edge: iiidim7 → iv
pub static EDGE_MINOR_IIIDIM7_TO_MINOR_IV: ProgressionEdge = ProgressionEdge {
    from: MINOR_IIIDIM7,
    to: MINOR_IV,
};

/// Progression edge: iiidim7 → iv
pub static EDGE_MINOR_IIIDIM7_TO_MINOR_IV_M6: ProgressionEdge = ProgressionEdge {
    from: MINOR_IIIDIM7,
    to: MINOR_IV_M6,
};

/// Progression edge: iiidim7 → iv
pub static EDGE_MINOR_IIIDIM7_TO_MINOR_IV_M7: ProgressionEdge = ProgressionEdge {
    from: MINOR_IIIDIM7,
    to: MINOR_IV_M7,
};

/// Progression edge: iiidim7 → iv
pub static EDGE_MINOR_IIIDIM7_TO_MINOR_IV_M9: ProgressionEdge = ProgressionEdge {
    from: MINOR_IIIDIM7,
    to: MINOR_IV_M9,
};

/// Progression edge: iiidim7 → bII/4
pub static EDGE_MINOR_IIIDIM7_TO_MINOR_FLAT_II_SLASH_4: ProgressionEdge = ProgressionEdge {
    from: MINOR_IIIDIM7,
    to: MINOR_FLAT_II_SLASH_4,
};

/// Progression edge: Vdim7 → I
pub static EDGE_MINOR_VDIM7_TO_I: ProgressionEdge = ProgressionEdge {
    from: MINOR_VDIM7,
    to: I,
};

/// Progression edge: Vdim7 → I
pub static EDGE_MINOR_VDIM7_TO_I_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_VDIM7,
    to: I_7,
};

/// Progression edge: Vdim7 → I
pub static EDGE_MINOR_VDIM7_TO_I_FLAT_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_VDIM7,
    to: I_FLAT_9,
};

/// Progression edge: Vm7b5 → I
pub static EDGE_MINOR_VM7FLAT_5_TO_I: ProgressionEdge = ProgressionEdge {
    from: MINOR_VM7FLAT_5,
    to: I,
};

/// Progression edge: Vm7b5 → I
pub static EDGE_MINOR_VM7FLAT_5_TO_I_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_VM7FLAT_5,
    to: I_7,
};

/// Progression edge: Vm7b5 → I
pub static EDGE_MINOR_VM7FLAT_5_TO_I_FLAT_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_VM7FLAT_5,
    to: I_FLAT_9,
};

/// Progression edge: I → iv
pub static EDGE_I_TO_MINOR_IV: ProgressionEdge = ProgressionEdge {
    from: I,
    to: MINOR_IV,
};

/// Progression edge: I → iv
pub static EDGE_I_TO_MINOR_IV_M6: ProgressionEdge = ProgressionEdge {
    from: I,
    to: MINOR_IV_M6,
};

/// Progression edge: I → iv
pub static EDGE_I_TO_MINOR_IV_M7: ProgressionEdge = ProgressionEdge {
    from: I,
    to: MINOR_IV_M7,
};

/// Progression edge: I → iv
pub static EDGE_I_TO_MINOR_IV_M9: ProgressionEdge = ProgressionEdge {
    from: I,
    to: MINOR_IV_M9,
};

/// Progression edge: I → iv
pub static EDGE_I_7_TO_MINOR_IV: ProgressionEdge = ProgressionEdge {
    from: I_7,
    to: MINOR_IV,
};

/// Progression edge: I → iv
pub static EDGE_I_7_TO_MINOR_IV_M6: ProgressionEdge = ProgressionEdge {
    from: I_7,
    to: MINOR_IV_M6,
};

/// Progression edge: I → iv
pub static EDGE_I_7_TO_MINOR_IV_M7: ProgressionEdge = ProgressionEdge {
    from: I_7,
    to: MINOR_IV_M7,
};

/// Progression edge: I → iv
pub static EDGE_I_7_TO_MINOR_IV_M9: ProgressionEdge = ProgressionEdge {
    from: I_7,
    to: MINOR_IV_M9,
};

/// Progression edge: I → iv
pub static EDGE_I_FLAT_9_TO_MINOR_IV: ProgressionEdge = ProgressionEdge {
    from: I_FLAT_9,
    to: MINOR_IV,
};

/// Progression edge: I → iv
pub static EDGE_I_FLAT_9_TO_MINOR_IV_M6: ProgressionEdge = ProgressionEdge {
    from: I_FLAT_9,
    to: MINOR_IV_M6,
};

/// Progression edge: I → iv
pub static EDGE_I_FLAT_9_TO_MINOR_IV_M7: ProgressionEdge = ProgressionEdge {
    from: I_FLAT_9,
    to: MINOR_IV_M7,
};

/// Progression edge: I → iv
pub static EDGE_I_FLAT_9_TO_MINOR_IV_M9: ProgressionEdge = ProgressionEdge {
    from: I_FLAT_9,
    to: MINOR_IV_M9,
};

/// Progression edge: I → bII/4
pub static EDGE_I_TO_MINOR_FLAT_II_SLASH_4: ProgressionEdge = ProgressionEdge {
    from: I,
    to: MINOR_FLAT_II_SLASH_4,
};

/// Progression edge: I → bII/4
pub static EDGE_I_7_TO_MINOR_FLAT_II_SLASH_4: ProgressionEdge = ProgressionEdge {
    from: I_7,
    to: MINOR_FLAT_II_SLASH_4,
};

/// Progression edge: I → bII/4
pub static EDGE_I_FLAT_9_TO_MINOR_FLAT_II_SLASH_4: ProgressionEdge = ProgressionEdge {
    from: I_FLAT_9,
    to: MINOR_FLAT_II_SLASH_4,
};

/// Progression edge: vim7b5 → II
pub static EDGE_MINOR_VIM7FLAT_5_TO_II: ProgressionEdge = ProgressionEdge {
    from: MINOR_VIM7FLAT_5,
    to: II,
};

/// Progression edge: vim7b5 → II
pub static EDGE_MINOR_VIM7FLAT_5_TO_II_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_VIM7FLAT_5,
    to: II_7,
};

/// Progression edge: vim7b5 → II
pub static EDGE_MINOR_VIM7FLAT_5_TO_II_FLAT_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_VIM7FLAT_5,
    to: II_FLAT_9,
};

/// Progression edge: II → V
pub static EDGE_II_TO_V: ProgressionEdge = ProgressionEdge { from: II, to: V };

/// Progression edge: II → V
pub static EDGE_II_TO_V_7: ProgressionEdge = ProgressionEdge { from: II, to: V_7 };

/// Progression edge: II → V
pub static EDGE_II_TO_V_FLAT_9: ProgressionEdge = ProgressionEdge {
    from: II,
    to: V_FLAT_9,
};

/// Progression edge: II → V
pub static EDGE_II_TO_V_FLAT_13: ProgressionEdge = ProgressionEdge {
    from: II,
    to: V_FLAT_13,
};

/// Progression edge: II → V
pub static EDGE_II_TO_V_SUS4: ProgressionEdge = ProgressionEdge {
    from: II,
    to: V_SUS4,
};

/// Progression edge: II → V
pub static EDGE_II_7_TO_V: ProgressionEdge = ProgressionEdge { from: II_7, to: V };

/// Progression edge: II → V
pub static EDGE_II_7_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: II_7,
    to: V_7,
};

/// Progression edge: II → V
pub static EDGE_II_7_TO_V_FLAT_9: ProgressionEdge = ProgressionEdge {
    from: II_7,
    to: V_FLAT_9,
};

/// Progression edge: II → V
pub static EDGE_II_7_TO_V_FLAT_13: ProgressionEdge = ProgressionEdge {
    from: II_7,
    to: V_FLAT_13,
};

/// Progression edge: II → V
pub static EDGE_II_7_TO_V_SUS4: ProgressionEdge = ProgressionEdge {
    from: II_7,
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
pub static EDGE_II_FLAT_9_TO_V_FLAT_9: ProgressionEdge = ProgressionEdge {
    from: II_FLAT_9,
    to: V_FLAT_9,
};

/// Progression edge: II → V
pub static EDGE_II_FLAT_9_TO_V_FLAT_13: ProgressionEdge = ProgressionEdge {
    from: II_FLAT_9,
    to: V_FLAT_13,
};

/// Progression edge: II → V
pub static EDGE_II_FLAT_9_TO_V_SUS4: ProgressionEdge = ProgressionEdge {
    from: II_FLAT_9,
    to: V_SUS4,
};

/// Progression edge: sivdim7 → V
pub static EDGE_MINOR_SIVDIM7_TO_V: ProgressionEdge = ProgressionEdge {
    from: MINOR_SIVDIM7,
    to: V,
};

/// Progression edge: sivdim7 → V
pub static EDGE_MINOR_SIVDIM7_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_SIVDIM7,
    to: V_7,
};

/// Progression edge: sivdim7 → V
pub static EDGE_MINOR_SIVDIM7_TO_V_FLAT_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_SIVDIM7,
    to: V_FLAT_9,
};

/// Progression edge: sivdim7 → V
pub static EDGE_MINOR_SIVDIM7_TO_V_FLAT_13: ProgressionEdge = ProgressionEdge {
    from: MINOR_SIVDIM7,
    to: V_FLAT_13,
};

/// Progression edge: sivdim7 → V
pub static EDGE_MINOR_SIVDIM7_TO_V_SUS4: ProgressionEdge = ProgressionEdge {
    from: MINOR_SIVDIM7,
    to: V_SUS4,
};

/// Progression edge: bVI → bVII
pub static EDGE_MINOR_FLAT_VI_TO_MINOR_FLAT_VII: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VI,
    to: MINOR_FLAT_VII,
};

/// Progression edge: bVI → bVII
pub static EDGE_MINOR_FLAT_VI_TO_MINOR_FLAT_VII_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VI,
    to: MINOR_FLAT_VII_9,
};

/// Progression edge: bVI → bVII
pub static EDGE_MINOR_FLAT_VI_6_TO_MINOR_FLAT_VII: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VI_6,
    to: MINOR_FLAT_VII,
};

/// Progression edge: bVI → bVII
pub static EDGE_MINOR_FLAT_VI_6_TO_MINOR_FLAT_VII_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VI_6,
    to: MINOR_FLAT_VII_9,
};

/// Progression edge: bVI → bVII
pub static EDGE_MINOR_FLAT_VI_MAJ7_TO_MINOR_FLAT_VII: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VI_MAJ7,
    to: MINOR_FLAT_VII,
};

/// Progression edge: bVI → bVII
pub static EDGE_MINOR_FLAT_VI_MAJ7_TO_MINOR_FLAT_VII_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VI_MAJ7,
    to: MINOR_FLAT_VII_9,
};

/// Progression edge: bVII → i
pub static EDGE_MINOR_FLAT_VII_TO_MINOR_I: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VII,
    to: MINOR_I,
};

/// Progression edge: bVII → i
pub static EDGE_MINOR_FLAT_VII_TO_MINOR_I_SUS2: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VII,
    to: MINOR_I_SUS2,
};

/// Progression edge: bVII → i
pub static EDGE_MINOR_FLAT_VII_TO_MINOR_I_SUS4: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VII,
    to: MINOR_I_SUS4,
};

/// Progression edge: bVII → i
pub static EDGE_MINOR_FLAT_VII_9_TO_MINOR_I: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VII_9,
    to: MINOR_I,
};

/// Progression edge: bVII → i
pub static EDGE_MINOR_FLAT_VII_9_TO_MINOR_I_SUS2: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VII_9,
    to: MINOR_I_SUS2,
};

/// Progression edge: bVII → i
pub static EDGE_MINOR_FLAT_VII_9_TO_MINOR_I_SUS4: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VII_9,
    to: MINOR_I_SUS4,
};

/// Progression edge: Vm7b5 → bVI
pub static EDGE_MINOR_VM7FLAT_5_TO_MINOR_FLAT_VI: ProgressionEdge = ProgressionEdge {
    from: MINOR_VM7FLAT_5,
    to: MINOR_FLAT_VI,
};

/// Progression edge: Vm7b5 → bVI
pub static EDGE_MINOR_VM7FLAT_5_TO_MINOR_FLAT_VI_6: ProgressionEdge = ProgressionEdge {
    from: MINOR_VM7FLAT_5,
    to: MINOR_FLAT_VI_6,
};

/// Progression edge: Vm7b5 → bVI
pub static EDGE_MINOR_VM7FLAT_5_TO_MINOR_FLAT_VI_MAJ7: ProgressionEdge = ProgressionEdge {
    from: MINOR_VM7FLAT_5,
    to: MINOR_FLAT_VI_MAJ7,
};

/// Progression edge: Vm7b5 → iv/b6
pub static EDGE_MINOR_VM7FLAT_5_TO_MINOR_IV_SLASH_FLAT_6: ProgressionEdge = ProgressionEdge {
    from: MINOR_VM7FLAT_5,
    to: MINOR_IV_SLASH_FLAT_6,
};

/// Progression edge: Vm7b5 → iv/b6
pub static EDGE_MINOR_VM7FLAT_5_TO_MINOR_IV_SLASH_FLAT_6_M6: ProgressionEdge = ProgressionEdge {
    from: MINOR_VM7FLAT_5,
    to: MINOR_IV_SLASH_FLAT_6_M6,
};

/// Progression edge: Vm7b5 → iv/b6
pub static EDGE_MINOR_VM7FLAT_5_TO_MINOR_IV_SLASH_FLAT_6_M7: ProgressionEdge = ProgressionEdge {
    from: MINOR_VM7FLAT_5,
    to: MINOR_IV_SLASH_FLAT_6_M7,
};

/// Progression edge: Vdim7 → bVI
pub static EDGE_MINOR_VDIM7_TO_MINOR_FLAT_VI: ProgressionEdge = ProgressionEdge {
    from: MINOR_VDIM7,
    to: MINOR_FLAT_VI,
};

/// Progression edge: Vdim7 → bVI
pub static EDGE_MINOR_VDIM7_TO_MINOR_FLAT_VI_6: ProgressionEdge = ProgressionEdge {
    from: MINOR_VDIM7,
    to: MINOR_FLAT_VI_6,
};

/// Progression edge: Vdim7 → bVI
pub static EDGE_MINOR_VDIM7_TO_MINOR_FLAT_VI_MAJ7: ProgressionEdge = ProgressionEdge {
    from: MINOR_VDIM7,
    to: MINOR_FLAT_VI_MAJ7,
};

/// Progression edge: Vdim7 → iv/b6
pub static EDGE_MINOR_VDIM7_TO_MINOR_IV_SLASH_FLAT_6: ProgressionEdge = ProgressionEdge {
    from: MINOR_VDIM7,
    to: MINOR_IV_SLASH_FLAT_6,
};

/// Progression edge: Vdim7 → iv/b6
pub static EDGE_MINOR_VDIM7_TO_MINOR_IV_SLASH_FLAT_6_M6: ProgressionEdge = ProgressionEdge {
    from: MINOR_VDIM7,
    to: MINOR_IV_SLASH_FLAT_6_M6,
};

/// Progression edge: Vdim7 → iv/b6
pub static EDGE_MINOR_VDIM7_TO_MINOR_IV_SLASH_FLAT_6_M7: ProgressionEdge = ProgressionEdge {
    from: MINOR_VDIM7,
    to: MINOR_IV_SLASH_FLAT_6_M7,
};

/// Progression edge: bIII → bVI
pub static EDGE_MINOR_FLAT_III_TO_MINOR_FLAT_VI: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_III,
    to: MINOR_FLAT_VI,
};

/// Progression edge: bIII → bVI
pub static EDGE_MINOR_FLAT_III_TO_MINOR_FLAT_VI_6: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_III,
    to: MINOR_FLAT_VI_6,
};

/// Progression edge: bIII → bVI
pub static EDGE_MINOR_FLAT_III_TO_MINOR_FLAT_VI_MAJ7: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_III,
    to: MINOR_FLAT_VI_MAJ7,
};

/// Progression edge: bIII → bVI
pub static EDGE_MINOR_FLAT_III_7_TO_MINOR_FLAT_VI: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_III_7,
    to: MINOR_FLAT_VI,
};

/// Progression edge: bIII → bVI
pub static EDGE_MINOR_FLAT_III_7_TO_MINOR_FLAT_VI_6: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_III_7,
    to: MINOR_FLAT_VI_6,
};

/// Progression edge: bIII → bVI
pub static EDGE_MINOR_FLAT_III_7_TO_MINOR_FLAT_VI_MAJ7: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_III_7,
    to: MINOR_FLAT_VI_MAJ7,
};

/// Progression edge: bIII → bVI
pub static EDGE_MINOR_FLAT_III_9_TO_MINOR_FLAT_VI: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_III_9,
    to: MINOR_FLAT_VI,
};

/// Progression edge: bIII → bVI
pub static EDGE_MINOR_FLAT_III_9_TO_MINOR_FLAT_VI_6: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_III_9,
    to: MINOR_FLAT_VI_6,
};

/// Progression edge: bIII → bVI
pub static EDGE_MINOR_FLAT_III_9_TO_MINOR_FLAT_VI_MAJ7: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_III_9,
    to: MINOR_FLAT_VI_MAJ7,
};

/// Progression edge: bIII → bVI
pub static EDGE_MINOR_FLAT_III_FLAT_9_TO_MINOR_FLAT_VI: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_III_FLAT_9,
    to: MINOR_FLAT_VI,
};

/// Progression edge: bIII → bVI
pub static EDGE_MINOR_FLAT_III_FLAT_9_TO_MINOR_FLAT_VI_6: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_III_FLAT_9,
    to: MINOR_FLAT_VI_6,
};

/// Progression edge: bIII → bVI
pub static EDGE_MINOR_FLAT_III_FLAT_9_TO_MINOR_FLAT_VI_MAJ7: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_III_FLAT_9,
    to: MINOR_FLAT_VI_MAJ7,
};

/// Progression edge: bIII → iv/b6
pub static EDGE_MINOR_FLAT_III_TO_MINOR_IV_SLASH_FLAT_6: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_III,
    to: MINOR_IV_SLASH_FLAT_6,
};

/// Progression edge: bIII → iv/b6
pub static EDGE_MINOR_FLAT_III_TO_MINOR_IV_SLASH_FLAT_6_M6: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_III,
    to: MINOR_IV_SLASH_FLAT_6_M6,
};

/// Progression edge: bIII → iv/b6
pub static EDGE_MINOR_FLAT_III_TO_MINOR_IV_SLASH_FLAT_6_M7: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_III,
    to: MINOR_IV_SLASH_FLAT_6_M7,
};

/// Progression edge: bIII → iv/b6
pub static EDGE_MINOR_FLAT_III_7_TO_MINOR_IV_SLASH_FLAT_6: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_III_7,
    to: MINOR_IV_SLASH_FLAT_6,
};

/// Progression edge: bIII → iv/b6
pub static EDGE_MINOR_FLAT_III_7_TO_MINOR_IV_SLASH_FLAT_6_M6: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_III_7,
    to: MINOR_IV_SLASH_FLAT_6_M6,
};

/// Progression edge: bIII → iv/b6
pub static EDGE_MINOR_FLAT_III_7_TO_MINOR_IV_SLASH_FLAT_6_M7: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_III_7,
    to: MINOR_IV_SLASH_FLAT_6_M7,
};

/// Progression edge: bIII → iv/b6
pub static EDGE_MINOR_FLAT_III_9_TO_MINOR_IV_SLASH_FLAT_6: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_III_9,
    to: MINOR_IV_SLASH_FLAT_6,
};

/// Progression edge: bIII → iv/b6
pub static EDGE_MINOR_FLAT_III_9_TO_MINOR_IV_SLASH_FLAT_6_M6: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_III_9,
    to: MINOR_IV_SLASH_FLAT_6_M6,
};

/// Progression edge: bIII → iv/b6
pub static EDGE_MINOR_FLAT_III_9_TO_MINOR_IV_SLASH_FLAT_6_M7: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_III_9,
    to: MINOR_IV_SLASH_FLAT_6_M7,
};

/// Progression edge: bIII → iv/b6
pub static EDGE_MINOR_FLAT_III_FLAT_9_TO_MINOR_IV_SLASH_FLAT_6: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_III_FLAT_9,
    to: MINOR_IV_SLASH_FLAT_6,
};

/// Progression edge: bIII → iv/b6
pub static EDGE_MINOR_FLAT_III_FLAT_9_TO_MINOR_IV_SLASH_FLAT_6_M6: ProgressionEdge =
    ProgressionEdge {
        from: MINOR_FLAT_III_FLAT_9,
        to: MINOR_IV_SLASH_FLAT_6_M6,
    };

/// Progression edge: bIII → iv/b6
pub static EDGE_MINOR_FLAT_III_FLAT_9_TO_MINOR_IV_SLASH_FLAT_6_M7: ProgressionEdge =
    ProgressionEdge {
        from: MINOR_FLAT_III_FLAT_9,
        to: MINOR_IV_SLASH_FLAT_6_M7,
    };

/// Progression edge: bviim7b5 → bIII
pub static EDGE_MINOR_FLAT_VIIM7FLAT_5_TO_MINOR_FLAT_III: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VIIM7FLAT_5,
    to: MINOR_FLAT_III,
};

/// Progression edge: bviim7b5 → bIII
pub static EDGE_MINOR_FLAT_VIIM7FLAT_5_TO_MINOR_FLAT_III_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VIIM7FLAT_5,
    to: MINOR_FLAT_III_7,
};

/// Progression edge: bviim7b5 → bIII
pub static EDGE_MINOR_FLAT_VIIM7FLAT_5_TO_MINOR_FLAT_III_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VIIM7FLAT_5,
    to: MINOR_FLAT_III_9,
};

/// Progression edge: bviim7b5 → bIII
pub static EDGE_MINOR_FLAT_VIIM7FLAT_5_TO_MINOR_FLAT_III_FLAT_9: ProgressionEdge =
    ProgressionEdge {
        from: MINOR_FLAT_VIIM7FLAT_5,
        to: MINOR_FLAT_III_FLAT_9,
    };

/// Progression edge: iidim → ivm7
pub static EDGE_MINOR_IIDIM_TO_MINOR_IVM7: ProgressionEdge = ProgressionEdge {
    from: MINOR_IIDIM,
    to: MINOR_IVM7,
};

/// Progression edge: viidim/2 → ivm7
pub static EDGE_MINOR_VIIDIM_SLASH_2_TO_MINOR_IVM7: ProgressionEdge = ProgressionEdge {
    from: MINOR_VIIDIM_SLASH_2,
    to: MINOR_IVM7,
};

/// Progression edge: ivm7 → i
pub static EDGE_MINOR_IVM7_TO_MINOR_I: ProgressionEdge = ProgressionEdge {
    from: MINOR_IVM7,
    to: MINOR_I,
};

/// Progression edge: ivm7 → i
pub static EDGE_MINOR_IVM7_TO_MINOR_I_SUS2: ProgressionEdge = ProgressionEdge {
    from: MINOR_IVM7,
    to: MINOR_I_SUS2,
};

/// Progression edge: ivm7 → i
pub static EDGE_MINOR_IVM7_TO_MINOR_I_SUS4: ProgressionEdge = ProgressionEdge {
    from: MINOR_IVM7,
    to: MINOR_I_SUS4,
};

/// Progression edge: bVI7 → i/5
pub static EDGE_MINOR_FLAT_VI7_TO_MINOR_I_SLASH_5: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VI7,
    to: MINOR_I_SLASH_5,
};

/// Progression edge: sivdim7 → i/5
pub static EDGE_MINOR_SIVDIM7_TO_MINOR_I_SLASH_5: ProgressionEdge = ProgressionEdge {
    from: MINOR_SIVDIM7,
    to: MINOR_I_SLASH_5,
};

/// Complete registry of all progression chords for minor keys
///
/// Contains 49 chord variants across all harmonic functions.
/// Used internally for graph traversal and chord lookup operations.
pub static ALL_NODES: &[&RomanChord] = &[
    &MINOR_IIDIM,
    &MINOR_VIIDIM_SLASH_2,
    &V,
    &V_7,
    &V_FLAT_9,
    &V_FLAT_13,
    &V_SUS4,
    &MINOR_FLAT_IIIAUG,
    &MINOR_I_SLASH_FLAT_3,
    &MINOR_FLAT_VI,
    &MINOR_FLAT_VI_6,
    &MINOR_FLAT_VI_MAJ7,
    &MINOR_IV_SLASH_FLAT_6,
    &MINOR_IV_SLASH_FLAT_6_M6,
    &MINOR_IV_SLASH_FLAT_6_M7,
    &MINOR_IV,
    &MINOR_IV_M6,
    &MINOR_IV_M7,
    &MINOR_IV_M9,
    &MINOR_FLAT_II_SLASH_4,
    &MINOR_I_SLASH_5,
    &MINOR_I,
    &MINOR_I_SUS2,
    &MINOR_I_SUS4,
    &MINOR_IV_SLASH_1,
    &V_SLASH_1,
    &MINOR_IIDIM7,
    &V7,
    &V7_FLAT_9,
    &MINOR_IIIDIM7,
    &MINOR_VDIM7,
    &MINOR_VM7FLAT_5,
    &I,
    &I_7,
    &I_FLAT_9,
    &MINOR_VIM7FLAT_5,
    &II,
    &II_7,
    &II_FLAT_9,
    &MINOR_SIVDIM7,
    &MINOR_FLAT_VII,
    &MINOR_FLAT_VII_9,
    &MINOR_FLAT_III,
    &MINOR_FLAT_III_7,
    &MINOR_FLAT_III_9,
    &MINOR_FLAT_III_FLAT_9,
    &MINOR_FLAT_VIIM7FLAT_5,
    &MINOR_IVM7,
    &MINOR_FLAT_VI7,
];

/// Complete registry of all progression edges for minor keys
///
/// Contains 361 harmonic connections between chord variants.
/// Each edge represents a musically valid progression with proper voice leading.
pub static ALL_EDGES: &[&ProgressionEdge] = &[
    &EDGE_MINOR_IIDIM_TO_MINOR_VIIDIM_SLASH_2,
    &EDGE_MINOR_IIDIM_TO_V,
    &EDGE_MINOR_IIDIM_TO_V_7,
    &EDGE_MINOR_IIDIM_TO_V_FLAT_9,
    &EDGE_MINOR_IIDIM_TO_V_FLAT_13,
    &EDGE_MINOR_IIDIM_TO_V_SUS4,
    &EDGE_MINOR_IIDIM_TO_MINOR_FLAT_IIIAUG,
    &EDGE_MINOR_IIDIM_TO_MINOR_I_SLASH_FLAT_3,
    &EDGE_MINOR_VIIDIM_SLASH_2_TO_MINOR_IIDIM,
    &EDGE_MINOR_VIIDIM_SLASH_2_TO_V,
    &EDGE_MINOR_VIIDIM_SLASH_2_TO_V_7,
    &EDGE_MINOR_VIIDIM_SLASH_2_TO_V_FLAT_9,
    &EDGE_MINOR_VIIDIM_SLASH_2_TO_V_FLAT_13,
    &EDGE_MINOR_VIIDIM_SLASH_2_TO_V_SUS4,
    &EDGE_MINOR_VIIDIM_SLASH_2_TO_MINOR_FLAT_IIIAUG,
    &EDGE_MINOR_VIIDIM_SLASH_2_TO_MINOR_I_SLASH_FLAT_3,
    &EDGE_V_TO_MINOR_FLAT_IIIAUG,
    &EDGE_V_7_TO_MINOR_FLAT_IIIAUG,
    &EDGE_V_FLAT_9_TO_MINOR_FLAT_IIIAUG,
    &EDGE_V_FLAT_13_TO_MINOR_FLAT_IIIAUG,
    &EDGE_V_SUS4_TO_MINOR_FLAT_IIIAUG,
    &EDGE_V_TO_MINOR_I_SLASH_FLAT_3,
    &EDGE_V_7_TO_MINOR_I_SLASH_FLAT_3,
    &EDGE_V_FLAT_9_TO_MINOR_I_SLASH_FLAT_3,
    &EDGE_V_FLAT_13_TO_MINOR_I_SLASH_FLAT_3,
    &EDGE_V_SUS4_TO_MINOR_I_SLASH_FLAT_3,
    &EDGE_V_TO_MINOR_FLAT_VI,
    &EDGE_V_TO_MINOR_FLAT_VI_6,
    &EDGE_V_TO_MINOR_FLAT_VI_MAJ7,
    &EDGE_V_7_TO_MINOR_FLAT_VI,
    &EDGE_V_7_TO_MINOR_FLAT_VI_6,
    &EDGE_V_7_TO_MINOR_FLAT_VI_MAJ7,
    &EDGE_V_FLAT_9_TO_MINOR_FLAT_VI,
    &EDGE_V_FLAT_9_TO_MINOR_FLAT_VI_6,
    &EDGE_V_FLAT_9_TO_MINOR_FLAT_VI_MAJ7,
    &EDGE_V_FLAT_13_TO_MINOR_FLAT_VI,
    &EDGE_V_FLAT_13_TO_MINOR_FLAT_VI_6,
    &EDGE_V_FLAT_13_TO_MINOR_FLAT_VI_MAJ7,
    &EDGE_V_SUS4_TO_MINOR_FLAT_VI,
    &EDGE_V_SUS4_TO_MINOR_FLAT_VI_6,
    &EDGE_V_SUS4_TO_MINOR_FLAT_VI_MAJ7,
    &EDGE_V_TO_MINOR_IV_SLASH_FLAT_6,
    &EDGE_V_TO_MINOR_IV_SLASH_FLAT_6_M6,
    &EDGE_V_TO_MINOR_IV_SLASH_FLAT_6_M7,
    &EDGE_V_7_TO_MINOR_IV_SLASH_FLAT_6,
    &EDGE_V_7_TO_MINOR_IV_SLASH_FLAT_6_M6,
    &EDGE_V_7_TO_MINOR_IV_SLASH_FLAT_6_M7,
    &EDGE_V_FLAT_9_TO_MINOR_IV_SLASH_FLAT_6,
    &EDGE_V_FLAT_9_TO_MINOR_IV_SLASH_FLAT_6_M6,
    &EDGE_V_FLAT_9_TO_MINOR_IV_SLASH_FLAT_6_M7,
    &EDGE_V_FLAT_13_TO_MINOR_IV_SLASH_FLAT_6,
    &EDGE_V_FLAT_13_TO_MINOR_IV_SLASH_FLAT_6_M6,
    &EDGE_V_FLAT_13_TO_MINOR_IV_SLASH_FLAT_6_M7,
    &EDGE_V_SUS4_TO_MINOR_IV_SLASH_FLAT_6,
    &EDGE_V_SUS4_TO_MINOR_IV_SLASH_FLAT_6_M6,
    &EDGE_V_SUS4_TO_MINOR_IV_SLASH_FLAT_6_M7,
    &EDGE_MINOR_FLAT_IIIAUG_TO_MINOR_I_SLASH_FLAT_3,
    &EDGE_MINOR_FLAT_IIIAUG_TO_MINOR_I,
    &EDGE_MINOR_FLAT_IIIAUG_TO_MINOR_I_SUS2,
    &EDGE_MINOR_FLAT_IIIAUG_TO_MINOR_I_SUS4,
    &EDGE_MINOR_FLAT_IIIAUG_TO_MINOR_IV,
    &EDGE_MINOR_FLAT_IIIAUG_TO_MINOR_IV_M6,
    &EDGE_MINOR_FLAT_IIIAUG_TO_MINOR_IV_M7,
    &EDGE_MINOR_FLAT_IIIAUG_TO_MINOR_IV_M9,
    &EDGE_MINOR_FLAT_IIIAUG_TO_MINOR_FLAT_II_SLASH_4,
    &EDGE_MINOR_FLAT_IIIAUG_TO_MINOR_FLAT_VI,
    &EDGE_MINOR_FLAT_IIIAUG_TO_MINOR_FLAT_VI_6,
    &EDGE_MINOR_FLAT_IIIAUG_TO_MINOR_FLAT_VI_MAJ7,
    &EDGE_MINOR_FLAT_IIIAUG_TO_MINOR_IV_SLASH_FLAT_6,
    &EDGE_MINOR_FLAT_IIIAUG_TO_MINOR_IV_SLASH_FLAT_6_M6,
    &EDGE_MINOR_FLAT_IIIAUG_TO_MINOR_IV_SLASH_FLAT_6_M7,
    &EDGE_MINOR_I_SLASH_FLAT_3_TO_MINOR_FLAT_IIIAUG,
    &EDGE_MINOR_I_SLASH_FLAT_3_TO_MINOR_I,
    &EDGE_MINOR_I_SLASH_FLAT_3_TO_MINOR_I_SUS2,
    &EDGE_MINOR_I_SLASH_FLAT_3_TO_MINOR_I_SUS4,
    &EDGE_MINOR_I_SLASH_FLAT_3_TO_MINOR_IV,
    &EDGE_MINOR_I_SLASH_FLAT_3_TO_MINOR_IV_M6,
    &EDGE_MINOR_I_SLASH_FLAT_3_TO_MINOR_IV_M7,
    &EDGE_MINOR_I_SLASH_FLAT_3_TO_MINOR_IV_M9,
    &EDGE_MINOR_I_SLASH_FLAT_3_TO_MINOR_FLAT_II_SLASH_4,
    &EDGE_MINOR_I_SLASH_FLAT_3_TO_MINOR_FLAT_VI,
    &EDGE_MINOR_I_SLASH_FLAT_3_TO_MINOR_FLAT_VI_6,
    &EDGE_MINOR_I_SLASH_FLAT_3_TO_MINOR_FLAT_VI_MAJ7,
    &EDGE_MINOR_I_SLASH_FLAT_3_TO_MINOR_IV_SLASH_FLAT_6,
    &EDGE_MINOR_I_SLASH_FLAT_3_TO_MINOR_IV_SLASH_FLAT_6_M6,
    &EDGE_MINOR_I_SLASH_FLAT_3_TO_MINOR_IV_SLASH_FLAT_6_M7,
    &EDGE_MINOR_FLAT_VI_TO_MINOR_IV_SLASH_FLAT_6,
    &EDGE_MINOR_FLAT_VI_TO_MINOR_IV_SLASH_FLAT_6_M6,
    &EDGE_MINOR_FLAT_VI_TO_MINOR_IV_SLASH_FLAT_6_M7,
    &EDGE_MINOR_FLAT_VI_6_TO_MINOR_IV_SLASH_FLAT_6,
    &EDGE_MINOR_FLAT_VI_6_TO_MINOR_IV_SLASH_FLAT_6_M6,
    &EDGE_MINOR_FLAT_VI_6_TO_MINOR_IV_SLASH_FLAT_6_M7,
    &EDGE_MINOR_FLAT_VI_MAJ7_TO_MINOR_IV_SLASH_FLAT_6,
    &EDGE_MINOR_FLAT_VI_MAJ7_TO_MINOR_IV_SLASH_FLAT_6_M6,
    &EDGE_MINOR_FLAT_VI_MAJ7_TO_MINOR_IV_SLASH_FLAT_6_M7,
    &EDGE_MINOR_FLAT_VI_TO_MINOR_IV,
    &EDGE_MINOR_FLAT_VI_TO_MINOR_IV_M6,
    &EDGE_MINOR_FLAT_VI_TO_MINOR_IV_M7,
    &EDGE_MINOR_FLAT_VI_TO_MINOR_IV_M9,
    &EDGE_MINOR_FLAT_VI_6_TO_MINOR_IV,
    &EDGE_MINOR_FLAT_VI_6_TO_MINOR_IV_M6,
    &EDGE_MINOR_FLAT_VI_6_TO_MINOR_IV_M7,
    &EDGE_MINOR_FLAT_VI_6_TO_MINOR_IV_M9,
    &EDGE_MINOR_FLAT_VI_MAJ7_TO_MINOR_IV,
    &EDGE_MINOR_FLAT_VI_MAJ7_TO_MINOR_IV_M6,
    &EDGE_MINOR_FLAT_VI_MAJ7_TO_MINOR_IV_M7,
    &EDGE_MINOR_FLAT_VI_MAJ7_TO_MINOR_IV_M9,
    &EDGE_MINOR_FLAT_VI_TO_MINOR_FLAT_II_SLASH_4,
    &EDGE_MINOR_FLAT_VI_6_TO_MINOR_FLAT_II_SLASH_4,
    &EDGE_MINOR_FLAT_VI_MAJ7_TO_MINOR_FLAT_II_SLASH_4,
    &EDGE_MINOR_FLAT_VI_TO_MINOR_IIDIM,
    &EDGE_MINOR_FLAT_VI_6_TO_MINOR_IIDIM,
    &EDGE_MINOR_FLAT_VI_MAJ7_TO_MINOR_IIDIM,
    &EDGE_MINOR_FLAT_VI_TO_MINOR_VIIDIM_SLASH_2,
    &EDGE_MINOR_FLAT_VI_6_TO_MINOR_VIIDIM_SLASH_2,
    &EDGE_MINOR_FLAT_VI_MAJ7_TO_MINOR_VIIDIM_SLASH_2,
    &EDGE_MINOR_IV_SLASH_FLAT_6_TO_MINOR_FLAT_VI,
    &EDGE_MINOR_IV_SLASH_FLAT_6_TO_MINOR_FLAT_VI_6,
    &EDGE_MINOR_IV_SLASH_FLAT_6_TO_MINOR_FLAT_VI_MAJ7,
    &EDGE_MINOR_IV_SLASH_FLAT_6_M6_TO_MINOR_FLAT_VI,
    &EDGE_MINOR_IV_SLASH_FLAT_6_M6_TO_MINOR_FLAT_VI_6,
    &EDGE_MINOR_IV_SLASH_FLAT_6_M6_TO_MINOR_FLAT_VI_MAJ7,
    &EDGE_MINOR_IV_SLASH_FLAT_6_M7_TO_MINOR_FLAT_VI,
    &EDGE_MINOR_IV_SLASH_FLAT_6_M7_TO_MINOR_FLAT_VI_6,
    &EDGE_MINOR_IV_SLASH_FLAT_6_M7_TO_MINOR_FLAT_VI_MAJ7,
    &EDGE_MINOR_IV_SLASH_FLAT_6_TO_MINOR_IV,
    &EDGE_MINOR_IV_SLASH_FLAT_6_TO_MINOR_IV_M6,
    &EDGE_MINOR_IV_SLASH_FLAT_6_TO_MINOR_IV_M7,
    &EDGE_MINOR_IV_SLASH_FLAT_6_TO_MINOR_IV_M9,
    &EDGE_MINOR_IV_SLASH_FLAT_6_M6_TO_MINOR_IV,
    &EDGE_MINOR_IV_SLASH_FLAT_6_M6_TO_MINOR_IV_M6,
    &EDGE_MINOR_IV_SLASH_FLAT_6_M6_TO_MINOR_IV_M7,
    &EDGE_MINOR_IV_SLASH_FLAT_6_M6_TO_MINOR_IV_M9,
    &EDGE_MINOR_IV_SLASH_FLAT_6_M7_TO_MINOR_IV,
    &EDGE_MINOR_IV_SLASH_FLAT_6_M7_TO_MINOR_IV_M6,
    &EDGE_MINOR_IV_SLASH_FLAT_6_M7_TO_MINOR_IV_M7,
    &EDGE_MINOR_IV_SLASH_FLAT_6_M7_TO_MINOR_IV_M9,
    &EDGE_MINOR_IV_SLASH_FLAT_6_TO_MINOR_FLAT_II_SLASH_4,
    &EDGE_MINOR_IV_SLASH_FLAT_6_M6_TO_MINOR_FLAT_II_SLASH_4,
    &EDGE_MINOR_IV_SLASH_FLAT_6_M7_TO_MINOR_FLAT_II_SLASH_4,
    &EDGE_MINOR_IV_SLASH_FLAT_6_TO_MINOR_IIDIM,
    &EDGE_MINOR_IV_SLASH_FLAT_6_M6_TO_MINOR_IIDIM,
    &EDGE_MINOR_IV_SLASH_FLAT_6_M7_TO_MINOR_IIDIM,
    &EDGE_MINOR_IV_SLASH_FLAT_6_TO_MINOR_VIIDIM_SLASH_2,
    &EDGE_MINOR_IV_SLASH_FLAT_6_M6_TO_MINOR_VIIDIM_SLASH_2,
    &EDGE_MINOR_IV_SLASH_FLAT_6_M7_TO_MINOR_VIIDIM_SLASH_2,
    &EDGE_MINOR_IV_TO_MINOR_FLAT_II_SLASH_4,
    &EDGE_MINOR_IV_M6_TO_MINOR_FLAT_II_SLASH_4,
    &EDGE_MINOR_IV_M7_TO_MINOR_FLAT_II_SLASH_4,
    &EDGE_MINOR_IV_M9_TO_MINOR_FLAT_II_SLASH_4,
    &EDGE_MINOR_IV_TO_MINOR_I,
    &EDGE_MINOR_IV_TO_MINOR_I_SUS2,
    &EDGE_MINOR_IV_TO_MINOR_I_SUS4,
    &EDGE_MINOR_IV_M6_TO_MINOR_I,
    &EDGE_MINOR_IV_M6_TO_MINOR_I_SUS2,
    &EDGE_MINOR_IV_M6_TO_MINOR_I_SUS4,
    &EDGE_MINOR_IV_M7_TO_MINOR_I,
    &EDGE_MINOR_IV_M7_TO_MINOR_I_SUS2,
    &EDGE_MINOR_IV_M7_TO_MINOR_I_SUS4,
    &EDGE_MINOR_IV_M9_TO_MINOR_I,
    &EDGE_MINOR_IV_M9_TO_MINOR_I_SUS2,
    &EDGE_MINOR_IV_M9_TO_MINOR_I_SUS4,
    &EDGE_MINOR_IV_TO_V,
    &EDGE_MINOR_IV_TO_V_7,
    &EDGE_MINOR_IV_TO_V_FLAT_9,
    &EDGE_MINOR_IV_TO_V_FLAT_13,
    &EDGE_MINOR_IV_TO_V_SUS4,
    &EDGE_MINOR_IV_M6_TO_V,
    &EDGE_MINOR_IV_M6_TO_V_7,
    &EDGE_MINOR_IV_M6_TO_V_FLAT_9,
    &EDGE_MINOR_IV_M6_TO_V_FLAT_13,
    &EDGE_MINOR_IV_M6_TO_V_SUS4,
    &EDGE_MINOR_IV_M7_TO_V,
    &EDGE_MINOR_IV_M7_TO_V_7,
    &EDGE_MINOR_IV_M7_TO_V_FLAT_9,
    &EDGE_MINOR_IV_M7_TO_V_FLAT_13,
    &EDGE_MINOR_IV_M7_TO_V_SUS4,
    &EDGE_MINOR_IV_M9_TO_V,
    &EDGE_MINOR_IV_M9_TO_V_7,
    &EDGE_MINOR_IV_M9_TO_V_FLAT_9,
    &EDGE_MINOR_IV_M9_TO_V_FLAT_13,
    &EDGE_MINOR_IV_M9_TO_V_SUS4,
    &EDGE_MINOR_IV_TO_MINOR_I_SLASH_5,
    &EDGE_MINOR_IV_M6_TO_MINOR_I_SLASH_5,
    &EDGE_MINOR_IV_M7_TO_MINOR_I_SLASH_5,
    &EDGE_MINOR_IV_M9_TO_MINOR_I_SLASH_5,
    &EDGE_MINOR_IV_TO_MINOR_IIDIM,
    &EDGE_MINOR_IV_M6_TO_MINOR_IIDIM,
    &EDGE_MINOR_IV_M7_TO_MINOR_IIDIM,
    &EDGE_MINOR_IV_M9_TO_MINOR_IIDIM,
    &EDGE_MINOR_IV_TO_MINOR_VIIDIM_SLASH_2,
    &EDGE_MINOR_IV_M6_TO_MINOR_VIIDIM_SLASH_2,
    &EDGE_MINOR_IV_M7_TO_MINOR_VIIDIM_SLASH_2,
    &EDGE_MINOR_IV_M9_TO_MINOR_VIIDIM_SLASH_2,
    &EDGE_MINOR_IV_TO_MINOR_I_SLASH_FLAT_3,
    &EDGE_MINOR_IV_M6_TO_MINOR_I_SLASH_FLAT_3,
    &EDGE_MINOR_IV_M7_TO_MINOR_I_SLASH_FLAT_3,
    &EDGE_MINOR_IV_M9_TO_MINOR_I_SLASH_FLAT_3,
    &EDGE_MINOR_FLAT_II_SLASH_4_TO_MINOR_IV,
    &EDGE_MINOR_FLAT_II_SLASH_4_TO_MINOR_IV_M6,
    &EDGE_MINOR_FLAT_II_SLASH_4_TO_MINOR_IV_M7,
    &EDGE_MINOR_FLAT_II_SLASH_4_TO_MINOR_IV_M9,
    &EDGE_MINOR_FLAT_II_SLASH_4_TO_MINOR_I,
    &EDGE_MINOR_FLAT_II_SLASH_4_TO_MINOR_I_SUS2,
    &EDGE_MINOR_FLAT_II_SLASH_4_TO_MINOR_I_SUS4,
    &EDGE_MINOR_FLAT_II_SLASH_4_TO_V,
    &EDGE_MINOR_FLAT_II_SLASH_4_TO_V_7,
    &EDGE_MINOR_FLAT_II_SLASH_4_TO_V_FLAT_9,
    &EDGE_MINOR_FLAT_II_SLASH_4_TO_V_FLAT_13,
    &EDGE_MINOR_FLAT_II_SLASH_4_TO_V_SUS4,
    &EDGE_MINOR_FLAT_II_SLASH_4_TO_MINOR_I_SLASH_5,
    &EDGE_MINOR_FLAT_II_SLASH_4_TO_MINOR_IIDIM,
    &EDGE_MINOR_FLAT_II_SLASH_4_TO_MINOR_VIIDIM_SLASH_2,
    &EDGE_MINOR_FLAT_II_SLASH_4_TO_MINOR_I_SLASH_FLAT_3,
    &EDGE_MINOR_I_SLASH_FLAT_3_TO_MINOR_IIDIM,
    &EDGE_MINOR_I_SLASH_FLAT_3_TO_MINOR_VIIDIM_SLASH_2,
    &EDGE_MINOR_IIDIM_TO_MINOR_I_SLASH_5,
    &EDGE_MINOR_VIIDIM_SLASH_2_TO_MINOR_I_SLASH_5,
    &EDGE_V_TO_MINOR_I,
    &EDGE_V_TO_MINOR_I_SUS2,
    &EDGE_V_TO_MINOR_I_SUS4,
    &EDGE_V_7_TO_MINOR_I,
    &EDGE_V_7_TO_MINOR_I_SUS2,
    &EDGE_V_7_TO_MINOR_I_SUS4,
    &EDGE_V_FLAT_9_TO_MINOR_I,
    &EDGE_V_FLAT_9_TO_MINOR_I_SUS2,
    &EDGE_V_FLAT_9_TO_MINOR_I_SUS4,
    &EDGE_V_FLAT_13_TO_MINOR_I,
    &EDGE_V_FLAT_13_TO_MINOR_I_SUS2,
    &EDGE_V_FLAT_13_TO_MINOR_I_SUS4,
    &EDGE_V_SUS4_TO_MINOR_I,
    &EDGE_V_SUS4_TO_MINOR_I_SUS2,
    &EDGE_V_SUS4_TO_MINOR_I_SUS4,
    &EDGE_MINOR_I_TO_V_SLASH_1,
    &EDGE_MINOR_I_SUS2_TO_V_SLASH_1,
    &EDGE_MINOR_I_SUS4_TO_V_SLASH_1,
    &EDGE_MINOR_I_TO_MINOR_IV_SLASH_1,
    &EDGE_MINOR_I_SUS2_TO_MINOR_IV_SLASH_1,
    &EDGE_MINOR_I_SUS4_TO_MINOR_IV_SLASH_1,
    &EDGE_V_SLASH_1_TO_MINOR_I,
    &EDGE_V_SLASH_1_TO_MINOR_I_SUS2,
    &EDGE_V_SLASH_1_TO_MINOR_I_SUS4,
    &EDGE_V_SLASH_1_TO_MINOR_IV_SLASH_1,
    &EDGE_MINOR_IV_SLASH_1_TO_MINOR_I,
    &EDGE_MINOR_IV_SLASH_1_TO_MINOR_I_SUS2,
    &EDGE_MINOR_IV_SLASH_1_TO_MINOR_I_SUS4,
    &EDGE_MINOR_IV_SLASH_1_TO_V_SLASH_1,
    &EDGE_MINOR_IIDIM7_TO_MINOR_FLAT_IIIAUG,
    &EDGE_MINOR_IIDIM7_TO_MINOR_I_SLASH_FLAT_3,
    &EDGE_V7_TO_MINOR_FLAT_IIIAUG,
    &EDGE_V7_FLAT_9_TO_MINOR_FLAT_IIIAUG,
    &EDGE_V7_TO_MINOR_I_SLASH_FLAT_3,
    &EDGE_V7_FLAT_9_TO_MINOR_I_SLASH_FLAT_3,
    &EDGE_MINOR_IIIDIM7_TO_MINOR_IV,
    &EDGE_MINOR_IIIDIM7_TO_MINOR_IV_M6,
    &EDGE_MINOR_IIIDIM7_TO_MINOR_IV_M7,
    &EDGE_MINOR_IIIDIM7_TO_MINOR_IV_M9,
    &EDGE_MINOR_IIIDIM7_TO_MINOR_FLAT_II_SLASH_4,
    &EDGE_MINOR_VDIM7_TO_I,
    &EDGE_MINOR_VDIM7_TO_I_7,
    &EDGE_MINOR_VDIM7_TO_I_FLAT_9,
    &EDGE_MINOR_VM7FLAT_5_TO_I,
    &EDGE_MINOR_VM7FLAT_5_TO_I_7,
    &EDGE_MINOR_VM7FLAT_5_TO_I_FLAT_9,
    &EDGE_I_TO_MINOR_IV,
    &EDGE_I_TO_MINOR_IV_M6,
    &EDGE_I_TO_MINOR_IV_M7,
    &EDGE_I_TO_MINOR_IV_M9,
    &EDGE_I_7_TO_MINOR_IV,
    &EDGE_I_7_TO_MINOR_IV_M6,
    &EDGE_I_7_TO_MINOR_IV_M7,
    &EDGE_I_7_TO_MINOR_IV_M9,
    &EDGE_I_FLAT_9_TO_MINOR_IV,
    &EDGE_I_FLAT_9_TO_MINOR_IV_M6,
    &EDGE_I_FLAT_9_TO_MINOR_IV_M7,
    &EDGE_I_FLAT_9_TO_MINOR_IV_M9,
    &EDGE_I_TO_MINOR_FLAT_II_SLASH_4,
    &EDGE_I_7_TO_MINOR_FLAT_II_SLASH_4,
    &EDGE_I_FLAT_9_TO_MINOR_FLAT_II_SLASH_4,
    &EDGE_MINOR_VIM7FLAT_5_TO_II,
    &EDGE_MINOR_VIM7FLAT_5_TO_II_7,
    &EDGE_MINOR_VIM7FLAT_5_TO_II_FLAT_9,
    &EDGE_II_TO_V,
    &EDGE_II_TO_V_7,
    &EDGE_II_TO_V_FLAT_9,
    &EDGE_II_TO_V_FLAT_13,
    &EDGE_II_TO_V_SUS4,
    &EDGE_II_7_TO_V,
    &EDGE_II_7_TO_V_7,
    &EDGE_II_7_TO_V_FLAT_9,
    &EDGE_II_7_TO_V_FLAT_13,
    &EDGE_II_7_TO_V_SUS4,
    &EDGE_II_FLAT_9_TO_V,
    &EDGE_II_FLAT_9_TO_V_7,
    &EDGE_II_FLAT_9_TO_V_FLAT_9,
    &EDGE_II_FLAT_9_TO_V_FLAT_13,
    &EDGE_II_FLAT_9_TO_V_SUS4,
    &EDGE_MINOR_SIVDIM7_TO_V,
    &EDGE_MINOR_SIVDIM7_TO_V_7,
    &EDGE_MINOR_SIVDIM7_TO_V_FLAT_9,
    &EDGE_MINOR_SIVDIM7_TO_V_FLAT_13,
    &EDGE_MINOR_SIVDIM7_TO_V_SUS4,
    &EDGE_MINOR_FLAT_VI_TO_MINOR_FLAT_VII,
    &EDGE_MINOR_FLAT_VI_TO_MINOR_FLAT_VII_9,
    &EDGE_MINOR_FLAT_VI_6_TO_MINOR_FLAT_VII,
    &EDGE_MINOR_FLAT_VI_6_TO_MINOR_FLAT_VII_9,
    &EDGE_MINOR_FLAT_VI_MAJ7_TO_MINOR_FLAT_VII,
    &EDGE_MINOR_FLAT_VI_MAJ7_TO_MINOR_FLAT_VII_9,
    &EDGE_MINOR_FLAT_VII_TO_MINOR_I,
    &EDGE_MINOR_FLAT_VII_TO_MINOR_I_SUS2,
    &EDGE_MINOR_FLAT_VII_TO_MINOR_I_SUS4,
    &EDGE_MINOR_FLAT_VII_9_TO_MINOR_I,
    &EDGE_MINOR_FLAT_VII_9_TO_MINOR_I_SUS2,
    &EDGE_MINOR_FLAT_VII_9_TO_MINOR_I_SUS4,
    &EDGE_MINOR_VM7FLAT_5_TO_MINOR_FLAT_VI,
    &EDGE_MINOR_VM7FLAT_5_TO_MINOR_FLAT_VI_6,
    &EDGE_MINOR_VM7FLAT_5_TO_MINOR_FLAT_VI_MAJ7,
    &EDGE_MINOR_VM7FLAT_5_TO_MINOR_IV_SLASH_FLAT_6,
    &EDGE_MINOR_VM7FLAT_5_TO_MINOR_IV_SLASH_FLAT_6_M6,
    &EDGE_MINOR_VM7FLAT_5_TO_MINOR_IV_SLASH_FLAT_6_M7,
    &EDGE_MINOR_VDIM7_TO_MINOR_FLAT_VI,
    &EDGE_MINOR_VDIM7_TO_MINOR_FLAT_VI_6,
    &EDGE_MINOR_VDIM7_TO_MINOR_FLAT_VI_MAJ7,
    &EDGE_MINOR_VDIM7_TO_MINOR_IV_SLASH_FLAT_6,
    &EDGE_MINOR_VDIM7_TO_MINOR_IV_SLASH_FLAT_6_M6,
    &EDGE_MINOR_VDIM7_TO_MINOR_IV_SLASH_FLAT_6_M7,
    &EDGE_MINOR_FLAT_III_TO_MINOR_FLAT_VI,
    &EDGE_MINOR_FLAT_III_TO_MINOR_FLAT_VI_6,
    &EDGE_MINOR_FLAT_III_TO_MINOR_FLAT_VI_MAJ7,
    &EDGE_MINOR_FLAT_III_7_TO_MINOR_FLAT_VI,
    &EDGE_MINOR_FLAT_III_7_TO_MINOR_FLAT_VI_6,
    &EDGE_MINOR_FLAT_III_7_TO_MINOR_FLAT_VI_MAJ7,
    &EDGE_MINOR_FLAT_III_9_TO_MINOR_FLAT_VI,
    &EDGE_MINOR_FLAT_III_9_TO_MINOR_FLAT_VI_6,
    &EDGE_MINOR_FLAT_III_9_TO_MINOR_FLAT_VI_MAJ7,
    &EDGE_MINOR_FLAT_III_FLAT_9_TO_MINOR_FLAT_VI,
    &EDGE_MINOR_FLAT_III_FLAT_9_TO_MINOR_FLAT_VI_6,
    &EDGE_MINOR_FLAT_III_FLAT_9_TO_MINOR_FLAT_VI_MAJ7,
    &EDGE_MINOR_FLAT_III_TO_MINOR_IV_SLASH_FLAT_6,
    &EDGE_MINOR_FLAT_III_TO_MINOR_IV_SLASH_FLAT_6_M6,
    &EDGE_MINOR_FLAT_III_TO_MINOR_IV_SLASH_FLAT_6_M7,
    &EDGE_MINOR_FLAT_III_7_TO_MINOR_IV_SLASH_FLAT_6,
    &EDGE_MINOR_FLAT_III_7_TO_MINOR_IV_SLASH_FLAT_6_M6,
    &EDGE_MINOR_FLAT_III_7_TO_MINOR_IV_SLASH_FLAT_6_M7,
    &EDGE_MINOR_FLAT_III_9_TO_MINOR_IV_SLASH_FLAT_6,
    &EDGE_MINOR_FLAT_III_9_TO_MINOR_IV_SLASH_FLAT_6_M6,
    &EDGE_MINOR_FLAT_III_9_TO_MINOR_IV_SLASH_FLAT_6_M7,
    &EDGE_MINOR_FLAT_III_FLAT_9_TO_MINOR_IV_SLASH_FLAT_6,
    &EDGE_MINOR_FLAT_III_FLAT_9_TO_MINOR_IV_SLASH_FLAT_6_M6,
    &EDGE_MINOR_FLAT_III_FLAT_9_TO_MINOR_IV_SLASH_FLAT_6_M7,
    &EDGE_MINOR_FLAT_VIIM7FLAT_5_TO_MINOR_FLAT_III,
    &EDGE_MINOR_FLAT_VIIM7FLAT_5_TO_MINOR_FLAT_III_7,
    &EDGE_MINOR_FLAT_VIIM7FLAT_5_TO_MINOR_FLAT_III_9,
    &EDGE_MINOR_FLAT_VIIM7FLAT_5_TO_MINOR_FLAT_III_FLAT_9,
    &EDGE_MINOR_IIDIM_TO_MINOR_IVM7,
    &EDGE_MINOR_VIIDIM_SLASH_2_TO_MINOR_IVM7,
    &EDGE_MINOR_IVM7_TO_MINOR_I,
    &EDGE_MINOR_IVM7_TO_MINOR_I_SUS2,
    &EDGE_MINOR_IVM7_TO_MINOR_I_SUS4,
    &EDGE_MINOR_FLAT_VI7_TO_MINOR_I_SLASH_5,
    &EDGE_MINOR_SIVDIM7_TO_MINOR_I_SLASH_5,
];

/// NodeType mapping for all progression chords in minor keys
///
/// Maps each chord to its harmonic role (Primary = stable, Secondary = transitional).
pub fn get_node_types() -> HashMap<&'static RomanChord, NodeType> {
    let mut map = HashMap::new();
    map.insert(&MINOR_IIDIM, NodeType::Primary);
    map.insert(&MINOR_VIIDIM_SLASH_2, NodeType::Primary);
    map.insert(&V, NodeType::Primary);
    map.insert(&V_7, NodeType::Primary);
    map.insert(&V_FLAT_9, NodeType::Primary);
    map.insert(&V_FLAT_13, NodeType::Primary);
    map.insert(&V_SUS4, NodeType::Primary);
    map.insert(&MINOR_FLAT_IIIAUG, NodeType::Primary);
    map.insert(&MINOR_I_SLASH_FLAT_3, NodeType::Primary);
    map.insert(&MINOR_FLAT_VI, NodeType::Primary);
    map.insert(&MINOR_FLAT_VI_6, NodeType::Primary);
    map.insert(&MINOR_FLAT_VI_MAJ7, NodeType::Primary);
    map.insert(&MINOR_IV_SLASH_FLAT_6, NodeType::Primary);
    map.insert(&MINOR_IV_SLASH_FLAT_6_M6, NodeType::Primary);
    map.insert(&MINOR_IV_SLASH_FLAT_6_M7, NodeType::Primary);
    map.insert(&MINOR_IV, NodeType::Primary);
    map.insert(&MINOR_IV_M6, NodeType::Primary);
    map.insert(&MINOR_IV_M7, NodeType::Primary);
    map.insert(&MINOR_IV_M9, NodeType::Primary);
    map.insert(&MINOR_FLAT_II_SLASH_4, NodeType::Primary);
    map.insert(&MINOR_I_SLASH_5, NodeType::Primary);
    map.insert(&MINOR_I, NodeType::Primary);
    map.insert(&MINOR_I_SUS2, NodeType::Primary);
    map.insert(&MINOR_I_SUS4, NodeType::Primary);
    map.insert(&MINOR_IV_SLASH_1, NodeType::Primary);
    map.insert(&V_SLASH_1, NodeType::Primary);
    map.insert(&MINOR_IIDIM7, NodeType::Secondary);
    map.insert(&V7, NodeType::Secondary);
    map.insert(&V7_FLAT_9, NodeType::Secondary);
    map.insert(&MINOR_IIIDIM7, NodeType::Secondary);
    map.insert(&MINOR_VDIM7, NodeType::Secondary);
    map.insert(&MINOR_VM7FLAT_5, NodeType::Secondary);
    map.insert(&I, NodeType::Secondary);
    map.insert(&I_7, NodeType::Secondary);
    map.insert(&I_FLAT_9, NodeType::Secondary);
    map.insert(&MINOR_VIM7FLAT_5, NodeType::Secondary);
    map.insert(&II, NodeType::Secondary);
    map.insert(&II_7, NodeType::Secondary);
    map.insert(&II_FLAT_9, NodeType::Secondary);
    map.insert(&MINOR_SIVDIM7, NodeType::Secondary);
    map.insert(&MINOR_FLAT_VII, NodeType::Secondary);
    map.insert(&MINOR_FLAT_VII_9, NodeType::Secondary);
    map.insert(&MINOR_FLAT_III, NodeType::Secondary);
    map.insert(&MINOR_FLAT_III_7, NodeType::Secondary);
    map.insert(&MINOR_FLAT_III_9, NodeType::Secondary);
    map.insert(&MINOR_FLAT_III_FLAT_9, NodeType::Secondary);
    map.insert(&MINOR_FLAT_VIIM7FLAT_5, NodeType::Secondary);
    map.insert(&MINOR_IVM7, NodeType::Secondary);
    map.insert(&MINOR_FLAT_VI7, NodeType::Secondary);
    map
}
