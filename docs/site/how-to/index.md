# How-to guides

Task recipes. Each assumes sibling checkouts of this repo and of the platform
repos it patches.

## Trace an evaluation

Dispatch is `kind → scheme → interpret`, so log the scheme term for the node kind before interpreting it — the term names the exact rule applied.

## Keep backend parity

Change evaluation semantics here first and let the bytecode conformance tests follow: this engine is the reference.
