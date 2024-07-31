use super::{
    unit::{Unit, UNITS},
    Duration, RelativeUnits, Tense,
};

use chrono::TimeDelta;

pub struct RelativeBuilder {
    pub delta: TimeDelta,
    pub granularity: Unit,
    pub units: usize,
    pub tense: bool,
}

impl RelativeBuilder {
    pub fn new(delta: TimeDelta) -> Self {
        Self {
            delta,
            granularity: Unit::Nanosecond,
            units: 1,
            tense: false,
        }
    }

    pub fn granularity(mut self, granularity: Unit) -> Self {
        self.granularity = granularity;
        self
    }

    pub fn units(mut self, units: usize) -> Self {
        self.units = units;
        self
    }

    pub fn with_tense(mut self, tense: bool) -> Self {
        self.tense = tense;
        self
    }

    pub fn iter(&self) -> RelativeUnits<impl Iterator<Item = Unit>> {
        let delta = self.delta;
        let granularity = self.granularity;
        let units = UNITS
            .into_iter()
            .take_while(move |&unit| unit >= granularity);

        RelativeUnits::new(delta, units)
    }

    pub fn format<F>(&self, f: F) -> String
    where
        F: Fn(Duration) -> String,
    {
        let relative = self.iter();
        let tense = relative.tense;
        let duration = relative
            .filter(|&duration| duration.count > 0)
            .map(f)
            .take(self.units)
            .collect::<Vec<_>>()
            .join(", ");

        if self.tense {
            match tense {
                Tense::Past => format!("{duration} ago"),
                Tense::Present => "now".into(),
                Tense::Future => format!("in {duration}"),
            }
        } else if tense == Tense::Present {
            "now".into()
        } else {
            duration
        }
    }

    pub fn long_format(&self) -> String {
        self.format(|duration| duration.long_format())
    }

    pub fn abbrev(&self) -> String {
        self.format(|duration| duration.abbrev())
    }
}
