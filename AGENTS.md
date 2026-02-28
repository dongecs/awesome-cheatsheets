# Repository Guidelines

## Project Structure & Module Organization
This repository is a collection of standalone cheatsheets, organized by topic:
- `languages/`: programming language references (`bash.sh`, `golang.go`, `rust.rs`, etc.).
- `tools/`: command-line and developer tool references (`git.txt`, `tmux.txt`, `ffmpeg.sh`, etc.).
- `editors/`: editor-specific references (`vim.txt`, `emacs.txt`, `org.org`).
- Root files: `README.md` (index), `LICENSE`, and `awesome.png`.

Place new content in the most specific directory and keep filenames lowercase and descriptive (for example, `tools/curl.txt`).

## Build, Test, and Development Commands
There is no formal build pipeline. Use lightweight local checks:
- `rg --files languages tools editors`: list tracked cheatsheet files.
- `bash -n languages/bash.sh tools/ffmpeg.sh tools/uv.sh`: syntax-check shell-based cheatsheets.
- `sed -n '1,120p' <path>`: preview a file before committing.
- `git diff -- README.md <path>`: ensure index and content changes stay in sync.

## Coding Style & Naming Conventions
- Keep entries concise, practical, and copy/paste friendly.
- Match the style and language of the target file (many files are Chinese-first).
- Use inline backticks for commands/flags and fenced blocks for multi-line examples.
- Prefer simple section headings and consistent spacing over long prose.
- Follow existing naming patterns: lowercase filenames with topic-oriented names (for example, `ip.txt`, `javascript.md`).

## Testing Guidelines
Automated tests and coverage gates are not configured. Validate changes manually:
- Confirm command snippets run or parse as expected.
- For Markdown/text changes, check readability in a renderer or plain terminal view.
- If adding a new cheatsheet, add/update its link in `README.md` and verify the relative path works.

## Commit & Pull Request Guidelines
Git history favors short, imperative subjects (for example, `update golang.go`, `fix typo`, `add Rust programming language quick reference guide`).
- Keep commits focused on one topic.
- In PRs, include: what changed, why it is useful, and which files were touched.
- Mention `README.md` index updates when adding, renaming, or removing cheatsheets.
- Include screenshots only when visual layout in Markdown rendering is materially affected.
