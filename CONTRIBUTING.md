# Contributing

```sh
cargo build
cargo test                          # unit, TUI and end-to-end CLI tests
cargo fmt && cargo clippy --all-targets
python3 scripts/bench.py            # latency benchmark (after cargo build --release)
```

Layout:

- `crates/c5-core`: model, vault, index, sync, resolution, brief, decisions, git history.
- `crates/c5`: the CLI, harness hooks and `c5 install`.
- `crates/c5-tui`: the terminal UI.
- `integrations/`: the Claude Code plugin and the `AGENTS.md` section (embedded in the binary).
- `docs/`: user guides, design and the project plan.

Keep the vault format backwards compatible: files written by an older c5 must still read. Tests never touch
`~/.c5`; they set `C5_VAULT` and `C5_HOME` to temporary directories.
