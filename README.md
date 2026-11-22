# c5

**Project memory for coding agents.** c5 keeps the plan, open work, decisions and progress of a long-running
project outside the chat, so every new session (after compaction, another agent, or you the next morning) starts
from facts instead of a reconstruction.

- **Works with any agent.** It's a CLI, so anything that can run a shell command can use it. A Claude Code plugin
  adds a brief at session start and recalls past decisions as you chat; Codex and others get an `AGENTS.md`
  section.
- **A decision log that comes back when it matters.** Choices, rejected options and changes of mind are recorded
  with their reasons, and resurface in later sessions when their topic does.
- **Context and history.** Each project keeps an About (what it is, stack, how to run it), its resources (datasets,
  tables, models, endpoints, services), and experiments with their outcomes and when they're worth retrying.
  `c5 history "topic"` tells the whole story; agents consult it before planning.
- **Plain Markdown you own.** Everything lives in `~/.c5` as Markdown with YAML front matter, committed to git
  automatically. Readable in any editor or Obsidian.
- **Fast and small.** One Rust binary, no daemon. Most commands take a few milliseconds and peak memory stays
  under 30 MB on a project with 50,000 journal entries.

## Install

```sh
curl -fsSL https://raw.githubusercontent.com/anupkalburgi/c5/main/install.sh | sh
```

This downloads the binary for your machine (macOS or Linux, arm64 or x86_64) from the
[latest release](https://github.com/anupkalburgi/c5/releases/latest), verifies its checksum, installs it to
`~/.local/bin`, and adds that folder to your `PATH` in your shell's startup file if it isn't there yet. Open a new
terminal afterwards. Options: `C5_INSTALL_DIR` to install elsewhere, `C5_VERSION=v0.2.0` to pin a version,
`C5_NO_MODIFY_PATH=1` to leave your shell files alone. To build from
source instead: `cargo install --git https://github.com/anupkalburgi/c5 c5` (Rust 1.89+).

Connect your agents:

```sh
c5 install claude                                  # writes the Claude Code plugin, then add it:
claude plugin marketplace add ~/.c5/integrations/claude
claude plugin install c5@c5
c5 install agents-md                               # adds the c5 section to ~/.codex/AGENTS.md (or --file PATH)
```

## Use

Set up each project once. In Claude Code, run `/c5:init` in the project folder: it proposes a name and key (or
an existing project to link), creates the project once you agree, and drafts its About from the repository for
you to review. From a terminal:

```sh
cd ~/code/my-project
c5 project init --name "My Project" --key MP   # once; worktrees and clones are found by git remote
```

Then look at what's recorded:

```sh
c5 web                                         # in your browser: overview, map, full plan, every item's story
c5 tui                                         # the same in the terminal
```

Agents do most of the writing. You can too:

```sh
c5 add plan "Ship v2"                                    # MP-1
c5 add feature "Search" --parent MP-1                    # MP-2
c5 add task "Index journal entries" --parent MP-2 --focus
c5 log "50k entries index in 4 s" -r bench/results.txt
c5 decide "Use SQLite FTS5 for search" --because "no extra process" -r "tantivy: separate index to sync"
c5 reject "Run a daemon for all writes" --because "the CLI must work with nothing running"
c5 about add "SQLite + Rust; tests: cargo test" --section stack
c5 add resource "Events table" --type table --at analytics.events --owner data-eng
c5 add experiment "Trigram tokenizer" --hypothesis "better matches on code identifiers"
c5 history "tokenizer"                         # everything on a topic, oldest first
c5 handoff "Indexing done; next: ranking"
c5 context        # the brief an agent reads at session start
```

## Docs

- [Concepts](docs/getting-started/concepts.md) · [For humans](docs/getting-started/humans.md) ·
  [For agents](docs/getting-started/agents.md) · [Decision log](docs/getting-started/decisions.md) ·
  [Context and history](docs/getting-started/context-and-history.md) · [TUI](docs/getting-started/tui.md) ·
  [CLI reference](docs/getting-started/cli-reference.md)
- [Architecture](docs/design/architecture.md) · [Design rationale](docs/design/design.md) ·
  [Performance](docs/design/performance.md)
- [Status](docs/project-plan/status.md) · [Roadmap](docs/project-plan/roadmap.md) ·
  [Backlog](docs/project-plan/backlog.md) · [Changelog](CHANGELOG.md)

## Status

Early but complete enough for daily use. Developed on macOS; Linux is built and tested in CI. See the
[roadmap](docs/project-plan/roadmap.md) for what's next: a local server for instant UI updates, an MCP server,
and importing existing plan documents.

## License

[MIT](LICENSE)
