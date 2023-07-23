# Rialight

> **Note:** Rialight is not yet done.

Rialight aims to be a multi-purpose graphical application framework combining reactivity and nodes and shipping various fundamental APIs, requiring you to just know the Rust standard library and the Rialight API.

Rialight can be used for creating applications, but **cannot be** used for creating websites. Rialight applications can be embedded in websites.

## Draft Ideas

### Application Template

The application templates, that can be created via the Rialight CLI, will share common functionality, including translation resources (using the Fluent syntax).

Building or publishing an application should bundle its assets files for the `app:` URI into the installer.

### Graphics

In regards to the graphics API, it'd be interesting to combine reactivity and node trees:

- Reactive Components
  - Similiar to either Angular or React. They can use graphical nodes.
- Graphical Nodes, the primary way to construct a visual application.
  - Consists of a `Node` object, which has a limited set of variants, such as `Rectangle`, `Button` and more with full customisation and properties like visibility and transform (including 3D matrix).
- Skins
  - Nodes share skins. Skins are inherited by default. They are similiar to CSS, but faster.

### File System

The `File` object can support the `file:`, `app:` and `app-storage:` URIs.

- `file:` refers to files in the user's device file system.
- `app:` refers to files in the application installation directory. They are assets originally included in the application source that are bundled within the application installer. These files are read-only and cannot be manipulated.
- `app-storage:` refers to files in the application data storage directory. They are data stored dynamically in the application with persistence.