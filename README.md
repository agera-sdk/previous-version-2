# Rialight

This framework has not began yet. One time it may start. I'm keeping the `src` directory temporarily as it contains previous code for internationalization using Fluent and file system APIs.

This framework will only require you to know the `std` API other than its own API, allowing you to create graphical applications for any purpose with no boilerplate configuration required, eliminating use of additional crates and using features that can be turned on and off. This project is not active in the moment and I will refactor everything once I get back to it.

Rialight can be similiar to Adobe AIR, Godot Engine and React.

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