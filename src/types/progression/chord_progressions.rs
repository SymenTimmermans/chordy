//! Curated chord progression database generated from chord_progressions.csv
//! Do not edit manually.

#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use crate::types::{RomanChord, RomanNumeral, RomanDegree, Accidental, Interval, IntervalSet};

// Standard chord constants (major triads)
static I_CHORD: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::I, Accidental::Natural),
    intervals: IntervalSet::const_from_array(
        [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH,
         Interval::NONE, Interval::NONE, Interval::NONE,
         Interval::NONE, Interval::NONE, Interval::NONE,
         Interval::NONE], 3),
    bass: None,
};

static II_CHORD: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::II, Accidental::Natural),
    intervals: IntervalSet::const_from_array(
        [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH,
         Interval::NONE, Interval::NONE, Interval::NONE,
         Interval::NONE, Interval::NONE, Interval::NONE,
         Interval::NONE], 3),
    bass: None,
};

static III_CHORD: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::III, Accidental::Natural),
    intervals: IntervalSet::const_from_array(
        [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH,
         Interval::NONE, Interval::NONE, Interval::NONE,
         Interval::NONE, Interval::NONE, Interval::NONE,
         Interval::NONE], 3),
    bass: None,
};

static IV_CHORD: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::IV, Accidental::Natural),
    intervals: IntervalSet::const_from_array(
        [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH,
         Interval::NONE, Interval::NONE, Interval::NONE,
         Interval::NONE, Interval::NONE, Interval::NONE,
         Interval::NONE], 3),
    bass: None,
};

static V_CHORD: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::V, Accidental::Natural),
    intervals: IntervalSet::const_from_array(
        [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH,
         Interval::NONE, Interval::NONE, Interval::NONE,
         Interval::NONE, Interval::NONE, Interval::NONE,
         Interval::NONE], 3),
    bass: None,
};

static VI_CHORD: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::VI, Accidental::Natural),
    intervals: IntervalSet::const_from_array(
        [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH,
         Interval::NONE, Interval::NONE, Interval::NONE,
         Interval::NONE, Interval::NONE, Interval::NONE,
         Interval::NONE], 3),
    bass: None,
};

static VII_CHORD: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::VII, Accidental::Natural),
    intervals: IntervalSet::const_from_array(
        [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH,
         Interval::NONE, Interval::NONE, Interval::NONE,
         Interval::NONE, Interval::NONE, Interval::NONE,
         Interval::NONE], 3),
    bass: None,
};

static ii_CHORD: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::II, Accidental::Natural),
    intervals: IntervalSet::const_from_array(
        [Interval::PERFECT_UNISON, Interval::MINOR_THIRD, Interval::PERFECT_FIFTH,
         Interval::NONE, Interval::NONE, Interval::NONE,
         Interval::NONE, Interval::NONE, Interval::NONE,
         Interval::NONE], 3),
    bass: None,
};

static iii_CHORD: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::III, Accidental::Natural),
    intervals: IntervalSet::const_from_array(
        [Interval::PERFECT_UNISON, Interval::MINOR_THIRD, Interval::PERFECT_FIFTH,
         Interval::NONE, Interval::NONE, Interval::NONE,
         Interval::NONE, Interval::NONE, Interval::NONE,
         Interval::NONE], 3),
    bass: None,
};

static vi_CHORD: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::VI, Accidental::Natural),
    intervals: IntervalSet::const_from_array(
        [Interval::PERFECT_UNISON, Interval::MINOR_THIRD, Interval::PERFECT_FIFTH,
         Interval::NONE, Interval::NONE, Interval::NONE,
         Interval::NONE, Interval::NONE, Interval::NONE,
         Interval::NONE], 3),
    bass: None,
};

static bII_CHORD: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::II, Accidental::Flat),
    intervals: IntervalSet::const_from_array(
        [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH,
         Interval::NONE, Interval::NONE, Interval::NONE,
         Interval::NONE, Interval::NONE, Interval::NONE,
         Interval::NONE], 3),
    bass: None,
};

static bIII_CHORD: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::III, Accidental::Flat),
    intervals: IntervalSet::const_from_array(
        [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH,
         Interval::NONE, Interval::NONE, Interval::NONE,
         Interval::NONE, Interval::NONE, Interval::NONE,
         Interval::NONE], 3),
    bass: None,
};

static bV_CHORD: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::V, Accidental::Flat),
    intervals: IntervalSet::const_from_array(
        [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH,
         Interval::NONE, Interval::NONE, Interval::NONE,
         Interval::NONE, Interval::NONE, Interval::NONE,
         Interval::NONE], 3),
    bass: None,
};

static bVI_CHORD: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::VI, Accidental::Flat),
    intervals: IntervalSet::const_from_array(
        [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH,
         Interval::NONE, Interval::NONE, Interval::NONE,
         Interval::NONE, Interval::NONE, Interval::NONE,
         Interval::NONE], 3),
    bass: None,
};

static bVII_CHORD: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::VII, Accidental::Flat),
    intervals: IntervalSet::const_from_array(
        [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH,
         Interval::NONE, Interval::NONE, Interval::NONE,
         Interval::NONE, Interval::NONE, Interval::NONE,
         Interval::NONE], 3),
    bass: None,
};

static iv_CHORD: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::IV, Accidental::Natural),
    intervals: IntervalSet::const_from_array(
        [Interval::PERFECT_UNISON, Interval::MINOR_THIRD, Interval::PERFECT_FIFTH,
         Interval::NONE, Interval::NONE, Interval::NONE,
         Interval::NONE, Interval::NONE, Interval::NONE,
         Interval::NONE], 3),
    bass: None,
};

/// I-V-vi-IV: The most popular progression in modern music - countless hits use this
/// Chords: I,V,vi,IV
static PROGRESSION_0: [&'static RomanChord; 4] = [
    &I_CHORD,
    &V_CHORD,
    &vi_CHORD,
    &IV_CHORD,
];

/// vi-IV-I-V: Slightly melancholic variant of the pop progression
/// Chords: vi,IV,I,V
static PROGRESSION_1: [&'static RomanChord; 4] = [
    &vi_CHORD,
    &IV_CHORD,
    &I_CHORD,
    &V_CHORD,
];

/// I-vi-IV-V: Classic '50s progression - doo-wop and early rock
/// Chords: I,vi,IV,V
static PROGRESSION_2: [&'static RomanChord; 4] = [
    &I_CHORD,
    &vi_CHORD,
    &IV_CHORD,
    &V_CHORD,
];

/// I-IV-V-I: The fundamental major cadence - timeless and stable
/// Chords: I,IV,V,I
static PROGRESSION_3: [&'static RomanChord; 4] = [
    &I_CHORD,
    &IV_CHORD,
    &V_CHORD,
    &I_CHORD,
];

/// vi-ii-V-I: Gentle resolution with smooth voice leading
/// Chords: vi,ii,V,I
static PROGRESSION_4: [&'static RomanChord; 4] = [
    &vi_CHORD,
    &ii_CHORD,
    &V_CHORD,
    &I_CHORD,
];

/// I-V-IV-I: Plagal resolution variant - slightly folk/gospel feel
/// Chords: I,V,IV,I
static PROGRESSION_5: [&'static RomanChord; 4] = [
    &I_CHORD,
    &V_CHORD,
    &IV_CHORD,
    &I_CHORD,
];

/// ii-V-I: The cornerstone of jazz harmony - most important cadence
/// Chords: ii,V,I,I
static PROGRESSION_6: [&'static RomanChord; 4] = [
    &ii_CHORD,
    &V_CHORD,
    &I_CHORD,
    &I_CHORD,
];

/// I-vi-ii-V: Circle of fifths progression - smooth jazz standard
/// Chords: I,vi,ii,V
static PROGRESSION_7: [&'static RomanChord; 4] = [
    &I_CHORD,
    &vi_CHORD,
    &ii_CHORD,
    &V_CHORD,
];

/// vi-ii-V-I: Minor to major resolution - sophisticated and pleasing
/// Chords: vi,ii,V,I
static PROGRESSION_8: [&'static RomanChord; 4] = [
    &vi_CHORD,
    &ii_CHORD,
    &V_CHORD,
    &I_CHORD,
];

/// I-iii-vi-ii: Descending by thirds - creates harmonic momentum
/// Chords: I,iii,vi,ii
static PROGRESSION_9: [&'static RomanChord; 4] = [
    &I_CHORD,
    &iii_CHORD,
    &vi_CHORD,
    &ii_CHORD,
];

/// iii-vi-ii-V: Extended circle movement - builds to dominant
/// Chords: iii,vi,ii,V
static PROGRESSION_10: [&'static RomanChord; 4] = [
    &iii_CHORD,
    &vi_CHORD,
    &ii_CHORD,
    &V_CHORD,
];

/// I-IV-V-vi: Deceptive cadence - unexpected but satisfying
/// Chords: I,IV,V,vi
static PROGRESSION_11: [&'static RomanChord; 4] = [
    &I_CHORD,
    &IV_CHORD,
    &V_CHORD,
    &vi_CHORD,
];

/// iii-vi-ii-V: Extended circle with chromatic mediant approach
/// Chords: iii,vi,ii,V
static PROGRESSION_12: [&'static RomanChord; 4] = [
    &iii_CHORD,
    &vi_CHORD,
    &ii_CHORD,
    &V_CHORD,
];

/// I-iii-IV-iv: Major to minor fourth - modal interchange flavor
/// Chords: I,iii,IV,iv
static PROGRESSION_13: [&'static RomanChord; 4] = [
    &I_CHORD,
    &iii_CHORD,
    &IV_CHORD,
    &iv_CHORD,
];

/// vi-IV-I-V: Minor start with strong resolution drive
/// Chords: vi,IV,I,V
static PROGRESSION_14: [&'static RomanChord; 4] = [
    &vi_CHORD,
    &IV_CHORD,
    &I_CHORD,
    &V_CHORD,
];

/// ii-bII-I-vi: Tritone substitution - classic jazz reharmonization
/// Chords: ii,bII,I,vi
static PROGRESSION_15: [&'static RomanChord; 4] = [
    &ii_CHORD,
    &bII_CHORD,
    &I_CHORD,
    &vi_CHORD,
];

/// I-bVII-IV-I: Mixolydian flavor - modal borrowed chord
/// Chords: I,bVII,IV,I
static PROGRESSION_16: [&'static RomanChord; 4] = [
    &I_CHORD,
    &bVII_CHORD,
    &IV_CHORD,
    &I_CHORD,
];

/// vi-bVI-IV-V: Chromatic mediant - sophisticated voice leading
/// Chords: vi,bVI,IV,V
static PROGRESSION_17: [&'static RomanChord; 4] = [
    &vi_CHORD,
    &bVI_CHORD,
    &IV_CHORD,
    &V_CHORD,
];

/// I-bII-bIII-IV: Chromatic ascending - modern film scoring
/// Chords: I,bII,bIII,IV
static PROGRESSION_18: [&'static RomanChord; 4] = [
    &I_CHORD,
    &bII_CHORD,
    &bIII_CHORD,
    &IV_CHORD,
];

/// vi-bVI-bVII-V: Triple flat progression - dark and mysterious
/// Chords: vi,bVI,bVII,V
static PROGRESSION_19: [&'static RomanChord; 4] = [
    &vi_CHORD,
    &bVI_CHORD,
    &bVII_CHORD,
    &V_CHORD,
];

/// I-III-vi-bVI: Chromatic mediants - neo-Riemannian harmony
/// Chords: I,III,vi,bVI
static PROGRESSION_20: [&'static RomanChord; 4] = [
    &I_CHORD,
    &III_CHORD,
    &vi_CHORD,
    &bVI_CHORD,
];

/// ii-V-I-bVII: Jazz-rock fusion - adds mixolydian twist
/// Chords: ii,V,I,bVII
static PROGRESSION_21: [&'static RomanChord; 4] = [
    &ii_CHORD,
    &V_CHORD,
    &I_CHORD,
    &bVII_CHORD,
];

/// iii-bIII-ii-bII: Descending chromaticism - experimental tension
/// Chords: iii,bIII,ii,bII
static PROGRESSION_22: [&'static RomanChord; 4] = [
    &iii_CHORD,
    &bIII_CHORD,
    &ii_CHORD,
    &bII_CHORD,
];

/// I-bV-IV-bVII: Tritone relationships - dissonant but structured
/// Chords: I,bV,IV,bVII
static PROGRESSION_23: [&'static RomanChord; 4] = [
    &I_CHORD,
    &bV_CHORD,
    &IV_CHORD,
    &bVII_CHORD,
];

/// Pop Standards (0.0-0.25) - 6 progressions
pub static TIER_1_PROGRESSIONS: &[&[&'static RomanChord; 4]] = &[
    &PROGRESSION_0,
    &PROGRESSION_1,
    &PROGRESSION_2,
    &PROGRESSION_3,
    &PROGRESSION_4,
    &PROGRESSION_5,
];

/// Jazz Fundamentals (0.25-0.5) - 6 progressions
pub static TIER_2_PROGRESSIONS: &[&[&'static RomanChord; 4]] = &[
    &PROGRESSION_6,
    &PROGRESSION_7,
    &PROGRESSION_8,
    &PROGRESSION_9,
    &PROGRESSION_10,
    &PROGRESSION_11,
];

/// Sophisticated Jazz (0.5-0.75) - 6 progressions
pub static TIER_3_PROGRESSIONS: &[&[&'static RomanChord; 4]] = &[
    &PROGRESSION_12,
    &PROGRESSION_13,
    &PROGRESSION_14,
    &PROGRESSION_15,
    &PROGRESSION_16,
    &PROGRESSION_17,
];

/// Modern/Experimental (0.75-1.0) - 6 progressions
pub static TIER_4_PROGRESSIONS: &[&[&'static RomanChord; 4]] = &[
    &PROGRESSION_18,
    &PROGRESSION_19,
    &PROGRESSION_20,
    &PROGRESSION_21,
    &PROGRESSION_22,
    &PROGRESSION_23,
];

/// Select progression tier based on complexity value (0.0-1.0)
pub fn select_progression_tier(complexity: f32) -> &'static [&'static [&'static RomanChord; 4]] {
    match complexity {
        x if x < 0.25 => TIER_1_PROGRESSIONS,
        x if x < 0.5 => TIER_2_PROGRESSIONS,
        x if x < 0.75 => TIER_3_PROGRESSIONS,
        _ => TIER_4_PROGRESSIONS,
    }
}

/// Progression metadata for descriptions and names
pub static PROGRESSION_METADATA: &[(usize, &str, &str)] = &[
    (0, "I-V-vi-IV", "The most popular progression in modern music - countless hits use this"),
    (1, "vi-IV-I-V", "Slightly melancholic variant of the pop progression"),
    (2, "I-vi-IV-V", "Classic '50s progression - doo-wop and early rock"),
    (3, "I-IV-V-I", "The fundamental major cadence - timeless and stable"),
    (4, "vi-ii-V-I", "Gentle resolution with smooth voice leading"),
    (5, "I-V-IV-I", "Plagal resolution variant - slightly folk/gospel feel"),
    (6, "ii-V-I", "The cornerstone of jazz harmony - most important cadence"),
    (7, "I-vi-ii-V", "Circle of fifths progression - smooth jazz standard"),
    (8, "vi-ii-V-I", "Minor to major resolution - sophisticated and pleasing"),
    (9, "I-iii-vi-ii", "Descending by thirds - creates harmonic momentum"),
    (10, "iii-vi-ii-V", "Extended circle movement - builds to dominant"),
    (11, "I-IV-V-vi", "Deceptive cadence - unexpected but satisfying"),
    (12, "iii-vi-ii-V", "Extended circle with chromatic mediant approach"),
    (13, "I-iii-IV-iv", "Major to minor fourth - modal interchange flavor"),
    (14, "vi-IV-I-V", "Minor start with strong resolution drive"),
    (15, "ii-bII-I-vi", "Tritone substitution - classic jazz reharmonization"),
    (16, "I-bVII-IV-I", "Mixolydian flavor - modal borrowed chord"),
    (17, "vi-bVI-IV-V", "Chromatic mediant - sophisticated voice leading"),
    (18, "I-bII-bIII-IV", "Chromatic ascending - modern film scoring"),
    (19, "vi-bVI-bVII-V", "Triple flat progression - dark and mysterious"),
    (20, "I-III-vi-bVI", "Chromatic mediants - neo-Riemannian harmony"),
    (21, "ii-V-I-bVII", "Jazz-rock fusion - adds mixolydian twist"),
    (22, "iii-bIII-ii-bII", "Descending chromaticism - experimental tension"),
    (23, "I-bV-IV-bVII", "Tritone relationships - dissonant but structured"),
];
