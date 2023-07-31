/*!
The Rialight prelude API.
*/

pub use std::{
    any::Any,
    borrow::Borrow,
    cell::{Cell, RefCell},
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
pub use rialight_util::serialization::{
    json,
    json::json,
    Serialize,
    Deserialize,
};
pub use rialight_util::lazy_statics::lazy_static;
pub use rialight_util::flags::flags;
pub use rialight_util::collection_literals::{map, set};
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
pub use rialight_util::string::StringIncognitoFormat;
pub use rialight_util::temporal;
pub use rialight_util::futures::{
    Future,
    exec_future,
    future_all,
    future_race,
    ready_future,
};
pub use rialight_util::number::{BigInt, UnsignedBigInt};