/*!
Work with timeouts and intervals.

# Non Rialight users

This module is only meant to be used within the Rialight asynchronous runtime.
*/

pub use tokio::time::{
    Duration,
    Timeout,
    Instant,
    Sleep as Wait,
    error::Elapsed as ElapsedError,
};
use std::future::Future;

/// Asynchronously waits for a `Future` to complete before the given
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
/// Cancelling a timeout is done by dropping the future. No additional cleanup
/// or other work is required.
///
/// The original future may be obtained by calling [`Timeout::into_inner`]. This
/// consumes the `Timeout`.
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
pub async fn timeout<F>(duration: Duration, future: F) -> Timeout<F>
    where F: Future,
{
    tokio::time::timeout(duration, future)
}

/// Asynchronously waits until `duration` has elapsed.
///
/// Equivalent to `wait_until(Instant::now() + duration)`.
/// 
/// No work is performed while awaiting on the wait future to complete. `Wait`
/// operates at millisecond granularity and should not be used for tasks that
/// require high-resolution timers. The implementation is platform specific,
/// and some platforms (specifically Windows) will provide timers with a
/// larger resolution than 1 ms.
/// 
/// To run something regularly on a schedule, see [`interval`].
/// 
/// The maximum duration for a wait is 68719476734 milliseconds (approximately 2.2 years).
/// 
/// # Cancellation
///
/// Canceling a wait instance is done by dropping the returned future. No additional
/// cleanup work is required.
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
    tokio::time::sleep(duration)
}