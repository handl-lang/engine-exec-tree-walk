# engine-exec-tree-walk

`hnl-engine-exec-tree-walk` — The **reference interpreter** for the eval operation (ADR-0044 §3, §4, ADR-0052). Interprets the arrow-calculus `Term` kernel directly by tree-walking: it reads `lang.eval[node.kind]` and evaluates the resulting scheme with the shared native-op table.

## Development

```bash
cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test
```

`ci.sh` is the gate (fmt → clippy → test → doc). Deps resolve from git, so no
sibling checkouts are needed; a plain clone test-runs as-is.

## License

MIT.

## Documentation

Published site (Diátaxis): <https://handl-lang.github.io/docs/engine-exec-tree-walk/>

- [Reference](https://handl-lang.github.io/docs/engine-exec-tree-walk/reference/) — public API surface
- [How-to](https://handl-lang.github.io/docs/engine-exec-tree-walk/how-to/) — task recipes
- [Explanation](https://handl-lang.github.io/docs/engine-exec-tree-walk/explanation/) — design and rationale

Decision records live on the `project` branch under `project/adrs/`.
