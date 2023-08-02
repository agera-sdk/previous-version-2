use super::{platform, Duration, ZonedDateTime, RangeError, TimeZone, Calendar, ZonedDateTimeOptions, zoned_date_time::ZonedDateTimeInner};
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct Instant {
    inner: platform::Instant,
}

impl Instant {
    pub fn from_epoch(epoch_duration: Duration) -> Self {
        Self { inner: platform::Instant::EPOCH + epoch_duration.try_into().expect("Called temporal::Instant::from_epoch() with a duration out of range") }
    }

    pub fn epoch(&self) -> Duration {
        self.inner.epoch().try_into().expect("Overflow on taking instant epoch duration")
    }

    pub fn to_zoned_date_time_iso(&self, timezone: TimeZone) -> Result<ZonedDateTime, RangeError> {
        self.to_zoned_date_time(timezone, Calendar::Iso8601)
    }

    pub fn to_zoned_date_time(&self, timezone: TimeZone, calendar: Calendar) -> Result<ZonedDateTime, RangeError> {
        match calendar {
            Calendar::Iso8601 => Ok(ZonedDateTime {
                calendar,
                inner: ZonedDateTimeInner::UtcTz(chrono::DateTime::<chrono::Utc>::from_utc(chrono::naive::NaiveDateTime::from_timestamp_millis(self.epoch().milliseconds()).ok_or(RangeError)?, offset)),
            }),
        }
    }

    /// Adds a duration to the instant, returning a new instant.
    /// `None` is returned if the result is earlier or later than
    /// the range that `temporal::Instant` can represent.
    pub fn try_add(&self, duration: Duration) -> Option<Instant> {
        if duration.0.num_milliseconds() < 0 {
            return self.try_subtract(duration.abs());
        }
        Some(Self { inner: self.inner.try_add(duration.try_into().ok()?)? })
    }

    /// Subtracts a duration from the instant, returning a new instant.
    /// `None` is returned if the result is earlier or later than
    /// the range that `temporal::Instant` can represent.
    pub fn try_subtract(&self, duration: Duration) -> Option<Instant> {
        if duration.0.num_milliseconds() < 0 {
            return self.try_add(duration.abs());
        }
        Some(Self { inner: self.inner.try_subtract(duration.try_into().ok()?)? })
    }
}

impl From<ZonedDateTime> for Instant {
    fn from(value: ZonedDateTime) -> Self {
        Self::from_epoch(value.epoch())
    }
}

impl TryFrom<&str> for Instant {
    type Error = RangeError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self::from(ZonedDateTime::try_from(value)?))
    }
}

impl Add<Duration> for Instant {
    type Output = Instant;
    fn add(self, rhs: Duration) -> Self::Output {
        self.try_add(rhs).expect("Overflow on temporal::Instant addition")
    }
}

impl AddAssign<Duration> for Instant {
    fn add_assign(&mut self, rhs: Duration) {
        self.inner = self.try_add(rhs).expect("Overflow on temporal::Instant addition").inner;
    }
}

impl Sub<Duration> for Instant {
    type Output = Instant;
    fn sub(self, rhs: Duration) -> Self::Output {
        self.try_subtract(rhs).expect("Overflow on temporal::Instant subtraction")
    }
}

impl SubAssign<Duration> for Instant {
    fn sub_assign(&mut self, rhs: Duration) {
        self.inner = self.try_subtract(rhs).expect("Overflow on temporal::Instant subtraction").inner;
    }
}