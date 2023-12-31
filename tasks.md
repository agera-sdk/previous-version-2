# Tasks

Working at file system:

- Design an API that works across all platforms, including Android.
  - [ ] Provide ways of requesting permissions using asynchronous results that works across all platforms
  - [ ] All `File` operations are asynchronous, except these with the `_sync` suffix.
  - [ ] File stores another inner, which is platform-specific. For instance, it can hold a handle obtained from a file picker from the browser.
  - For synchronous operations:
    - Panic for the browser
    - For non-browser targets, use `std::fs`
  - For asynchronous operations:
    - Use Tokio runtime for non-browser targets
    - Use JavaScript promises for browser targets
  - Always use `rialight_util::file_paths::os_based` instead of `rialight_util::file_paths` internally in path manipulations, passing the Windows variant where appropriate.
  - [ ] Android
    - [ ] On Android, `app:` and `app-storage:` do not use a static from the Rialight core internals; call the Java API function [`context.getFilesDir`](https://developer.android.com/reference/android/content/Context#getFilesDir()).
  - Consult the _Additional Platform Detection_ section for how WebAssembly-based platforms are detected, including the browser.
  - [ ] Document the API

When futurely working on graphical nodes:

- See [How Nodes Are Implemented](./README.md#how-nodes-are-implemented)
- Derive `Hash` for `Node` (just uses the `Arc` inside)
- Provide the types `Node` and `WeakRefNode`. Inside `Node` is stored an internal `Arc<NonRefNode>` and inside `WeakRefNode` is an internal `Weak<Gc<NonRefNode>>` to which it dereferences.
- The parent is stored as a `WeakRefNode` internally.
- The equality operator compares by reference (`Arc::ptr_eq`) and the clone method clones the reference (`Arc::clone`). _Do not_ use `#[derive(Clone)]`; implement it manually to make sure no content is cloned:
```rust
impl Clone for Node {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.n))
    }
}
```
- `node.clone_by_content()` clones the node by content and not reference, including the children. This is equivalent to the next fine-tuned method.
  - This method will clone all events and attributes
  - Custom UI components may not support this method, panicking instead.
- `node.clone_by_content_fine_tuned(flags)` is similiar to the previous method, but reserves for future flags.
- Rendering
  - `lyon` might be used for rendering vector graphics for the GPU. How to anti-alias though, it'll have to be learnt.

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
- Pass `--no-default-features` to CLI when not targetting the browser and `--features rialight_browser_export`.

## When futurely working in the macros `rialight::main` and `rialight::build_script`

The base `rialight::main` method is already implemented at `rialight::util::runtime::main`.

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