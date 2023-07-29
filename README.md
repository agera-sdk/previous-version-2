# Rialight

> **Note:** Rialight is not yet done.

Rialight aims to be a multi-purpose gaming and graphical application framework combining reactivity and nodes and shipping various fundamental APIs, requiring you to just know the Rust standard library and the Rialight API.

Rialight can be used for creating graphical applications, both two-dimensional (2D) and three-dimensional (3D), but **cannot be** used for creating websites. Rialight applications can be embedded in websites.

Rialight experiences can be run in mobile, desktop, gaming consoles and web browsers.

## Draft Ideas

### Project Template

When using the `rialight` command, you create new projects instantly:

- `rialight new my-app`
  - Creates an empty graphical application.
- `rialight new --game my-game`
  - Creates an empty game.
- `rialight new --cli my-command`
  - Creates an empty command-line interface application.

The project templates share common functionality, including translation resources which use the [Fluent syntax](https://projectfluent.org).

There is always a build script, `build.rs`, at the root of the project, which uses an empty function with a `#[rialight::build_main]` attribute. It is used internally by Rialight, but you don't need to touch it.

### Debugging and Exporting

Exporting a project should bundle its assets files into the installer, which can be later retrieved through the File API using an `app:` URI.

Rialight uses the Rust's package manager that comes with its installation, however you mustn't use `cargo run` to debug your application as it needs passing a feature to Cargo. You can debug with `rialight run` or `rialight debug`.

To export your application, use a Rialight command such as:

```
rialight export --platform browser
```

If you simply try using aa Cargo command instead to export your application, the runtime will panic during initialization.

### Graphics

The `rialight::graphics` and `rialight::ui` APIs co-work together.

- Nodes, the primary way to construct a visual application.
  - The `Node` object is the primary item of the graphics API, which has a limited set of various variants, such as `Rectangle`, `Canvas`, `Button`, `TabBar` and `Modal`. All of them share full customisation and common properties like visibility and transform (including 3D matrix) which are inherited by default from their parent.
    - _Reference:_ A `Node` is a thread-safe reference type that uses reference counting internally. If you need a weak reference to a node, you can downgrade it to a `WeakRefNode`.
    - _Children:_ The `Node` type supports common child operations. It also supports node paths described later in this list. `node.children()` returns an iterator.
    - Meta data (optional mapping from `String` to `MetaDataValue` for attaching any data)
      - `pub type MetaDataValue = Box<dyn Any + Send + Sync + Clone>;`
  - Nodes don't describe just graphics. They also emit events, accessed as `node.on_some_event().listen(listen_fn)`, such as `on_enter_frame` and `on_click` events.
    - Somes nodes may not have a certain event, which is a rare case, panicking when retrieving it. In that case, for an event that is not supported by all node kinds, the documentation can list the only supported node kinds.
  - Few events are not accessed as listeners, using a single callback instead:
    - `node.on_enter_frame(enter_frame_fn)` sets listener for the enter frame event
    - `node.on_user_input(user_input_fn)` sets listener for an user input event
  - A node has an identifier. `node.id()` and `node.set_id(id)`
  - _Finding nodes_: The most common method for finding nodes by identifier is `by_path`, which accepts a node path.
  - _Node paths:_ Node paths are paths using the slash (`/`) separator and `..` and `.` portions. A `..` portion resolves to the parent and a `.` portion resolves to the current node. If a node path is absolute, that is, it starts with a path separator, it resolves a node relative to the topmost parent.
    - `node.get_path()` returns the absolute node path.
  - _Node kinds:_ The `node.is::<NodeKind>` method can be used to test if a node is of a specific kind (it uses a private trait `NodeIs` that only the supported node kinds implement). Note that the set of node kinds cannot be extended. Node kinds have dedicated types for consulting their API documentation, such as `Button`. Calling `Button::new` returns a `Node`; however `Button` itself is not the `Node` type. The home API documentation for `rialight::graphics` has a list of supported node kinds, referencing the dedicated types.
  - _Node representation:_ Internally, a node kind holds internal data that is stored behind a `Arc` inside `Node`. The `Node` type contains a single internal `Arc` that refers to further data, including common properties and an union of node kinds (stored in an `Arc`).
  - _Chaining:_ Most of the `Node` methods, such as `set_visibility`, are chainable, returning a clone of the node's reference. These methods are defined similiarly to:
```rust
impl Node {
    pub fn set_something(&self) -> Self {
        // set something
        self.clone()
    }
}
```
  - _Cloning:_ `Node` is cloned by reference by default, not by content. Use `node.clone_by_content()` to clone a node by content and not by reference.
  - _UI:_ Node kinds that are user interface specific (such as `Button`) are exported at the `rialight::graphics::ui` submodule to avoid confusion. They are also exported by the user interface API.
    - [ ] Optional text selection on non text inputs (text labels)
  - _Inheritance:_ Properties such as visibility, opacity, rotation and scale are inherited by default, with an _inherited_ variant. There may be more of such properties other than these that are inherited.
  - _Responsivity:_ Node measures are device-oriented. They use the mathematical API.
  - _Positioning:_ A node's position can be either derived, absolute or relative.
    - Derived means the node's position is determined by the parent.
    - Absolute means the node is positioned at a given global position.
    - Relative means the node is positioned relative to the parent's global position with given coordinates.
  - _Common properties:_ Scale, opacity, visibility, position, rotation, size and maybe some more.
  - _Sizing:_ A node can have a size variant: none, full and specific (given measurement units). Nodes whose position is not _derived_ often need to specify a size variant, otherwise they may not be displayed.
  - _Not covered here yet:_ Alignment, minimum sizing and maybe more.
- Skins
  - Nodes share skins. Skins are inherited by default. Skins describe styles, style transitions and some behaviors.
  - Skins are divided by node kind. That is, a specific style applies to a specific node kind.
  - Skins are described in Rust code.
  - Rialight may use either raw Rust or a markup macro for writing components.

Accessibility:

- Focus
  - Focus neighbors
    - Automatic focus neighbors on containers
  - Focus configuration
    - You can optionally allow user inputs other than touch or pointer to switch control focus.
- Touch

### 3D Graphics

The 3D graphics API, `rialight::graphics_3d`.

- The most important type is `Node3d`. It is not compatible with the two-dimensional `Node` type and cannot be mixed with it.

### UI

The UI API, `rialight::ui`.

- The UI API exports all the node kinds related to user interface (such as `Button`) from the submodule `rialight::graphics::ui` of the graphics API. Such node kinds are not exported directly by the graphics API to avoid confusion.
- The UI API defines interfaces for reactive UI component which are defined by the developer.
  - An UI component may use graphics nodes from the graphics API, `rialight::graphics`.
  - _Reactive_ data can be shared across all UI components. There may be a proper API for that. In that case, when a state changes, it causes parts of a component that use that state to render again.

### File System

Ideas for the File System API, `rialight::filesystem`.

The `File` object can support the `file:`, `app:` and `app-storage:` URIs.

- `file:` refers to files in the user's device file system.
- `app:` refers to files in the application installation directory. They are assets originally included in the application source that are bundled within the application installer. These files are read-only and cannot be manipulated.
  - In the browser, these files are stored in the RAM.
- `app-storage:` refers to files in the application data storage directory. They are data stored dynamically in the application with persistence.

If you need to use `app-storage:` in the browser, switch to using `rialight::filesystem::webcompat::File`.

#### Web-Compatible File System

The `app-storage:` URI does not work when exporting the project to the browser, because of lacking API in the browser, including synchronous operations. In case you need this feature, use a specialized version of `File` that works with `app-storage:` across all platforms, `rialight::filesystem::webcompat::File`.

For the `app-storage:` URI, this uses the origin-private file system API in the browser.
  - https://users.rust-lang.org/t/bindings-for-browser-origin-private-fs/97417/2?u=hydroper1

Due to this, the web-compatible file system API is entirely asynchronous.

### Gaming

Rialight supports a gaming API based on the Entity-Component-System pattern, which is essential for game developers, with support for physics. This API runs concurrent systems, however platforms without multi-threading support (browser) do not run systems concurrently.

The Gaming API is an optional feature that can be turned on or off.

### Events

Ideas for the event API, `rialight::event`.

- `Event<T>`
  - An event that can be listened to. `event.listen(|e| {});`
- `EventListener`
  - Object returned by `event.listen(listen_fn)`.
  - Can be cancelled: `event_listener.cancel();`
- Structures for native events, including touch and keyboard events.

### Mathematics

Ideas for the mathematics API, `rialight::math`.

- Geometry
  - Defines shapes and intersections.
  - Shapes have coordinates, including rectangles.
- SI (Système International d'unités)
  - Measurement units and their conversions.

### Utilities

Ideas for the utilities API, `rialight::util`. The utilities API is standalone and does not require the other Rialight APIs, so it can be used for unrelated Rust projects.

- Temporal API
  - Based on [this TC39 Temporal API](https://github.com/tc39/proposal-temporal).
- Lazy Statics
- Collection Literals (map and set)
- Flags
- Bytes for working with binaries
- Serialization
- Regular Expression pattern
  - Support for comments and whitespace using the `x` flag.
  - API strives to be as flexible as the JavaScript's one.
  - `regex.replace(str, replacement)` accepts either a string or callback as argument and is the same as JavaScript's `str.replace(regex, replacement)`.
  - `regex.replace_all(str, replacement)`
- `Observable`
  - Based on [this TC39 proposal](https://github.com/tc39/proposal-observable).
- URI and URI Component Encoding

### Network

The network API, `rialight::net`. The internationalization API uses the HTTP part of this API for loading developer translations occasionally if the developer desires.

- HTTP client (not _server_)
  - For multi-thread platforms: use the crate `hyper` internally (not `reqwest` due to how it detects the browser via `wasm32` arch.)
    - However, if https://github.com/seanmonstar/reqwest/issues/1917 is solved, use:
      - `reqwest::browser`
      - `reqwest::tokio`
      - And perform two `#[cfg(...)]` according to export platform.
- Sockets (TCP abstraction; in the browser it uses WebSockets)
- UDP

### Media

The media API, `rialight::media`.

- Video
- Camera

### Sound

The sound API, `rialight::sound`.

- No ideas yet.

### Crypto

The crypto API, `rialight::crypto`.

- No ideas yet.

### Security

The security API, `rialight::security`.

- No ideas yet.

### Accessibility

The accessibility API, `rialight::a11y`.

- No ideas yet.

### Internationalization

The internationalization API, `rialight::intl`.

- Locale object
  - Text direction
- Display Names and More
- Translations
  - This API can use the network API for downloading translations if the developer desires; however, most developers will simply use the `app:` file URI from the file system API.

### Core

The core API, `rialight::core`, basically defines the application interfaces. It can cover:

- Application Translations
- Application Input Maps
  - They can be remapped in the runtime.
- Application Shortcuts
  - They can be remapped in the runtime.
  - Used for instance by media editing softwares.
- Command Line Interface
  - Allows a graphical application to also be used as a command in a terminal. An application can be configured to be launched graphically manually, allowing to only launch it according to the given command line arguments.
  - Help should be included by default, not launching the graphical application if `--help` or `-h` is specified.

The core internals, `rialight::core_internals`, should not be used anywhere. They are used by the APIs, including file system, for instance, to determine the application's installation directory.

### Prelude

The `rialight::prelude` crate can be used to include commonly used things in scope. It includes:

- Some of the Rust standard library, including:
  - `Any`
  - `Map` and `Set` as aliases to `HashMap` an `HashSet`
  - Types for concurrency and reference counted boxes
- Map and Set Collections that use a hash algorithm (same from the standard library, `std::collections`)
- Collection Literals
- Regular expressions
- Bitwise Flags
- Lazily Evaluated Statics
- JSON Serialization
- Observables
- Temporal API (`temporal` as a global module)

All of these are described in the utilities API.

### Working With the Browser

When a developer wants to run a portion of Rust code for the browser only, it is recommended to detect the browser via `#[cfg(feature = "rialight_browser_export")]` and not `#[cfg(target_arch = "wasm32")]` as WebAssembly is used for unknown platforms.

Rialight provides no browser-specific APIs. If you need to communicate with JavaScript, you may want to add this to your Cargo.toml:

```
# browser export only dependencies
js-sys = { version = "0.3.64", optional = true }
web-sys = { version = "0.3.64", optional = true }
wasm-bindgen = { version = "0.2.87", optional = true }
wasm-bindgen-futures = { version = "0.4.37", optional = true }

# browser export only dependencies
rialight_browser_export = [
    "js-sys",
    "web-sys",
    "wasm-bindgen",
    "wasm-bindgen-futures",
]
```

The [`wasm-bindgen`](https://rustwasm.github.io/wasm-bindgen/introduction.html) library allows communicating with JavaScript.

### Visual Editor

Once Rialight develops, it can have a visual editor for the following use-cases:

- Generic software
- Gaming

This visual editor will require an external IDE for logical programming, such as Visual Studio Code.

## Additional Platform Detection

Internally, Rialight uses Cargo features, including `rialight_browser_export`, to detect platforms that are not operating systems, including browsers, and futurely gaming consoles, since most of these use a WebAssembly target such as `wasm32-unknown-unknown`, where the OS does not exist.

Current features used for platform detection:

- `rialight_browser_export`

You should not worry about specifying these Cargo features as you'll be using the Rialight commands to build or export your application to a specific platform, such as `rialight export --platform browser`.

## Comparison to Other Technologies

- The concept of nodes is similiar to the concept of DOM elements: you cannot subtype a specific DOM element kind and instead use the existing ones. Although the framework strives to have as many node kinds as possible, you may need to wrap it into an unrelated type or create an UI component from the UI API (`rialight::ui`).

## Rust Setup

The framework currently requires the nightly Rust to be used. It is easy to switch to `nightly`:

```
rustup default nightly
```

## Tasks

Working at timeouts:

- [x] wrap `interval`
- [x] wrap `interval_at`
  - In browser, `interval_at` uses a `setTimeout` internally (uses `Date.now() - instant.epochMilliseconds`).
- [ ] Note the following types wrap yet another "private" type, which is defined according to a `#[cfg]` attribute (also add a `#[cfg]` case for both export features not listed so that the code the compiles).
- [ ] wrap `Interval`
  - Do not define missed_tick_behavior as it may not be possible on JavaScript
  - On JavaScript: interval works like this:
    - https://users.rust-lang.org/t/future-based-interval-in-the-browser/97693/3
    - Use approach 1 (**WITHOUT** `setInterval`) in the above post, using the optimal solution not using `setInterval` nor `setTimeout`
- [ ] wrap `Timeout`
- [x] wrap `Instant`
- [ ] wrap `Wait`
- [ ] For each function of the timeout module, provide two `#[cfg]`-based implementations: one that uses Tokio and one that uses a browser's JavaScript promise. The existing Tokio implementation needs to use conversion. Make sure each feature `#[cfg]` works.
  - In JavaScript, instants contain the number of milliseconds elapsed since the epoch, obted from `Date.now()` most commonly. This is used for things like `wait_until`.
  - For `interval`, panic if given period is zero
  - [`web-sys`](https://crates.io/crates/web-sys)
  - [`wasm-bindgen-futures`](https://crates.io/crates/wasm-bindgen-futures)
- [ ] Add an additional "cancellable" timeout function, `background_timeout` that returns a `BackgroundTimeout` which is not a future. This function receives a `FnOnce() + Send + 'static` callback.
  - In browser this uses `setTimeout` (tracked timeout is assigned to -1 before polling to Rust future) + `clearTimeout` (invoked if tracked timeout is not -1)
  - In Tokio this spawns a thread with a wait. The thread holds an `Arc` which it receives from the spawning thread, that indicates whether the timeout was cancelled. After the wait, if this `Arc` indicates the timeout was "not" cancelled, it calls the callback from the developer.
- [ ] Add a `background_interval` function similiar to background timeout function that returns `BackgroundInterval`.
  - On JavaScript, **NEVER** use `clearInterval`! Just use the solution at https://users.rust-lang.org/t/future-based-interval-in-the-browser/97693/3

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
  - For asynchronous operations:
    - Use Tokio runtime for non-browser targets
    - Use JavaScript promises for browser targets
  - Always use `rialight_util::file_paths::os_based` instead of `rialight_util::file_paths` internally in path manipulations.
  - [ ] Windows
    - [ ] For native paths, the path prefix is either `drive:` or `\\`.  `drive` is a case-insensitive letter.
  - [ ] Android
    - [ ] On Android, `app:` and `app-storage:` do not use a static from the Rialight core internals; call the Java API function [`context.getFilesDir`](https://developer.android.com/reference/android/content/Context#getFilesDir()).
  - [ ] Web-compatible `File` API at `rialight::filesystem::webcompat` 
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

When futurely working in the attribute macros `#[rialight::main]` and `#[rialight::build_main]`:

- `rialight::main` expands to use either the multi-threaded Tokio runtime (`#[tokio::main]`) or a single-threaded one for the web browser by using [`wasm_bindgen`](https://crates.io/crates/wasm_bindgen) and `wasm_bindgen_futures` together.
```
#[wasm_bindgen]
fn my_entry_point() {
    wasm_bindgen_futures::spawn_local(async move {
        // async code goes here
    });
}
```
  - If none of the features `rialight_default_export` and `rialight_browser_export` are passed, then `rialight::main` will panic telling the runtime was incorrectly configured. They are passed by the Rialight CLI automatically.

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