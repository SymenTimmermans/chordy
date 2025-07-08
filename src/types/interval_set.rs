//! Stack-allocated interval storage for musical structures
//!
//! IntervalSet provides a unified, efficient way to store intervals for chords, scales,
//! and other musical structures. It uses const generics to allow compile-time sizing
//! while maintaining stack allocation for performance.

use super::Interval;
use std::fmt;

/// A stack-allocated set of musical intervals
///
/// This type provides efficient storage for intervals used in chords, scales, and other
/// musical structures. It avoids heap allocation by using a fixed-size array with
/// runtime length tracking.
///
/// # Type Parameters
/// - `N`: The maximum number of intervals that can be stored (default: 10)
///
/// # Examples
/// ```
/// use chordy::{IntervalSet, Interval};
/// 
/// // Create an empty interval set (default size of 10)
/// let mut intervals: IntervalSet = IntervalSet::new();
/// 
/// // Add some intervals
/// intervals.push(Interval::PERFECT_UNISON);
/// intervals.push(Interval::MAJOR_THIRD);
/// intervals.push(Interval::PERFECT_FIFTH);
/// 
/// // Check if an interval is present
/// assert!(intervals.contains(Interval::MAJOR_THIRD));
/// 
/// // Iterate over intervals
/// for interval in intervals.iter() {
///     println!("{}", interval);
/// }
/// ```
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct IntervalSet<const N: usize = 10> {
    /// The intervals stored in the set
    intervals: [Interval; N],
    /// The number of intervals actually stored
    len: u8,
}

impl<const N: usize> IntervalSet<N> {
    /// Create a new empty IntervalSet
    pub const fn new() -> Self {
        Self {
            intervals: [Interval::PERFECT_UNISON; N],
            len: 0,
        }
    }
    
    /// Create an IntervalSet with specific intervals and length (for const contexts)
    pub const fn const_from_array(intervals: [Interval; N], len: u8) -> Self {
        Self { intervals, len }
    }
    
    /// Create an IntervalSet from a slice of intervals
    ///
    /// # Panics
    /// Panics if the slice contains more than N intervals
    pub fn from_slice(intervals: &[Interval]) -> Self {
        assert!(intervals.len() <= N, "Too many intervals for IntervalSet<{}>", N);
        
        let mut set = Self::new();
        for &interval in intervals {
            set.push(interval);
        }
        set
    }
    
    
    /// Get the number of intervals in the set
    pub const fn len(&self) -> usize {
        self.len as usize
    }
    
    /// Check if the set is empty
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }
    
    /// Check if the set contains a specific interval
    pub fn contains(&self, interval: Interval) -> bool {
        self.intervals[..self.len()].contains(&interval)
    }
    
    /// Add an interval to the set
    ///
    /// # Panics
    /// Panics if the set is already full (contains N intervals)
    pub fn push(&mut self, interval: Interval) {
        assert!((self.len as usize) < N, "IntervalSet is full");
        self.intervals[self.len as usize] = interval;
        self.len += 1;
    }
    
    /// Get an iterator over the intervals
    pub fn iter(&self) -> impl Iterator<Item = Interval> + '_ {
        self.intervals[..self.len()].iter().copied()
    }
    
    /// Get a slice of the intervals
    pub fn as_slice(&self) -> &[Interval] {
        &self.intervals[..self.len()]
    }
    
    /// Clear all intervals from the set
    pub fn clear(&mut self) {
        self.len = 0;
    }
    
    /// Remove duplicate intervals, keeping only unique values
    pub fn dedup(&mut self) {
        if self.len <= 1 {
            return;
        }
        
        let mut write_idx = 1;
        for read_idx in 1..self.len() {
            let interval = self.intervals[read_idx];
            if !self.intervals[..write_idx].contains(&interval) {
                self.intervals[write_idx] = interval;
                write_idx += 1;
            }
        }
        self.len = write_idx as u8;
    }
    
    /// Create a new IntervalSet containing the union of two sets
    pub fn union(&self, other: &Self) -> Self {
        let mut result = *self;
        for interval in other.iter() {
            if !result.contains(interval) && (result.len as usize) < N {
                result.push(interval);
            }
        }
        result
    }
    
    /// Create a new IntervalSet containing the intersection of two sets
    pub fn intersection(&self, other: &Self) -> Self {
        let mut result = Self::new();
        for interval in self.iter() {
            if other.contains(interval) {
                result.push(interval);
            }
        }
        result
    }
    
    /// Sort the intervals by their semitone value
    pub fn sort(&mut self) {
        let len = self.len();
        self.intervals[..len].sort_by_key(|i| i.semitones());
    }
    
    /// Check if this set is a subset of another
    pub fn is_subset(&self, other: &Self) -> bool {
        self.iter().all(|interval| other.contains(interval))
    }
    
    /// Check if this set is a superset of another
    pub fn is_superset(&self, other: &Self) -> bool {
        other.is_subset(self)
    }
}

impl<const N: usize> Default for IntervalSet<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> fmt::Display for IntervalSet<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for (i, interval) in self.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", interval)?;
        }
        write!(f, "]")
    }
}

impl<const N: usize> FromIterator<Interval> for IntervalSet<N> {
    fn from_iter<T: IntoIterator<Item = Interval>>(iter: T) -> Self {
        let mut set = Self::new();
        for interval in iter {
            set.push(interval);
        }
        set
    }
}

impl<const N: usize> IntoIterator for IntervalSet<N> {
    type Item = Interval;
    type IntoIter = std::iter::Take<std::array::IntoIter<Interval, N>>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.intervals.into_iter().take(self.len())
    }
}

impl<'a, const N: usize> IntoIterator for &'a IntervalSet<N> {
    type Item = Interval;
    type IntoIter = std::iter::Copied<std::slice::Iter<'a, Interval>>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.as_slice().iter().copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_new_empty() {
        let set: IntervalSet = IntervalSet::new();
        assert_eq!(set.len(), 0);
        assert!(set.is_empty());
    }
    
    #[test]
    fn test_push_and_contains() {
        let mut set: IntervalSet = IntervalSet::new();
        set.push(Interval::MAJOR_THIRD);
        set.push(Interval::PERFECT_FIFTH);
        
        assert_eq!(set.len(), 2);
        assert!(set.contains(Interval::MAJOR_THIRD));
        assert!(set.contains(Interval::PERFECT_FIFTH));
        assert!(!set.contains(Interval::MINOR_THIRD));
    }
    
    #[test]
    fn test_from_slice() {
        let intervals = &[
            Interval::PERFECT_UNISON,
            Interval::MAJOR_THIRD,
            Interval::PERFECT_FIFTH,
        ];
        let set: IntervalSet = IntervalSet::from_slice(intervals);
        
        assert_eq!(set.len(), 3);
        assert!(set.contains(Interval::PERFECT_UNISON));
        assert!(set.contains(Interval::MAJOR_THIRD));
        assert!(set.contains(Interval::PERFECT_FIFTH));
    }
    
    #[test]
    fn test_dedup() {
        let mut set: IntervalSet = IntervalSet::new();
        set.push(Interval::MAJOR_THIRD);
        set.push(Interval::PERFECT_FIFTH);
        set.push(Interval::MAJOR_THIRD);
        set.push(Interval::PERFECT_FIFTH);
        
        assert_eq!(set.len(), 4);
        set.dedup();
        assert_eq!(set.len(), 2);
        assert!(set.contains(Interval::MAJOR_THIRD));
        assert!(set.contains(Interval::PERFECT_FIFTH));
    }
    
    #[test]
    fn test_union() {
        let mut set1: IntervalSet = IntervalSet::new();
        set1.push(Interval::MAJOR_THIRD);
        set1.push(Interval::PERFECT_FIFTH);
        
        let mut set2: IntervalSet = IntervalSet::new();
        set2.push(Interval::PERFECT_FIFTH);
        set2.push(Interval::MAJOR_SEVENTH);
        
        let union = set1.union(&set2);
        assert_eq!(union.len(), 3);
        assert!(union.contains(Interval::MAJOR_THIRD));
        assert!(union.contains(Interval::PERFECT_FIFTH));
        assert!(union.contains(Interval::MAJOR_SEVENTH));
    }
    
    #[test]
    fn test_intersection() {
        let mut set1: IntervalSet = IntervalSet::new();
        set1.push(Interval::MAJOR_THIRD);
        set1.push(Interval::PERFECT_FIFTH);
        set1.push(Interval::MAJOR_SEVENTH);
        
        let mut set2: IntervalSet = IntervalSet::new();
        set2.push(Interval::PERFECT_FIFTH);
        set2.push(Interval::MAJOR_SEVENTH);
        set2.push(Interval::MAJOR_NINTH);
        
        let intersection = set1.intersection(&set2);
        assert_eq!(intersection.len(), 2);
        assert!(intersection.contains(Interval::PERFECT_FIFTH));
        assert!(intersection.contains(Interval::MAJOR_SEVENTH));
        assert!(!intersection.contains(Interval::MAJOR_THIRD));
        assert!(!intersection.contains(Interval::MAJOR_NINTH));
    }
    
    #[test]
    fn test_copy_semantics() {
        let set1: IntervalSet = IntervalSet::from_slice(&[
            Interval::PERFECT_UNISON,
            Interval::MAJOR_THIRD,
            Interval::PERFECT_FIFTH,
        ]);
        
        let set2 = set1; // Copy, not move
        let set3 = set1; // Can still use set1
        
        assert_eq!(set1.len(), 3);
        assert_eq!(set2.len(), 3);
        assert_eq!(set3.len(), 3);
    }
    
    #[test]
    fn test_const_generic_sizes() {
        // Test with different sizes
        let small_set: IntervalSet<5> = IntervalSet::new();
        let large_set: IntervalSet<20> = IntervalSet::new();
        
        assert_eq!(small_set.len(), 0);
        assert_eq!(large_set.len(), 0);
    }
}