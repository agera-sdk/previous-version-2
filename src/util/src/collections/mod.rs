/*!
Work with collections.

# Map type

Use the `Map` type to pair key-values. It uses a hash algorithm internally.
You can initialize any map type, including `Map`, using the `map!` literal:

```
# use rialight_util::collections::{Map, Set};
# use rialight_util::collection_literals::{map, set};
let _: Map<_, _> = map! {"key" => "value"};
```

# Set type

Use the `Set` type to construct a set of values. It uses a hash algorithm internally.
You can initialize any set type, including `Set`, using the `set!` literal:

```
# use rialight_util::collections::{Map, Set};
# use rialight_util::collection_literals::{map, set};
let _: Set<_> = set! ["value1", "value2"];
```
*/

pub use std::collections::{
    HashMap as Map,
    HashSet as Set,
};