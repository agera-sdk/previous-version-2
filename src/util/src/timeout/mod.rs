/*!
Work with timeouts and intervals.

# Non Rialight users

This module is only meant to be used within the Rialight asynchronous runtime.
*/

pub use std::time::Duration;
use std::future::Future;

/// Requires for a `Future` to complete before the given
/// `duration` has elapsed.
/// 
/// If the future completes before the duration has elapsed,
/// then the completed value is returned.
/// Otherwise, an error is returned and the future is canceled.
/// 
/// Note that the timeout is checked before polling the future, so if the future
/// does not yield during execution then it is possible for the future to complete
/// and exceed the timeout _without_ returning an error.
/// 
/// This function returns a future whose return type is [`Result`]`<T,`[`ElapsedError`]`>`, where `T` is the
/// return type of the provided future.
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
/// use rialight_util::timeout::*;
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
/// # Panics
///
/// For non Rialight users, if you're not calling this function
/// within the Rialight asynchronous runtime, it might panic.
/// 
pub async fn timeout<F: Future>(duration: Duration, future: F) -> Timeout<F> {
    #[cfg(feature = "rialight_default_export")] {
        return tokio::time::timeout(duration, future);
    }
    #[cfg(feature = "rialight_browser_export")] {
        todo!();
    }
    #[cfg(not(any(feature = "rialight_default_export", feature = "rialight_browser_export")))] {
        let _ = (duration, future);
        panic!("Incorrectly configured Rialight runtime");
    }
}

/// Requires a `Future` to complete before the specified instant in time.
///
/// If the future completes before the instant is reached, then the completed
/// value is returned. Otherwise, an error is returned.
///
/// This function returns a future whose return type is [`Result`]`<T,`[`ElapsedError`]`>`, where `T` is the
/// return type of the provided future.
///
/// If the provided future completes immediately, then the future returned from
/// this function is guaranteed to complete immediately with an [`Ok`] variant
/// no matter the provided deadline.
/// 
/// # Examples
/// 
/// ```
/// use rialight_util::timeout::*;
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
/// # Panics
///
/// For non Rialight users, if you're not calling this function
/// within the Rialight asynchronous runtime, it might panic.
/// 
pub async fn timeout_at<F: Future>(deadline: Instant, future: F) -> Timeout<F> {
    #[cfg(feature = "rialight_default_export")] {
        return tokio::time::timeout_at(deadline, future);
    }
    #[cfg(feature = "rialight_browser_export")] {
        todo!();
    }
    #[cfg(not(any(feature = "rialight_default_export", feature = "rialight_browser_export")))] {
        let _ = (deadline, future);
        panic!("Incorrectly configured Rialight runtime");
    }
}

/// Asynchronously waits until `duration` has elapsed.
///
/// Equivalent to `wait_until(Instant::now() + duration)`.
/// 
/// No work is performed while awaiting on the wait future to complete. `Wait`
/// operates at millisecond granularity and should not be used for tasks that
/// require high-resolution timers.
/// 
/// To run something regularly on a schedule, see [`interval`].
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
/// use rialight_util::timeout::*;
///
/// async fn example_fn() {
///     wait(Duration::from_millis(100)).await;
///     println!("100 ms have elapsed");
/// }
/// ```
/// 
/// See the documentation for the [`Wait`] type for more examples.
///
/// # Panics
/// 
/// For non Rialight users, if you're not calling this function
/// within the Rialight asynchronous runtime, it might panic.
/// 
pub async fn wait(duration: Duration) -> Wait {
    #[cfg(feature = "rialight_default_export")] {
        return tokio::time::sleep(duration);
    }
    #[cfg(feature = "rialight_browser_export")] {
        todo!();
    }
    #[cfg(not(any(feature = "rialight_default_export", feature = "rialight_browser_export")))] {
        let _ = duration;
        panic!("Incorrectly configured Rialight runtime");
    }
}

/// Asynchronously waits until `deadline` is reached.
///
/// No work is performed while awaiting on the wait future to complete. `Wait`
/// operates at millisecond granularity and should not be used for tasks that
/// require high-resolution timers.
///
/// To run something regularly on a schedule, see [`interval`].
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
/// use rialight_util::timeout::*;
///
/// async fn example_fn() {
///     wait(Instant::now() + Duration::from_millis(100)).await;
///     println!("100 ms have elapsed");
/// }
/// ```
/// 
/// See the documentation for the [`Wait`] type for more examples.
/// 
/// # Panics
/// 
/// For non Rialight users, if you're not calling this function
/// within the Rialight asynchronous runtime, it might panic.
/// 
pub async fn wait_until(deadline: Instant) -> Wait {
    #[cfg(feature = "rialight_default_export")] {
        return tokio::time::sleep_until(deadline);
    }
    #[cfg(feature = "rialight_browser_export")] {
        todo!();
    }
    #[cfg(not(any(feature = "rialight_default_export", feature = "rialight_browser_export")))] {
        let _ = deadline;
        panic!("Incorrectly configured Rialight runtime");
    }
}

/// Creates a new [`Interval`] that yields with interval of `period`. The first
/// tick completes immediately.
///
/// An interval will tick indefinitely. At any time, the [`Interval`] value can
/// be dropped. This cancels the interval.
///
/// This function is equivalent to
/// [`interval_at(Instant::now(), period)`](interval_at).
/// 
/// # Cancellation
///
/// An interval is disposed when its variable is dropped.
/// Use [`background_interval`] if you need an interval that runs
/// separately and can be cancelled dynamically.
///
/// # Panics
///
/// This function panics if `period` is zero.
/// 
/// # Examples
/// 
/// ```
/// use rialight_util::timeout::*;
///
/// async fn example_fn() {
///     let mut interval = interval(Duration::from_millis(10));
///     interval.tick().await; // ticks immediately
///     interval.tick().await; // ticks after 10ms
///     interval.tick().await; // ticks after 10ms
///
///     // approximately 20ms have elapsed.
/// }
/// ```
/// 
/// A simple example using `interval` to execute a task every two seconds.
///
/// The difference between `interval` and [`wait`] is that an [`Interval`]
/// measures the time since the last tick, which means that [`.tick().await`]
/// may wait for a shorter time than the duration specified for the interval
/// if some time has passed between calls to [`.tick().await`].
///
/// If the tick in the example below was replaced with [`wait`], the task
/// would only be executed once every three seconds, and not every two
/// seconds.
///
/// ```
/// use rialight_util::timeout::*;
///
/// async fn task_that_takes_a_second() {
///     println!("hello");
///     wait(Duration::from_secs(1)).await
/// }
///
/// async fn example() {
///     let mut interval = interval(Duration::from_secs(2));
///     for _i in 0..5 {
///         interval.tick().await;
///         task_that_takes_a_second().await;
///     }
/// }
/// ```
/// 
/// [`.tick().await`]: Interval::tick
///
pub fn interval(period: Duration) -> Interval {
    #[cfg(feature = "rialight_default_export")] {
        return tokio::time::interval(period);
    }
    #[cfg(feature = "rialight_browser_export")] {
        todo!();
    }
    #[cfg(not(any(feature = "rialight_default_export", feature = "rialight_browser_export")))] {
        let _ = period;
        panic!("Incorrectly configured Rialight runtime");
    }
}

/// Creates a new [`Interval`] that yields with interval of `period` with the
/// first tick completing at `start`.
///
/// # Cancellation
///
/// An interval is disposed when its variable is dropped.
/// Use [`background_interval`] if you need an interval that runs
/// separately and can be cancelled dynamically.
/// 
/// # Panics
///
/// This function panics if `period` is zero.
/// 
/// # Examples
///
/// ```
/// use rialight_util::timeout::*;
///
/// async fn example() {
///     let start = Instant::now() + Duration::from_millis(50);
///     let mut interval = interval_at(start, Duration::from_millis(10));
///
///     interval.tick().await; // ticks after 50ms
///     interval.tick().await; // ticks after 10ms
///     interval.tick().await; // ticks after 10ms
///
///     // approximately 70ms have elapsed.
/// }
/// ```
/// 
pub fn interval_at(start: Instant, period: Duration) -> Interval {
    #[cfg(feature = "rialight_default_export")] {
        return tokio::time::interval_at(start, period);
    }
    #[cfg(feature = "rialight_browser_export")] {
        todo!();
    }
    #[cfg(not(any(feature = "rialight_default_export", feature = "rialight_browser_export")))] {
        let _ = (start, period);
        panic!("Incorrectly configured Rialight runtime");
    }
}