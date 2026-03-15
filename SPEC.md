# OpenSquirrel

GPU-rendered, keyboard-driven agent session hub. A native control plane for AI coding agents with coordinator/worker delegation, remote machine targeting, and persistent sessions.

Written in Rust. Built on GPUI. No Electron. No web tech. Keyboard-first.

## Current State (what exists and works)

### Core Stack
- **UI**: GPUI (standalone crate, not a Zed fork)
- **Language**: Rust, 100%
- **Platform**: macOS (Metal) primary, Linux (Vulkan via Blade) possible
- **App bundle**: `scripts/launch-opensquirrel-app.sh` builds release binary and opens `dist/OpenSquirrel.app` with proper icon

### Agent Model
- Agents are subprocesses spawned from CLI runtimes (Claude Code, Cursor, Codex, OpenCode)
- Local Claude coordinator uses persistent stream-json stdin for multi-turn conversations
- Other runtimes use one-process-per-prompt
- Coordinators get a delegation preamble instructing them to spawn workers via ```delegate fenced blocks
- Workers are fresh-context agents that return condensed results (final text + metadata + diff summary) to the coordinator
- Workers can target local or remote (SSH + tmux) machines

### Machine Targets
- Configured in `~/.opensquirrel/config.toml` under `[[machines]]`
- Default: `local` and `theodolos` (SSH)
- Remote workers launch inside named tmux sessions on the target machine
- Remote session names and line cursors are persisted for reattach on app restart
- Machine selection is available in the setup wizard and in delegated task JSON

### Persistence
- Config: `~/.opensquirrel/config.toml` (runtimes, machines, MCPs, theme, font, settings)
- State: `~/.opensquirrel/state.json` (agents, transcripts, scroll positions, worker assignments, pending prompts, turn state, remote session info)
- Turn-boundary journaling: pending prompts and turn state are saved so interrupted turns can be resumed
- Restored agents show a banner (not injected into transcript history)

### UI Layout
- Top bar: minimal — ⚙ settings (opens command palette), ⊞ stats toggle. No search bar (use `/` key).
- Left sidebar: agents tab / workers tab, group navigation, agent list with role/runtime/machine indicators
- Main area: focused agent tile (default view), grid view, pipeline view
- Agent tile: single compact header row (squirrel icon | name | status | elapsed | model | tokens/cost | context% | action icons), optional badges row, worker strip, transcript area, input bar
- Confirmation modal for destructive remove action (red yes / normal no)
- Command palette (Cmd-K): themes, settings toggles, new agent, mic selection

### Keybinds (current)
- `Esc` → command mode
- `i` → insert mode
- `Enter` → send prompt (default) or insert newline (cautious mode)
- `Cmd+Enter` → always send
- `j/k` → scroll transcript
- `w/s` → switch groups
- `a/d` → switch panes (left/right)
- `n` → new agent (opens setup wizard)
- `c` → change agent runtime/model/machine
- `r` → relaunch agent
- `x` → stop agent
- `f` → toggle favorite
- `p` → toggle auto-scroll
- `|` → pipe output to next agent
- `g t` → open working directory in Terminal
- `/` → open search panel
- `t` → cycle theme
- `?` → stats/shortcuts panel
- `` ` `` → toggle voice recording (whisper.cpp)
- `1/2/3` → grid/pipeline/focus view
- `Cmd-K` → command palette

### Settings (toggleable via command palette)
- Cautious Enter (off by default): makes Enter insert newline, Cmd+Enter send
- Terminal Text (off by default): uses monospace font for transcript instead of prose font
- Whisper Model: configurable model name (default: large-v3-turbo)
- Audio Device: selectable microphone from available input devices

### Voice Input
- whisper.cpp via `whisper-rs` crate with Metal GPU acceleration
- `cpal` for audio capture at device native sample rate
- Resamples to 16kHz before inference
- Model files expected at `~/.opensquirrel/models/ggml-{name}.bin`
- Toggle with backtick key, shows red REC indicator while recording

### Themes
midnight, charcoal, gruvbox, solarized-dark, light, solarized-light, ops, monokai-pro

### Transcript Rendering
- Prose font (Helvetica Neue) by default, monospace optional via setting
- Message blocks with rounded card styling and spacing
- User prompts in distinct bordered cards
- Code blocks with monospace font and syntax-aware background
- Headings, bullets, inline markdown (bold, italic, code spans)
- Diff lines color-coded (green add, red remove, blue hunk)
- System/error/thinking lines styled distinctly
- Per-message copy icon (not per-line)
- Mouse/trackpad scroll wheel support on transcript area

## Next Steps (from raw thoughts + session direction)

### Immediate (ship-quality polish)
- [x] Hide search bar from top bar entirely; search is `/`-only, no visible trigger needed
- [x] Compress agent info into a single thin bar: model | status | tokens | cost — one line, not multi-row
- [x] Replace remaining text-based icons with proper icon symbols (⚙ settings, ⊞ stats)
- [x] Remove voice feature for v1 ship (gated behind `VOICE_ENABLED` const, keybind + palette hidden)

### Short-term (product direction)
- [ ] Let the model control delegation entirely — don't build swarm UX with keybinds, give Opus the ability to spawn/manage sub-agents and trust it to improve over time
- [ ] Group chat mode: API-only chat room where multiple agents can be added, with @mentions, configurable reply order, and manual turn-taking option
- [ ] Test coordinator → worker delegation with Opus actually driving it end-to-end on a real task
- [ ] Customizable keybinds settings UI

### Medium-term (differentiation)
- [ ] Tab completion model for IDE actions (requires fast local inference — not feasible with current token generation speeds, revisit when local models are faster)
- [ ] Streaming/chunked voice transcription instead of record-then-transcribe
- [ ] Remote Parakeet on CUDA targets for voice-to-text on GPU boxes
- [ ] In-app model download for whisper variants

### Non-Goals (for now)
- Code editor (agents edit code, not the user)
- File tree browser
- LSP integration
- Git UI
- Plugin/extension system
- Collaboration/team features
- Approval queue (removed — not part of the workflow)
