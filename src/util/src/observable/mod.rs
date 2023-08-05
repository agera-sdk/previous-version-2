/*!
Work with observables.

The [`Observable`] type can be used to model push-based
data sources. In addition, observables are:

- _Compositional:_ Observables can be composed with higher-order
combinators.
- _Lazy:_ Observables do not start emitting data until an **observer**
has subscribed.

This module follows the [TC39 `Observable`](https://github.com/tc39/proposal-observable) proposal.
User observers other than `Observer` can be defined by implementing
the `AbstractObserver` trait.

# Example

```
# use rialight_util::observable::*;

fn my_observable() -> Observable<String> {
    Observable::new(|observer| {
        // send initial data
        observer.next("initial value".into());

        // return a cleanup function that runs once all observers
        // unsubscribe.
        || {
            println!("cleanup on unsubscribe");
        }
    })
}

let _ = my_observable()
    .subscribe(observer! {
        next: |value| {},
        error: |error| {},
        complete: || {},
        start: |subscription| {},
    })
    .unsubscribe();

// you can also use functional methods such as `filter` and `map`.
let _ = my_observable()
    .filter(|value| true)
    .map(|value| value);
```

You can directly construct an `Observable` from a list of values:

```
# use rialight_util::observable::*;
Observable::from(["red", "green", "blue"])
    .subscribe(observer! {
        next: |color| {
            println!("{}", color);
        },
    });
```
*/

pub use rust_observable::*;