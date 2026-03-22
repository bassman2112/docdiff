# Doc Diff

A desktop app for comparing documents side-by-side. Drop in two files and instantly see what changed — additions, deletions, and modifications — in a clean diff view.

**Supported formats:** `.docx`, `.rtf`, `.txt`, `.md`, `.csv`, `.json`, `.xml`, `.html`, `.yaml`, `.toml`, `.ini`, `.log`, `.rst`, and more.

<!-- ![Screenshot](screenshot.png) -->

## Features

- **Drag & drop** — open files by dragging them into the window
- **Word-level diffing** — highlights individual word changes, not just line-level
- **Side-by-side & unified views** — toggle between diff display modes
- **Cross-platform** — runs on macOS, Windows, and Linux
- **Fast & private** — everything runs locally, no files are uploaded anywhere

## Download

Grab the latest release for your platform from the [Releases](../../releases/latest) page:

| Platform | File |
|----------|------|
| macOS (Intel & Apple Silicon) | `.dmg` |
| Windows | `.msi` / `.exe` |
| Linux | `.deb` / `.AppImage` |

## Development

**Prerequisites:** Node.js 20+, Rust stable, and [Tauri v2 system dependencies](https://v2.tauri.app/start/prerequisites/).

```sh
# Install dependencies
make install

# Run in dev mode (hot reload)
make dev

# Production build
make build
```

### Cutting a release

```sh
make release VERSION=0.2.0
```

This bumps the version in `package.json` and `Cargo.toml`, commits, tags `v0.2.0`, and pushes the tag. GitHub Actions then builds binaries for all platforms and publishes them as a GitHub Release.

## License

MIT
