/*!
When the Rialight runtime is targetting the browser.
*/

use std::{time::Duration, ops::{Add, AddAssign, Sub, SubAssign}, future::Future};

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct Instant {
    epoch_ms: u128,
}

impl Instant {
    pub fn since(&self, other: Instant) -> Duration {
        Duration::from_millis(if self.epoch_ms < other.epoch_ms { 0 } else { (self.epoch_ms - other.epoch_ms).try_into().unwrap_or(u64::MAX) })
    }

    pub fn now() -> Self {
        let epoch_ms: u64 = unsafe { js_sys::Date::now().to_int_unchecked() };
        Self {
            epoch_ms: epoch_ms.try_into().unwrap_or(u64::MAX.into()),
        }
    }
}

impl Add<Duration> for Instant {
    type Output = Instant;
    fn add(self, rhs: Duration) -> Self::Output {
        Self { epoch_ms: self.epoch_ms + rhs.as_millis() }
    }
}

impl AddAssign<Duration> for Instant {
    fn add_assign(&mut self, rhs: Duration) {
        self.epoch_ms += rhs.as_millis();
    }
}

impl Sub<Duration> for Instant {
    type Output = Instant;
    fn sub(self, rhs: Duration) -> Self::Output {
        Self { epoch_ms: self.epoch_ms - rhs.as_millis() }
    }
}

impl Sub<Instant> for Instant {
    type Output = Duration;
    fn sub(self, rhs: Instant) -> Self::Output {
        Duration::from_millis((self.epoch_ms - rhs.epoch_ms).try_into().unwrap_or(u64::MAX))
    }
}

impl SubAssign<Duration> for Instant {
    fn sub_assign(&mut self, rhs: Duration) {
        self.epoch_ms -= rhs.as_millis();
    }
}