/*!
Code used for the Rialight runtime.
*/

/// Initialises the Rialight runtime, executing
/// the application entry point.
/// 
/// # Example
/// 
/// ```ignore
/// rialight::main!(async {
///     // main code here.
/// });
/// ```
pub macro main {
    ($user_future:expr) => {
        // Tokio runtime
        #[cfg(feature = "rialight_default_export")]
        use ::rialight::util::runtime::__tokio_runtime__ as __rialight_tokio_runtime__;

        #[cfg(feature = "rialight_default_export")]
        #[__rialight_tokio_runtime::main(crate = "__rialight_tokio_runtime__")]
        async fn main() {
            // asynchronous tasks
            let local_task_set = ::rialight::util::runtime::__tokio_runtime__::task::LocalSet::new();
            local_task_set.run_until(async {
                user_future.await;
            }).await;
        }

        #[cfg(feature = "rialight_browser_export")]
        fn main() {
            // asynchronous tasks
            ::rialight::util::futures::exec_future(async {
                user_future.await;
            });
        }

        #[cfg(any(not(any(feature = "rialight_default_export", feature = "rialight_browser_export"))), all(feature = "rialight_default_export", feature = "rialight_browser_export"))]
        fn main() {
            compile_error!("Incorrect Rialight runtime configuration..");
        }
    },
}

#[doc(hidden)]
#[cfg(feature = "rialight_default_export")]
pub use tokio as __tokio_runtime__;