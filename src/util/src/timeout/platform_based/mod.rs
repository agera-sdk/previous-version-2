/*!
Internal definitions for platform-based types and functions.
It defines browser and non-browser versions for types and functions in the
timeout API.
*/

pub mod no_runtime;
pub use no_runtime::*;

#[cfg(feature = "rialight_default_export")]
pub mod tokio_runtime;
#[cfg(feature = "rialight_default_export")]
pub use tokio_runtime::*;

#[cfg(feature = "rialight_browser_export")]
pub mod browser_runtime;
#[cfg(feature = "rialight_browser_export")]
pub use browser_runtime::*;