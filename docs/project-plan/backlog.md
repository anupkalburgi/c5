# Backlog

Known limitations and small improvements not yet scheduled in the [roadmap](roadmap.md).

## Known limitations

| Limitation | Impact | Likely fix |
|---|---|---|
| An in-place edit of an older item file can take up to 5 minutes to appear in lists and the brief. The item being viewed and journals from the last 7 days are always current. | Low | File watcher (M2) |
| Every command pays process start (about 10 ms). | Low for agents, noticeable for UIs | Server (M2) |
| Two non-Claude agents in the same directory without `C5_SESSION` share a session. | Attribution blurs | Export `C5_SESSION` from `c5 session start`; MCP sessions (M3) |
| Agents other than Claude Code show as `unknown` unless they pass `--agent`. | Cosmetic | Detect common harness environment variables |
| No command to delete an item or move it between projects. | Edit files by hand, then `c5 reindex` | `c5 rm`, `c5 move` (M3) |
| An item's file name keeps its original title after a rename. | Cosmetic | `c5 update --rename-file` |
| `--supersedes` accepts any item, not only decisions. | Possibly confusing | Warn when the target is not a decision |
| Decision recall is lexical: different words for the same topic don't match unless an area or scope connects them. | Missed recall | Embeddings (M4); use areas consistently |
| Detecting decision language uses an English phrase list. | Missed reminders | Tune from use; decision mining (M3) |
| The per-message hook exists for Claude Code only. | Codex relies on `c5 check` | Codex hooks when available; MCP (M3) |
| The TUI polls every 2 seconds. | Fine at 2,000 items and 50k entries | Server events (M2) |

## Improvements

- `c5 context --since-last-session`: what changed since this agent's previous session.
- `c5 ls --agent NAME`: items created by one agent.
- `c5 export` of a project's state as Markdown or JSON (for example a weekly summary).
- Shell completions.
- `c5 add` caches the next item number instead of listing the items folder.
