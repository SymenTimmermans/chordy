//! Traits for musical structures
use crate::{Chord, Interval, NoteName, ChordQuality};

/// A trait for musical structures that have a root note
pub trait HasRoot {
    /// Returns the root note of the structure
    fn root(&self) -> NoteName;

    /// Optional: Provides mutable access to root
    fn root_mut(&mut self) -> &mut NoteName {
        unimplemented!("Mutable root access not implemented for this type")
    }
}

/// A trait for musical structures that contain intervals
pub trait HasIntervals {
    /// Returns a slice of intervals contained in the structure
    fn intervals(&self) -> &[Interval];

    /// Checks if the structure contains a specific interval
    fn contains_interval(&self, interval: Interval) -> bool {
        self.intervals().contains(&interval)
    }

    /// Detects the chord quality based on the intervals
    /// 
    /// Analyzes the third and fifth intervals to determine the chord quality.
    /// Returns None if the quality cannot be determined (e.g., no clear third).
    fn quality(&self) -> Option<ChordQuality> {
        ChordQuality::from_intervals(self.intervals())
    }

    /// Calculates the dissonance level of the intervals
    /// 
    /// Returns a value between 0.0 (perfectly consonant) and 1.0 (highly dissonant).
    /// The calculation considers:
    /// - Dissonance of intervals from the root
    /// - Dissonance between adjacent intervals in the chord
    /// 
    /// # Examples
    /// 
    /// ```
    /// use chordy::prelude::*;
    /// 
    /// let c_major = Chord::major(note!("C"));
    /// assert!(c_major.dissonance_level() < 0.2); // Very consonant
    /// 
    /// let c_dim = Chord::diminished(note!("C"));
    /// assert!(c_dim.dissonance_level() > 0.5); // Quite dissonant
    /// ```
    fn dissonance_level(&self) -> f32 {
        use crate::Interval;
        
        let intervals = self.intervals();
        if intervals.is_empty() {
            return 0.0;
        }
        
        // Define dissonance weights for intervals
        let interval_dissonance = |interval: &Interval| -> f32 {
            match interval.semitones() % 12 {
                0 => 0.0,   // Unison/Octave - perfect consonance
                1 => 0.8,   // Minor second - harsh dissonance
                2 => 0.4,   // Major second - mild dissonance
                3 => 0.15,  // Minor third - imperfect consonance
                4 => 0.1,   // Major third - imperfect consonance
                5 => 0.2,   // Perfect fourth - contextual
                6 => 0.9,   // Tritone - strong dissonance
                7 => 0.0,   // Perfect fifth - perfect consonance
                8 => 0.45,  // Augmented fifth/Minor sixth - moderately dissonant
                9 => 0.15,  // Major sixth - imperfect consonance
                10 => 0.35, // Minor seventh - mild dissonance
                11 => 0.4,  // Major seventh - mild dissonance
                _ => 0.5,   // Should not happen
            }
        };
        
        // Calculate dissonance from root
        let mut total_dissonance = 0.0;
        let mut count = 0.0;
        
        for interval in intervals {
            // Skip unison
            if interval.semitones() != 0 {
                let mut weight = interval_dissonance(interval);
                
                // Add extra weight for extended intervals
                if interval.semitones() > 12 {
                    weight += 0.1; // 9ths, 11ths, 13ths add tension
                }
                
                total_dissonance += weight;
                count += 1.0;
            }
        }
        
        // Calculate dissonance between chord tones
        let notes: Vec<_> = intervals.iter()
            .map(|i| i.semitones() % 12)
            .collect();
            
        for i in 0..notes.len() {
            for j in i+1..notes.len() {
                let interval_between = (notes[j] - notes[i] + 12) % 12;
                let dissonance = match interval_between {
                    1 => 0.7,  // Minor second between chord tones
                    2 => 0.3,  // Major second between chord tones
                    6 => 0.6,  // Tritone between chord tones
                    11 => 0.5, // Major seventh between chord tones
                    _ => 0.0,
                };
                if dissonance > 0.0 {
                    total_dissonance += dissonance * 0.5; // Weight internal dissonances less
                    count += 0.5;
                }
            }
        }
        
        // Normalize to 0.0-1.0 range
        if count > 0.0 {
            (total_dissonance / count).min(1.0)
        } else {
            0.0
        }
    }

    /// Sets the intervals of the structure, preserving other properties (like bass notes)
    /// 
    /// This method allows modifying the intervals while maintaining the structural integrity
    /// of the musical object (root note, bass note, etc.). This is preferred over creating
    /// new objects which may lose important context.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use chordy::prelude::*;
    /// 
    /// let mut chord = Chord::major(note!("C")).with_slash_bass(note!("E"));
    /// let original_bass = chord.bass_note();
    /// 
    /// // Add a 7th while preserving the bass note
    /// chord.set_intervals(vec![
    ///     Interval::PERFECT_UNISON,
    ///     Interval::MAJOR_THIRD,
    ///     Interval::PERFECT_FIFTH,
    ///     Interval::MINOR_SEVENTH,
    /// ]);
    /// 
    /// assert_eq!(chord.bass_note(), original_bass); // Bass preserved!
    /// assert!(chord.contains_interval(Interval::MINOR_SEVENTH));
    /// ```
    fn set_intervals(&mut self, _intervals: Vec<Interval>) {
        unimplemented!("set_intervals not implemented for this type")
    }

    /// Removes a specific interval from the structure
    /// 
    /// This method removes the first occurrence of the specified interval while
    /// preserving other properties like bass notes. If the interval is not found,
    /// the structure remains unchanged.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use chordy::prelude::*;
    /// 
    /// let mut chord = Chord::major(note!("C")).with_slash_bass(note!("F"));
    /// let original_bass = chord.bass_note();
    /// 
    /// chord.remove_interval(Interval::PERFECT_FIFTH);
    /// 
    /// assert_eq!(chord.bass_note(), original_bass); // Bass preserved!
    /// assert!(!chord.contains_interval(Interval::PERFECT_FIFTH));
    /// ```
    fn remove_interval(&mut self, _interval: Interval) {
        unimplemented!("remove_interval not implemented for this type")
    }

    /// Adds an interval to the structure if it doesn't already exist
    /// 
    /// This method adds the specified interval while preserving other properties
    /// like bass notes. If the interval already exists, the structure remains unchanged.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use chordy::prelude::*;
    /// 
    /// let mut chord = Chord::major(note!("C")).with_slash_bass(note!("F"));
    /// let original_bass = chord.bass_note();
    /// 
    /// // Add a 7th
    /// chord.add_interval(Interval::MINOR_SEVENTH);
    /// 
    /// assert_eq!(chord.bass_note(), original_bass); // Bass preserved!
    /// assert!(chord.contains_interval(Interval::MINOR_SEVENTH));
    /// ```
    fn add_interval(&mut self, _interval: Interval) {
        unimplemented!("add_interval not implemented for this type")
    }
}

/// A trait for structures that can be inverted
pub trait Invertible {
    /// Returns a new inverted version
    fn inverted(&self, inversion: u8) -> Self;

    /// Inverts in-place (if mutable access is available)
    fn invert(&mut self, inversion: u8)
    where
        Self: Sized,
    {
        *self = self.inverted(inversion);
    }
}

/// A trait for structures that can be transposed
pub trait Transposable {
    /// Returns a new transposed version
    fn transposed(&self, interval: Interval) -> Self;

    /// Transposes in-place (if mutable access is available)
    fn transpose(&mut self, interval: Interval)
    where
        Self: Sized,
    {
        *self = self.transposed(interval);
    }
}

/// A combined trait for chord-like structures
pub trait ChordLike: HasRoot + HasIntervals {
    /// Returns an iterator of NoteName within this structure based on the root note and intervals.
    fn notes_iter(&self) -> impl Iterator<Item = NoteName> + '_ {
        self.intervals()
            .iter()
            .map(move |&interval| self.root() + interval)
    }

    /// Returns a vector of NoteName within this structure based on the root note and intervals.
    fn notes(&self) -> Vec<NoteName> {
        self.notes_iter().collect()
    }

    /// Returns an iterator over all possible triads that can be built from this type's
    /// root and intervals.
    ///
    /// Each triad will have:
    /// - A root from one scale degree
    /// - A third from another scale degree
    /// - A fifth from another scale degree
    fn triads(&self) -> impl Iterator<Item = Chord> + '_ {
        let intervals = self.intervals();
        let tonic = self.root();

        (0..intervals.len()).flat_map(move |i| {
            (0..intervals.len()).flat_map(move |j| {
                (0..intervals.len()).filter_map(move |k| {
                    if i == j || i == k || j == k {
                        return None;
                    }

                    let root_interval = intervals[i];
                    let third_interval = intervals[j] - root_interval;
                    let fifth_interval = intervals[k] - root_interval;

                    if third_interval.is_third() && fifth_interval.is_fifth() {
                        let root = tonic + root_interval;
                        Some(Chord::new(
                            root,
                            vec![Interval::PERFECT_UNISON, third_interval, fifth_interval],
                        ))
                    } else {
                        None
                    }
                })
            })
        })
    }

    /// Returns an iterator over all possible seventh chords that can be built from this type's
    /// root and intervals.
    ///
    /// Each seventh chord will have:
    /// - A root from one scale degree
    /// - A third from another scale degree
    /// - A fifth from another scale degree
    /// - A seventh from yet another scale degree
    fn sevenths(&self) -> impl Iterator<Item = Chord> + '_ {
        let intervals = self.intervals();
        let tonic = self.root();
        
        (0..intervals.len()).flat_map(move |i| {
            (0..intervals.len()).flat_map(move |j| {
                (0..intervals.len()).flat_map(move |k| {
                    (0..intervals.len()).filter_map(move |l| {
                        // Skip if any indices are equal
                        if i == j || i == k || i == l || j == k || j == l || k == l {
                            return None;
                        }
                        
                        let root_interval = intervals[i];
                        let third_interval = intervals[j] - root_interval;
                        let fifth_interval = intervals[k] - root_interval;
                        let seventh_interval = intervals[l] - root_interval;
                        
                        if third_interval.is_third() && fifth_interval.is_fifth() && seventh_interval.is_seventh() {
                            let root = tonic + root_interval;
                            Some(Chord::new(
                                root,
                                vec![
                                    Interval::PERFECT_UNISON,
                                    third_interval,
                                    fifth_interval,
                                    seventh_interval
                                ]
                            ))
                        } else {
                            None
                        }
                    })
                })
            })
        })
    }
}

// Blanket implementations for ergonomics
impl<T: HasRoot + HasIntervals> ChordLike for T {}

/// Auto-implement Transposable for all ChordLike types
impl<T: ChordLike + Clone> Transposable for T {
    fn transposed(&self, interval: Interval) -> Self {
        let mut new = self.clone();
        *new.root_mut() = new.root() + interval;
        new
    }
}

/// A trait to formalize the torsor relationship
pub trait Torsor<Group> {
    /// The action: torsor + group_element → torsor
    fn act(&self, g: Group) -> Self;

    /// The difference: torsor - torsor → group_element
    fn difference(&self, other: &Self) -> Group;
}
