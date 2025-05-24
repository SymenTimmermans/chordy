use crate::Interval;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScaleBitmask(pub u16);

impl ScaleBitmask {
    pub fn from_intervals(intervals: &[Interval]) -> Self {
        let mut mask = 0u16;
        let mut semitones = 0u8;
        for interval in intervals {
            assert!(interval.semitones() >= 0, "Cannot have negative intervals in a bitmask");
            semitones = semitones.wrapping_add(interval.semitones() as u8);
            mask |= 1 << (semitones % 12);
        }
        ScaleBitmask(mask)
    }

    pub fn contains(&self, pitch_class: u8) -> bool {
        self.0 & (1 << (pitch_class % 12)) != 0
    }
}
