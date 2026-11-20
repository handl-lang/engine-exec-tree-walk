# engine-exec-tree-walk

`hnl-engine-exec-tree-walk` — the reference interpreter for the eval operation.

An `engine-*` crate: interprets the arrow-calculus `Term` kernel directly by
tree-walking, reading `lang.eval[node.kind]` and evaluating the resulting
scheme with the shared native-op table. It is the reference implementation
the other eval engines are checked against.

## Development

```bash
cargo xtask gate        # fmt --check → clippy -D warnings → test → doc
```

`cargo xtask` wraps the repo's `xtask/` crate (`fmt`, `clippy`, `test`, `doc`,
`check`, `gate`); `ci.sh` is the gate entry. Deps resolve from git, so no
sibling checkouts are needed; a plain clone test-runs as-is.

## License

MIT.

## Documentation

<https://handl-lang.github.io/docs/engine-exec-tree-walk/>
