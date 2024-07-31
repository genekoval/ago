use super::RelativeBuilder;

use chrono::{DateTime, Local, TimeDelta, TimeZone, Utc};

trait Now: TimeZone {
    fn now() -> DateTime<Self>;
}

impl Now for Local {
    fn now() -> DateTime<Self> {
        Local::now()
    }
}

impl Now for Utc {
    fn now() -> DateTime<Self> {
        Utc::now()
    }
}

pub trait Relative: Sized {
    fn relative(self) -> RelativeBuilder;
}

impl Relative for TimeDelta {
    fn relative(self) -> RelativeBuilder {
        RelativeBuilder::new(self)
    }
}

impl<Tz: Now> Relative for DateTime<Tz> {
    fn relative(self) -> RelativeBuilder {
        (Tz::now() - self).relative()
    }
}
