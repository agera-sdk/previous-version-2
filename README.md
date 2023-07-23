# Rialight

> **Note:** Rialight is not yet done.

Rialight aims to be a multi-purpose gaming and graphical application framework combining reactivity and nodes and shipping various fundamental APIs, requiring you to just know the Rust standard library and the Rialight API.

Rialight can be used for creating graphical applications, but **cannot be** used for creating websites. Rialight applications can be embedded in websites.

Rialight takes inspiration from:

- Adobe AIR (or Flash Player)
- Godot Engine
- Reactive UI Frameworks such as React

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

In regards to the graphics API, it'd be interesting to combine reactivity and node trees:

- Nodes, the primary way to construct a visual application.
  - The `Node` object is the primary item of the graphics API, which has a limited set of various variants, such as `Rectangle`, `Button`, `TabBar` and `Modal`. All of them share full customisation and common properties like visibility and transform (including 3D matrix) which are inherited by default from their parent.
    - Children manipulation
  - Nodes don't describe just graphics. They also emit events, such as `on_enter_frame` and `on_click` events.
    - Somes nodes may not have a certain event, which is a rare case, panicking when retrieving it. In that case, for an event that is not supported by all node kinds, the documentation can list the only supported node kinds.
- Skins
  - Nodes share skins. Skins are inherited by default. Skins describe styles, style transitions and some behaviors.
- Reactive Components
  - Similiar to either Angular or React. They can make use of graphical nodes, similiar as to how components from reactive web frameworks use DOM elements.

Accessibility:

- Focus
  - Focus neighbors
    - Automatic focus neighbors on containers
  - Focus configuration
    - You can optionally allow user inputs other than touch or pointer to switch control focus.
- Touch

### File System

The `File` object can support the `file:`, `app:` and `app-storage:` URIs.

- `file:` refers to files in the user's device file system.
- `app:` refers to files in the application installation directory. They are assets originally included in the application source that are bundled within the application installer. These files are read-only and cannot be manipulated.
  - In the browser, these files are stored in the RAM.
- `app-storage:` refers to files in the application data storage directory. They are data stored dynamically in the application with persistence.
  - In the browser, it uses the origin-private file system API.

### Gaming

Rialight supports a gaming API based on the Entity-Component-System pattern, which is essential for game developers, with support for physics.

The Gaming API is an optional feature that can be turned on or off.