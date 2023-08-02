use super::{platform, Duration, TimeZone, Calendar};

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct ZonedDateTime {
    inner: platform::ZonedDateTimeInner,
    calendar: Calendar,
}

impl ZonedDateTime {
    pub fn epoch(&self) -> Duration {
        self.inner.epoch()
    }
}