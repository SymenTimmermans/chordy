//! Chord analysis logic for converting intervals to structured chord names
//!
//! This module contains the core analysis engine that examines a set of intervals
//! and determines the appropriate chord name components (quality, extensions, alterations, etc.).

use crate::types::{ChordQuality, Interval, NoteName};
use super::types::*;

/// Helper for checking interval presence in a chord to reduce code duplication
/// and improve readability of chord analysis logic.
pub(super) struct IntervalChecker<'a> {
    intervals: &'a [Interval],
}

impl<'a> IntervalChecker<'a> {
    pub fn new(intervals: &'a [Interval]) -> Self {
        Self { intervals }
    }
    
    /// Check if the chord contains a specific interval
    pub fn has(&self, interval: Interval) -> bool {
        self.intervals.contains(&interval)
    }
    
    /// Check if the chord has any of the provided intervals
    pub fn has_any(&self, intervals: &[Interval]) -> bool {
        intervals.iter().any(|&interval| self.has(interval))
    }
    
    /// Check if the chord has all of the provided intervals
    pub fn has_all(&self, intervals: &[Interval]) -> bool {
        intervals.iter().all(|&interval| self.has(interval))
    }
    
    // Common chord tone checks for better readability
    
    pub fn has_major_third(&self) -> bool { self.has(Interval::MAJOR_THIRD) }
    pub fn has_minor_third(&self) -> bool { self.has(Interval::MINOR_THIRD) }
    pub fn has_any_third(&self) -> bool { self.has_major_third() || self.has_minor_third() }
    
    pub fn has_perfect_fifth(&self) -> bool { self.has(Interval::PERFECT_FIFTH) }
    pub fn has_diminished_fifth(&self) -> bool { self.has(Interval::DIMINISHED_FIFTH) }
    pub fn has_augmented_fifth(&self) -> bool { self.has(Interval::AUGMENTED_FIFTH) }
    
    pub fn has_minor_seventh(&self) -> bool { self.has(Interval::MINOR_SEVENTH) }
    pub fn has_major_seventh(&self) -> bool { self.has(Interval::MAJOR_SEVENTH) }
    pub fn has_diminished_seventh(&self) -> bool { self.has(Interval::DIMINISHED_SEVENTH) }
    pub fn has_any_seventh(&self) -> bool { self.has_minor_seventh() || self.has_major_seventh() }
    
    pub fn has_major_second(&self) -> bool { self.has(Interval::MAJOR_SECOND) }
    pub fn has_major_ninth(&self) -> bool { self.has(Interval::MAJOR_NINTH) }
    /// Check if the chord has a natural 9th (major 2nd or major 9th)
    /// 
    /// In jazz harmony, the "natural" 9th is always the major 9th. This method checks
    /// for both simple (major 2nd) and compound (major 9th) forms of this interval.
    /// The major 2nd is included because it's enharmonically equivalent and may appear
    /// in closely-voiced chords.
    pub fn has_natural_ninth(&self) -> bool { self.has_major_second() || self.has_major_ninth() }
    pub fn has_minor_ninth(&self) -> bool { self.has(Interval::MINOR_NINTH) }
    pub fn has_augmented_ninth(&self) -> bool { self.has(Interval::AUGMENTED_NINTH) }
    
    pub fn has_perfect_fourth(&self) -> bool { self.has(Interval::PERFECT_FOURTH) }
    pub fn has_perfect_eleventh(&self) -> bool { self.has(Interval::PERFECT_ELEVENTH) }
    /// Check if the chord has a natural 11th (perfect 11th)
    /// 
    /// In jazz harmony, the "natural" 11th is the perfect 11th. Note that perfect 4th
    /// is typically not considered an extension but rather a suspension when present
    /// in chord voicings.
    pub fn has_natural_eleventh(&self) -> bool { self.has_perfect_eleventh() }
    pub fn has_augmented_eleventh(&self) -> bool { self.has(Interval::AUGMENTED_ELEVENTH) }
    pub fn has_any_eleventh(&self) -> bool { self.has_perfect_fourth() || self.has_perfect_eleventh() }
    
    pub fn has_major_sixth(&self) -> bool { self.has(Interval::MAJOR_SIXTH) }
    pub fn has_major_thirteenth(&self) -> bool { self.has(Interval::MAJOR_THIRTEENTH) }
    /// Check if the chord has a natural 13th (major 13th)
    /// 
    /// In jazz harmony, the "natural" 13th is always the major 13th. Any minor 13th
    /// intervals should be treated as ♭13 alterations, not natural extensions.
    pub fn has_natural_thirteenth(&self) -> bool { self.has_major_thirteenth() }
    pub fn has_minor_thirteenth(&self) -> bool { self.has(Interval::MINOR_THIRTEENTH) }
    pub fn has_augmented_thirteenth(&self) -> bool { self.has(Interval::AUGMENTED_THIRTEENTH) }
    
    // Chord quality detection helpers
    
    /// Check if the chord has diminished quality (minor 3rd + diminished 5th)
    /// 
    /// This is the fundamental test for diminished chord quality, requiring both
    /// a minor third and diminished fifth to be present.
    pub fn is_diminished_quality(&self) -> bool {
        self.has_minor_third() && self.has_diminished_fifth()
    }
    
    /// Check if the chord has augmented quality (major 3rd + augmented 5th)
    /// 
    /// This is the fundamental test for augmented chord quality, requiring both
    /// a major third and augmented fifth to be present.
    pub fn is_augmented_quality(&self) -> bool {
        self.has_major_third() && self.has_augmented_fifth()
    }
    
    /// Check if the chord has half-diminished quality (minor 3rd + diminished 5th + minor 7th)
    /// 
    /// Half-diminished chords (also known as minor 7♭5) are commonly found in jazz harmony
    /// and classical music, particularly as ii°7 chords in minor keys.
    pub fn is_half_diminished(&self) -> bool {
        self.has_minor_seventh() && self.has_diminished_fifth() && self.has_minor_third()
    }
}

/// Chord analyzer that converts intervals into structured chord names
pub struct ChordAnalyzer;

impl ChordAnalyzer {
    /// Analyze a chord's intervals and produce a ChordName
    pub fn analyze(root: NoteName, intervals: &[Interval]) -> ChordName {
        let chord_root = ChordRoot::Note(root);
        Self::analyze_with_root(chord_root, intervals)
    }
    
    /// Analyze a chord's intervals with a given root (Note or Roman) and produce a ChordName
    pub fn analyze_with_root(chord_root: ChordRoot, intervals: &[Interval]) -> ChordName {
        // Start with basic quality analysis using unified logic
        let quality = ChordQuality::from_intervals(intervals).unwrap_or(ChordQuality::Major);
        let mut chord_name = ChordName::new(chord_root, quality);
        
        // Analyze seventh
        if let Some(seventh_type) = Self::analyze_seventh(intervals) {
            chord_name = chord_name.with_seventh(seventh_type);
        }
        
        // Analyze extensions
        let extensions = Self::analyze_extensions(intervals);
        for extension in extensions {
            chord_name = chord_name.with_extension(extension);
        }
        
        // Analyze alterations
        let alterations = Self::analyze_alterations(intervals);
        for alteration in alterations {
            chord_name = chord_name.with_alteration(alteration);
        }
        
        // Analyze added tones
        let added_tones = Self::analyze_added_tones(intervals);
        for added_tone in added_tones {
            chord_name = chord_name.with_added_tone(added_tone);
        }
        
        // Analyze suspensions
        let suspensions = Self::analyze_suspensions(intervals);
        for suspension in suspensions {
            chord_name = chord_name.with_suspension(suspension);
        }
        
        // Analyze omissions (compared to expected chord type)
        let omissions = Self::analyze_omissions(intervals, &chord_name);
        for omission in omissions {
            chord_name = chord_name.with_omission(omission);
        }
        
        chord_name
    }
    
    /// Analyze seventh intervals to determine the seventh type
    /// 
    /// Returns the appropriate SeventhType based on the intervals present:
    /// - Diminished 7th: Used in fully diminished chords (dim7)
    /// - Half-diminished 7th: Minor 7th with diminished quality (ø7)
    /// - Minor 7th: Standard dominant and minor seventh chords (7)
    /// - Major 7th: Jazz and classical major seventh chords (maj7)
    fn analyze_seventh(intervals: &[Interval]) -> Option<SeventhType> {
        let interval_checker = IntervalChecker::new(intervals);
        
        if interval_checker.has_diminished_seventh() {
            Some(SeventhType::Diminished)
        } else if interval_checker.has_minor_seventh() {
            // Check if it's half-diminished (minor 7th with diminished 5th and minor 3rd)
            if interval_checker.is_half_diminished() {
                Some(SeventhType::HalfDiminished)
            } else {
                Some(SeventhType::Minor)
            }
        } else if interval_checker.has_major_seventh() {
            Some(SeventhType::Major)
        } else {
            None
        }
    }
    
    /// Analyze extension intervals (9, 11, 13) in jazz harmony context
    /// 
    /// Extensions are only recognized when a seventh is present, as they represent
    /// chord tones beyond the basic seventh chord. Only "natural" extensions are
    /// considered extensions - altered forms are treated as alterations.
    /// 
    /// Jazz harmony rules:
    /// - 9th: Must be major (minor 9th = ♭9 alteration)
    /// - 11th: Must be perfect and have 9th present (perfect 4th handled separately)
    /// - 13th: Must be major (minor 13th = ♭13 alteration)
    fn analyze_extensions(intervals: &[Interval]) -> Vec<Extension> {
        let mut chord_extensions = Vec::new();
        let interval_checker = IntervalChecker::new(intervals);
        
        // Extensions only exist in the context of seventh chords
        if interval_checker.has_any_seventh() {
            // Natural 9th: Major second or major ninth (compound equivalent)
            if interval_checker.has_natural_ninth() {
                chord_extensions.push(Extension::Ninth);
            }
            
            // Natural 11th: Perfect eleventh, only when 9th is also present
            // (Perfect 4th is handled as suspension, not extension)
            if interval_checker.has_natural_eleventh() && interval_checker.has_natural_ninth() {
                chord_extensions.push(Extension::Eleventh);
            }
            
            // Natural 13th: Major thirteenth (minor 13th is always ♭13 alteration)
            if interval_checker.has_natural_thirteenth() {
                chord_extensions.push(Extension::Thirteenth);
            }
        }
        
        chord_extensions.sort();
        chord_extensions
    }
    
    /// Analyze altered intervals (chromatically modified chord tones)
    /// 
    /// Alterations represent non-diatonic modifications to chord tones, commonly
    /// found in jazz harmony. These are intervals that deviate from the natural
    /// extensions and are typically notated with ♭ or ♯ symbols.
    /// 
    /// Note: Alterations implied by chord quality (e.g., ♭5 in diminished chords)
    /// are not included as they're redundant with the quality designation.
    fn analyze_alterations(intervals: &[Interval]) -> Vec<Alteration> {
        let mut chord_alterations = Vec::new();
        let interval_checker = IntervalChecker::new(intervals);
        
        // ♭5: Only when not implied by diminished chord quality
        if interval_checker.has_diminished_fifth() && !interval_checker.is_diminished_quality() {
            chord_alterations.push(Alteration::FlatFifth);
        }
        
        // ♯5: Only when not implied by augmented chord quality
        if interval_checker.has_augmented_fifth() && !interval_checker.is_augmented_quality() {
            chord_alterations.push(Alteration::SharpFifth);
        }
        
        // ♭9: Minor ninth is always an alteration (never a natural extension)
        if interval_checker.has_minor_ninth() {
            chord_alterations.push(Alteration::FlatNinth);
        }
        
        // ♯9: Augmented ninth alteration
        if interval_checker.has_augmented_ninth() {
            chord_alterations.push(Alteration::SharpNinth);
        }
        
        // ♯11: Augmented eleventh (lydian sound)
        if interval_checker.has_augmented_eleventh() {
            chord_alterations.push(Alteration::SharpEleventh);
        }
        
        // ♭13: Minor thirteenth is always an alteration, never an extension
        if interval_checker.has_minor_thirteenth() {
            chord_alterations.push(Alteration::FlatThirteenth);
        }
        
        // ♯13: Augmented thirteenth alteration
        if interval_checker.has_augmented_thirteenth() {
            chord_alterations.push(Alteration::SharpThirteenth);
        }
        
        chord_alterations.sort();
        chord_alterations
    }
    
    /// Analyze added tones (non-extension intervals in triads and simple chords)
    /// 
    /// Added tones are intervals that don't follow the standard extension pattern
    /// (which requires a seventh). They're commonly found in pop and rock music
    /// where a triad has an extra tone "added" without implying a full jazz extension.
    /// 
    /// Examples: Cadd9 (C-E-G-D), Cadd11 (C-E-G-F)
    fn analyze_added_tones(intervals: &[Interval]) -> Vec<AddedTone> {
        let mut added_tones = Vec::new();
        let interval_checker = IntervalChecker::new(intervals);
        
        // Add9: Natural ninth without seventh present (common in pop/rock)
        if interval_checker.has_natural_ninth() && interval_checker.has_any_third() && !interval_checker.has_any_seventh() {
            added_tones.push(AddedTone::Ninth);
        }
        
        // Add4: Perfect fourth with third present (not a sus4 since third is present)
        // Compound interval parsing ensures P4 stays as fourth, not converted to 11th
        if interval_checker.has_perfect_fourth() && interval_checker.has_any_third() && !interval_checker.has_any_seventh() {
            added_tones.push(AddedTone::Fourth);
        }
        
        // Add11: Perfect eleventh as compound interval, but without natural 9th
        // This covers cases like "C,E,G,B,F" where F is P11 but no 9th makes it add11
        if interval_checker.has_perfect_eleventh() && interval_checker.has_any_third() && !interval_checker.has_natural_ninth() {
            added_tones.push(AddedTone::Eleventh); // Compound interval rendered as add11
        }
        
        // Add6: Major sixth without seventh (classic jazz/pop chord)
        if interval_checker.has_major_sixth() && !interval_checker.has_any_seventh() {
            added_tones.push(AddedTone::Sixth);
        }
        
        added_tones.sort();
        added_tones
    }
    
    /// Analyze suspensions (intervals that replace the third)
    /// 
    /// Suspensions are non-chord tones that temporarily replace the third,
    /// creating harmonic tension that typically resolves to the expected third.
    /// In chord naming, sus chords explicitly lack the third.
    /// 
    /// Common types:
    /// - sus2: Major second replaces the third
    /// - sus4: Perfect fourth replaces the third
    fn analyze_suspensions(intervals: &[Interval]) -> Vec<Suspension> {
        let mut chord_suspensions = Vec::new();
        let interval_checker = IntervalChecker::new(intervals);
        
        // Sus2: Major second present, but no third (2nd replaces 3rd)
        if interval_checker.has_major_second() && !interval_checker.has_any_third() {
            chord_suspensions.push(Suspension::Second);
        }
        
        // Sus4: Perfect fourth present, but no third (4th replaces 3rd)
        if interval_checker.has_perfect_fourth() && !interval_checker.has_any_third() {
            chord_suspensions.push(Suspension::Fourth);
        }
        
        chord_suspensions.sort();
        chord_suspensions
    }
    
    /// Analyze omissions (expected chord tones that are missing)
    /// 
    /// Omissions represent chord tones that are typically expected to be present
    /// but are deliberately left out. Common in jazz notation to clarify voicings.
    /// 
    /// Examples:
    /// - 13(no11): 13th chord without the 11th
    /// - C5: Power chord (no third)
    fn analyze_omissions(intervals: &[Interval], chord_name: &ChordName) -> Vec<Omission> {
        let mut chord_omissions = Vec::new();
        let interval_checker = IntervalChecker::new(intervals);
        
        // Legacy behavior: Don't explicitly note missing fifths in extended chords
        // as they're commonly omitted for voice leading or harmonic clarity
        
        // Missing 11th in 13th chords: Special case for jazz notation clarity
        // When a 13th is present but 11th is omitted, it's explicitly notated
        if chord_name.extensions.contains(&Extension::Thirteenth) && !interval_checker.has_any_eleventh() {
            chord_omissions.push(Omission::Eleventh);
        }
        
        // Missing third in power chords: Only when it's clearly a power chord
        // (no suspensions, sevenths, or extensions that would explain the missing third)
        if !interval_checker.has_any_third() &&
           chord_name.suspensions.is_empty() &&
           chord_name.seventh.is_none() &&
           chord_name.extensions.is_empty() {
            chord_omissions.push(Omission::Third);
        }
        
        chord_omissions.sort();
        chord_omissions
    }
}