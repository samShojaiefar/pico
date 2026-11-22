# Pico

**A fast, lightweight emoji picker for Windows.**

Pico lets you quickly find and insert emojis without interrupting your workflow. Open it with a keyboard shortcut and get back to what you're doing.

<p align="center">
  <img src="assets/icon.png" width="120" alt="Pico logo">
</p>

## Features

-  **Fast:** Optimized for quick startup and responsive interaction.
-  **Improved rendering:** A cleaner rendering style.
-  **Keyboard shortcut:** Open Pico using `Right Alt + /`.
-  **Lightweight:** Built with Rust.
-  **System tray:** Runs in the Windows system tray.
-  **Automatic startup:** Pico automatically launches when you start Windows.

## Preview

<p align="center">
  <img src="assets/screenshots/pico-perview.png" width="600" alt="Pico emoji picker">
</p>

## Installation

1. Download the latest Pico release.
2. Extract the downloaded files if necessary.
3. Run `pico.exe`.
4. Pico will register itself to launch automatically when Windows starts.
5. Press `Right Alt + /` to open the emoji picker or just type `//` everywhere!.

## Build from source

### Requirements

- Windows
- [Rust](https://www.rust-lang.org/tools/install)
- Cargo

### Clone the repository

```bash
git clone https://github.com/YOUR_USERNAME/pico.git
cd pico
```

### Run in development

```bash
cargo run
```

### Build for release

```bash
cargo build --release
```

Executable location:

```text
target/release/pico.exe
```

## Keyboard shortcut

| Shortcut | Action |
|---|---|
| `Right Alt + /` | Open Pico |

## Changelog

### v0.1.2

- Added automatic startup with Windows.
- Pico now launches automatically when Windows starts.

### v0.1.1

- Improved application speed.
- Updated rendering style.
- General performance improvements.

### v0.1.0

- Initial release.
- Basic emoji picker functionality.

## License

Pico is licensed under the [MIT License](LICENSE).

<p align="center">
  Made with Rust.
</p>
