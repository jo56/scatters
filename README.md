# Scatters

Scatters takes your text files (`.txt`, `.md`, `.epub`) and creates randomized word collages inspired by the cut-up technique. Navigate through scattered words with an interactive terminal UI.

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

## Preview

![Scatters running with gruvbox theme](assets/screenshot-gruvbox.png)

*Example run with the gruvbox theme*


## Installation

### From GitHub

```bash
# Install the latest version
cargo install --git https://github.com/jo56/scatters

# Install a specific release
cargo install --git https://github.com/jo56/scatters --tag v0.1.0

# Or clone and build locally
git clone https://github.com/jo56/scatters.git
cd scatters
cargo install --path .
```

## Usage

```bash
# Basic usage
scatters /path/to/text/files

# Use the last path (after running once)
scatters

# With a specific theme
scatters /path/to/text/files --theme rosepine
scatters -t nord  # Uses last path with nord theme
```

Scatters remembers the last directory you used, so after the first run, you can simply type `scatters` without a path argument. The last-used path is saved in your system's config directory (`~/.config/scatters/` on Linux/macOS, `%APPDATA%\scatters\` on Windows).

### Available Themes

- `monochrome` - Black and white (default)
- `nord` - Cool arctic palette 
- `gruvbox` - Retro warm colors
- `rosepine` - Soft purple and pink tones

### Controls

- `↑/↓` - Adjust word density
- `←/→` - Navigate between words (highlights visited words)
- `r` - Reroll/regenerate the scatter
- `q` or `Ctrl+C` - Quit

## How It Works

1. **Parsing**: Scatters reads all text files from the specified directory
2. **Filtering**: Removes common stop words and keeps words 3+ characters long
3. **Generation**: Randomly places words across the terminal canvas
4. **Interaction**: Navigate and explore the scattered text with keyboard controls

The density control affects how many words appear on screen, and each reroll creates a new random arrangement from your word pool.

## Dependencies

Built with:
- [ratatui](https://github.com/ratatui/ratatui) - Terminal UI framework
- [crossterm](https://github.com/crossterm-rs/crossterm) - Cross-platform terminal manipulation
- [clap](https://github.com/clap-rs/clap) - Command line argument parsing
- [epub](https://github.com/danigm/epub-rs) - EPUB file parsing
- [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark) - Markdown parsing

## License

This project is licensed under the MIT License.
