/*!
The Rialight runtime uses the asynchronous Tokio runtime internally
for any platform other than the browser.
*/

use std::{time::Duration, ops::{Add, AddAssign, Sub, SubAssign}};

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct Instant(tokio::time::Instant);

impl Instant {
    pub fn since(&self, other: Instant) -> Duration {
        self.0.duration_since(other.0)
    }

    pub fn now() -> Instant {
        Self(tokio::time::Instant::now())
    }
}

impl Add<Duration> for Instant {
    type Output = Instant;
    fn add(self, rhs: Duration) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl AddAssign<Duration> for Instant {
    fn add_assign(&mut self, rhs: Duration) {
        self.0 = self.0 + rhs;
    }
}

impl Sub<Duration> for Instant {
    type Output = Instant;
    fn sub(self, rhs: Duration) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl Sub<Instant> for Instant {
    type Output = Duration;
    fn sub(self, rhs: Instant) -> Self::Output {
        self.0 - rhs.0
    }
}

impl SubAssign<Duration> for Instant {
    fn sub_assign(&mut self, rhs: Duration) {
        self.0 = self.0 - rhs;
    }
}