# Getting started: for humans

This guide installs c5, links your first project, connects your coding agents, and shows how to review what
they record. It takes about 10 minutes. The vocabulary is explained in [concepts.md](concepts.md).

## 1. Install

```sh
curl -fsSL https://raw.githubusercontent.com/anupkalburgi/c5/main/install.sh | sh
c5 --version
```

The script picks the right binary (macOS or Linux, arm64 or x86_64), checks it against the release's
`SHA256SUMS`, and installs it to `~/.local/bin` (`C5_INSTALL_DIR` to change, `C5_VERSION` to pin). Run it again
to upgrade.

If `~/.local/bin` is not on your `PATH`, the script adds one line to your shell's startup file (`~/.zshrc`;
`~/.bash_profile` or `~/.bashrc` for bash; `conf.d/c5.fish` for fish; `~/.profile` otherwise), marked
"Added by the c5 installer", and never twice. Open a new terminal, or run the `export PATH=…` line it prints, to
use `c5` right away. Set `C5_NO_MODIFY_PATH=1` to skip this and add it yourself.

From source, with Rust 1.89 or newer:

```sh
cargo install --git https://github.com/anupkalburgi/c5 c5
```

## 2. The vault and its history

Everything c5 records lives in `~/.c5`:

```
~/.c5/
  projects/ inbox/     your data: Markdown files, the source of truth
  config.toml          settings (optional)
  .state/              index, locks, logs; disposable and git-ignored
  integrations/        generated plugins (`c5 install`); git-ignored
```

`~/.c5` is a git repository. After every command that changes it, c5 commits in the background. Commits use
your git identity (or `c5 <c5@localhost>` if you have none) and run your global git hooks. Browse the history
with:

```sh
c5 vault git log --oneline -20        # or: git -C ~/.c5 log
c5 vault git show HEAD
```

To back it up or sync it between machines, add a remote and push: `c5 vault git remote add origin …`.

Settings in `~/.c5/config.toml`:

```toml
git = true            # automatic commits (default true); C5_GIT=0 turns them off for one command
git_delay_ms = 1500   # how long the background committer waits, so a burst of writes becomes one commit
```

To use a different vault for a while, for example for experiments, set `C5_VAULT=/some/dir`.

## 3. Link your first project

If you skip this and just start an agent in the folder, c5 still notices: the agent is told to ask you in its
first reply (with a suggested name and key), its notes wait in the inbox, and the folder appears under
"not set up" in `c5 tui`, `c5 project ls` and `c5 doctor` until you set it up. To do it yourself:

```sh
cd ~/code/billing-service
c5 context
```

In a directory that isn't linked yet, `c5 context` says so and lists existing projects that look similar.

If you use Claude Code, the quickest way is `/c5:init` (once the plugin from [step 5](#5-connect-your-agents) is
installed). It proposes a name and key from the README or manifest, offers to link a similar existing project
instead, and creates the project once you agree. Then it drafts the project's About from the repository and
records it after you review it. To skip the question, pass the name and key: `/c5:init "Billing Service" BIL`.

To create a project from a terminal:

```sh
c5 project init --name "Billing Service" --key BIL
```

To add another checkout, worktree or clone to an existing project:

```sh
cd ~/worktrees/billing-service/feature-x
c5 project link BIL
```

Worktrees and clones with the same git remote usually resolve without linking; check with `c5 whereami`. To pin a
directory to a project explicitly, write a `.c5` file in it that contains the key.

## 4. Seed the plan

Agents add items as they work, but it helps to give them the big picture first:

```sh
c5 add plan "Move invoicing to the new payments API" -p 1
c5 add phase "Invoices and refunds" --parent BIL-1
c5 add feature "Exact tax rounding" --parent BIL-2
c5 add revisit "Totals differ by one cent on multi-currency invoices" --parent BIL-3 -b - <<'EOF'
3 of 200 sample invoices differ from the old system by one cent.
Evidence: runs/2026-11-05-invoice-diff/
EOF
c5 decide "Drop the legacy XML export" --because "no customer has used it in a year"
c5 ls --tree
```

## 5. Connect your agents

**Claude Code** uses a plugin that shows the brief at session start and after every compaction, and teaches
Claude when to record things:

```sh
c5 install claude
claude plugin marketplace add ~/.c5/integrations/claude
claude plugin install c5@c5
```

Start a new Claude Code session in the project and the brief appears at the top. If the folder isn't linked to a
project yet, run `/c5:init` (see [step 3](#3-link-your-first-project)). Re-run `c5 install claude` after upgrading
c5, then `claude plugin marketplace update c5`.

The plugin gives Claude Code:

- the brief at session start, on resume, after `/clear` and after compaction
- earlier decisions recalled when your message touches their topic, and reminders when something needs attention
- `/c5:init`, which sets up the current folder
- a skill that tells Claude when to record plans, bugs, decisions, experiments and handoffs

**Codex and other agents** that read an `AGENTS.md` file:

```sh
c5 install agents-md                          # adds a section to ~/.codex/AGENTS.md
c5 install agents-md --file ./AGENTS.md       # or into one repository's AGENTS.md
c5 install agents-md --print                  # or just print the section
```

Running it again updates the section in place.

**Any other agent** that can run shell commands: tell it to run `c5 context` first and follow
[agents.md](agents.md).

## 6. Daily use

The quickest way to review a project is the terminal UI: `c5 tui` (see [tui.md](tui.md)). It shows the brief,
the plan tree, the decision log, the journal and triage, refreshes as agents work, and lets you record
decisions, notes and status changes. You can also open `~/.c5` in Obsidian or any editor.

From the shell:

| You want to… | Run |
|---|---|
| Browse everything interactively | `c5 tui` |
| See where a project stands | `c5 context` |
| See the open work as a tree | `c5 ls --tree` (add `--all` to include closed items) |
| See only bugs and revisits | `c5 ls -k bug -k revisit` |
| Look at one item and its history | `c5 show BIL-12` |
| See what happened recently | `c5 journal -n 30`, or `c5 journal --days 2 -A` across all projects |
| See what one agent session did | `c5 session ls`, then `c5 journal --of-session ABC123` |
| Find something | `c5 search "temp table decimal"` (`-A` searches all projects) |
| Record a decision | `c5 decide "…" --because "…" -r "alternative: why not" --area testing` |
| Rule something out | `c5 reject "…" --because "…"` |
| Change your mind | `c5 decide "…" -s BIL-17 --because "…"` |
| Set a rule for every project | `c5 decide "Always use uv" --global` |
| Review the decision log | `c5 decisions [--area A] [--md]`; `c5 check "topic"` |
| Park an idea | `c5 add idea "…"` |
| Describe the project | `c5 about`, `c5 about edit`, `c5 about add "…" --section stack` |
| Record a dataset, model, service … | `c5 add resource "…" --type dataset --at s3://… --owner …` |
| See everything on a topic | `c5 history "topic"` |
| See what happened when | `c5 timeline --days 30`, or tab 6 in `c5 tui` |
| Add a thought about existing work | `c5 add revisit "…"` |
| Tidy up unfiled notes | `c5 triage`, then `c5 attach ENTRY BIL-12` |
| Fix a typo or rewrite a description | `c5 edit BIL-12` (opens `$EDITOR`) |
| Check for problems | `c5 doctor` |

Every command accepts `--json`.

## 7. Editing the vault by hand

Item files are ordinary Markdown, for example `~/.c5/projects/billing-service/items/BIL-12-….md`. You can
change front-matter fields such as `status`, `priority`, `tags` and `parent`, or the body, in any editor. Things
to know:

- Leave `id` alone. Ids are permanent.
- Journal bodies can be edited too. Don't touch the `<!-- c5 {...} -->` line above each entry.
- c5 notices your change immediately when it involves the item you then look at (`c5 show`, `c5 update`), a new
  or renamed file, or a journal from the last 7 days. Other in-place edits show up within 5 minutes. Run
  `c5 reindex` to force it.
- Keys you add to front matter are kept when c5 rewrites the file.

## 8. Troubleshooting

| Symptom | Fix |
|---|---|
| Changes are not being committed | `c5 doctor` reports the last failed commit; details are in `~/.c5/.state/git.log`. `c5 vault commit` commits now and shows the error. |
| "this directory is not linked to a project" | `/c5:init` in Claude Code, or `c5 project link KEY` / `c5 project init` |
| `/c5:init` is not found in Claude Code | Re-run `c5 install claude`, then `claude plugin marketplace update c5`, and start a new session. |
| An agent's entries land in the inbox | The agent ran in an unlinked directory. Link it; the entries move over automatically. |
| The brief didn't appear in Claude Code | `claude plugin list` should list c5. Check `c5` is on `PATH`, or set `C5_BIN`. |
| Something looks stale | `c5 reindex` |
| You want to know what c5 sees | `c5 whereami` shows the project, how it was found, the git branch, the session and the vault |
| It feels slow | `C5_TRACE=1 c5 context` prints timings; see [../design/performance.md](../design/performance.md) |
