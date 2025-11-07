use std::collections::BTreeSet;
use std::fmt;

use super::{Accidental, Letter, NoteName, Pitch};

/// Represents a set of pitch classes (notes without octave information)
///
/// Pitch class sets are fundamental to 20th century music theory and atonal analysis.
/// They provide a mathematical framework for analyzing harmonic structures in post-tonal music,
/// particularly useful for analyzing atonal, twelve-tone, and contemporary classical compositions.
///
/// # What is a Pitch Class Set?
///
/// A pitch class set is a collection of unique pitch classes, where a pitch class represents
/// all pitches that are octave equivalents. For example, all C's (C0, C1, C2, etc.) belong to
/// the same pitch class. This allows for analysis of harmonic relationships independent of
/// register or voicing.
///
/// # Key Concepts
///
/// - **Normal Form**: The most compact ordering of the pitch classes
/// - **Prime Form**: A canonical representation starting at 0, used to identify set classes
/// - **Interval Vector**: Shows the count of each interval class (1-6) present in the set
/// - **Set Operations**: Subset, intersection, and complement operations
/// - **Chord Recognition**: Identify common chord types from pitch class collections
///
/// # Examples
///
/// ## Basic Usage
///
/// ```
/// use chordy::{PitchClassSet, note};
///
/// // Create from note names
/// let notes = vec![note!("C"), note!("E"), note!("G")];
/// let pc_set = PitchClassSet::new(&notes);
/// assert!(pc_set.is_major_triad());
/// assert_eq!(pc_set.len(), 3);
/// ```
///
/// ## Analyzing Chord Progressions
///
/// ```
/// use chordy::{PitchClassSet, note};
///
/// // Analyze the pitch content of a chord
/// let chord = PitchClassSet::new(&[
///     note!("C"), note!("E"), note!("G"), note!("Bb")
/// ]);
///
/// assert!(chord.is_dominant_seventh());
/// assert_eq!(chord.prime_form(), vec![0, 4, 7, 10]);
/// assert_eq!(chord.interval_vector(), [0, 1, 2, 1, 1, 1]);
/// ```
///
/// ## Set Theory Analysis
///
/// ```
/// use chordy::{PitchClassSet, note};
///
/// // Create a pitch class set
/// let set = PitchClassSet::new(&[
///     note!("C"), note!("D"), note!("E"), note!("F#"), note!("G#"), note!("A#")
/// ]);
///
/// // Get normal form (most compact ordering)
/// let normal = set.normal_form();
///
/// // Get prime form (canonical representation)
/// let prime = set.prime_form();
/// println!("Prime form: {:?}", prime);
///
/// // Calculate interval vector
/// let intervals = set.interval_vector();
/// println!("Interval vector: {:?}", intervals);
/// ```
///
/// ## Enharmonic Equivalence
///
/// ```
/// use chordy::{PitchClassSet, note};
///
/// // These sets are enharmonically equivalent
/// let set1 = PitchClassSet::new(&[note!("C"), note!("E"), note!("G")]);
/// let set2 = PitchClassSet::new(&[note!("C"), note!("Fb"), note!("G")]); // Fb = E
///
/// assert_eq!(set1.prime_form(), set2.prime_form());
/// assert_eq!(set1.interval_vector(), set2.interval_vector());
/// ```
///
/// ## Set Operations
///
/// ```
/// use chordy::{PitchClassSet, note};
///
/// let set1 = PitchClassSet::new(&[note!("C"), note!("E"), note!("G")]);
/// let set2 = PitchClassSet::new(&[note!("E"), note!("G"), note!("B")]);
///
/// // Find common tones
/// let common = set1.intersection(&set2);
/// assert_eq!(common.len(), 2); // E and G
///
/// // Check subset relationship
/// let subset = PitchClassSet::new(&[note!("C"), note!("E")]);
/// assert!(subset.is_subset_of(&set1));
///
/// // Find complement (all pitch classes not in the set)
/// let complement = set1.complement();
/// assert_eq!(complement.len(), 9); // 12 - 3
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PitchClassSet(BTreeSet<NoteName>);

impl PitchClassSet {
    /// Creates a new PitchClassSet from a slice of NoteNames
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{PitchClassSet, note};
    ///
    /// let notes = vec![note!("C"), note!("E"), note!("G")];
    /// let pc_set = PitchClassSet::new(&notes);
    /// assert_eq!(pc_set.len(), 3);
    /// ```
    pub fn new(notes: &[NoteName]) -> Self {
        let mut set = BTreeSet::new();
        for &note in notes {
            set.insert(note);
        }
        PitchClassSet(set)
    }

    /// Creates a PitchClassSet from a slice of Pitches (ignores octave information)
    ///
    /// This method extracts the pitch class from each pitch, discarding octave information.
    /// Duplicate pitch classes are automatically removed.
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{PitchClassSet, Pitch, Letter, Accidental};
    ///
    /// let pitches = vec![
    ///     Pitch::new(Letter::C, Accidental::Natural, 4),
    ///     Pitch::new(Letter::E, Accidental::Natural, 4),
    ///     Pitch::new(Letter::G, Accidental::Natural, 4),
    ///     Pitch::new(Letter::C, Accidental::Natural, 5), // Duplicate pitch class
    /// ];
    /// let pc_set = PitchClassSet::from_pitches(&pitches);
    /// assert_eq!(pc_set.len(), 3); // C5 is same pitch class as C4
    /// ```
    pub fn from_pitches(pitches: &[Pitch]) -> Self {
        let mut set = BTreeSet::new();
        for pitch in pitches {
            set.insert(pitch.name);
        }
        PitchClassSet(set)
    }

    /// Returns the number of distinct pitch classes in the set
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if the set contains no pitch classes
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns true if the set contains the given note
    pub fn contains(&self, note: NoteName) -> bool {
        self.0.contains(&note)
    }

    /// Returns an iterator over the pitch classes in the set
    pub fn iter(&self) -> impl Iterator<Item = &NoteName> {
        self.0.iter()
    }

    /// Returns the set as a sorted vector of NoteNames (sorted by pitch class)
    pub fn to_vec(&self) -> Vec<NoteName> {
        let mut notes: Vec<NoteName> = self.0.iter().cloned().collect();
        notes.sort_by_key(|note| note.base_midi_number());
        notes
    }

    /// Returns the normal form of the pitch class set
    ///
    /// Normal form is the most compact ordering of the pitch classes that spans
    /// the smallest possible interval. This is the standard representation used
    /// in set theory analysis.
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{PitchClassSet, note};
    ///
    /// let notes = vec![note!("E"), note!("G"), note!("C")];
    /// let pc_set = PitchClassSet::new(&notes);
    /// let normal_form = pc_set.normal_form();
    /// assert_eq!(normal_form, vec![note!("C"), note!("E"), note!("G")]);
    /// ```
    pub fn normal_form(&self) -> Vec<NoteName> {
        if self.0.is_empty() {
            return Vec::new();
        }

        // Get notes sorted by pitch class
        let mut notes: Vec<NoteName> = self.0.iter().cloned().collect();
        notes.sort_by_key(|note| note.base_midi_number());

        // For sets with 1 or 2 elements, normal form is just sorted order
        if notes.len() <= 2 {
            return notes;
        }

        // For larger sets, find the ordering with smallest outer interval
        self.find_normal_form(&notes)
    }

    /// Helper method to find the normal form of a pitch class set
    fn find_normal_form(&self, notes: &[NoteName]) -> Vec<NoteName> {
        let mut candidates = Vec::new();

        // Generate all rotations
        for i in 0..notes.len() {
            let rotated: Vec<NoteName> = notes.iter()
                .cycle()
                .skip(i)
                .take(notes.len())
                .cloned()
                .collect();
            candidates.push(rotated);
        }

        // Find candidate with smallest outer interval
        let mut best_candidate = candidates[0].clone();
        let mut smallest_outer_interval = self.outer_interval(&best_candidate);

        for candidate in &candidates[1..] {
            let outer_interval = self.outer_interval(candidate);
            if outer_interval < smallest_outer_interval {
                smallest_outer_interval = outer_interval;
                best_candidate = candidate.clone();
            } else if outer_interval == smallest_outer_interval {
                // If outer intervals are equal, choose the one that starts with the smallest pitch class
                let best_first_pc = best_candidate[0].base_midi_number() as u8;
                let candidate_first_pc = candidate[0].base_midi_number() as u8;
                if candidate_first_pc < best_first_pc {
                    best_candidate = candidate.clone();
                }
            }
        }

        best_candidate
    }


    /// Calculate the outer interval (distance from first to last note in semitones)
    fn outer_interval(&self, notes: &[NoteName]) -> u8 {
        if notes.len() < 2 {
            return 0;
        }

        let first_pc = notes[0].base_midi_number() as u8;
        let last_pc = notes[notes.len() - 1].base_midi_number() as u8;

        // For normal form calculation, we need to consider the circular nature of pitch class space
        // If the last note is lower than the first note, it means we've wrapped around
        // In this case, we add 12 to the last note to get the correct interval
        if last_pc >= first_pc {
            last_pc - first_pc
        } else {
            (last_pc + 12) - first_pc
        }
    }

    /// Returns the prime form of the pitch class set
    ///
    /// Prime form is a canonical representation that identifies the set class.
    /// For this implementation, it returns the sorted pitch classes transposed to start at 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{PitchClassSet, note};
    ///
    /// let notes = vec![note!("C"), note!("E"), note!("G")];
    /// let pc_set = PitchClassSet::new(&notes);
    /// let prime_form = pc_set.prime_form();
    /// assert_eq!(prime_form, vec![0, 4, 7]); // Major triad
    /// ```
    pub fn prime_form(&self) -> Vec<u8> {
        let notes = self.to_vec();
        if notes.is_empty() {
            return Vec::new();
        }

        // Get pitch classes modulo 12 to handle enharmonic equivalents
        let mut pcs: Vec<u8> = notes.iter()
            .map(|note| (note.base_midi_number() % 12) as u8)
            .collect();

        // Remove duplicates and sort
        pcs.sort();
        pcs.dedup();

        // Transpose so first element is 0
        let first_pc = pcs[0];
        let prime: Vec<u8> = pcs.iter()
            .map(|&pc| {
                if pc >= first_pc {
                    pc - first_pc
                } else {
                    (pc + 12) - first_pc
                }
            })
            .collect();

        prime
    }

    /// Returns the interval vector (also called interval class vector)
    ///
    /// The interval vector shows the number of each interval class present in the set.
    /// The vector has 6 elements representing interval classes 1 through 6.
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{PitchClassSet, note};
    ///
    /// let notes = vec![note!("C"), note!("E"), note!("G")];
    /// let pc_set = PitchClassSet::new(&notes);
    /// let interval_vector = pc_set.interval_vector();
    /// assert_eq!(interval_vector, [0, 0, 1, 1, 1, 0]); // Major triad vector
    /// ```
    pub fn interval_vector(&self) -> [u8; 6] {
        let mut vector = [0u8; 6];
        let notes: Vec<NoteName> = self.0.iter().cloned().collect();

        for i in 0..notes.len() {
            for j in (i + 1)..notes.len() {
                let interval = self.interval_class_between(notes[i], notes[j]);
                if interval >= 1 && interval <= 6 {
                    vector[interval as usize - 1] += 1;
                }
            }
        }

        vector
    }

    /// Calculate interval class between two notes
    fn interval_class_between(&self, note1: NoteName, note2: NoteName) -> u8 {
        let pc1 = note1.base_midi_number() as u8;
        let pc2 = note2.base_midi_number() as u8;

        let interval = if pc2 >= pc1 {
            pc2 - pc1
        } else {
            (pc2 + 12) - pc1
        };

        // Convert to interval class (1-6)
        if interval <= 6 {
            interval
        } else {
            12 - interval
        }
    }

    /// Returns true if this set is a subset of another set
    pub fn is_subset_of(&self, other: &PitchClassSet) -> bool {
        self.0.is_subset(&other.0)
    }

    /// Returns the intersection of this set with another set
    pub fn intersection(&self, other: &PitchClassSet) -> PitchClassSet {
        PitchClassSet(self.0.intersection(&other.0).cloned().collect())
    }

    /// Returns the complement of this set (all pitch classes not in the set)
    pub fn complement(&self) -> PitchClassSet {
        // Get the pitch class numbers (0-11) of the current set
        let current_pc_nums: std::collections::BTreeSet<u8> = self.0
            .iter()
            .map(|note| (note.base_midi_number() % 12) as u8)
            .collect();

        // Generate all 12 pitch classes using pitch class numbers (0-11)
        // This ensures enharmonic equivalence
        let all_pcs: BTreeSet<NoteName> = (0..12)
            .filter(|&pc_num| !current_pc_nums.contains(&pc_num))
            .map(|pc_num| {
                // Find a note name that represents this pitch class number
                // We'll use the standard chromatic scale starting from C
                let base_c = NoteName::new(Letter::C, Accidental::Natural);
                let target_pc_num = (base_c.base_midi_number() + pc_num as i8) % 12;

                // Try different spellings to find one that matches the target pitch class
                let candidates = [
                    NoteName::new(Letter::C, Accidental::Natural),
                    NoteName::new(Letter::C, Accidental::Sharp),
                    NoteName::new(Letter::D, Accidental::Natural),
                    NoteName::new(Letter::D, Accidental::Sharp),
                    NoteName::new(Letter::E, Accidental::Natural),
                    NoteName::new(Letter::F, Accidental::Natural),
                    NoteName::new(Letter::F, Accidental::Sharp),
                    NoteName::new(Letter::G, Accidental::Natural),
                    NoteName::new(Letter::G, Accidental::Sharp),
                    NoteName::new(Letter::A, Accidental::Natural),
                    NoteName::new(Letter::A, Accidental::Sharp),
                    NoteName::new(Letter::B, Accidental::Natural),
                ];

                // Find the candidate that matches our target pitch class
                *candidates.iter()
                    .find(|&&candidate| candidate.base_midi_number() % 12 == target_pc_num)
                    .unwrap_or(&base_c)
            })
            .collect();

        PitchClassSet(all_pcs)
    }

    /// Returns true if this set represents a major triad
    pub fn is_major_triad(&self) -> bool {
        self.prime_form() == vec![0, 4, 7]
    }

    /// Returns true if this set represents a minor triad
    pub fn is_minor_triad(&self) -> bool {
        self.prime_form() == vec![0, 3, 7]
    }

    /// Returns true if this set represents a diminished triad
    pub fn is_diminished_triad(&self) -> bool {
        self.prime_form() == vec![0, 3, 6]
    }

    /// Returns true if this set represents an augmented triad
    pub fn is_augmented_triad(&self) -> bool {
        self.prime_form() == vec![0, 4, 8]
    }

    /// Returns true if this set represents a dominant seventh chord
    pub fn is_dominant_seventh(&self) -> bool {
        self.prime_form() == vec![0, 4, 7, 10]
    }

    /// Returns true if this set represents a major seventh chord
    pub fn is_major_seventh(&self) -> bool {
        self.prime_form() == vec![0, 4, 7, 11]
    }

    /// Returns true if this set represents a minor seventh chord
    pub fn is_minor_seventh(&self) -> bool {
        self.prime_form() == vec![0, 3, 7, 10]
    }

    /// Returns true if this set represents a half-diminished seventh chord
    pub fn is_half_diminished_seventh(&self) -> bool {
        self.prime_form() == vec![0, 3, 6, 10]
    }

    /// Returns true if this set represents a fully diminished seventh chord
    pub fn is_fully_diminished_seventh(&self) -> bool {
        self.prime_form() == vec![0, 3, 6, 9]
    }
}

impl fmt::Display for PitchClassSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        let notes: Vec<String> = self.0.iter().map(|n| n.to_string()).collect();
        write!(f, "{}", notes.join(", "))?;
        write!(f, "}}")
    }
}

impl From<Vec<NoteName>> for PitchClassSet {
    fn from(notes: Vec<NoteName>) -> Self {
        PitchClassSet::new(&notes)
    }
}

impl From<Vec<Pitch>> for PitchClassSet {
    fn from(pitches: Vec<Pitch>) -> Self {
        PitchClassSet::from_pitches(&pitches)
    }
}