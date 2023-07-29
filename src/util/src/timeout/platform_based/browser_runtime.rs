/*!
When the Rialight runtime is targetting the browser.
*/

use std::{time::Duration, ops::{Add, AddAssign, Sub, SubAssign}};

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct Instant;

impl Instant {
    pub fn since(&self, other: Instant) -> Duration {
        panic!("Incorrect Rialight runtime configuration");
    }

    pub fn now() -> Instant {
        panic!("Incorrect Rialight runtime configuration");
    }
}

impl Add<Duration> for Instant {
    type Output = Instant;
    fn add(self, rhs: Duration) -> Self::Output {
        panic!("Incorrect Rialight runtime configuration");
    }
}

impl AddAssign<Duration> for Instant {
    fn add_assign(&mut self, rhs: Duration) {
        panic!("Incorrect Rialight runtime configuration");
    }
}

impl Sub<Duration> for Instant {
    type Output = Instant;
    fn sub(self, rhs: Duration) -> Self::Output {
        panic!("Incorrect Rialight runtime configuration");
    }
}

impl Sub<Instant> for Instant {
    type Output = Duration;
    fn sub(self, rhs: Instant) -> Self::Output {
        panic!("Incorrect Rialight runtime configuration");
    }
}

impl SubAssign<Duration> for Instant {
    fn sub_assign(&mut self, rhs: Duration) {
        panic!("Incorrect Rialight runtime configuration");
    }
}