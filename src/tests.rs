use crate::*;

use chrono::{Local, TimeDelta};

use Unit::*;

macro_rules! micros {
    ($n:expr) => {
        TimeDelta::microseconds($n)
    };
}

macro_rules! millis {
    ($n:expr) => {
        TimeDelta::try_milliseconds($n).unwrap()
    };
}

macro_rules! seconds {
    ($n:expr) => {
        TimeDelta::try_seconds($n).unwrap()
    };
}

macro_rules! hours {
    ($n:expr) => {
        TimeDelta::try_hours($n).unwrap()
    };
}

macro_rules! days {
    ($n:expr) => {
        TimeDelta::try_days($n).unwrap()
    };
}

#[test]
fn now() {
    let duration = TimeDelta::zero().relative().long_format();
    assert_eq!(duration, "now");
}

#[test]
fn granularity() {
    let delta = hours!(4) + millis!(500);

    let mut duration = delta.relative().units(2).long_format();
    assert_eq!(duration, "4 hours, 500 milliseconds");

    duration = delta.relative().units(2).granularity(Second).long_format();
    assert_eq!(duration, "4 hours");
}

#[test]
fn units() {
    let delta = seconds!(150);

    let mut duration = delta.relative().units(2).long_format();
    assert_eq!(duration, "2 minutes, 30 seconds");

    duration = delta.relative().units(1).long_format();
    assert_eq!(duration, "2 minutes");
}

#[test]
fn abbrev() {
    let delta = days!(39) + micros!(120);

    let mut duration = delta.relative().units(4).abbrev();
    assert_eq!(duration, "1mo, 1w, 2d, 120μs");

    duration = micros!(1050).relative().units(3).abbrev();
    assert_eq!(duration, "1ms, 50μs")
}

#[test]
fn date_time() {
    let now = Local::now();
    let delta = days!(1) + hours!(12);

    assert_eq!(
        (now - delta)
            .relative()
            .units(3)
            .granularity(Second)
            .with_tense(true)
            .long_format(),
        "1 day, 12 hours ago"
    );
    assert_eq!(
        (now + delta)
            .relative()
            .units(3)
            .granularity(Second)
            .with_tense(true)
            .long_format(),
        "in 1 day, 11 hours, 59 minutes"
    );
}
