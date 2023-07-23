# Rialight

> **Note:** Rialight is not yet done.

Rialight aims to be a multi-purpose graphical application framework combining reactivity and nodes and shipping various fundamental APIs, requiring you to just know the Rust standard library and the Rialight API.

Rialight can be used for creating graphical applications, but **cannot be** used for creating websites. Rialight applications can be embedded in websites.

Rialight also supports a gaming API.

## Draft Ideas

### Application Template

The application templates, that can be created via the Rialight CLI, will share common functionality, including translation resources (using the Fluent syntax).

Building or publishing an application should bundle its assets files for the `app:` URI into the installer.

### Graphics

In regards to the graphics API, it'd be interesting to combine reactivity and node trees:

- Reactive Components
  - Similiar to either Angular or React. They can use graphical nodes.
- Nodes, the primary way to construct a visual application.
  - The `Node` object is the primary item of the graphics API, which has a limited set of variants, such as `Rectangle` and `Button`. All of them share full customisation and common properties like visibility and transform (including 3D matrix) which are inherited by default from their parent.
- Skins
  - Nodes share skins. Skins are inherited by default. They are similiar to CSS, but faster.

### File System

The `File` object can support the `file:`, `app:` and `app-storage:` URIs.

- `file:` refers to files in the user's device file system.
- `app:` refers to files in the application installation directory. They are assets originally included in the application source that are bundled within the application installer. These files are read-only and cannot be manipulated.
- `app-storage:` refers to files in the application data storage directory. They are data stored dynamically in the application with persistence.

### Gaming

Rialight supports an API based on the Entity-Component-System pattern, which is essential for game developers, with support for physics.

The Gaming API is an optional feature that can be turned on or off.