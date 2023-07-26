/*!
Work with observables.

The [`Observable`] type can be used to model push-based
data sources. In addition, observables are:

- _Compositional:_ Observables can be composed with higher-order
combinators.
- _Lazy:_ Observables do not start emitting data until an **observer**
has subscribed.

This module follows the [TC39 `Observable`](https://github.com/tc39/proposal-observable) proposal.

# Example

```
use rialight::prelude::*;

fn my_observable() -> Observable<String> {
    Observable::new(|observer| {
        // send initial data
        observer.next("initial value");

        // return a cleanup function that runs once all observers
        // unsubscribe.
        || {
            dispose_of_observable();
        }
    })
}

let _= my_observable()
    .subscribe(Observer {
        next: |value| {
            // next
        },
        error: |error| {
            // error
        },
        complete: || {
            // complete
        },
    })
    .unsubscribe();

// you can also use functional methods such as `filter` and `map`.
let _ = my_observable()
    .filter(|value| should_filter)
    .map(|value| new_value);
```
*/

/// An `Observable` represents a sequence of values which
/// may be observed.
pub struct Observable<'a, T, Error = ()> {
    subscriber: SubscriberFunction<'a, T, Error>,
}

impl<'a, T, Error> Observable<'a, T, Error> {
    pub fn new(subscriber: SubscriberFunction<'a, T, Error>) -> Self {
        Self { subscriber }
    }

    pub fn subscribe(observer: impl Into<Box<dyn AbstractObserver<T, Error>>>) -> Subscription<T, Error> {
    }
}

pub type SubscriberFunction<'a, T, Error = ()> = &'a dyn FnMut(SubscriptionObserver<T, Error>) -> &'a dyn FnMut();

pub struct Subscription<T, Error = ()> {
}

/// `Observer` is the default implementation for the `AbstractObserver` trait.
pub struct Observer<'a, 'b, 'c, T, Error = ()> {
    pub next: &'a dyn FnMut(T),
    pub error: &'b dyn FnMut(Error),
    pub complete: &'c dyn FnMut(),
}

impl<'a, 'b, 'c, T, Error> AbstractObserver<T> for Observer<'a, 'b, 'c, T, Error> {
}

/// The `AbstractObserver` trait represents an abstract observer
/// that may be defined by the user. [`Observer`] is the default
/// implementation of this trait.
pub trait AbstractObserver<T, Error = ()> {
    fn start(&self, subscription: Subscription<T, Error>);
    fn next(&self, value: T);
    fn error(&self, error: Error) {}
    fn complete(&self) {}
}

impl<'a, 'b, 'c, T, Error> Into<Box<dyn AbstractObserver<T, Error>>> for Observer<'a, 'b, 'c, T, Error> {
    fn into(self) -> Box<dyn AbstractObserver<T, Error>> {
        Box::new(self)
    }
}

impl<T> Into<Box<dyn AbstractObserver<T>>> for &dyn FnMut() {
    fn into(self) -> Box<dyn AbstractObserver<T>> {
        Box::new(Observer {
            next: self,
            error: |_| {},
            complete: |_| {},
        })
    }
}