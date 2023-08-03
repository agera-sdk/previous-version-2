#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct TimeZone(pub(crate) chrono_tz::Tz);