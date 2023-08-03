# Rialight

> **Note:** Rialight is not yet done. [Here are the current tasks.](tasks.md)

Rialight aims to be a multi-purpose gaming and graphical application framework combining reactivity and nodes and shipping various fundamental APIs, requiring you to just know the Rust standard library and the Rialight API. Rialight is designed for the Rust language.

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

The `Cargo.toml` file contains a `package.metadata.rialight` section, which contains configuration for the Rialight application, contains `features.default = ["rialight_default_export"]` and it also passes two features to the `rialight` crate to indicate whether the build target is the browser or not. You don't need to touch this.

### Debugging and Exporting

Exporting a project should bundle its assets files into the installer, which can be later retrieved through the File API using an `app:` URI.

Rialight uses the Rust's package manager that comes with its installation, Cargo. You can debug either with Cargo or the Rialight command interface, through `rialight run` or `rialight debug`.

To export your application, use a Rialight command such as:

```
rialight export --platform browser
```

### Graphics

The `rialight::graphics` and `rialight::ui` APIs co-work together.

- Nodes, the primary way to construct a visual application.
  - The `Node` object is the primary item of the graphics API, which has various variants, such as `Rectangle`, `Canvas`, `Button`, `TabBar` and `Modal`. All of them share full customisation and common properties like visibility, skin and transform (including 3D matrix) which are inherited by default from their parent.
    - _Reference:_ A `Node` is a thread-safe reference type that uses reference counting internally. If you need a weak reference to a node, you can downgrade it to a `WeakRefNode`.
    - _Children:_ The `Node` type supports common child operations. It also supports node paths described later in this list. `node.children()` returns an iterator. Methods like `append_children` are chainable and accept an iterable.
    - Meta data (optional mapping from `String` to `MetaDataValue` for attaching any data)
      - `pub type MetaDataValue = Box<dyn Any + Send + Sync + Clone>;`
  - _Events_: _They also emit events, accessed as `node.on_some_event().listen(listen_fn)`, such as `on_enter_frame` and `on_click` events.
    - Somes nodes may not have a certain event, which is a rare case, in which case you may want to access that event from the node kind instead (as by `node.to::<SpecificKind>().unwrap().on_some_event()`). In that case, for an event that is not supported by all node kinds, the documentation can list the only supported node kinds.
    - Few events are not accessed as listeners, using a single callback instead:
      - `node.on_enter_frame(enter_frame_fn)` sets listener for the enter frame event
      - `node.on_user_input(user_input_fn)` sets listener for an user input event
    - The enter frame event receives the delta (the time elapsed since the last frame).
  - _Identifier:_ A node has an identifier. `node.id()` and `node.set_id(id)` (optional, so `id` is `Option<String>`). If an identifier is none, node paths take it as if it were the zero-based index of the node in the children collection as a string.
  - _Finding nodes_: The most common method for finding nodes by identifier is `by_path`, which accepts a node path.
  - _Node paths:_ Node paths are paths using the slash (`/`) separator and `..` and `.` portions. A `..` portion resolves to the parent and a `.` portion resolves to the current node. If a node path is absolute, that is, it starts with a path separator, it resolves a node relative to the topmost parent.
    - `node.get_path()` returns the absolute node path.
  - _Node kinds:_ The `node.is::<NodeKind>` method can be used to test if a node is of a specific kind and `node.to::<NodeKind>` performs a conversion for accessing very specific properties. `node.to()` returns an `Arc<SpecificNodeKind>`. `node.try_into::<K>()` can be used to convert optionally. Every node kind implements `NodeKind`, and Rialight implements `NodeKind` as well for custom UI components (it includes a `reference_cast` method that results into an `Option<Arc<K>>`, which is used by the general `Node` type itself).
    - _Children:_ Any node can contain other child nodes. For instance, a button can contain further content inside, whether label, SVG or anything.
    - _Focus:_ Any node that supports focus can be focused by default. This can be disabled for a specific node through `set_focusable()`.
    - _Building nodes:_ `K::new()` constructs an empty node. Although property and children addition methods are chainable, you can use `markup!` to build nodes appropriately. How it looks like: https://users.rust-lang.org/t/generic-markup-macro/97830
      - Custom UI components can contain a `NodeOutlet`, which is a node that is replaced by input child nodes.
      - `<Svg src="path"/>` would work like `Svg::from_file`.
    - _Button:_ The `Button` node kind has variants, such as `primary()`, `secondary()` and `warning()`.
      - Highly-customized buttons are often created as user UI components.
    - _Bitmap:_ The `Bitmap` node kind identifies a pixel grid including transparency. It is optimized and uses different representations inside (like RGB, RGBA and maybe more targetting the GPU).
    - _Svg:_ The `Svg` node kind represents scalable vector graphics, specifically the SVG file format. It can be configured to use RGBA bitmap caching (`use_bitmap_cache`) at any size.
      - `Svg::from_file`
        - This method constructs a `Svg` directly from a file (as if by using `File::new(path).read_utf8_sync().unwrap()`). This is often used for loading `app:` SVG resources. It panics if loading fails for any reason.
      - Bitmap caching is clever and will generate a limited amount of bitmap caches.
        - The limit of caches could be something like 7.
        - If the size isn't near the size of any of the existing bitmap caches and the limit of caches is reached, no new bitmap cache is created and the nearest-size cache is used, yielding a blinear resized bitmap.
        - If the size is near to any of the existing bitmap caches, that cache is used, yielding a blinear resized bitmap.
        - If the size is not near to any of the existing bitmap caches and the limit of caches has not been reached yet, create a new bitmap cache by rendering the SVG again.
    - _NodeOutlet:_ The `NodeOutlet` node kind represents an empty node that meant to be replaced by other nodes. It is used, for instance, by the `markup!` macro for user UI components.
  - _Very specific properties:_ Very specific properties from node kinds are often manipulated after a `.to::<SpecificNodeKind>` conversion.
  - _Node representation:_ Internally, a node kind holds internal data that is stored behind a `Arc` inside `Node`. The `Node` type contains a single internal `Arc` that refers to further data, including common properties and a `Arc<dyn Any>` that refers to the node kind's data (which is downcasted to another `Arc` via `Arc::downcast`).
  - _Chaining:_ Most of the `Node` methods, such as `set_visibility`, are chainable, returning a clone of the node's reference. Node kinds also have chainable methods. These methods are defined similiarly to:
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
    - _Text selection:_ Optional text selection on non text inputs (text labels)
  - _Inheritance:_ Properties such as visibility, opacity, rotation and scale are inherited by default, with an _inherited_ variant. There may be more of such properties other than these that are inherited.
  - _Responsivity:_ Node measures are device-oriented. They use the mathematical API.
  - _Positioning:_ A node's position can be either derived, absolute or relative.
    - Derived means the node's position is determined by the parent.
    - Absolute means the node is positioned at a given global position.
    - Relative means the node is positioned relative to the parent's global position with given coordinates.
  - _Common properties:_ Skin, scale, opacity, visibility, position, rotation, size and maybe some more.
  - _Sizing:_ A node can have a size variant: none, full and specific (given measurement units). Nodes whose position is not _derived_ often need to specify a size variant, otherwise they may not be displayed.
  - _Not covered here yet:_ Alignment, minimum sizing and maybe more.
- Skins
  - Nodes share skins. Skins are inherited by default if a node's skin is `None`. Every application has a default skin that applies to the root node. Skins describe styles and style transitions.
```rust
// get_skin() and set_skin() work across all nodes,
// even if a specific node kind doesn't need a skin,
// such as `Column` and `Row`. the specified skin
// is **inherited** by children.
node.set_skin(Some(custom_skin));
```
  - Skins are divided by node kind. That is, a specific style applies to a specific node kind. Only native node kinds have applicable skins.
  - Skins are described in Rust code.
  - _RenderingTarget:_ The `RenderingTarget` can be constructed manually, however the application comes with its own.
    - The `RenderingTarget` can only render Rialight nodes, a RGB pixel rectangle and a RGBA (transparent) pixel rectangle. This includes 3D nodes. Separate methods are used: `render_2d`, `render_3d`, `render_rgb_grid`, `render_rgba_grid`.
    - Support rendering a 3D world from different viewpoints at the same rendering target at different rectangles inside a `RenderingTarget`, useful for multiple cameras.
      - I think this might involve generating triangles from the 3D nodes and subtracting parts of these triangles that overflow the viewpoint rectangle.
      - See if there is a way to implement an efficient method for that which doesn't need another `RenderingTarget` as a screenshot. (To start with, I don't even think another `RenderingTarget` is possible to construct as a native form of rendering is used.)
    - A `RenderingTarget` can be converted into pixels without the alpha channel, which can be useful for screenshots.
  - _Canvas:_
    - The `Canvas` node, although normally rendered automatically, can also be rendered separately to a RGBA (transparent) channel through a method. Useful for drawing tools.
  - _WebView:_
    - A simple `WebView` node should be supported. It's not meant as a building block for browsers.

Accessibility:

- Focus
  - Focus neighbors
    - Automatic focus neighbors on containers
    - The focus neighbors are set as node paths
  - Focus configuration
    - You can optionally allow user inputs other than touch or pointer to switch control focus.
- Touch

Common skin pratices:

- Static skins are conventionally created as lazily-evaluated statics:
```rust
use rialight::{prelude::*, graphics::*};

lazy_static! {
    static ref SOME_SKIN: Skin = {
        // initialize skin here.
    };
}

// use `SOME_SKIN` where desired!
```

#### Graphics Markup and Custom Nodes

Define two [procedural macros](https://doc.rust-lang.org/reference/procedural-macros.html) that facilitate defining nodes and custom UI components. For example, `define_node!` generates a separate `KKindData` structure, which is contained by `K` (node kind) itself. `K` contains (_base_, _data_). `K::new()` constructs an empty `K`. `K` inherits `NodeKind`, inheriting everything from _base_ (such as `set_skin` and `parent()`). _data_ is an `Arc<KKindData>`.

Syntax:

- `define_node!` is given field-like attributes somewhere, aggregating to a `struct` and aggregating `set_` prefixed methods to `impl K`.

It makes sense for UI components to be nodes, therefore `UiComponent` inherits `NodeKind`. They are defined with a similiar macro `define_ui_component!`.

Ideally there'll be three macros: `markup!`, `define_node!` and `define_ui_component!`.

#### How Nodes Are Implemented

- `Node` contains an `Arc<NonRefNode>`
- `NonRefNode` contains common fields and the stores the actual node kind data as `Arc<dyn Any>`.
- `K::new()` takes no arguments and returns _K_
- _K_ contains (_base_, _kind data_).
- _kind data_ is of type `Arc<KKindData>`
- The macros `define_node!` and `define_ui_component!` generate a `KKindData` structure
  - `KKindData` must have a `#[doc(hidden)]` attribute
- `NodeKind` will implement `Into<Node>`, evaluating to _base_ (the kind as the `Node` type).
- The `markup!` macro will build nodes using something like `let node: Node = K::new().into(); node`, chaining `set_` methods after `::new()`

### 3D Graphics

The 3D graphics API, `rialight::graphics_3d`.

- The most important type is `Node3d`. It is not compatible with the two-dimensional `Node` type and cannot be mixed with it.

### UI

The UI API, `rialight::ui`.

- The UI API exports all the node kinds related to user interface (such as `Button`) from the submodule `rialight::graphics::ui` of the graphics API. Such node kinds are not exported directly by the graphics API to avoid confusion. Even `define_ui_component!` comes from there.
- The UI API exports interfaces for reactive UI components which are defined by the developer.
  - An UI component may use graphics nodes from the graphics API, `rialight::graphics`. Inclusively, it is already a node too.
  - _Reactive_ data can be shared across all UI components. There may be a proper API for that. In that case, when a state changes, it causes parts of a component that use that state to render again.

UI components are graphical nodes, since `UiComponent` inherits `NodeKind`. They are usually defined with `define_ui_component!`, which is similiar to `define_node!`.

### File System

Ideas for the File System API, `rialight::filesystem`.

The `File` object can support the `file:`, `app:` and `app-storage:` URIs.

- `file:` refers to files in the user's device file system.
- `app:` refers to files in the application installation directory. They are assets originally included in the application source that are bundled within the application installer. These files are read-only and cannot be manipulated.
  - In the browser, these files are stored in the RAM.
- `app-storage:` refers to files in the application data storage directory. They are data stored dynamically in the application with persistence.

If you need to use `app-storage:` in the browser, never use synchronous operations (these with the `_sync` suffix) as they will currently panic since the browser has no support for synchronous operations.

#### Web Compatibility in the File System

Synchronous operations do not work for the `app-storage:` URI when exporting the project to the browser currently. Any synchronous operation on `File` will panic. If you need to target the browser, always use asynchronous operations.

For the browser, Rialight uses its origin-private file system API for the `app-storage:` URI.
  - https://users.rust-lang.org/t/bindings-for-browser-origin-private-fs/97417/2?u=hydroper1

For the browser, Rialight uses the RAM for the `app:` URI; that is, the files all load together with the runtime; therefore, both synchronous and asynchronous operations work.

### Gaming

Rialight supports a gaming API based on the Entity-Component-System pattern, which is essential for game developers, with support for physics. This API runs concurrent systems, however platforms without multi-threading support (browser) do not run systems concurrently. Since the nodes from the graphics API use atomic reference counting, they are used successfully the gaming systems.

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
- Big Integer
- Futures
- Flags
- Bytes for working with binaries
- Serialization
- Regular Expression
- `Observable`
  - Based on [this TC39 proposal](https://github.com/tc39/proposal-observable).
- Generic File Paths
- String Incognito
- URI and URI Component Encoding
- Timing API, including handy animation frame functions

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

### Concurrency

The concurrency API, `rialight::concurrent`.

- Workers
  - Workers behind an optional feature that is enabled by default.
  - A worker is created by evaluating a given [Rune script](https://rune-rs.github.io/book) (written in the Rune scripting language) as a string (usually given by an `include_str!` macro). Or it can be written as simply `worker!("./worker_script.rune")` which expands to `Worker::new(include_str!("./worker_script.rune"))`.
    - Rialight will choose one of these languages:
      - Try asking the community what would work well for a cross-platform worker.
      - [Rune language](https://rune-rs.github.io/book)
      - [Rhai language](https://rhai.rs/book)
  - Allows exchanging bytes and primitives such as strings between workers and sharing byte arrays.
  - It uses `SharedArrayBuffer` internally in the browser. The `SharedArrayBuffer` HTTP header should be set properly.
  - For the browser, here's research on how the JavaScript worker will load the worker scripting language and call developer functions:
    - https://rustwasm.github.io/wasm-bindgen/examples/wasm-in-web-worker.html
    - Pass all as much of the API as possible to the private JavaScript worker in WebAssembly

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
- Open link function (useful for authentication, so there might be variatns of that function that return a `Future`, allowing to receive data from the browser)
  - This should work at all platforms. In the browser, it simply uses the Web API for opening links.

The core internals, `rialight::core_internals`, should not be used anywhere. They are used by the APIs, including file system, for instance, to determine the application's installation directory.

### Prelude

The `rialight::prelude` crate can be used to include commonly used things in scope. It includes:

- Some of the Rust standard library, including:
  - `Any`
  - `Future`
  - `Map` and `Set` as aliases to `HashMap` an `HashSet`
  - Types for concurrency and reference counted boxes
- Map and Set Collections that use a hash algorithm (same from the standard library, `std::collections`)
  - Some more
- Collection Literals
- Regular expressions
- Bitwise Flags
- Lazily Evaluated Statics
- JSON Serialization
- Observables
- Temporal API (`temporal` as a global module)
- Futures
  - `exec_future`
  - Other methods, like `future_race`
- Big Integer

These other than the Rust standard library come from the utilities API.

### JavaScript

When a developer wants to run a portion of Rust code for the browser only, it is recommended to detect the browser via `#[cfg(feature = "rialight_browser_export")]` and not `#[cfg(target_arch = "wasm32")]` as WebAssembly is used for unknown platforms.

Rialight provides an alias API for communicating with the browser and JavaScript, `rialight::javascript`, which is only available for the browser export.

This crate provides aliases for these crates with basic documentation, so no worries:

- `js_sys`
- `web-sys`
- `wasm-bindgen`
- `wasm-bindgen-futures`

### Frame Control

For gaming, some might want control on how the game frames loop. In that case, either an animation interval or a default interval from the timing API is used inside the node renderer, according to developer configuration.

### Visual Editor

Once Rialight develops, it can have a visual editor for the following use-cases:

- Generic software
- Gaming

This visual editor will require an external IDE for logical programming, such as Visual Studio Code.

Features:

- Construct runtime styles and nodes from visually-edited styles and nodes. Visually-edited UI components? Might be possible.

## Additional Platform Detection

Internally, Rialight uses Cargo features, including `rialight_browser_export`, to detect certain platforms that are not operating systems, including browsers, and futurely gaming consoles, since most of these use a WebAssembly target such as `wasm32-unknown-unknown`, where the OS does not exist.

Current features used for platform detection:

- `rialight_default_export`
- `rialight_browser_export`

You should not worry about specifying these Cargo features as you'll be using the Rialight commands to build or export your application to a specific platform, such as `rialight export --platform browser`.

## Comparison to Other Technologies

- The concept of nodes is similiar to the concept of DOM elements: you cannot subtype a specific DOM element kind and instead use the existing ones. Although the framework strives to have as many node kinds as possible, you may need to wrap it into an unrelated type or create an UI component from the UI API (`rialight::ui`).

## Rust Setup

The framework currently requires the nightly Rust to be used. It is easy to switch to `nightly`:

```
rustup default nightly
```