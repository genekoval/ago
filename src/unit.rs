use Unit::*;

use std::fmt::{self, Display};

const NANOS_PER_MICROSECOND: i64 = 1000;
const NANOS_PER_MILLISECOND: i64 = NANOS_PER_MICROSECOND * 1000;
const NANOS_PER_SECOND: i64 = NANOS_PER_MILLISECOND * 1000;
const NANOS_PER_MINUTE: i64 = NANOS_PER_SECOND * 60;
const NANOS_PER_HOUR: i64 = NANOS_PER_MINUTE * 60;
const NANOS_PER_DAY: i64 = NANOS_PER_HOUR * 24;
const NANOS_PER_WEEK: i64 = NANOS_PER_DAY * 7;
const NANOS_PER_MONTH: i64 = NANOS_PER_DAY * 30;
const NANOS_PER_YEAR: i64 = NANOS_PER_DAY * 365;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(i64)]
pub enum Unit {
    Year = NANOS_PER_YEAR,
    Month = NANOS_PER_MONTH,
    Week = NANOS_PER_WEEK,
    Day = NANOS_PER_DAY,
    Hour = NANOS_PER_HOUR,
    Minute = NANOS_PER_MINUTE,
    Second = NANOS_PER_SECOND,
    Millisecond = NANOS_PER_MILLISECOND,
    Microsecond = NANOS_PER_MICROSECOND,
    Nanosecond = 1,
}

impl Unit {
    pub fn in_nanoseconds(self) -> i64 {
        self as i64
    }

    pub fn in_seconds(self) -> i64 {
        self.in_nanoseconds() / NANOS_PER_SECOND
    }

    pub fn full(self) -> &'static str {
        match self {
            Year => "year",
            Month => "month",
            Week => "week",
            Day => "day",
            Hour => "hour",
            Minute => "minute",
            Second => "second",
            Millisecond => "millisecond",
            Microsecond => "microsecond",
            Nanosecond => "nanosecond",
        }
    }

    pub fn symbol(self) -> &'static str {
        match self {
            Year => "y",
            Month => "mo",
            Week => "w",
            Day => "d",
            Hour => "h",
            Minute => "mi",
            Second => "s",
            Millisecond => "ms",
            Microsecond => "μs",
            Nanosecond => "ns",
        }
    }
}

impl Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.full())
    }
}

pub const UNITS: [Unit; 10] = [
    Year,
    Month,
    Week,
    Day,
    Hour,
    Minute,
    Second,
    Millisecond,
    Microsecond,
    Nanosecond,
];
