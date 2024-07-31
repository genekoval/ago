use chrono::TimeDelta;
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Tense {
    Past,
    Present,
    Future,
}

impl From<TimeDelta> for Tense {
    fn from(value: TimeDelta) -> Self {
        const ZERO: TimeDelta = TimeDelta::zero();

        match value.cmp(&ZERO) {
            Ordering::Less => Self::Future,
            Ordering::Equal => Self::Present,
            Ordering::Greater => Self::Past,
        }
    }
}
