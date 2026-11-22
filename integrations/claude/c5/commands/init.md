---
description: Set up c5 for this folder. Create a project (or link an existing one) and draft its About.
argument-hint: "[name] [KEY]"
allowed-tools: Bash(c5:*), Read, Glob, Grep
---

Set up c5 project memory for the current folder. Arguments (optional): $ARGUMENTS

## 1. Where things stand

Run `c5 whereami` and `c5 project ls`.

- If this folder already resolves to a project, say which one (name and key). Then run `c5 about`. If the About
  is written, stop there and say it's ready. If it isn't, go to step 3.
- If `c5` is not found, tell the user to install the c5 binary first and stop.

## 2. Create or link the project

Work out a proposal:

- **Name**: the project's real name, from the README title or the manifest (Cargo.toml, package.json,
  pyproject.toml, go.mod), not just the folder name. If the arguments give a name, use it.
- **Key**: 2 to 10 capital letters or digits, short and easy to recognize, such as `BIL` for "Billing Service".
  It prefixes every item id. If the arguments give a key, use it.
- **Existing projects**: if one in `c5 project ls` looks like the same project (similar name, or the same repo
  under another remote), offer to link this folder to it instead.

If the arguments gave both a name and a key, take that as the user's go-ahead and run the command. Otherwise
show your proposal in one or two lines and ask the user to confirm or change it. Then run one of:

- `c5 project init --name "NAME" --key KEY -d "one-line description"`
- `c5 project link KEY`

Worktrees and clones of the same repository find the project through its git remote, so this is only needed once
per project.

## 3. Draft the About

Read the repository: the README, AGENTS.md or CLAUDE.md, the manifests, and the top-level layout. Draft a short
About with these sections, only including the ones you can support from the repository:

- `what`: what the project is and who it's for, in one or two sentences
- `stack`: languages, frameworks, storage
- `run`: how to build, test and run it
- `architecture`: the main parts and how they fit together
- `constraints`: anything that must stay true

Show the draft to the user. Once they agree (with any changes), record it:

- `c5 about set what "…"`
- `c5 about add "…" --section stack` (one bullet per fact, and the same for the other sections)

Don't record credentials. Record where they're kept instead.

## 4. Finish

Run `c5 context` and tell the user in a few lines what was set up: the project name and key, and what's in the
About. Mention that `c5 web` shows the project in the browser, and that you will record plans, decisions and
progress with c5 from now on. Offer to record the current plan if they describe what they're working on
(`c5 add plan "…"`).
