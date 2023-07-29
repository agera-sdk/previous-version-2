/*!
Internal definitions for platform-based types and functions.
It defines browser and non-browser versions for types and functions in the
timeout API.
*/

// #[cfg(not(any(feature = "rialight_default_export", feature = "rialight_browser_export")))]
pub mod no_runtime;
// #[cfg(not(any(feature = "rialight_default_export", feature = "rialight_browser_export")))]
pub use no_runtime::*;