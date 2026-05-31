# AI Light

Cross-platform desktop traffic light widget for monitoring AI coding assistants (Claude Code, Codex).


<img width="806" height="458" alt="image" src="https://github.com/user-attachments/assets/924ac6a8-feeb-4a98-af09-01afd2fb3e24" />


## Status

🚧 **In Development** - MVP implementation in progress

## Architecture

- **Backend:** Rust (Tauri 2.x)
- **Frontend:** Vanilla HTML/CSS/JS
- **Platforms:** Windows, macOS, Linux (X11)

## Development

```bash
# Run in dev mode
cd src-tauri
cargo tauri dev

# Build
cargo tauri build

# Run tests
cargo test
```

## Documentation

- [Design Spec](docs/superpowers/specs/2026-05-30-ai-light-design.md)
- [Implementation Plan](docs/superpowers/plans/2026-05-30-ai-light-implementation.md)
- [Build & Packaging Guide](docs/BUILDING.md)
