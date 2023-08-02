use super::{Duration, TimeZone, Calendar, RangeError};

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct ZonedDateTime {
    inner: chrono::DateTime<chrono_tz::Tz>,
    calendar: Calendar,
}

impl ZonedDateTime {
    pub fn epoch(&self) -> Duration {
        crate::temporal::Duration::from_milliseconds(self.inner.timestamp_millis())
    }
}

impl From<ZonedDateTimeOptions> for ZonedDateTime {
    fn from(value: ZonedDateTimeOptions) -> Self {
        match value.calendar {
            Calendar::Iso8601 => {
                Self {
                    calendar: value.calendar,
                }
            },
        }
    }
}

impl TryFrom<&str> for ZonedDateTime {
    type Error = RangeError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(ZonedDateTime {
            inner: <chrono::DateTime::<chrono_tz::Tz> as std::str::FromStr>::from_str(value).or(Err(RangeError))?,
            calendar: Calendar::Iso8601,
        })
    }
}

impl TryFrom<String> for ZonedDateTime {
    type Error = RangeError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_ref())
    }
}

#[derive(Clone, Debug)]
pub struct ZonedDateTimeOptions {
    pub timezone: Option<TimeZone>,
    pub calendar: Calendar,
    pub year: Option<i64>,
    pub month: Option<i64>,
    pub day: Option<i64>,
    pub hour: i64,
    pub minute: i64,
    pub second: i64,
    pub millisecond: i64,
    pub microsecond: i64,
    pub nanosecond: i64,
}

impl Default for ZonedDateTimeOptions {
    fn default() -> Self {
        Self {
            timezone: None,
            calendar: Calendar::Iso8601,
            year: None,
            month: None,
            day: None,
            hour: 0,
            minute: 0,
            second: 0,
            millisecond: 0,
            microsecond: 0,
            nanosecond: 0,
        }
    }
}