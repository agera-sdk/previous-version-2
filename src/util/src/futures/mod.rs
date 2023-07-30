/*!
Additional functions for futures.

The Rust standard library comes with no asynchronous runtime.
This module provides a common function, `exec_future`, which
executes a future without awaiting for its completion.

# Example

The following example uses an `async` block of the Rust language,
which returns a future.

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