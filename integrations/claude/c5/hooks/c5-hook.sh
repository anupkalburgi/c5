#!/bin/sh
# Forwards a Claude Code hook payload (stdin) to `c5 hook claude <event>`.
# Never fails the session: if c5 is not installed, it says so on stderr and exits 0.
C5="${C5_BIN:-$(command -v c5 2>/dev/null)}"
[ -z "$C5" ] && [ -x "$HOME/.cargo/bin/c5" ] && C5="$HOME/.cargo/bin/c5"
[ -z "$C5" ] && [ -x "$HOME/.local/bin/c5" ] && C5="$HOME/.local/bin/c5"
if [ -z "$C5" ]; then
  echo "c5: c5 not found on PATH; install it (cargo install --path crates/c5) or set C5_BIN" >&2
  exit 0
fi
exec "$C5" hook claude "$1"
