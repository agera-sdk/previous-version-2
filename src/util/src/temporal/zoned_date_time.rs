use super::{Duration, TimeZone, Calendar, RangeError};

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct ZonedDateTime {
    inner: ZonedDateTimeInner,
    calendar: Calendar,
}

impl ZonedDateTime {
    pub fn epoch(&self) -> Duration {
        self.inner.epoch()
    }
}

impl TryFrom<&str> for ZonedDateTime {
    type Error = RangeError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(ZonedDateTime {
            inner: ZonedDateTimeInner::try_from(value)?,
            calendar: Calendar::ISO_8601,
        })
    }
}

impl TryFrom<String> for ZonedDateTime {
    type Error = RangeError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_ref())
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

impl TryFrom<&str> for ZonedDateTimeInner {
    type Error = RangeError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(ZonedDateTimeInner::UtcTz(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(value).or(Err(RangeError))?))
    }
}

impl TryFrom<String> for ZonedDateTimeInner {
    type Error = RangeError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_ref())
    }
}