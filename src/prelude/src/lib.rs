/*!
The Rialight prelude API.

This module can be imported to include commonly used
types and functions into scope, including some of the utilities API.

# Rust standard library

- The `Map` and `Set` types are the Rust's `HashMap` and `HashSet` types
  respectively.
*/

pub mod rust_standard_library {
    pub use std::{
        any::Any,
        borrow::Borrow,
        rc::Rc,
        sync::{
            Arc,
            RwLock,
            Mutex,
            Weak,
        },
    };
    pub use std::collections::{
        HashMap as Map,
        HashSet as Set,
    };
}
pub use rust_standard_library::*;

pub mod serialization {
    pub use rialight_util::serialization::{
        json,
        json::json,
        Serialize,
        Deserialize,
    };
}
pub use serialization::*;

pub use rialight_util::lazy_statics::lazy_static;
pub use rialight_util::flags::flags;
pub use rialight_util::collection_literals::{
    map, set,
    hash_map, hash_set,
    btree_map, btree_set,
};

pub mod reg_exp {
    pub use rialight_util::reg_exp::{
        StaticRegExp,
        RegExp,
        RegExpCaptures,
        reg_exp,
        static_reg_exp,
        reg_exp_captures,
        reg_exp_find,
        reg_exp_is_match,
        reg_exp_replace,
        reg_exp_replace_all,
    };
}
pub use reg_exp::*;

pub mod observable {
    pub use rialight_util::observable::{
        Observable,
        AbstractObserver,
        BoxedObserver,
        observer,
        Observer,
        SubscriberFunction,
        Subscription,
        SubscriptionObserver,
    };
}
pub use observable::*;

pub use rialight_util::string::StringIncognitoFormat;
pub use rialight_util::temporal;

pub mod futures {
    pub use rialight_util::futures::{
        Future,
        exec_future,
        future_all,
        future_race,
        ready_future,
    };
}
pub use futures::*;

pub use rialight_util::number::{BigInt, NonNegBigInt};