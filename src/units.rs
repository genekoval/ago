use super::{
    Duration, Tense,
    Unit::{self, Second},
};

use chrono::TimeDelta;

pub struct RelativeUnits<U> {
    pub(crate) delta: TimeDelta,
    pub(crate) unit: U,
    pub(crate) tense: Tense,
}

impl<U> RelativeUnits<U> {
    pub(crate) fn new(delta: TimeDelta, unit: U) -> Self {
        Self {
            delta: delta.abs(),
            unit,
            tense: delta.into(),
        }
    }
}

impl<U> Iterator for RelativeUnits<U>
where
    U: Iterator<Item = Unit>,
{
    type Item = Duration;

    fn next(&mut self) -> Option<Self::Item> {
        let unit = self.unit.next()?;

        let duration = if unit >= Second {
            let seconds = self.delta.num_seconds();
            let seconds_per_unit = unit.in_seconds();
            let count = seconds / seconds_per_unit;

            self.delta -=
                TimeDelta::try_seconds(count * seconds_per_unit).unwrap();

            Duration { count, unit }
        } else {
            let nanos = self.delta.subsec_nanos() as i64;
            let nanos_per_unit = unit.in_nanoseconds();
            let count = nanos / nanos_per_unit;

            self.delta -= TimeDelta::nanoseconds(count * nanos_per_unit);

            Duration { count, unit }
        };

        Some(duration)
    }
}
