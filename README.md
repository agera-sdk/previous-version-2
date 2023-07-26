# Rialight

> **Note:** Rialight is not yet done.

Rialight aims to be a multi-purpose gaming and graphical application framework combining reactivity and nodes and shipping various fundamental APIs, requiring you to just know the Rust standard library and the Rialight API.

Rialight can be used for creating graphical applications, but **cannot be** used for creating websites. Rialight applications can be embedded in websites.

Rialight takes inspiration from:

- Godot Engine
- Reactive UI Frameworks such as React
- Adobe AIR (or Flash Player)

## Draft Ideas

### Project Template

When using the `rialight` command, you create new projects instantly:

- `rialight new my-app`
- `rialight new --game my-game`

The project templates share common functionality, including translation resources which use the [Fluent syntax](https://projectfluent.org).

### Debugging and Exporting

Exporting a project should bundle its assets files into the installer, which can be later retrieved through the File API using an `app:` URI.

Rialight uses Cargo, meaning `cargo run` works for debugging. You can also use `rialight run` or its alias `rialight debug`.

### Graphics

The `rialight::graphics` and `rialight::ui` APIs co-work together.

In regards to the graphics API, it'd be interesting to combine reactivity and node trees:

- Nodes, the primary way to construct a visual application.
  - The `Node` object is the primary item of the graphics API, which has a limited set of various variants, such as `Rectangle`, `Canvas`, `Button`, `TabBar` and `Modal`. All of them share full customisation and common properties like visibility and transform (including 3D matrix) which are inherited by default from their parent.
    - Children manipulation
    - Meta data (optional mapping from `String` to `Box<dyn Any>` for attaching any data)
  - Nodes don't describe just graphics. They also emit events, accessed as `node.on_some_event().listen(listen_fn)`, such as `on_enter_frame` and `on_click` events.
    - Somes nodes may not have a certain event, which is a rare case, panicking when retrieving it. In that case, for an event that is not supported by all node kinds, the documentation can list the only supported node kinds.
  - Few events are not accessed as listeners, using a single callback instead:
    - `node.on_enter_frame(enter_frame_fn)` sets listener for the enter frame event
    - `node.on_user_input(user_input_fn)` sets listener for an user input event
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

### UI

The UI API, `rialight::ui`.

- Some parts of the UI API are used by the graphics API.
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

Rialight supports a gaming API based on the Entity-Component-System pattern, which is essential for game developers, with support for physics.

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

The network API, `rialight::net`.

- HTTP
- TCP
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

### Temporal

The temporal API, `rialight::temporal`.

- Based on [this TC39 Temporal API](https://github.com/tc39/proposal-temporal).

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

- Map and Set Collections that use a hash algorithm (same from the standard library, `std::collections`)
- Collection Literals
- Regular expressions
- Bitwise Flags
- Lazily Evaluated Statics
- JSON Serialization

All of these are described in the utilities API.

## Comparison to Other Technologies

- The concept of nodes is similiar to the concept of DOM elements: you cannot subtype a specific DOM element kind and instead use the existing ones. Although the framework strives to have as many node kinds as possible, you may need to wrap it into an unrelated type or create an UI component from the UI API (`rialight::ui`).

## Rust Setup

The framework currently requires the nightly Rust to be used. It is easy to switch to `nightly`:

```
rustup default nightly
```

## Tasks

Working at the utilities API. All done (using third-party crates under the hood), except:

- [ ] Observable
- [ ] Generic File Path functions