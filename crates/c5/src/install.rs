//! `c5 install`: writes the harness integrations that are embedded in the binary.

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use c5_core::env::{display_path, expand_tilde};
use clap::Subcommand;

const MARKETPLACE: &str = include_str!("../../../integrations/claude/.claude-plugin/marketplace.json");
const PLUGIN: &str = include_str!("../../../integrations/claude/c5/.claude-plugin/plugin.json");
const HOOKS: &str = include_str!("../../../integrations/claude/c5/hooks/hooks.json");
const HOOK_SH: &str = include_str!("../../../integrations/claude/c5/hooks/c5-hook.sh");
const README: &str = include_str!("../../../integrations/claude/c5/README.md");
const SKILL: &str = include_str!("../../../integrations/claude/c5/skills/c5/SKILL.md");
const AGENTS_MD: &str = include_str!("../../../integrations/agents-md/c5.md");

const BEGIN: &str = "<!-- c5:begin -->";
const END: &str = "<!-- c5:end -->";

#[derive(Subcommand)]
pub enum InstallCmd {
    /// Write the Claude Code plugin (a local marketplace) and print the commands that add it.
    Claude {
        /// Where to write it. Default: ~/.c5/integrations/claude.
        #[arg(long)]
        dir: Option<PathBuf>,
    },
    /// Add or update the c5 section of an AGENTS.md (Codex and other agents that read one).
    AgentsMd {
        /// Default: ~/.codex/AGENTS.md (Codex's global instructions).
        #[arg(long)]
        file: Option<PathBuf>,
        /// Only print the section.
        #[arg(long)]
        print: bool,
    },
}

fn write(path: &Path, content: &str) -> Result<()> {
    if let Some(d) = path.parent() {
        fs::create_dir_all(d).with_context(|| format!("creating {}", d.display()))?;
    }
    fs::write(path, content).with_context(|| format!("writing {}", path.display()))
}

/// Replaces the marked c5 section of `text`, or appends it.
pub fn upsert_section(text: &str, section: &str) -> String {
    let section = section.trim_end();
    if let (Some(b), Some(e)) = (text.find(BEGIN), text.find(END))
        && b < e
    {
        return format!("{}{section}{}", &text[..b], &text[e + END.len()..]);
    }
    let mut out = text.trim_end().to_string();
    if !out.is_empty() {
        out.push_str("\n\n");
    }
    out.push_str(section);
    out.push('\n');
    out
}

pub fn run(env: &c5_core::Env, cmd: InstallCmd) -> Result<()> {
    match cmd {
        InstallCmd::Claude { dir } => {
            let dir = dir.unwrap_or_else(|| env.integrations_dir().join("claude"));
            write(&dir.join(".claude-plugin/marketplace.json"), MARKETPLACE)?;
            let p = dir.join("c5");
            write(&p.join(".claude-plugin/plugin.json"), PLUGIN)?;
            write(&p.join("hooks/hooks.json"), HOOKS)?;
            write(&p.join("hooks/c5-hook.sh"), HOOK_SH)?;
            write(&p.join("skills/c5/SKILL.md"), SKILL)?;
            write(&p.join("README.md"), README)?;
            let shown = display_path(&dir);
            println!("Wrote the c5 Claude Code plugin to {shown}\n");
            println!("Add it from a shell:");
            println!("  claude plugin marketplace add {}", dir.display());
            println!("  claude plugin install c5@c5\n");
            println!("or inside Claude Code: /plugin marketplace add {shown}, then /plugin install c5@c5");
            println!("Re-run `c5 install claude` after upgrading c5, then `claude plugin marketplace update c5`.");
        }
        InstallCmd::AgentsMd { file, print } => {
            if print {
                print!("{AGENTS_MD}");
                return Ok(());
            }
            let file = file.unwrap_or_else(|| expand_tilde("~/.codex/AGENTS.md"));
            let existing = fs::read_to_string(&file).unwrap_or_default();
            let updated = upsert_section(&existing, AGENTS_MD);
            if updated == existing {
                println!("{} already has the current c5 section", display_path(&file));
            } else {
                write(&file, &updated)?;
                println!("Updated the c5 section in {}", display_path(&file));
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn section_is_appended_then_replaced() {
        let once = upsert_section("# Mine\n", AGENTS_MD);
        assert!(once.starts_with("# Mine\n\n<!-- c5:begin -->"));
        assert_eq!(upsert_section(&once, AGENTS_MD), once);
        let edited = once.replace("First thing in every session", "OLD");
        let fixed = upsert_section(&edited, AGENTS_MD);
        assert_eq!(fixed, once);
    }

    #[test]
    fn embedded_json_is_valid() {
        for j in [MARKETPLACE, PLUGIN, HOOKS] {
            serde_json::from_str::<serde_json::Value>(j).unwrap();
        }
    }
}
