use std::{time::Duration, ops::{Add, AddAssign, Sub, SubAssign}};

use crate::temporal::RangeError;

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct Instant {
    epoch_ms: u128,
}

impl Instant {
    pub const EPOCH: Instant = Instant { epoch_ms: 0 };

    pub fn since(&self, other: Instant) -> Duration {
        if self.epoch_ms < other.epoch_ms { Duration::from_nanos(0) } else { Duration::from_millis((self.epoch_ms - other.epoch_ms).try_into().unwrap_or(u64::MAX)) }
    }

    pub fn now() -> Instant {
        Instant {
            epoch_ms: unsafe { js_sys::Date::now().to_int_unchecked::<u128>() },
        }
    }

    pub fn epoch(&self) -> Duration {
        Duration::from_millis(self.epoch_ms.try_into().unwrap_or(u64::MAX))
    }

    pub fn try_add(&self, duration: Duration) -> Option<Instant> {
        Some(Instant { epoch_ms: self.epoch_ms.checked_add(duration.as_millis())? })
    }

    pub fn try_subtract(&self, duration: Duration) -> Option<Instant> {
        Some(Instant { epoch_ms: self.epoch_ms.checked_sub(duration.as_millis())? })
    }
}

impl Add<Duration> for Instant {
    type Output = Instant;
    fn add(self, rhs: Duration) -> Self::Output {
        Instant { epoch_ms: self.epoch_ms.checked_add(rhs.as_millis()).expect("Overflow when adding duration to instant") }
    }
}

impl AddAssign<Duration> for Instant {
    fn add_assign(&mut self, rhs: Duration) {
        self.epoch_ms = self.epoch_ms.checked_add(rhs.as_millis()).expect("Overflow when adding duration to instant");
    }
}

impl Sub<Duration> for Instant {
    type Output = Instant;
    fn sub(self, rhs: Duration) -> Self::Output {
        Instant { epoch_ms: self.epoch_ms.checked_sub(rhs.as_millis()).expect("Overflow when subtracting duration from instant") }
    }
}

impl SubAssign<Duration> for Instant {
    fn sub_assign(&mut self, rhs: Duration) {
        self.epoch_ms = self.epoch_ms.checked_sub(rhs.as_millis()).expect("Overflow when subtracting duration from instant");
    }
}

/*
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct ZonedDateTimeInner {
    m_epoch: crate::temporal::Duration,
    m_tz_offset_minutes: i32,
}

impl ZonedDateTimeInner {
    pub fn epoch(&self) -> crate::temporal::Duration {
        self.m_epoch - crate::temporal::Duration::from_minutes(self.m_tz_offset_minutes.into())
    }
}

impl TryFrom<&str> for ZonedDateTimeInner {
    type Error = RangeError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(ZonedDateTimeInner::UtcTz(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(value).or(Err(RangeError))?))
    }
}
*/