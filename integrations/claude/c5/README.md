# c5 for Claude Code

c5 is a shared, persistent project memory for coding agents and the people running them. This plugin
connects Claude Code to it:

- **SessionStart hook**: prints the c5 brief for the current directory's project (last handoff, current
  focus, plan outline, open bugs/regressions/revisits, current decisions, recent activity) at startup, resume,
  `/clear` and after compaction, and registers a c5 session for this conversation.
- **UserPromptSubmit hook**: on each message, recalls earlier decisions that bear on it, and adds a short
  reminder when something new needs attention (a question the user asked the agents, due, overdue, ripe, waiting
  on the user), once per item per day.
- **SessionEnd hook**: closes that session.
- **`/c5:init` command**: sets up the current folder. It proposes a project name and key (or an existing project
  to link), runs `c5 project init` once you agree, then drafts the project's About from the repository for you to
  review. `/c5:init "Billing Service" BIL` skips the question.
- **c5 skill**: tells Claude when to record work with the `c5` CLI: focus, progress, bugs, revisits,
  decisions and handoffs.

Requires the `c5` binary on `PATH` (or `C5_BIN`). Install it with `cargo install --path crates/c5`, then run
`c5 install claude` to write this plugin and print the commands that add it.
