# exec-tree-walk

`hnl-exec-tree-walk` — the reference interpreter for the eval operation:
interprets the arrow-calculus `Term` kernel directly by tree-walking,
reading `lang.eval[node.kind]` and evaluating the resulting scheme with
the shared native-op table. It is the reference implementation the
other eval engines (`exec-bytecode`, `exec-cranelift`) are checked
against.

## Layout

| path | role |
| --- | --- |
| `src/lib.rs` | the tree-walk `EvalEngine` |
| `tests/` | engine tests |
| `xtask/` | vendored gate crate (`fmt`, `clippy`, `test`, `doc`, `gate`) |
| `docs/` | engine docs |
| `workflows/gate.yaml` | per-push gate — `handl-gates` rust campaign via bupha |

## Contract

- The oracle: other engines must match its values; do not "fix"
  divergences here without checking the contract tests.
- Deps resolve from git — a plain clone test-runs as-is.

## Verification

```bash
cargo run --manifest-path xtask/Cargo.toml -- gate   # fmt → clippy → test → doc
```

<!-- myco:agent-awareness -->

## Agent awareness

- Session start / any work discussion: run `myco status --prompt`
  before proposing next steps — know the board first.
- User describes new work → offer to capture it (`myco todo add`,
  `issue new`, `proposal new`). Don't wait to be asked.
- Picking up a task: `myco resume <ID>` is the briefing.
- Wrapping up: `myco handoff <ID>` anything resumable; keep statuses
  honest (claimed → in-progress → blocked/done).

<!-- /myco:agent-awareness -->
