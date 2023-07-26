/*!
Work with collections.

# Map type

Use the `Map` type to pair key-values using a hash algorithm.
You can initialize any map type, including `Map`, using the `map!` literal:

```
let _: Map<_, _> = map! {"key" => "value"};
```

# Set type

Use the `Set` type to construct a set of values using a hash algorithm.
You can initialize any set type, including `Set`, using the `set!` literal:

```
let _: Set<_> = set! ["value1", "value2"];
```
*/

pub use std::collections::{
    HashMap as Map,
    HashSet as Set,
};