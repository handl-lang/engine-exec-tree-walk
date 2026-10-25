# Explanation

## Why this crate exists

A reference interpreter without compilation keeps the semantics legible: if a
language definition and this engine disagree, the definition is wrong. The
bytecode engine exists for speed and must match this one exactly — that pairing
is what lets the platform optimise without redefining the language.

## Where it sits

This crate belongs to the `engine-*` family. Plugins, kits and hosts reach it
through the kit's operation slot, so the engine can be swapped without touching a
language definition.

See the [reference](/engine-exec-tree-walk/reference/) for the API and the
[how-to guides](/engine-exec-tree-walk/how-to/) for tasks.
