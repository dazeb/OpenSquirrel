# OpenSquirrel

Native, GPU-rendered control plane for AI coding agents. Rust + GPUI. No Electron.

Run Claude Code, Codex, Cursor, and OpenCode side by side with automatic sub-agent delegation, remote machine targeting via SSH, and persistent multi-turn sessions.

## What it does

- **Multi-agent grid** — Run multiple agents simultaneously in a responsive tiled layout. Agents auto-arrange based on count (1=full, 2=split, 4=2×2, etc).
- **Coordinator/worker delegation** — A primary agent (Opus) can automatically spawn sub-agents for focused tasks. Workers return condensed results, not full transcripts.
- **Remote machine targeting** — Agents can target local or remote machines via SSH + tmux. Configure machines in `~/.opensquirrel/config.toml`.
- **MCP integration** — MCP servers (Playwright, browser-use, etc.) are wired directly to agent CLI args. Select per-agent during setup.
- **Persistent sessions** — Agent state, transcripts, scroll positions, and pending prompts survive app restarts. Interrupted turns can be resumed.
- **Structured output parsing** — Parses `stream-json` output from all runtimes. Custom markdown rendering with code blocks, diffs, headings, bullets.

## Supported runtimes

| Runtime | Mode | Permission bypass |
|---------|------|-------------------|
| Claude Code | Persistent stdin (multi-turn) | `--dangerously-skip-permissions` |
| Codex | One-shot per prompt | `--dangerously-bypass-approvals-and-sandbox` |
| Cursor Agent | One-shot per prompt | `--yolo` |
| OpenCode | One-shot per prompt | Auto-approved in `run` mode |

## Build & run

```bash
# Build release and launch as macOS .app bundle
bash scripts/launch-opensquirrel-app.sh

# Or build directly
cargo build --release
./target/release/opensquirrel
```

Requires Rust toolchain and macOS (Metal GPU). The launch script creates `dist/OpenSquirrel.app` with the proper icon and shell environment.

## Keybinds

| Key | Action |
|-----|--------|
| `Esc` | Command mode |
| `i` | Insert mode (type prompts) |
| `Enter` | Send prompt |
| `Cmd-]` / `Cmd-[` | Next/prev pane |
| `Cmd-}` / `Cmd-{` | Next/prev group |
| `j/k` | Scroll transcript |
| `n` | New agent |
| `c` | Change agent config |
| `r` | Restart agent |
| `x` | Kill agent |
| `/` | Search |
| `Cmd-K` | Command palette (themes, settings, compact context) |
| `1/2/3` | Grid / pipeline / focus view |

## Configuration

Config lives at `~/.opensquirrel/config.toml`. Defines runtimes, models, MCP servers, machines, themes, and settings.

State is persisted at `~/.opensquirrel/state.json` (agents, transcripts, scroll positions).

## Architecture

~7,200 lines of Rust across 3 files:
- `src/main.rs` — UI, agent lifecycle, rendering, keybinds, persistence
- `src/lib.rs` — Line classification, markdown parsing, diff summarization, helpers
- `tests/state_tests.rs` — 30 integration tests covering navigation, scrolling, themes, search, agent lifecycle

Built on [GPUI](https://crates.io/crates/gpui) (the UI framework from Zed, used as a standalone crate). GPU-rendered via Metal on macOS.

## Themes

midnight, charcoal, gruvbox, solarized-dark, light, solarized-light, ops, monokai-pro

## License

MIT
