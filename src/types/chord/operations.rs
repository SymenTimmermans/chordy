//! Chord manipulation and fluent interface operations
//!
//! This module contains methods for manipulating chords:
//! - Fluent interface methods for method chaining
//! - Interval manipulation
//! - Transposition
//! - Chord type conversions

use crate::{
    traits::Transposable,
    types::{Chord, Interval, IntervalSet},
};

impl Chord {
    // Fluent interface methods for method chaining

    /// Add an interval to this chord (fluent interface)
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Chord, Interval, note};
    ///
    /// let chord = Chord::major(note!("C"))
    ///     .with_interval(Interval::MINOR_SEVENTH);
    /// // Creates a C7 chord
    /// ```
    pub fn with_interval(mut self, interval: Interval) -> Self {
        if !self.intervals.contains(interval) {
            self.intervals.push(interval);
            self.intervals.sort();
        }
        self
    }

    /// Remove an interval from this chord (fluent interface)
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Chord, Interval, note};
    ///
    /// let chord = Chord::major(note!("C"))
    ///     .without_interval(Interval::PERFECT_FIFTH);
    /// // Creates a C chord without the fifth (C power chord)
    /// ```
    pub fn without_interval(self, interval: Interval) -> Self {
        let mut new_intervals = IntervalSet::new();
        for i in self.intervals.iter() {
            if i != interval {
                new_intervals.push(i);
            }
        }
        Self::from_interval_set(self.root, new_intervals)
    }

    /// Transpose this chord (fluent interface)
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Chord, Interval, note};
    ///
    /// let chord = Chord::major(note!("C"))
    ///     .transposed_by(Interval::MAJOR_THIRD);
    /// // Creates an E major chord
    /// ```
    pub fn transposed_by(mut self, interval: Interval) -> Self {
        self.transpose(interval);
        self
    }

    /// Convert to a different chord type while keeping the root (fluent interface)
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Chord, note};
    ///
    /// let minor_chord = Chord::major(note!("C"))
    ///     .as_minor();
    /// // Converts C major to C minor
    /// ```
    pub fn as_minor(self) -> Self {
        Chord::minor(self.root)
    }

    /// Convert to major chord while keeping the root (fluent interface)
    pub fn as_major(self) -> Self {
        Chord::major(self.root)
    }

    /// Convert to diminished chord while keeping the root (fluent interface)
    pub fn as_diminished(self) -> Self {
        Chord::diminished(self.root)
    }

    /// Convert to augmented chord while keeping the root (fluent interface)
    pub fn as_augmented(self) -> Self {
        Chord::augmented(self.root)
    }
}