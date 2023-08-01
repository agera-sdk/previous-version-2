use super::RangeError;

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct Duration(pub(crate) chrono::Duration);

impl TryFrom<std::time::Duration> for Duration {
    type Error = RangeError;
    fn try_from(value: std::time::Duration) -> Result<Self, Self::Error> {
    }
}

impl TryInto<std::time::Duration> for Duration {
    type Error = RangeError;
    fn try_into(self) -> Result<std::time::Duration, Self::Error> {
        self.0.to_std().or(Err(RangeError))
    }
}