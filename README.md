<p align="center">
  <img src=".github/assets/landing_view.png" alt="Difference" width="800" />
</p>

<h1 align="center">DIFFERENCE</h1>

<p align="center">
  <strong>A fast, minimal git branch diff viewer for the AI era.</strong>
</p>

<p align="center">
  <a href="#why">Why</a> •
  <a href="#features">Features</a> •
  <a href="#install">Install</a> •
  <a href="#usage">Usage</a> •
  <a href="#stack">Stack</a>
</p>

---

## Why

AI coding tools generate massive diffs. Hundreds of files. Thousands of lines.

The existing options are painful:

- **GitHub** — Open a PR just to see your changes? Then wait for their slow diff renderer to choke on large changesets? No thanks.
- **Git GUIs** — Bloated apps with features you'll never use. Slow to launch, slower to navigate.
- **Terminal** — `git diff` works, but scrolling through 50 files in a terminal isn't reviewing, it's suffering.

**Difference** is built for one thing: viewing branch diffs. Fast. Locally. Without the cruft.

## Features

```
[+] instant local diffs     — no PR required, no network latency
[+] unified & split views   — toggle between diff styles
[+] file tree navigation    — browse changes by directory structure
[+] status filters          — show/hide added, modified, deleted files
[+] cosmetic detection      — identify comment-only and whitespace changes
[+] fuzzy search            — quickly find files in large changesets
[+] keyboard-first          — navigate without touching your mouse
```

<p align="center">
  <img src=".github/assets/diff_view.png" alt="Diff View" width="800" />
</p>

## Install

```bash
# clone
git clone https://github.com/yourusername/difference.git
cd difference

# install dependencies
bun install

# run in development
bun tauri dev

# build for production
bun tauri build
```

Requires [Rust](https://rustup.rs/) and [Bun](https://bun.sh/).

## Usage

1. Click **open** or drag a git repository folder
2. Select your base branch (defaults to `main`)
3. Browse changed files, review diffs

That's it. No accounts. No sync. No telemetry.

## Stack

- **[Tauri](https://tauri.app)** — Lightweight native shell
- **[Svelte](https://svelte.dev)** — Reactive UI
- **[git2-rs](https://github.com/rust-lang/git2-rs)** — Native git operations
- **[shadcn-svelte](https://shadcn-svelte.com)** — Component primitives

---

<p align="center">
  <sub>Built for developers who ship fast and review faster.</sub>
</p>
