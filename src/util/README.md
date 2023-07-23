# rialight::util

Rialight utilities module.

## Features

- Literal initializers for HashMap, HashSet, BTreeMap and BTreeSet, such as `hashmap!{}`.
- Lazy statics. Refer to the crate [lazy_static](https://crates.io/crates/lazy_static) for more information.
- Regular expressions. Refer to the crate [lazy-regex](https://crates.io/crates/lazy-regex) for more information.
- Bit flags. Refer to the crate [bitflags](https://crates.io/crates/bitflags) for more information.
- String formatting as the module `rialight::util::template_string`.
- Escaping URIs and their components. This includes the methods `encode_uri`, `decode_uri`, `encode_uri_component` and `decode_uri_component`.
- File path functions such as `resolve()` and `relative()`.