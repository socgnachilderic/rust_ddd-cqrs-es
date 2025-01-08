use chrono::Utc;
use shared_kernel::domain::date::{Date, IDateProvider};

#[derive(Clone)]
pub struct ChronoDateProvider;

impl IDateProvider for ChronoDateProvider {
    fn now(&self) -> Date {
        let chrono_now = Utc::now().to_rfc3339();

        Date::new(&chrono_now)
    }

    fn parse(&self, date_str: &str) -> Result<Date, String> {
        Ok(Date::new(date_str))
    }
}
