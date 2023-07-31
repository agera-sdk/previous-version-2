/*!
When the Rialight runtime is incorrectly configured.
*/

use std::{time::Duration, ops::{Add, AddAssign, Sub, SubAssign}, future::Future, marker::PhantomData};

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

#[derive(Debug)]
pub struct Wait;

impl Future for Wait {
    type Output = ();
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        panic!("Incorrect Rialight runtime configuration");
    }
}

#[derive(Debug)]
pub struct Timeout<T>(PhantomData<T>);

impl<T: Future> Future for Timeout<T> {
    type Output = Result<(), super::ElapsedError>;
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        panic!("Incorrect Rialight runtime configuration");
    }
}

#[derive(Debug)]
pub struct Interval;

impl Interval {
    pub async fn tick(&mut self) -> Duration {
        panic!("Incorrect Rialight runtime configuration");
    }
}

impl Drop for Interval {
    fn drop(&mut self) {
        panic!("Incorrect Rialight runtime configuration");
    }
}

/*
#[derive(Hash)]
pub struct BackgroundTimeout<'a> {
    _callback: &'a (dyn Fn() + Send + 'static),
}

impl BackgroundTimeout {
    pub fn stop(&self) {
        panic!("Incorrect Rialight runtime configuration");
    }
}
*/