/*!
Work with common timing and animation intervals.
*/

pub use std::time::Duration;
use std::{fmt::Display, ops::{Add, AddAssign, Sub, SubAssign}, sync::{Arc, RwLock}};

use crate::futures::*;

mod platform;

/// Error returned by [`timeout`] and [`timeout_at`].
/// 
/// This error is returned when a timeout expires before the function
/// was able to finish.
#[derive(PartialEq, Clone, Debug, Copy)]
pub struct ElapsedError;

impl Display for ElapsedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Timeout expired")
    }
}

impl std::error::Error for ElapsedError {}

/// A measurement of a monotonically nondecreasing clock. Opaque and useful only with `Duration`.
/// 
/// Instants are always guaranteed to be no less than any previously measured
/// instant when created.
/// 
/// Instants are opaque types that can only be compared to one another. There is
/// no method to get "the number of seconds" from an instant. Instead, it only
/// allows measuring the duration between two instants (or comparing two
/// instants).
///
/// # Mix with the temporal API
/// 
/// This `Instant` type is not the same as the one from the temporal API, however, in a future version,
/// the `Instant` type from the temporal API will be able to convert to the `Instant` type from the timeout API,
/// usually by just calling `.into()`.
/// 
/// ```
/// use rialight_util::{timeout::*, temporal};
/// 
/// let instant: Instant = temporal::now::instant().into();
/// ```
/// 
/// # Mix with the Rust standard library
/// 
/// This `Instant` type is not the same as the one from the Rust standard library.
/// 
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct Instant {
    inner: platform::Instant,
}

impl Instant {
    /// Returns the elapsed time since `other` or zero
    /// if the `self` instant is earlier than `other`.
    pub fn since(&self, other: Instant) -> Duration {
        self.inner.since(other.inner)
    }

    /// Returns the current instant from the host environment.
    pub fn now() -> Instant {
        Self { inner: platform::Instant::now() }
    }
}

impl Add<Duration> for Instant {
    type Output = Instant;
    fn add(self, rhs: Duration) -> Self::Output {
        Self { inner: self.inner + rhs }
    }
}

impl AddAssign<Duration> for Instant {
    fn add_assign(&mut self, rhs: Duration) {
        self.inner += rhs;
    }
}

impl Sub<Duration> for Instant {
    type Output = Instant;
    fn sub(self, rhs: Duration) -> Self::Output {
        Self { inner: self.inner - rhs }
    }
}

impl Sub<Instant> for Instant {
    type Output = Duration;
    fn sub(self, rhs: Instant) -> Self::Output {
        self.inner - rhs.inner
    }
}

impl SubAssign<Duration> for Instant {
    fn sub_assign(&mut self, rhs: Duration) {
        self.inner -= rhs;
    }
}

/// Interval returned by [`default_interval`],
/// [`default_interval_at`], [`animation_interval`] and
/// [`animation_interval_at`].
#[derive(Debug)]
pub struct Interval {
    inner: platform::Interval,
}

impl Interval {
    /// Completes when the next instant in the interval has been reached,
    /// yielding the time elapsed since the last tick.
    pub async fn tick(&mut self) -> Duration {
        self.inner.tick().await
    }
}

/*
/// Requires for a `Future` to complete before the given
/// `duration` has elapsed.
/// 
/// If the future completes before the duration has elapsed,
/// then `Ok(())` is returned.
/// Otherwise, an error is returned and the future is canceled.
/// 
/// This function returns a future whose return type is [`Result`]`<(),`[`ElapsedError`]`>`.
/// 
/// If the provided future completes immediately, then the future returned from
/// this function is guaranteed to complete immediately with an [`Ok`] variant
/// no matter the provided duration.
/// 
/// # Cancellation
///
/// Canceling a timeout being awaited for via the `.await` operator is not possible.
/// Use [`background_timeout`] for such a purpose.
/// 
/// # Examples
/// 
/// ```
/// use rialight_util::timing::*;
/// 
/// async fn example_fn() {
///     if let Err(_) = timeout(Duration::from_millis(10), f()).await {
///         println!("did not receive value within 10 ms");
///     }
/// }
/// 
/// async fn f() -> u64 { 10 }
/// ```
/// 
pub async fn timeout<F>(duration: Duration, future: F) -> Result<(), ElapsedError>
where
    F: Future<Output = ()> + Send + 'static,
{
    #[cfg(feature = "rialight_default_export")] {
        return match tokio::time::timeout(duration, future).await {
            Err(_) => Err(ElapsedError),
            Ok(_) => Ok(()),
        };
    }
    #[cfg(feature = "rialight_browser_export")] {
        return platform::browser_runtime::timeout(duration, future).await;
    }
    #[cfg(not(any(feature = "rialight_default_export", feature = "rialight_browser_export")))] {
        let _ = (duration, future);
        panic!("Incorrectly configured Rialight runtime");
    }
}
*/

/*
/// Requires a `Future` to complete before the specified instant in time.
///
/// If the future completes before the instant is reached, then `Ok(())`
/// is returned. Otherwise, an error is returned.
///
/// This function returns a future whose return type is [`Result`]`<(),`[`ElapsedError`]`>`.
///
/// If the provided future completes immediately, then the future returned from
/// this function is guaranteed to complete immediately with an [`Ok`] variant
/// no matter the provided deadline.
/// 
/// # Examples
/// 
/// ```
/// use rialight_util::timing::*;
/// 
/// async fn example_fn() {
///     if let Err(_) = timeout_at(Instant::now() + Duration::from_millis(10), f()).await {
///         println!("did not receive value within 10 ms");
///     }
/// }
/// 
/// async fn f() -> u64 { 10 }
/// ```
///
/// # Cancellation
///
/// Canceling a timeout being awaited for via the `.await` operator is not possible.
/// Use [`background_timeout`] for such a purpose.
/// 
pub async fn timeout_at<F>(deadline: Instant, future: F) -> Result<(), ElapsedError>
where
    F: Future<Output = ()> + Send + 'static,
{
    #[cfg(feature = "rialight_default_export")] {
        return match tokio::time::timeout_at(deadline.inner.0, future).await {
            Err(_) => Err(ElapsedError),
            Ok(_) => Ok(()),
        };
    }
    #[cfg(feature = "rialight_browser_export")] {
        return platform::browser_runtime::timeout(deadline.since(Instant::now()), future).await;
    }
    #[cfg(not(any(feature = "rialight_default_export", feature = "rialight_browser_export")))] {
        let _ = (deadline, future);
        panic!("Incorrectly configured Rialight runtime");
    }
}
*/

/// Asynchronously waits until `duration` has elapsed.
///
/// Equivalent to `wait_until(Instant::now() + duration)`.
/// 
/// No work is performed while awaiting on the wait future to complete. This
/// operates at millisecond granularity and should not be used for tasks that
/// require high-resolution timers.
/// 
/// To run something regularly on a schedule, see interval functions in this module.
/// 
/// The maximum duration for a wait is 68719476734 milliseconds (approximately 2.2 years).
/// 
/// # Cancellation
///
/// Canceling a wait being awaited for via the `.await` operator is not possible.
/// Use [`background_timeout`] for such a purpose.
/// 
/// # Examples
/// 
/// Wait 100ms and print "100 ms have elapsed".
/// 
/// ```
/// use rialight_util::timing::*;
///
/// async fn example_fn() {
///     wait(Duration::from_millis(100)).await;
///     println!("100 ms have elapsed");
/// }
/// ```
/// 
pub async fn wait(duration: Duration) {
    #[cfg(feature = "rialight_default_export")] {
        tokio::time::sleep(duration).await;
    }
    #[cfg(feature = "rialight_browser_export")] {
        platform::browser_runtime::wait(duration).await;
    }
    #[cfg(not(any(feature = "rialight_default_export", feature = "rialight_browser_export")))] {
        let _ = duration;
        panic!("Incorrectly configured Rialight runtime");
    }
}

/// Asynchronously waits until `deadline` is reached.
///
/// No work is performed while awaiting on the wait future to complete. This
/// operates at millisecond granularity and should not be used for tasks that
/// require high-resolution timers.
///
/// To run something regularly on a schedule, see interval functions in this module.
///
/// The maximum duration for a wait is 68719476734 milliseconds (approximately 2.2 years).
///
/// # Cancellation
///
/// Canceling a wait being awaited for via the `.await` operator is not possible.
/// Use [`background_timeout`] for such a purpose.
/// 
/// # Examples
/// 
/// Wait 100ms and print "100 ms have elapsed".
/// 
/// ```
/// use rialight_util::timing::*;
///
/// async fn example_fn() {
///     wait(Instant::now() + Duration::from_millis(100)).await;
///     println!("100 ms have elapsed");
/// }
/// ```
/// 
pub async fn wait_until(deadline: Instant) {
    #[cfg(feature = "rialight_default_export")] {
        tokio::time::sleep_until(deadline.inner.0).await;
    }
    #[cfg(feature = "rialight_browser_export")] {
        platform::browser_runtime::wait(deadline.since(Instant::now())).await;
    }
    #[cfg(not(any(feature = "rialight_default_export", feature = "rialight_browser_export")))] {
        let _ = deadline;
        panic!("Incorrectly configured Rialight runtime");
    }
}

/// Creates a new [`Interval`] that yields with interval of `period`. The first
/// tick completes immediately.
///
/// An interval will tick indefinitely.
/// 
/// # Animations
/// 
/// For animations, you might want to use [`animation_interval`]
/// instead of `default_interval`.
/// 
/// # Cancellation
///
/// An interval is disposed when its variable is dropped.
/// Use [`background_default_interval`] if you need an interval that runs
/// separately and can be cancelled dynamically.
///
/// # Panics
///
/// This function panics if `period` is zero.
/// 
/// # Examples
/// 
/// ```
/// use rialight_util::timing::*;
///
/// async fn example_fn() {
///     let mut interval = default_interval(Duration::from_millis(10));
///     interval.tick().await; // ticks immediately
///     interval.tick().await; // ticks after 10ms
///     interval.tick().await; // ticks after 10ms
///
///     // approximately 20ms have elapsed.
/// }
/// ```
/// 
/// A simple example using `default_interval` to execute a task every two seconds.
///
/// The difference between `default_interval` and [`wait`] is that an [`Interval`]
/// measures the time since the last tick, which means that [`.tick().await`]
/// may wait for a shorter time than the duration specified for the interval
/// if some time has passed between calls to [`.tick().await`].
///
/// If the tick in the example below was replaced with [`wait`], the task
/// would only be executed once every three seconds, and not every two
/// seconds.
///
/// ```
/// use rialight_util::timing::*;
///
/// async fn task_that_takes_a_second() {
///     println!("hello");
///     wait(Duration::from_secs(1)).await
/// }
///
/// async fn example() {
///     let mut interval = default_interval(Duration::from_secs(2));
///     for _i in 0..5 {
///         interval.tick().await;
///         task_that_takes_a_second().await;
///     }
/// }
/// ```
/// 
/// [`.tick().await`]: Interval::tick
///
pub fn default_interval(period: Duration) -> Interval {
    #[cfg(feature = "rialight_default_export")] {
        return Interval {
            inner: platform::tokio_runtime::Interval(tokio::time::interval(period)),
        };
    }
    #[cfg(feature = "rialight_browser_export")] {
        assert!(period.as_millis() != 0, "rialight::util::timing::interval() must be called with non-zero period");
        return Interval {
            inner: platform::browser_runtime::Interval {
                for_animation: false,
                period,
                start: Instant::now(),
                ticker: None,
            },
        };
    }
    #[cfg(not(any(feature = "rialight_default_export", feature = "rialight_browser_export")))] {
        let _ = period;
        panic!("Incorrectly configured Rialight runtime");
    }
}

/// Creates a new [`Interval`] that yields with interval of `period` with the
/// first tick completing at `start`.
///
/// # Animations
/// 
/// For animations, you might want to use [`animation_interval_at`]
/// instead of `default_interval_at`.
/// 
/// # Cancellation
///
/// An interval is disposed when its variable is dropped.
/// Use [`background_default_interval`] if you need an interval that runs
/// separately and can be cancelled dynamically.
/// 
/// # Panics
///
/// This function panics if `period` is zero.
/// 
/// # Examples
///
/// ```
/// use rialight_util::timing::*;
///
/// async fn example() {
///     let start = Instant::now() + Duration::from_millis(50);
///     let mut interval = default_interval_at(start, Duration::from_millis(10));
///
///     interval.tick().await; // ticks after 50ms
///     interval.tick().await; // ticks after 10ms
///     interval.tick().await; // ticks after 10ms
///
///     // approximately 70ms have elapsed.
/// }
/// ```
/// 
pub fn default_interval_at(start: Instant, period: Duration) -> Interval {
    #[cfg(feature = "rialight_default_export")] {
        return Interval {
            inner: platform::tokio_runtime::Interval(tokio::time::interval_at(start.inner.0, period)),
        };
    }
    #[cfg(feature = "rialight_browser_export")] {
        assert!(period.as_millis() != 0, "rialight::util::timing::interval_at() must be called with non-zero period");
        return Interval {
            inner: platform::browser_runtime::Interval {
                for_animation: false,
                period,
                start: start,
                ticker: None,
            },
        };
    }
    #[cfg(not(any(feature = "rialight_default_export", feature = "rialight_browser_export")))] {
        let _ = (start, period);
        panic!("Incorrectly configured Rialight runtime");
    }
}

/// Creates a new [`Interval`] that yields with interval of `period`. The first
/// tick completes immediately, meant for animations.
///
/// An interval will tick indefinitely.
/// 
/// # Cancellation
///
/// An interval is disposed when its variable is dropped.
/// Use [`background_animation_interval`] if you need an interval that runs
/// separately and can be cancelled dynamically.
///
/// # Panics
///
/// This function panics if `period` is zero.
/// 
/// # Examples
/// 
/// ```
/// use rialight_util::timing::*;
///
/// async fn example_fn() {
///     let mut interval = animation_interval(Duration::from_millis(10));
///     interval.tick().await; // ticks immediately
///     interval.tick().await; // ticks after 10ms
///     interval.tick().await; // ticks after 10ms
///
///     // approximately 20ms have elapsed.
/// }
/// ```
/// 
/// [`.tick().await`]: Interval::tick
///
pub fn animation_interval(period: Duration) -> Interval {
    #[cfg(feature = "rialight_default_export")] {
        return Interval {
            inner: platform::tokio_runtime::Interval(tokio::time::interval(period)),
        };
    }
    #[cfg(feature = "rialight_browser_export")] {
        assert!(period.as_millis() != 0, "rialight::util::timing::interval() must be called with non-zero period");
        return Interval {
            inner: platform::browser_runtime::Interval {
                for_animation: true,
                period,
                start: Instant::now(),
                ticker: None,
            },
        };
    }
    #[cfg(not(any(feature = "rialight_default_export", feature = "rialight_browser_export")))] {
        let _ = period;
        panic!("Incorrectly configured Rialight runtime");
    }
}

/// Creates a new [`Interval`] that yields with interval of `period` with the
/// first tick completing at `start`, meant for animations.
///
/// # Cancellation
///
/// An interval is disposed when its variable is dropped.
/// Use [`background_animation_interval`] if you need an interval that runs
/// separately and can be cancelled dynamically.
/// 
/// # Panics
///
/// This function panics if `period` is zero.
/// 
/// # Examples
///
/// ```
/// use rialight_util::timing::*;
///
/// async fn example() {
///     let start = Instant::now() + Duration::from_millis(50);
///     let mut interval = animation_interval_at(start, Duration::from_millis(10));
///
///     interval.tick().await; // ticks after 50ms
///     interval.tick().await; // ticks after 10ms
///     interval.tick().await; // ticks after 10ms
///
///     // approximately 70ms have elapsed.
/// }
/// ```
/// 
pub fn animation_interval_at(start: Instant, period: Duration) -> Interval {
    #[cfg(feature = "rialight_default_export")] {
        return Interval {
            inner: platform::tokio_runtime::Interval(tokio::time::interval_at(start.inner.0, period)),
        };
    }
    #[cfg(feature = "rialight_browser_export")] {
        assert!(period.as_millis() != 0, "rialight::util::timing::interval_at() must be called with non-zero period");
        return Interval {
            inner: platform::browser_runtime::Interval {
                for_animation: true,
                period,
                start: start,
                ticker: None,
            },
        };
    }
    #[cfg(not(any(feature = "rialight_default_export", feature = "rialight_browser_export")))] {
        let _ = (start, period);
        panic!("Incorrectly configured Rialight runtime");
    }
}

/// Executes a given function after some elapsed time. This function
/// returns a `BackgroundTimeout` object with a `stop()` method that can
/// be used to stop the execution of the function.
pub fn background_timeout(callback: Box<(dyn Fn() + Send + Sync + 'static)>, duration: Duration) -> BackgroundTimeout {
    let mut stopped = Arc::new(RwLock::new(false));
    exec_future({
        let stopped = Arc::clone(&mut stopped);
        async move {
            wait(duration).await;
            if !*stopped.read().unwrap() {
                callback();
            }
        }
    });
    BackgroundTimeout {
        stopped,
    }
}

/// A timeout that can be stopped at anytime, returned
/// from the [`background_timeout`] function.
/// 
/// To stop the timeout, call `timeout.stop`.
pub struct BackgroundTimeout {
    // inner: platform::BackgroundTimeout,
    stopped: Arc<RwLock<bool>>,
}

impl BackgroundTimeout {
    pub fn stop(&self) {
        *self.stopped.write().unwrap() = true;
    }
}

/// Executes a given function after each period. This function
/// returns a `BackgroundInterval` object with a `stop()` method that can
/// be used to stop the execution of the function and dispose of the interval.
/// 
/// The callback function receives the elapsed time since the last time
/// it was called by this function.
pub fn background_animation_interval(callback: Box<(dyn Fn(Duration) + Send + Sync + 'static)>, period: Duration) -> BackgroundInterval {
    let mut stopped = Arc::new(RwLock::new(false));
    exec_future({
        let stopped = Arc::clone(&mut stopped);
        async move {
            let mut interval = animation_interval(period);
            interval.tick().await;
            loop {
                let delta = interval.tick().await;
                if *stopped.read().unwrap() {
                    break;
                }
                callback(delta);
            }
        }
    });
    BackgroundInterval {
        stopped,
    }
}

/// Executes a given function after each period. This function
/// returns a `BackgroundInterval` object with a `stop()` method that can
/// be used to stop the execution of the function and dispose of the interval.
/// 
/// The callback function receives the elapsed time since the last time
/// it was called by this function.
/// 
/// For animations, consider using [`background_animation_interval`] instead.
pub fn background_default_interval(callback: Box<(dyn Fn(Duration) + Send + Sync + 'static)>, period: Duration) -> BackgroundInterval {
    let mut stopped = Arc::new(RwLock::new(false));
    exec_future({
        let stopped = Arc::clone(&mut stopped);
        async move {
            let mut interval = default_interval(period);
            interval.tick().await;
            loop {
                let delta = interval.tick().await;
                if *stopped.read().unwrap() {
                    break;
                }
                callback(delta);
            }
        }
    });
    BackgroundInterval {
        stopped,
    }
}

/// An interval that can be stopped at anytime, returned
/// from the [`background_animation_interval`] and [`background_default_interval`] functions.
/// 
/// To stop the interval, call `interval.stop`.
pub struct BackgroundInterval {
    // inner: platform::BackgroundTimeout,
    stopped: Arc<RwLock<bool>>,
}

impl BackgroundInterval {
    pub fn stop(&self) {
        *self.stopped.write().unwrap() = true;
    }
}