---
name: c5
description: Shared, persistent project memory through the `c5` CLI: the project's About, decision log, experiments, resources (datasets, models, services …) and history. Use throughout any coding session where a c5 brief appeared at session start, whenever the user makes or rejects a choice (architecture, testing, tooling, product, process), before proposing an approach or asking the user to choose between options, and whenever the work involves the plan, phases, features, tasks, bugs, regressions, things to revisit, ideas, open questions, progress notes, or handing off to a later session. Record these with c5 instead of only in chat; run `c5 context` when resuming work.
---

# c5: project memory that outlives the conversation

c5 keeps a project's plan, open items, **decision log** and progress journal outside the chat, so the next
session (yours after compaction, another agent's, or the human's review) starts from facts instead of a
reconstruction. Everything is plain Markdown in `~/.c5`, committed to git automatically; `c5` is the only
interface you use.

## At the start of a session

- The brief usually arrives automatically (SessionStart hook). If it did not, run `c5 context`.
- Continue from **Last handoff** and **Current focus**. Mention the item ids you are picking up.
- **Asked by the user** (in the brief, or a "c5 reminder" mid-session): questions the user asked the agents about
  an item, from the web UI or `c5 ask`. Answer each from evidence (`c5 history`, the repo, git log, reports), citing
  where it comes from: `c5 answer ID "…"`. If you cannot tell, answer that and say what would settle it. Tell the
  user what you answered, then carry on with their request.
- **Needs attention** lists what is overdue, due, ripe (what it waited on is done), waiting on the user, stale or
  going in circles. Tell the user about the ones that concern them in your first reply. Mid-session a hook may add
  "c5 reminder" lines when something new comes up: mention it briefly, then carry on with their request.
- Respect **Decisions that apply to your focus**, **Rules for every project** and **Ruled out**. Do not re-open
  them unless the user does or you have new information; if you do, say which decision you are questioning.
- If the brief says the folder is **not linked to a c5 project**, say so to the user in your **first reply**,
  before other work: offer the suggested `c5 project init --name "…" --key …` from the brief (they may pick another
  name or key), or `c5 project link KEY` for a similar existing project it lists (they can also run `/c5:init`,
  which does the same and drafts the About). Run it once they agree. Do not
  create or link a project without asking, and do not stay silent about it.

## Project context: About and resources

- If the brief says **About (not written yet)**, draft it from the repository (README, AGENTS.md, manifests such
  as Cargo.toml, package.json or pyproject.toml), show the user, and record it once they agree:
  `c5 about set what "…"`, then `c5 about add "…" --section stack|run|architecture|data|terms|constraints|links`.
- When you learn a durable fact about the project (how to run something, a constraint, a term), add it:
  `c5 about add "integration tests need Docker running" --section run`.
- When you work with a **resource**, record it once: datasets, tables, pipelines, dashboards, notebooks, models,
  endpoints, prompts, evals, indexes, features, services, APIs, databases, queues, buckets, environments.
  `c5 add resource "Invoices table" --type table --at analytics.billing.invoices --owner data-eng -b "one row per
  invoice; refreshed hourly; amounts in cents"`. Check `c5 resources` first.
- **Never record credentials.** Record where they are kept ("in 1Password: billing/prod-db", "env var
  DB_PASSWORD"). c5 refuses text that looks like a secret.

## History: consult it before planning

Before planning any non-trivial approach, and whenever the user asks "should we…", "why don't we…", "what about…":

```sh
c5 history "webhook batching"      # everything on the topic, oldest first
c5 timeline --days 14              # what happened recently, and in what order
c5 insights                        # are we going in circles? flip-flops, repeats of dead ends, spinning branches
```

Build the plan from what comes back, and cite ids:
- follow accepted decisions; don't re-propose rejected options without new information;
- don't repeat a **failed experiment** unless what made it fail has changed (its `retry when` condition, or a
  passed retry date flagged as "worth retrying"); if it has, say so and propose the retry;
- look for **ideas** that can be combined with what is known now;
- mention dead ends the user may have forgotten.
- if **Needs attention** lists a `loop` (or `c5 insights` shows signals), tell the user you may be going in
  circles, name the ids, and suggest a different approach or a decision before going another round.

## Experiments

When trying something whose outcome is uncertain (a performance change, a model, a prompt, a library, a schema):

```sh
c5 add experiment "Batch webhooks in groups of 50" --hypothesis "fewer 429 errors" --focus
c5 log -k finding "provider rejects batches above 10" -r runs/2026-11-18-batch/
c5 done BIL-12 -n "429s down 80%"                                       # it worked
c5 status BIL-12 failed -n "batch cap is 10" --retry-when "provider raises the cap" --retry-after 90d
c5 status BIL-12 inconclusive -n "sandbox too small to tell" --retry-when "staging data available"
```

## Decisions: the most important part

Choices made in chat are lost unless recorded, and then get re-litigated in the next session.

**Before** you propose a design, pick a library or tool, or ask the user to choose between options:

```sh
c5 check "search engine tantivy"     # decided? rejected? parked?
```

If something comes back, follow it and cite its id ("per BIL-12 we use FTS5"). Messages from the user may
also arrive with "c5: earlier decisions that may bear on this message" added by a hook: treat those the same way.

**When** the user decides something, or says no to something, record it **immediately**, then mention the id:

| The user… | Command |
|---|---|
| Chooses an option | `c5 decide "Use FTS5 for search" --because "no extra process" -r "tantivy: separate index, +4 MB" --area search` |
| Rules an option out | `c5 reject "Use a daemon for all writes" --because "CLI must work with nothing running" --area architecture` |
| Changes an earlier decision | `c5 decide "Use tantivy after all" -s BIL-12 --because "need fuzzy search"` |
| Decides for one part of the plan | add `--for BIL-3` (applies to BIL-3 and everything under it) |
| States a rule for all projects ("always use uv", "never deploy without asking") | add `--global` |
| Floats an idea to look at later | `c5 add idea "Trigram tokenizer for identifiers" --area search` |
| Has not decided yet, but you recommend something | `c5 decide "…" --proposed --because "…"` |

Signals: "let's go with", "use X instead", "no", "don't", "never", "always", "from now on", "rather than",
picking one of the options you offered. Areas are short topics: `architecture`, `testing`, `tooling`,
`storage`, `api`, `ui`, `product`, `process`, or whatever the project uses consistently.

## While working

Write a few entries per hour of work, not one per tool call. One or two sentences each, specific.

| When | Command |
|---|---|
| Starting a unit of work | `c5 focus BIL-12 --goal "why: what it serves"` (prints what applies), or `c5 add task "…" --parent BIL-3 --goal "…" --focus` |
| A meaningful step, result or finding | `c5 log "…"` (attaches to the focus; `-r path` for files, commits, run folders) |
| You find a bug | `c5 add bug "…" [-l regression-of:BIL-9] [-p 1]` |
| Something worked before and broke | `c5 add regression "…" -l regression-of:BIL-9` |
| "We should come back to this" / a loose end | `c5 add revisit "…"` |
| A question only the human can answer | `c5 add question "Short question?" -b -` with the options, their trade-offs and your recommendation in the body (not in the title), plus where the evidence is (`-r` or a path) |
| Blocked, or to do after something else | `c5 status BIL-12 blocked -n "waiting on …"`; `--after BIL-9` makes it come back as ripe when BIL-9 closes |
| The user gives a deadline or says "remind me" | `--due fri` (or `2026-12-01`, `3d`); `--after ID` for "once X is done". Never invent a due date |
| Still needed, but not touched for a while | `c5 still BIL-12 -n "waiting on …"`; no longer needed: `c5 status BIL-12 dropped -n "why"` |
| Finished | `c5 done BIL-12 -n "what came out of it: result, commits, what it led to"` |

Structure big work as a tree with `--parent`: plan → phase → feature → task. Give features and tasks an
`--area` so decisions in that area are recalled when they are focused.

## Before you stop

1. Sweep: were any choices made or options rejected in this session that are not recorded yet? Record them.
2. `c5 handoff "where I stopped; what is next (ids); open risks"`.

## Rules

- Look before adding: `c5 ls --tree`, `c5 search "…"`, `c5 check "…"`. Reuse existing ids.
- Quote ids (BIL-12) in your replies so the human can look them up.
- Reference large outputs by path (`-r local/runs/…`); never paste them into entries.
- For long text pass `-` and pipe it in: `c5 log - <<'EOF' … EOF`, `c5 decide "…" -b - <<'EOF' … EOF`.
- Do not edit vault files directly; use `c5` (the human may edit them by hand; c5 picks that up).
- If a `c5` command fails, say so briefly and carry on with the user's task. c5 must never block the work.

## Reading

- `c5 pending` — everything that needs attention (`--json` for you); `c5 context` — the brief; `c5 ls --tree` — open items as a tree (`--all` includes closed)
- `c5 decisions [--area A] [--global] [--md]` — the decision log; `c5 check "…"` — decided before?
- `c5 history "…"` — everything on a topic; `c5 timeline` — what happened when; `c5 insights` — signs of going in circles; `c5 about`; `c5 resources`
- `c5 show BIL-12` — an item with its children, links and journal
- `c5 journal -n 20` — recent entries; `c5 search "…"` — full-text search (`-A` for all projects)
- `c5 triage` — entries not filed under an item; `c5 attach ENTRY BIL-12` files one
- Add `--json` to any command for machine-readable output.
