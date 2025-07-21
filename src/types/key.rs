use crate::traits::HasRoot;

use super::{NoteName, scale::ScaleDegree, Chord, RomanNumeral, HarmonicFunction, Scale, Accidental, Letter};
use super::progression::{ChordProgressionOptions, ProgressionGraph};

/// The mode of a key (Major, Minor, etc.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    /// Major key signature
    ///
    /// This does not imply a specific scale, just the key signature.
    Major(NoteName),
    /// Minor key signature
    ///
    /// This does not imply a specific scale, just the key signature.
    Minor(NoteName),
}

/// Represents a key signature with sharps or flats
impl Key {
    /// Number of sharps (positive) or flats (negative)
    pub fn accidentals(&self) -> i8 {
        match self {
            Key::Major(note) | Key::Minor(note) => note.fifths(),
        }
    }

    /// Returns the scale degree of a given note within this key
    ///
    /// Calculates the scale degree by finding the interval from the key's root
    /// to the given note, then converting that interval to a ScaleDegree.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chordy::{Key, note, ScaleDegree, Accidental};
    ///
    /// let c_major = Key::Major(note!("C"));
    /// 
    /// // Natural scale degrees in C major
    /// assert_eq!(c_major.degree_of(note!("C")), ScaleDegree::new(1, None));
    /// assert_eq!(c_major.degree_of(note!("D")), ScaleDegree::new(2, None));
    /// assert_eq!(c_major.degree_of(note!("E")), ScaleDegree::new(3, None));
    /// 
    /// // Altered notes
    /// assert_eq!(c_major.degree_of(note!("C#")), ScaleDegree::new(1, Some(Accidental::Sharp)));
    /// assert_eq!(c_major.degree_of(note!("F#")), ScaleDegree::new(4, Some(Accidental::Sharp)));
    /// ```
    pub fn degree_of(&self, note: NoteName) -> ScaleDegree {
        // Calculate the interval from the key root to the given note
        let interval = self.root().interval_to(note);
        
        // Convert the interval to a scale degree
        ScaleDegree::from(interval)
    }

    /// Analyze a chord in this key, returning its roman numeral representation
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Key, Chord, RomanNumeral, note};
    ///
    /// let c_major_key = Key::Major(note!("C"));
    /// let g_chord = Chord::major(note!("G"));
    /// let roman = c_major_key.analyze_chord(&g_chord);
    /// assert_eq!(roman, RomanNumeral::V());
    /// ```
    pub fn analyze_chord(&self, chord: &Chord) -> RomanNumeral {
        let root_degree = self.degree_of(chord.root);
        RomanNumeral::from(root_degree)
    }

    /// Get the harmonic function of a chord in this key
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Key, Chord, HarmonicFunction, note};
    ///
    /// let c_major_key = Key::Major(note!("C"));
    /// let g_chord = Chord::major(note!("G"));
    /// let function = c_major_key.harmonic_function(&g_chord);
    /// assert_eq!(function, Some(HarmonicFunction::Dominant));
    /// ```
    pub fn harmonic_function(&self, chord: &Chord) -> Option<HarmonicFunction> {
        // Get the scale for this key
        let scale = match self {
            Key::Major(tonic) => Scale::major(*tonic),
            Key::Minor(tonic) => Scale::minor(*tonic),
        };
        
        scale.harmonic_function(chord)
    }

    /// Get the scale associated with this key
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Key, Scale, note};
    ///
    /// let c_major_key = Key::Major(note!("C"));
    /// let scale = c_major_key.scale();
    /// assert_eq!(scale, Scale::major(note!("C")));
    /// ```
    pub fn scale(&self) -> Scale {
        match self {
            Key::Major(tonic) => Scale::major(*tonic),
            Key::Minor(tonic) => Scale::minor(*tonic),
        }
    }

    /// Get the expected accidental for a given note letter in this key signature
    /// 
    /// Returns the accidental that should be applied to this note letter
    /// based on the key signature, or None if the note should be natural.
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Key, Letter, Accidental, note};
    ///
    /// let d_major = Key::Major(note!("D"));
    /// assert_eq!(d_major.expected_accidental(Letter::F), Some(Accidental::Sharp)); // F# in D major
    /// assert_eq!(d_major.expected_accidental(Letter::C), Some(Accidental::Sharp)); // C# in D major
    /// assert_eq!(d_major.expected_accidental(Letter::G), None); // G natural in D major
    /// ```
    pub fn expected_accidental(&self, letter: Letter) -> Option<Accidental> {
        let num_accidentals = self.accidentals();
        
        match num_accidentals.cmp(&0) {
            std::cmp::Ordering::Greater => {
                // Sharp keys: F#, C#, G#, D#, A#, E#, B#
                let sharp_order = [Letter::F, Letter::C, Letter::G, Letter::D, Letter::A, Letter::E, Letter::B];
                if (0..num_accidentals as usize).any(|i| sharp_order.get(i) == Some(&letter)) {
                    Some(Accidental::Sharp)
                } else {
                    None
                }
            }
            std::cmp::Ordering::Less => {
                // Flat keys: Bb, Eb, Ab, Db, Gb, Cb, Fb
                let flat_order = [Letter::B, Letter::E, Letter::A, Letter::D, Letter::G, Letter::C, Letter::F];
                if (0..(-num_accidentals) as usize).any(|i| flat_order.get(i) == Some(&letter)) {
                    Some(Accidental::Flat)
                } else {
                    None
                }
            }
            std::cmp::Ordering::Equal => {
                // C major/A minor - no accidentals
                None
            }
        }
    }

    /// Check if a note needs an accidental symbol when displayed in this key
    ///
    /// Returns the accidental symbol to display, or None if no symbol is needed.
    /// This compares the note's actual accidental with what the key signature expects.
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Key, note, Accidental};
    ///
    /// let d_major = Key::Major(note!("D"));
    /// 
    /// // F# is expected in D major, so no accidental symbol needed
    /// assert_eq!(d_major.accidental_to_display(note!("F#")), None);
    /// 
    /// // F natural needs a natural symbol in D major
    /// assert_eq!(d_major.accidental_to_display(note!("F")), Some(Accidental::Natural));
    /// 
    /// // G natural is expected, so no symbol needed
    /// assert_eq!(d_major.accidental_to_display(note!("G")), None);
    /// ```
    pub fn accidental_to_display(&self, note: NoteName) -> Option<Accidental> {
        let expected = self.expected_accidental(note.letter());
        let actual = if note.accidental() == Accidental::Natural { None } else { Some(note.accidental()) };
        
        if expected != actual {
            // Need to show the actual accidental
            Some(note.accidental())
        } else {
            // Note matches key signature expectation
            None
        }
    }

    /// Get the sharps or flats to display in the key signature
    ///
    /// Returns a vector of (Letter, Accidental) pairs representing the accidentals
    /// to display in the key signature, in the correct order.
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Key, Letter, Accidental, note};
    ///
    /// let d_major = Key::Major(note!("D"));
    /// let key_sig = d_major.key_signature_accidentals();
    /// assert_eq!(key_sig, vec![(Letter::F, Accidental::Sharp), (Letter::C, Accidental::Sharp)]);
    /// ```
    pub fn key_signature_accidentals(&self) -> Vec<(Letter, Accidental)> {
        let num_accidentals = self.accidentals();
        let mut result = Vec::new();
        
        match num_accidentals.cmp(&0) {
            std::cmp::Ordering::Greater => {
                // Sharp keys
                let sharp_order = [Letter::F, Letter::C, Letter::G, Letter::D, Letter::A, Letter::E, Letter::B];
                for i in 0..(num_accidentals as usize) {
                    if let Some(&letter) = sharp_order.get(i) {
                        result.push((letter, Accidental::Sharp));
                    }
                }
            }
            std::cmp::Ordering::Less => {
                // Flat keys
                let flat_order = [Letter::B, Letter::E, Letter::A, Letter::D, Letter::G, Letter::C, Letter::F];
                for i in 0..((-num_accidentals) as usize) {
                    if let Some(&letter) = flat_order.get(i) {
                        result.push((letter, Accidental::Flat));
                    }
                }
            }
            std::cmp::Ordering::Equal => {
                // No accidentals for C major/A minor
            }
        }
        
        result
    }

    /// Get chord progressions that can be made FROM a given chord in this key
    /// 
    /// Returns categorized progression options as actual Chord objects
    /// based on Stephen Mugglin's progression map:
    /// - Strong: explicit arrows showing natural voice leading
    /// - Moderate: jumps to primary nodes (stable but less directed)
    /// - Weak: jumps to secondary nodes (creates tension, needs resolution)
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use chordy::{Key, Chord, note};
    /// 
    /// let c_major = Key::Major(note!("C"));
    /// let c_chord = Chord::major(note!("C"));
    /// 
    /// let options = c_major.progressions_from(&c_chord).unwrap();
    /// assert!(!options.strong.is_empty());
    /// ```
    pub fn progressions_from(&self, chord: &Chord) -> Option<ChordProgressionOptions> {
        // Find the progression node for this chord
        let node = self.find_node_for_chord(chord)?;
        
        // Get the appropriate progression graph
        let graph = match self {
            Key::Major(_) => ProgressionGraph::major(),
            Key::Minor(_) => ProgressionGraph::minor(),
        };
        
        // Get progression options using the node directly
        let node_options = graph.progressions_from(node)?;
        
        // Convert RomanChords to Chords in this key context
        let mut chord_options = ChordProgressionOptions::new();
        
        // Convert strong options
        for roman_chord in &node_options.strong {
            let chord = roman_chord.in_key(self);
            chord_options.strong.push(chord);
        }
        
        // Convert moderate options
        for roman_chord in &node_options.moderate {
            let chord = roman_chord.in_key(self);
            chord_options.moderate.push(chord);
        }
        
        // Convert weak options
        for roman_chord in &node_options.weak {
            let chord = roman_chord.in_key(self);
            chord_options.weak.push(chord);
        }
        
        Some(chord_options)
    }
    
    /// Get chord progressions that lead TO a given chord in this key
    /// 
    /// Returns categorized progression options showing which chords can
    /// naturally lead to the target chord:
    /// - Strong: explicit arrows pointing to this chord (cadential motion)
    /// - Moderate: primary chords that can jump to this chord
    /// - Weak: secondary chords that can jump to this chord
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use chordy::{Key, Chord, note};
    /// 
    /// let c_major = Key::Major(note!("C"));
    /// let c_chord = Chord::major(note!("C")); // I chord
    /// 
    /// let options = c_major.progressions_to(&c_chord).unwrap();
    /// // Should include V -> I as a strong progression
    /// ```
    pub fn progressions_to(&self, chord: &Chord) -> Option<ChordProgressionOptions> {
        // Find the progression node for this chord
        let node = self.find_node_for_chord(chord)?;
        
        // Get the appropriate progression graph
        let graph = match self {
            Key::Major(_) => ProgressionGraph::major(),
            Key::Minor(_) => ProgressionGraph::minor(),
        };
        
        // Get progressions that lead to this node
        let node_options = graph.progressions_to(node)?;
        
        // Convert RomanChords to Chords in this key context
        let mut chord_options = ChordProgressionOptions::new();
        
        // Convert strong options
        for roman_chord in &node_options.strong {
            let chord = roman_chord.in_key(self);
            chord_options.strong.push(chord);
        }
        
        // Convert moderate options
        for roman_chord in &node_options.moderate {
            let chord = roman_chord.in_key(self);
            chord_options.moderate.push(chord);
        }
        
        // Convert weak options
        for roman_chord in &node_options.weak {
            let chord = roman_chord.in_key(self);
            chord_options.weak.push(chord);
        }
        
        Some(chord_options)
    }
    
    /// Deprecated: Use `progressions_from` instead.
    #[deprecated(since = "0.2.0", note = "Use `progressions_from` for clarity")]
    pub fn progression_options(&self, chord: &Chord) -> Option<ChordProgressionOptions> {
        self.progressions_from(chord)
    }

    /// Internal helper to get the progression graph for this key
    fn get_progression_graph(&self) -> ProgressionGraph {
        match self {
            Key::Major(_) => ProgressionGraph::major(),
            Key::Minor(_) => ProgressionGraph::minor(),
        }
    }

    /// Internal helper to find the progression node that best matches a given chord
    fn find_node_for_chord(&self, chord: &Chord) -> Option<crate::types::RomanChord> {
        // Convert chord to roman chord in this key context
        let roman_chord = chord.to_roman(self)?;
        
        // Get the progression graph for this key
        let graph = self.get_progression_graph();
        
        // Collect all nodes and sort them to ensure deterministic behavior
        let mut nodes: Vec<_> = graph.nodes().collect();
        nodes.sort_by_key(|node| (node.root, node.intervals.len()));
        
        // First try to find exact match with intervals AND bass
        for node in &nodes {
            if node.root == roman_chord.root() && node.bass == roman_chord.bass {
                // Check if intervals match exactly
                let node_intervals: std::collections::HashSet<_> = node.intervals.iter().collect();
                let chord_intervals: std::collections::HashSet<_> = roman_chord.intervals().iter().copied().collect();
                
                // Exact match (root, bass, and intervals all match)
                if node_intervals == chord_intervals {
                    return Some(*node);
                }
            }
        }
        
        // Strict matching: bass structure must match exactly
        for node in &nodes {
            if node.root == roman_chord.root() &&
               node.intervals.len() == roman_chord.intervals().len() &&
               node.bass == roman_chord.bass {
                // Check if intervals match exactly
                let node_intervals: std::collections::HashSet<_> = node.intervals.iter().collect();
                let chord_intervals: std::collections::HashSet<_> = roman_chord.intervals().iter().copied().collect();
                
                if node_intervals == chord_intervals {
                    return Some(*node);
                }
            }
        }
        
        // If no exact match, find nodes with matching root, intervals, and bass structure
        for node in &nodes {
            if node.root == roman_chord.root() &&
               node.intervals.len() == roman_chord.intervals().len() &&
               node.bass.is_some() == roman_chord.bass.is_some() {
                return Some(*node);
            }
        }
        
        // Final fallback: match root and ensure bass compatibility
        for node in &nodes {
            if node.root == roman_chord.root() {
                // If chord has no bass, only match nodes with no bass
                if roman_chord.bass.is_none() && node.bass.is_none() {
                    return Some(*node);
                }
                // If chord has bass, match any node with same root (less strict)
                if roman_chord.bass.is_some() {
                    return Some(*node);
                }
            }
        }
        
        None
    }

}

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Key::Major(note) => write!(f, "{} Major", note),
            Key::Minor(note) => write!(f, "{} Minor", note),
        }
    }
}

impl HasRoot for Key {
    fn root(&self) -> NoteName {
        match self {
            Key::Major(note) => *note,
            Key::Minor(note) => *note,
        }
    }

    fn root_mut(&mut self) -> &mut NoteName {
        match self {
            Key::Major(note) => note,
            Key::Minor(note) => note,
        }
    }
}

