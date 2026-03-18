# Linux AppImage Packaging Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Make OpenSquirrel feel like a proper Linux desktop app with Linux-facing launch behavior, desktop metadata, AppImage packaging scripts, and presentable Linux documentation.

**Architecture:** Keep runtime behavior cross-platform in Rust, but move Linux distribution concerns into packaging assets and shell scripts. Package as an AppDir/AppImage first so the repo can produce a shareable Linux artifact without distro-specific packaging assumptions.

**Tech Stack:** Rust, GPUI, whisper-rs, shell packaging scripts, AppImage tooling, desktop-entry metadata

---

### Task 1: Stabilize Linux runtime behavior

**Files:**
- Modify: `Cargo.toml`
- Modify: `src/lib.rs`
- Modify: `src/main.rs`
- Test: `tests/state_tests.rs`

1. Keep whisper Metal enabled only on macOS.
2. Keep Linux launcher behavior native, with WSL using `explorer.exe` and desktop Linux using `xdg-open`.
3. Add/keep regression tests for Linux launcher and Linux reader font behavior.
4. Run focused Linux tests and `cargo check`.

### Task 2: Add Linux desktop packaging assets

**Files:**
- Create: `linux/AppDir/usr/share/applications/opensquirrel.desktop`
- Create: `linux/AppDir/usr/share/icons/hicolor/256x256/apps/opensquirrel.png`
- Create: `linux/README.md`

1. Add a Linux desktop file with name, icon, categories, and executable entry.
2. Stage icon assets for AppDir packaging.
3. Document Linux packaging assumptions and artifact layout.

### Task 3: Add reproducible packaging scripts

**Files:**
- Create: `scripts/build-linux-release.sh`
- Create: `scripts/build-appimage.sh`
- Modify: `scripts/check-linux-build.sh`

1. Build the release binary deterministically.
2. Assemble an AppDir from the binary and Linux assets.
3. Fetch or reuse `appimagetool` when needed.
4. Emit a final `.AppImage` into `dist/`.

### Task 4: Make the repo presentable for Linux users

**Files:**
- Modify: `README.md`

1. Add a Linux section with prerequisites, build instructions, X11 fallback guidance, and AppImage usage.
2. Keep macOS instructions intact but stop presenting the project as macOS-only.

### Task 5: Verify and publish

**Files:**
- Modify: git metadata only

1. Run `cargo check`.
2. Run `cargo test --lib` and `cargo test --test state_tests`.
3. Build the Linux release and AppImage if tooling is available.
4. Create a `codex/...` branch, commit changes, and push to GitHub.
