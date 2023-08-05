/*!
The Rialight utilities API.
This API is fully standalone and does not depend
on other APIs of the framework.
*/
#![feature(decl_macro)]

pub mod lazy_statics;
pub mod collections;
pub mod collection_literals;
pub mod flags;
pub mod bytes;
pub mod serialization;
pub mod reg_exp;
pub mod uri;
pub mod observable;
pub mod string;
pub mod timing;
pub mod futures;
pub mod number;
pub mod runtime;

pub use rust_temporal as temporal;
pub use ::file_paths as file_paths;

pub(crate) macro incorrect_runtime_panic {
    () => {
        panic!("Incorrect Rialight runtime configuration");
    }
}