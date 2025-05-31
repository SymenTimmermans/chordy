use crate::{Chord, Interval, NoteName};

/// A trait for musical structures that have a root note
pub trait HasRoot {
    fn root(&self) -> NoteName;

    /// Optional: Provides mutable access to root
    fn root_mut(&mut self) -> &mut NoteName {
        unimplemented!("Mutable root access not implemented for this type")
    }
}

/// A trait for musical structures that contain intervals
pub trait HasIntervals {
    fn intervals(&self) -> &[Interval];

    /// Optional: Provides mutable access to intervals
    fn intervals_mut(&mut self) -> &mut Vec<Interval> {
        unimplemented!("Mutable interval access not implemented for this type")
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
    fn notes_iter(&self) -> impl Iterator<Item = NoteName> + '_ {
        self.intervals()
            .iter()
            .map(move |&interval| self.root() + interval)
    }

    fn notes(&self) -> Vec<NoteName> {
        self.notes_iter().collect()
    }

    /// Checks if the chord contains a specific interval
    fn contains_interval(&self, interval: Interval) -> bool {
        self.intervals().contains(&interval)
    }

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

/// Blanket implementation for anything that has root and intervals
impl<T> ChordLike for T where T: HasRoot + HasIntervals {}

/// A trait to formalize the torsor relationship
pub trait Torsor<Group> {
    /// The action: torsor + group_element → torsor
    fn act(&self, g: Group) -> Self;

    /// The difference: torsor - torsor → group_element
    fn difference(&self, other: &Self) -> Group;
}
