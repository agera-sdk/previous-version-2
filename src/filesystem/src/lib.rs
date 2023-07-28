/*!
The Rialight file system API.
*/

use rialight_prelude::*;
use rialight_util::file_paths::os_based as file_paths;

pub mod webcompat;

mod error;
pub use error::FileError;
