use super::Unit;

use std::fmt::{self, Display};

#[derive(Clone, Copy, Debug)]
pub struct Duration {
    pub count: i64,
    pub unit: Unit,
}

impl Duration {
    pub fn abbrev(self) -> String {
        format!("{}{}", self.count, self.unit.symbol())
    }

    pub fn long_format(self) -> String {
        self.to_string()
    }
}

impl Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}{}",
            self.count,
            self.unit,
            match self.count {
                1 => "",
                _ => "s",
            }
        )
    }
}
