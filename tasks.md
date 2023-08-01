# Tasks

Working at temporal:

- References
  - https://tc39.es/proposal-temporal/docs
  - https://github.com/tc39/proposal-temporal
- [ ] Start and finish the ambiguity documentation in docs/ambiguity.rs
  - https://tc39.es/proposal-temporal/docs/ambiguity.html
- [ ] There are a few documentation pages in addition to ambiguity too; just iterate the TC39 Temporal API docs. Add all the home sections too to summarize the API and the _Other documentation_ section.
- [ ] Fully document the public API according to the TC39 proposal
- [ ] Check if `chrono` and `chrono-tz` can really be used to implement the functionality in the temporal API internally. It doesn't need to be exactly the same as the TC39's proposal. If not really viable, remove them from `Cargo.toml` and find an alternative, but it looks like it'll work well! About plain types: use the naive module from `chrono`.
- [ ] `temporal::now`
  - [ ] `instant`
  - [ ] `timezone_id`
  - [ ] `zoned_date_time`
  - [ ] `zoned_date_time_iso`
  - [ ] `plain_date`
  - [ ] `plain_date_iso`
  - [ ] `plain_time_iso`
  - [ ] `plain_date_time`
  - [ ] `plain_date_time_iso`
- [ ] Types with addition and subtraction also implement `(Add|Sub)Assign` (`+=` and `-=`)
- [ ] `temporal::Instant`
- [ ] `temporal::ZonedDateTime`
- [ ] `temporal::PlainDate`
- [ ] `temporal::PlainTime`
- [ ] `temporal::PlainDateTime`
- [ ] `temporal::PlainYearMonth`
- [ ] `temporal::PlainMonthDay`
- [ ] `temporal::Duration`
  - Add `From<std::time::Duration>`
  - Add `Into<std::time::Duration>`
  - Wraps a `std::time::Duration` and uses `chrono::Duration` for access. It is constructed as `Duration::new()` and chained in various ways like `years(n)` and `months(n)` and things can be accessed like `num_years()`.
- [ ] `temporal::TimeZone`
- [ ] `temporal::Calendar`

Working at file system:

- Design an API that works across all platforms, including Android.
  - [ ] Provide ways of requesting permissions using asynchronous results that works across all platforms
  - For synchronous operations:
    - Panic for the browser
    - For non-browser targets, use `std::fs`
  - For asynchronous operations:
    - Use Tokio runtime for non-browser targets
    - Use JavaScript promises for browser targets
  - Always use `rialight_util::file_paths::os_based` instead of `rialight_util::file_paths` internally in path manipulations.
  - [ ] Windows
    - [ ] For native paths, the path prefix is either `drive:` or `\\`.  `drive` is a case-insensitive letter.
  - [ ] Android
    - [ ] On Android, `app:` and `app-storage:` do not use a static from the Rialight core internals; call the Java API function [`context.getFilesDir`](https://developer.android.com/reference/android/content/Context#getFilesDir()).
  - Consult the _Additional Platform Detection_ section for how WebAssembly-based platforms are detected, including the browser.
  - [ ] Document the API

When futurely working on graphical nodes:

- Provide the types `Node` and `WeakRefNode`. Inside `Node` is stored an internal `Arc<NonRefNode>` and inside `WeakRefNode` is an internal `Weak<Gc<NonRefNode>>` to which it dereferences.
- The parent is stored as a `WeakRefNode` internally.
- Store a node kind in a `Node` behind an `Arc`, inside an union containing other node kinds.
- The equality operator compares by reference (`Arc::ptr_eq`) and the clone method clones the reference (`Arc::clone`). _Do not_ use `#[derive(Clone)]`; implement it manually to make sure no content is cloned:
```rust
impl Clone for Node {
    fn clone(&self) -> Self {
        Self { n: Arc::clone(&self.n) }
    }
}
```
- `node.clone_by_content()` clones the node by content and not reference, including the children. This is equivalent to the next fine-tuned method.
  - This method will clone all events and attributes
- `node.clone_by_content_fine_tuned(flags)` is similiar to the previous method, but reserves for future flags.

When futurely working with internationalization:

- Use ICU internally and wrap it _entirely_ instead of aliasing to it:
  - [Display names for language and region](https://github.com/unicode-org/icu4x/issues/3167)
  - [Default data provider](https://github.com/unicode-org/icu4x/issues/3180)
  - [Locale directionality](https://github.com/unicode-org/icu4x/issues/3172)

When futurely working with the application entry point:

- For Android, mutate the `Context` static. Define that static only for the Android OS (`#[cfg(target_os = "android")]`).
- File system statics
  - Mutate things such as application installation and storage path
    - For Android, no static path is used. A static variable holds the Android `Context` instead.

When futurely working in the CLI:

- `rialight debug` or `run`
  - Pass the feature `rialight_default_export` to `cargo run` internally as the host environment for debugging is generally not a web browser.
- `rialight export`
  - Pass the feature `rialight_default_export` to `cargo run` internally for a non-browser export

## When futurely working in the attribute macros `rialight::main` and `rialight::build_main`

The asynchronous runtime looks like this in `feature = "rialight_default_export"`:

```rust
#[tokio::main]
async fn main() {
    let local_task_set = tokio::task::LocalSet::new();
    local_task_set.run_until(async move {
        // async code goes here
    }).await;
}
```

The asynchronous runtime looks like this in `feature = "rialight_browser_export"`:

```rust
#[wasm_bindgen]
fn main() {
    wasm_bindgen_futures::spawn_local(async move {
        // async code goes here
    });
}
```

The asynchronous runtime looks like this when `not(any(feature = "rialight_default_export", feature = "rialight_browser_export"))`:

```rust
fn main() {
    panic!("Incorrect Rialight runtime configuration. \
            If you are running a Cargo command to build \
            your application, you must always instead use \
            a Rialight command to build it.");
}
```

## Web Browser Tasks

- When working with the browser, refer mostly to the `wasm_bindgen` library.
  - API: https://rustwasm.github.io/wasm-bindgen/api/web_sys
    - Just search there and you'll find JavaScript things
  - JavaScript Glue
    - https://rustwasm.github.io/wasm-bindgen/examples/import-js.html
- [ ] The browser does not use the Tokio runtime as the browsers are single-threaded. For the timeouts, use `setTimeout` (and some maybe... `setInterval`) from JavaScript inside a JavaScript promise and pass it to Rust via `wasm-bindgen-futures`.
  - https://users.rust-lang.org/t/does-tokio-work-in-the-browser-if-i-use-only-a-single-thread/97663?u=hydroper1
  - Conversion between Rust futures and JavaScript promises: https://crates.io/crates/wasm-bindgen-futures
- [ ] In the graphics APIs, User inputs and some events have to be handled based on the browser's page events

## Inspire

- ECS: Bevy Engine
- https://github.com/timi-liuliang/echo