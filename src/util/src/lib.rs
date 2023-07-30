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
pub mod file_paths;
pub mod observable;
pub mod string;
pub mod timing;
pub mod temporal;
pub mod futures;
pub mod number;

// not initiated or designed:
// pub mod gc;