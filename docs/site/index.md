# engine-exec-tree-walk

The reference interpreter for the eval operation.

## What it is

The **reference** eval engine. It interprets the arrow-calculus `Term` kernel
directly: look up `lang.eval[node.kind]` → `EvalSchemeDef` → `Term`, then
interpret the scheme. Entirely data-driven, and the oracle the bytecode engine is
conformance-tested against (ADR-0044 §3, §4; ADR-0052).

## Start here

- **Reference** — [public API](/engine-exec-tree-walk/reference/)
- **How-to** — [task recipes](/engine-exec-tree-walk/how-to/)
- **Explanation** — [design and rationale](/engine-exec-tree-walk/explanation/)

No tutorials: this repo is a building block used from Rust. End-to-end
walkthroughs live with the product that composes it — see the
[Handl platform](/handl/).
