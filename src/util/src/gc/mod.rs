/*!
Work with garbage collection.

This module provides interfaces for creating shared types that are
disposed once no reference exists to that type in the program.
All garbage collected types are thread-safe, therefore they must
implement `Send + Sync`.

Garbage collected types can bes useful when you have a
hiearchy of nodes in a document object model.

The most important type for garbage collection is [`Gc`].
After `Gc` is the [`GcTrait`] trait.
*/

// not implemented nor designed