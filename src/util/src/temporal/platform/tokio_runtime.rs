/*!
This module targets the Tokio runtime, but there is
currently no rely on the Tokio runtime.
*/

use std::{time::Duration, ops::{Add, AddAssign, Sub, SubAssign}};

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct Instant(std::time::SystemTime);

impl Instant {
    pub const EPOCH: Instant = Instant(std::time::SystemTime::UNIX_EPOCH);

    pub fn since(&self, other: Instant) -> Duration {
        self.0.duration_since(other.0).unwrap_or(Duration::from_nanos(0))
    }

    pub fn now() -> Instant {
        Instant(std::time::SystemTime::now())
    }

    pub fn epoch(&self) -> Duration {
        self.0.duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap_or(Duration::from_nanos(0))
    }

    pub fn try_add(&self, duration: Duration) -> Option<Instant> {
        Some(Instant(self.0.checked_add(duration)?))
    }

    pub fn try_subtract(&self, duration: Duration) -> Option<Instant> {
        Some(Instant(self.0.checked_sub(duration)?))
    }
}

impl Add<Duration> for Instant {
    type Output = Instant;
    fn add(self, rhs: Duration) -> Self::Output {
        Instant(self.0 + rhs)
    }
}

impl AddAssign<Duration> for Instant {
    fn add_assign(&mut self, rhs: Duration) {
        self.0 += rhs;
    }
}

impl Sub<Duration> for Instant {
    type Output = Instant;
    fn sub(self, rhs: Duration) -> Self::Output {
        Instant(self.0 - rhs)
    }
}

impl SubAssign<Duration> for Instant {
    fn sub_assign(&mut self, rhs: Duration) {
        self.0 -= rhs;
    }
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum ZonedDateTimeInner {
    UtcTz(chrono::DateTime<chrono::Utc>),
}

impl ZonedDateTimeInner {
    pub fn epoch(&self) -> crate::temporal::Duration {
        match self {
            ZonedDateTimeInner::UtcTz(dt) => crate::temporal::Duration::from_milliseconds(dt.timestamp_millis()),
        }
    }
}