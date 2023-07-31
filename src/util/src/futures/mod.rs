/*!
Additional functions for futures.

# Executing future without awaiting it

This module provides a common function, `exec_future`, which
executes a future without awaiting for its completion.

The following example uses an `async` block of the Rust language,
which returns a future, as an argument to `exec_future`.

```
use rialight_util::futures::exec_future;

# fn f() {
exec_future(async {
    // asynchronous code
});
# }
```
*/

use std::future::Future;

/// Executes a future without awaiting for its completion. Its result
/// is ignored.
/// 
/// The following example uses an `async` block of the Rust language,
/// which returns a future.
/// 
/// # Example
/// 
/// ```
/// use rialight_util::futures::exec_future;
/// 
/// # fn f() {
/// exec_future(async {
///     // asynchronous code
/// });
/// # }
/// ```
/// 
pub fn exec_future<F>(future: F)
where
    F: Future<Output = ()> + Send + 'static,
{
    #[cfg(not(any(feature = "rialight_default_export", feature = "rialight_browser_export")))] {
        panic!("Incorrect Rialight runtime configuration");
    }
    #[cfg(feature = "rialight_default_export")] {
        tokio::task::spawn(future);
    }
    #[cfg(feature = "rialight_browser_export")] {
        wasm_bindgen_futures::spawn_local(future);
    }
}

pub use futures::future::ready as ready_future;

/// The `future_race` function takes an iterable of futures as input and returns
/// a single [`Future`]. The returned future completes with
/// a group (_v_, _i_), where _v_ is the output from the first
/// completed future and _i_ is the index of the first completed future
/// from the given iterator.
/// 
/// # Exceptions
/// 
/// Panics if the iterator specified contains no futures.
/// 
/// # Example
/// 
/// ```
/// # use rialight_util::futures::*;
/// # async fn f() {
/// let (value, index) = future_race(list_of_futures).await;
/// # }
/// ```
/// 
pub async fn future_race<I, IteratorFuture>(iterable: I) -> (IteratorFuture::Output, usize)
where
    I: IntoIterator<Item = IteratorFuture>,
    IteratorFuture: Future + Unpin,
{
    let (v, i, _) = futures::future::select_all(iterable).await;
    (v, i)
}

pub async fn future_all<I, IteratorFuture>(iterable: I) -> Vec<IteratorFuture::Output>
where
    I: IntoIterator<Item = IteratorFuture>,
    IteratorFuture: Future + Unpin,
{
    futures::future::join_all(iterable).await
}

#[cfg(feature = "rialight_browser_export")]
pub(crate) mod browser;