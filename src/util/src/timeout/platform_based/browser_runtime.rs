/*!
When the Rialight runtime is targetting the browser.
*/

use std::{time::Duration, ops::{Add, AddAssign, Sub, SubAssign}, future::Future, marker::PhantomData};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn setTimeout(closure: &Closure<dyn FnMut()>, millis: u32) -> f64;
    fn clearTimeout(token: i32);
}

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

#[derive(Debug)]
pub struct Wait {
    promise: wasm_bindgen_futures::JsFuture,
}

impl Future for Wait {
    type Output = ();
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        std::pin::pin!(self.promise).poll(cx).map(|r| ())
    }
}

#[derive(Debug)]
pub struct Timeout<T: Future>(wasm_bindgen_futures::JsFuture, PhantomData<T>);

impl<T: Future> Future for Timeout<T> {
    type Output = Result<(), super::ElapsedError>;
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        std::pin::pin!(self.0).poll(cx).map(|r| r.map(|r| ()).map_err(|_| super::ElapsedError))
    }
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Interval {
    pub at_first_tick: bool,
    pub start: Instant,
    pub start_timeout_id: i32,
    pub interval_abort_controller: Option<web_sys::AbortController>,
}

impl Interval {
    pub async fn tick(&mut self) -> Duration {
        if self.at_first_tick {
            // initial timeout
        }
        await_promise_here()
    }
}

impl Drop for Interval {
    fn drop(&mut self) {
        if self.start_timeout_id != -1 {
            web_sys::window().unwrap().clear_timeout_with_handle(self.start_timeout_id);
        }
        if let Some(c) = self.interval_abort_controller {
            c.abort();
        }
    }
}