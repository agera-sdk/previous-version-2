use super::RangeError;

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct Duration(pub(crate) chrono::Duration);

impl Duration {
    pub fn from_milliseconds(milliseconds: i64) -> Self {
        Self(chrono::Duration::milliseconds(milliseconds))
    }

    pub fn milliseconds(&self) -> i64 {
        self.0.num_milliseconds()
    }

    pub fn abs(&self) -> Self {
        Self::from_milliseconds(self.milliseconds().abs())
    }
}

impl TryFrom<std::time::Duration> for Duration {
    type Error = RangeError;
    fn try_from(value: std::time::Duration) -> Result<Self, Self::Error> {
        Ok(Self(chrono::Duration::from_std(value).or(Err(RangeError))?))
    }
}

impl TryInto<std::time::Duration> for Duration {
    type Error = RangeError;
    fn try_into(self) -> Result<std::time::Duration, Self::Error> {
        self.0.to_std().or(Err(RangeError))
    }
}