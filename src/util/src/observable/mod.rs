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
use rialight::prelude::*;

fn my_observable() -> Observable<String> {
    Observable::new(&|observer| {
        // send initial data
        observer.next("initial value");

        // return a cleanup function that runs once all observers
        // unsubscribe.
        &|| {
            dispose_of_observable();
        }
    })
}

let _ = my_observable()
    .subscribe(observer! {
        // subconsequent listeners can be omitted
        next: |value| {},
        error: |error| {},
        complete: || {},
        start: || {},
    })
    .unsubscribe();

// alias

let _ = my_observable()
    .subscribe(|value| { /* next */});

let _ = my_observable()
    .subscribe((
        |value| { /* next     */ },
        |error| { /* error    */ }
        ||      { /* complete */ }
    ));

// you can also use functional methods such as `filter` and `map`.
let _ = my_observable()
    .filter(|value| should_filter)
    .map(|value| new_value);
```

You can directly construct an `Observable` from a list of values:

```
Observable::from(["red", "green", "blue"])
    .subscribe(|color| {
        println!("{}", color);
    });
```
*/

use std::sync::{RwLock, Arc};

/// An `Observable` represents a sequence of values which
/// may be observed.
pub struct Observable<'a, T, Error = ()> {
    subscriber: SubscriberFunction<'a, T, Error>,
}

impl<'a, T, Error> Observable<'a, T, Error> {
    pub fn new(subscriber: SubscriberFunction<'a, T, Error>) -> Self {
        Self { subscriber }
    }

    pub fn subscribe(&self, observer: impl Into<BoxedObserver<T, Error>>) -> Arc<Subscription<T, Error>> {
        Subscription::new(observer.into(), self.subscriber)
    }
}

impl<'a, T, Iterable> From<Iterable> for Observable<'a, T, ()>
    where Iterable: IntoIterator<Item = T> + Send + Sync + 'a
{
    /// Constructs an `Observable` from a list of values.
    fn from(value: Iterable) -> Self {
        Self::new(&|observer| {
            for item in value.into_iter() {
                observer.next(item);
                if observer.closed() {
                    return &|| {};
                }
            }
            observer.complete();
            &|| {}
        })
    }
}

pub type SubscriberFunction<'a, T, Error = ()> = &'a (dyn Fn(SubscriptionObserver<T, Error>) -> &'a (dyn Fn() + Sync + Send) + Sync + Send);

pub struct Subscription<T, Error = ()> {
    cleanup: RwLock<Option<Arc<dyn Fn() + Sync + Send>>>,
    observer: RwLock<Option<Arc<RwLock<BoxedObserver<T, Error>>>>>,
}

impl<T, Error> Subscription<T, Error> {
    pub fn new<'a>(observer: BoxedObserver<T, Error>, subscriber: SubscriberFunction<'a, T, Error>) -> Arc<Self> {
        let this = Arc::new(Self {
            cleanup: RwLock::new(None),
            observer: RwLock::new(Some(Arc::new(RwLock::new(observer)))),
        });
        this.observer.read().unwrap().as_ref().unwrap().read().unwrap().start(Arc::clone(&this));

        // if the observer has unsubscribed from the start method, exit
        if subscription_closed(&this) {
            return this;
        }

        let observer = SubscriptionObserver { subscription: Arc::clone(&this) };

        // call the subscriber function.
        let cleanup = subscriber(observer);

        // the return value of the cleanup is always a function.
        *this.cleanup.write().unwrap() = Some(Arc::new(cleanup));

        if subscription_closed(&this) {
            cleanup_subscription(&this);
        }

        this
    }

    pub fn closed(&self) -> bool {
        subscription_closed(self)
    }

    pub fn unsubscribe(&self) {
        close_subscription(self);
    }
}

pub struct SubscriptionObserver<T, Error = ()> {
    subscription: Arc<Subscription<T, Error>>,
}

impl<T, Error> SubscriptionObserver<T, Error> {

    pub fn closed(&self) -> bool {
        subscription_closed(&self.subscription)
    }

    pub fn next(&self, value: T) {
        let subscription = self.subscription;

        // if the stream if closed, then exit.
        if subscription_closed(&subscription) {
            return;
        }

        let observer = subscription.observer.read().unwrap();
        if observer.is_none() {
            return;
        }

        // send the next value to the sink.
        observer.unwrap().read().unwrap().next(value);
    }

    pub fn error(&self, error: Error) {
        let subscription = self.subscription;

        // if the stream if closed, throw the error to the caller.
        if subscription_closed(&subscription) {
            return;
        }

        let observer = subscription.observer.read().unwrap();
        if let Some(o) = *observer {
            let o = o.read().unwrap();
            *subscription.observer.write().unwrap() = None;
            o.error(error);
        } else {
            // host_report_errors(e)
        }

        cleanup_subscription(&subscription);
    }

    pub fn complete(&self) {
        let subscription = self.subscription;

        // if the stream if closed, throw the error to the caller.
        if subscription_closed(&subscription) {
            return;
        }

        let observer = subscription.observer.read().unwrap();
        if let Some(o) = *observer {
            let o = o.read().unwrap();
            *subscription.observer.write().unwrap() = None;
            o.complete();
        }
        cleanup_subscription(&subscription);
    }
}

pub type BoxedObserver<T: Sized, Error = ()> = Box<dyn AbstractObserver<T, Error>>;

/// The `observer!` macro constructs an `Observer` by allowing
/// you to omit any of the listeners and not needing to box them explictly.
pub macro observer {
    // only next (no trailing comma)
    (
        next: $next_fn:expr
    ) => { Observer::<_, _> { next: Box::new($next_fn), error: Box::new(|error| {}), complete: Box::new(|| {}), start: Box::new(|| {}), } },
    // only next (with trailing comma)
    (
        next: $next_fn:expr,
    ) => { Observer::<_, _> { next: Box::new($next_fn), error: Box::new(|error| {}), complete: Box::new(|| {}), start: Box::new(|| {}), } },
    // next and error (no trailing comma)
    (
        next: $next_fn:expr, error: $error_fn:expr
    ) => { Observer::<_, _> { next: Box::new($next_fn), error: Box::new($error_fn), complete: Box::new(|| {}), start: Box::new(|| {}), } },
    // next and error (with trailing comma)
    (
        next: $next_fn:expr, error: $error_fn:expr,
    ) => { Observer::<_, _> { next: Box::new($next_fn), error: Box::new($error_fn), complete: Box::new(|| {}), start: Box::new(|| {}), } },
    // next, error, complete (no trailing comma)
    (
        next: $next_fn:expr, error: $error_fn:expr, complete: $complete_fn:expr
    ) => { Observer::<_, _> { next: Box::new($next_fn), error: Box::new($error_fn), complete: Box::new($complete_fn), start: Box::new(|| {}), } },
    // next, error, complete (with trailing comma)
    (
        next: $next_fn:expr, error: $error_fn:expr, complete: $complete_fn:expr,
    ) => { Observer::<_, _> { next: Box::new($next_fn), error: Box::new($error_fn), complete: Box::new($complete_fn), start: Box::new(|| {}), } },
    // next, error, complete, start (no trailing comma)
    (
        next: $next_fn:expr, error: $error_fn:expr, complete: $complete_fn:expr, start: $start_fn:expr
    ) => { Observer::<_, _> { next: Box::new($next_fn), error: Box::new($error_fn), complete: Box::new($complete_fn), start: Box::new($start_fn), } },
    // next, error, complete, start (with trailing comma)
    (
        next: $next_fn:expr, error: $error_fn:expr, complete: $complete_fn:expr, start: $start_fn:expr,
    ) => { Observer::<_, _> { next: Box::new($next_fn), error: Box::new($error_fn), complete: Box::new($complete_fn), start: Box::new($start_fn), } },
}

/// An `Observer` is used to receive data from an `Observable`, and
/// is supplied as an argument to `subscribe`.
pub struct Observer<T, Error = ()> {
    pub next: Box<dyn Fn(T) + Sync + Send>,
    pub error: Box<dyn Fn(Error) + Sync + Send>,
    pub complete: Box<dyn Fn() + Sync + Send>,
    pub start: Option<Box<dyn Fn(Arc<Subscription<T, Error>>) + Sync + Send>>,
}

impl<T, Error> AbstractObserver<T, Error> for Observer<T, Error> {
    fn next(&self, value: T) {
        self.next(value);
    }
    fn error(&self, error: Error) {
        self.error(error);
    }
    fn complete(&self) {
        self.complete();
    }
    fn start(&self, subscription: Arc<Subscription<T, Error>>) {
        self.start.map(|start| start(subscription));
    }
}

impl<T, Error> Default for Observer<T, Error> {
    fn default() -> Self {
        Self {
            next: Box::new(|_| {}),
            error: Box::new(|_| {}),
            complete: Box::new(|| {}),
            start: None,
        }
    }
}

impl<T> Into<Observer<T>> for &(dyn Fn(T) + Sync + Send) {
    fn into(self) -> Observer<T> {
        Observer { next: Box::new(self), error: Box::new(|_| {}), complete: Box::new(|| {}), start: None }
    }
}

impl<T, Error> Into<Observer<T, Error>> for (&(dyn Fn(T) + Sync + Send), &(dyn Fn(Error) + Sync + Send)) {
    fn into(self) -> Observer<T, Error> {
        Observer { next: Box::new(self.0), error: Box::new(self.1), complete: Box::new(|| {}), start: None }
    }
}

impl<T, Error> Into<Observer<T, Error>> for (&(dyn Fn(T) + Sync + Send), &(dyn Fn(Error) + Sync + Send), &(dyn Fn() + Sync + Send)) {
    fn into(self) -> Observer<T, Error> {
        Observer { next: Box::new(self.0), error: Box::new(self.1), complete: Box::new(self.2), start: None }
    }
}

impl<T, Error> Into<BoxedObserver<T, Error>> for Observer<T, Error> {
    fn into(self) -> BoxedObserver<T, Error> {
        Box::new(self)
    }
}

impl<T> Into<BoxedObserver<T>> for &(dyn Fn(T) + Sync + Send) {
    fn into(self) -> BoxedObserver<T> {
        Box::<Observer<T>>::new(self.into())
    }
}

impl<T, Error> Into<BoxedObserver<T, Error>> for (&(dyn Fn(T) + Sync + Send), &(dyn Fn(Error) + Sync + Send)) {
    fn into(self) -> BoxedObserver<T, Error> {
        Box::<Observer<T, Error>>::new(self.into())
    }
}

impl<T, Error> Into<BoxedObserver<T, Error>> for (&(dyn Fn(T) + Sync + Send), &(dyn Fn(Error) + Sync + Send), &(dyn Fn() + Sync + Send)) {
    fn into(self) -> BoxedObserver<T, Error> {
        Box::<Observer<T, Error>>::new(self.into())
    }
}

/// An `AbstractObserver` is used to receive data from an `Observable`, and
/// is supplied as an argument to `subscribe` in boxed form.
pub trait AbstractObserver<T, Error = ()>: Send + Sync {
    fn next(&self, value: T) {}
    fn error(&self, error: Error) {}
    fn complete(&self) {}
    fn start(&self, subscription: Arc<Subscription<T, Error>>) {}
}

fn cleanup_subscription<T, Error>(subscription: &Subscription<T, Error>) {
    assert!(subscription.observer.read().unwrap().is_none());
    let cleanup = *subscription.cleanup.read().unwrap();
    if cleanup.is_none() {
        return;
    }
    let cleanup = Arc::clone(&cleanup.unwrap());

    // drop the reference to the cleanup function so that we won't call it
    // more than once.
    *subscription.cleanup.write().unwrap() = None;

    // call the cleanup function.
    cleanup();
}

fn subscription_closed<T, Error>(subscription: &Subscription<T, Error>) -> bool {
    let observer = *subscription.observer.read().unwrap();
    observer.is_none()
}

fn close_subscription<T, Error>(subscription: &Subscription<T, Error>) {
    if subscription_closed(subscription) {
        return;
    }
    *subscription.observer.write().unwrap() = None;
    cleanup_subscription(subscription);
}