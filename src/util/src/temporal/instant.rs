use super::{platform, Duration};

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct Instant {
    inner: platform::Instant,
}

impl Instant {
    pub fn from_epoch(epoch_duration: Duration) -> Self {
        Self { inner: platform::Instant::EPOCH + epoch_duration.try_into().expect("Called temporal::Instant::from_epoch() with a duration out of range") }
    }

    /// Adds a duration to the instant, returning a new instant.
    /// `None` is returned if the result is earlier or later than
    /// the range that `temporal::Instant` can represent.
    pub fn try_add(&self, duration: Duration) -> Option<Instant> {
        Some(Self { inner: self.inner.try_add(duration.try_into().ok()?)? })
    }

    /// Subtracts a duration from the instant, returning a new instant.
    /// `None` is returned if the result is earlier or later than
    /// the range that `temporal::Instant` can represent.
    pub fn try_subtract(&self, duration: Duration) -> Option<Instant> {
        Some(Self { inner: self.inner.try_subtract(duration.try_into().ok()?)? })
    }
}