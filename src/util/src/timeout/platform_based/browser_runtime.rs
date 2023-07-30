/*!
When the Rialight runtime is targetting the browser.
*/

use std::{time::Duration, ops::{Add, AddAssign, Sub, SubAssign}, future::Future, marker::PhantomData, fmt::Debug};
use wasm_bindgen::prelude::*;

use super::cross_platform_wait_until;

#[wasm_bindgen]
extern "C" {
    fn setTimeout(closure: &Closure<dyn FnMut()>, millis: u32) -> f64;
    fn clearTimeout(token: i32);
}

#[wasm_bindgen(module = "browser.js")]
extern "C" {
    #[wasm_bindgen(js_name = nonAnimationInterval)]
    fn non_animation_interval(closure: &Closure<dyn FnMut(f64)>, ms: u32) -> web_sys::AbortController;
    #[wasm_bindgen(js_name = animationInterval)]
    fn animation_interval(closure: &Closure<dyn FnMut(f64)>, ms: u32) -> web_sys::AbortController;

    // Ticker

    type Ticker;

    #[wasm_bindgen(constructor)]
    fn new(for_animation: bool, ms: u32) -> Ticker;

    #[wasm_bindgen(method)]
    fn tick(this: &Ticker, callback: &Closure<dyn FnMut(f64)>);

    #[wasm_bindgen(method, js_name = tickInJSPromise)]
    fn tick_in_js_promise(this: &Ticker) -> js_sys::Promise;
}

impl Debug for Ticker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Ticker()")
    }
}

impl Ticker {
    async fn tick_in_future(&self) -> Duration {
        let delta = wasm_bindgen_futures::JsFuture::from(self.tick_in_js_promise()).await;
        Duration::from_millis(unsafe { delta.unwrap().as_f64().unwrap().to_int_unchecked::<u64>() })
    }
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

#[derive(Debug)]
pub struct Interval {
    pub for_animation: bool,
    pub period: Duration,
    pub start: super::SuperInstant,
    pub ticker: Option<Ticker>,
}

impl Interval {
    pub async fn tick(&mut self) -> Duration {
        match self.ticker.as_ref() {
            Some(ticker) => ticker.tick_in_future().await,
            None => {
                // initial tick
                cross_platform_wait_until(self.start).await;
                self.ticker = Some(Ticker::new(self.for_animation, self.period.as_millis().try_into().expect("Developer has given too large period for interval")));
                return Duration::from_millis(0);
            },
        }
    }
}

impl Drop for Interval {
    fn drop(&mut self) {
    }
}