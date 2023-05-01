use chrono::{DateTime, Duration, Utc};

pub struct MinuteDateRange(pub DateTime<Utc>, pub DateTime<Utc>);

impl Iterator for MinuteDateRange {
    type Item = DateTime<Utc>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 <= self.1 {
            let next = self.0 + Duration::minutes(1);
            Some(std::mem::replace(&mut self.0, next))
        } else {
            None
        }
    }
}
