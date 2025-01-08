use std::fmt::Debug;

use serde::{Deserialize, Serialize};

pub trait IDateProvider: Send + Sync {
    fn now(&self) -> Date;
    fn parse(&self, date_str: &str) -> Result<Date, String>;
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Date(String);

impl Date {
    pub fn new(date_str: &str) -> Self {
        Self(date_str.to_string())
    }

    pub fn to_iso8601(&self) -> String {
        self.0.clone()
    }
}
