/*!
When the Rialight runtime is incorrectly configured.
*/

use crate::incorrect_runtime_panic;
use std::{time::Duration, ops::{Add, AddAssign, Sub, SubAssign}};

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct Instant;

impl Instant {
    pub const EPOCH: Instant = Instant;

    pub fn since(&self, _other: Instant) -> Duration {
        incorrect_runtime_panic!();
    }

    pub fn now() -> Instant {
        incorrect_runtime_panic!();
    }

    pub fn epoch(&self) -> Duration {
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

impl SubAssign<Duration> for Instant {
    fn sub_assign(&mut self, _rhs: Duration) {
        incorrect_runtime_panic!();
    }
}