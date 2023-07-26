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