/*!
When the Rialight runtime is incorrectly configured.
*/

use std::{time::Duration, ops::{Add, AddAssign, Sub, SubAssign}};
use crate::incorrect_runtime_panic;

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct Instant;

impl Instant {
    pub fn since(&self, _other: Instant) -> Duration {
        incorrect_runtime_panic!();
    }

    pub fn now() -> Instant {
        incorrect_runtime_panic!();
    }

    pub fn try_add(&self, _duration: Duration) -> Option<Instant> {
        incorrect_runtime_panic!();
    }

    pub fn try_subtract(&self, _duration: Duration) -> Option<Instant> {
        incorrect_runtime_panic!();
    }
}

impl Add<Duration> for Instant {
    type Output = Instant;
    fn add(self, _rhs: Duration) -> Self::Output {
        incorrect_runtime_panic!();
    }
}

impl AddAssign<Duration> for Instant {
    fn add_assign(&mut self, _rhs: Duration) {
        incorrect_runtime_panic!();
    }
}

impl Sub<Duration> for Instant {
    type Output = Instant;
    fn sub(self, _rhs: Duration) -> Self::Output {
        incorrect_runtime_panic!();
    }
}

impl Sub<Instant> for Instant {
    type Output = Duration;
    fn sub(self, _rhs: Instant) -> Self::Output {
        incorrect_runtime_panic!();
    }
}

impl SubAssign<Duration> for Instant {
    fn sub_assign(&mut self, _rhs: Duration) {
        incorrect_runtime_panic!();
    }
}

#[derive(Debug)]
pub struct Interval;

impl Interval {
    pub async fn tick(&mut self) -> Duration {
        crate::futures::not_sendable_async!();
        incorrect_runtime_panic!();
    }
}

impl Drop for Interval {
    fn drop(&mut self) {
        incorrect_runtime_panic!();
    }
}