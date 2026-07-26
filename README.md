# MDView

[![Build](https://github.com/rweijnen/MDView/actions/workflows/release.yml/badge.svg)](https://github.com/rweijnen/MDView/actions/workflows/release.yml)

A fast, lightweight Markdown viewer for Windows. Available as a Total Commander Lister plugin and standalone executable with rich terminal output.

## Total Commander Plugin

![MDView in Total Commander](assets/screenshot.png)

## Features

- **WebView2 rendering** - Modern HTML rendering with full Markdown support (GUI mode)
- **Floating table of contents** - Collapsible H1–H6 outline docked to the side, with active-heading highlighting as you scroll (GUI mode)
- **Live reload** - The view refreshes automatically when the file changes on disk, and preserves your scroll position (GUI mode)
- **Keyboard navigation** - Vim-style keys (`j`/`k`, `gg`/`G`, `Ctrl+d`/`Ctrl+u`, `Ctrl+f`/`Ctrl+b`) plus `[`/`]` to jump between headings (GUI mode)
- **In-page find** - Press `/` to search the document (GUI mode)
- **Auto dark mode** - Follows Windows appearance settings (GUI mode)
- **Rich terminal output** - ANSI colors, clickable hyperlinks, unicode tables (terminal mode)
- **Clickable links** - Click `.md` links to navigate, external links open in browser
- **ESC to close** - Quick keyboard navigation
- **Syntax highlighting** - Code blocks with proper formatting
- **GitHub Flavored Markdown** - Tables, task lists, strikethrough, and more

## Installation

### Total Commander Plugin

1. Download the latest release ZIP
2. Open the ZIP file in Total Commander
3. Total Commander will automatically detect `pluginst.inf` and offer to install
4. Select your preferred installation directory
5. Configure file associations (`.md`, `.markdown`) in TC settings

> **Updating an existing install:** close all Lister / Quick View windows (or
> fully exit Total Commander) *before* installing a newer ZIP. A plugin DLL that
> is still loaded cannot be overwritten in place, and Total Commander may crash
> with an access violation if you overwrite it while it is loaded. After
> installing, start Total Commander and reopen the file.

### Standalone Executable

1. Download `mdview.exe` from the latest release
2. Place it anywhere in your PATH or desired location
3. Associate `.md` files with `mdview.exe` or run from command line

## Standalone Executable

The standalone `mdview.exe` automatically detects its environment:

- **From terminal** (cmd.exe, PowerShell, Windows Terminal): Renders markdown with ANSI formatting
- **Double-clicked** or no console: Opens GUI window with WebView2

### Windows Terminal

Full support for modern terminal features including clickable hyperlinks (OSC 8), true color, and unicode box drawing for tables.

![MDView in Windows Terminal](assets/screenshot-wt.png)

### Legacy cmd.exe

ANSI escape processing is automatically enabled for color support in legacy consoles.

![MDView in cmd.exe](assets/screenshot-cmd.png)

### Command Line Options

```
mdview [OPTIONS] [FILE]

Arguments:
  [FILE]  Markdown file to view (reads from stdin if not provided)

Options:
  --gui          Force GUI window mode
  --term         Force terminal output mode
  --html         Output full HTML document to stdout
  --body         Output HTML body only (no wrapper)
  --text         Output plain text (no formatting)
  --register     Register as .md file viewer (Open With)
  --unregister   Remove .md file viewer registration
  -h, --help     Show help message
```

### Examples

```bash
# Auto-detect: terminal output when run from console
mdview README.md

# Pipe content from another command
cat notes.md | mdview

# Force GUI window
mdview --gui README.md

# Output HTML for further processing
mdview --html README.md > output.html
```

### File Association

MDView can register itself as a handler for `.md` and `.markdown` files:

- **First launch (GUI mode):** MDView will offer to register as a viewer. Choose "Yes" to register and open Windows Settings where you can set it as the default. Choose "No" to be asked again next time, or "Cancel" to suppress the prompt permanently.
- **Command line:** Use `mdview --register` to register and `mdview --unregister` to remove the registration. Registration adds MDView to the "Open With" list; you then confirm the default in Windows Settings.

```bash
# Register as .md viewer and open Windows Settings
mdview --register

# Remove registration
mdview --unregister
```

### Terminal Features

| Feature | Windows Terminal | Legacy cmd.exe |
|---------|-----------------|----------------|
| ANSI colors | Yes | Yes |
| Bold/Italic | Yes | Yes |
| Clickable hyperlinks | Yes | No (text only) |
| Unicode tables | Yes | Yes |
| True color (24-bit) | Yes | Limited |

### Keyboard Shortcuts (GUI mode)

| Key | Action |
|-----|--------|
| ESC | Close viewer |
| Ctrl+O | Open file |
| `r` | Refresh (re-read the file from disk) |
| `e` or `i` | Edit the current file in an external editor |
| `t` | Toggle the table of contents panel |
| `j` / `k` | Scroll down / up (line); moves between TOC entries when the TOC is focused |
| `Ctrl+d` / `Ctrl+u` | Scroll half a page down / up |
| `Ctrl+f` / `Ctrl+b` | Scroll a full page down / up |
| `gg` / `G` | Jump to the top / bottom |
| `[` / `]` | Jump to the previous / next heading |
| `/` | Find text (Enter = next, Shift+Enter = previous, Esc = close) |
| `Tab` | Switch to the other Total Commander pane |
| ↑ ↓ PgUp PgDn Home End Space | Standard scrolling (browser defaults) |
| Click link | Navigate .md files in viewer, open external URLs in browser |
| Ctrl+Click | Always open link in browser |

### Table of Contents

The GUI viewer shows a collapsible outline of all headings (H1–H6) docked to the
left. Click a heading to jump to it; click the triangles to expand/collapse
sections. The current heading is highlighted as you scroll. Use `t` (or the ☰
button) to show/hide the panel. Its open/collapsed state is remembered per file.

When the panel has focus (click an entry), `j` / `k` move between entries. From
anywhere in the document, `[` and `]` jump to the previous / next heading.

### Live Reload and Editing

The viewer polls the open file and re-renders automatically whenever it changes
on disk, so you can keep it open beside your editor. Press `r` to force a
refresh, or `e` (or `i`) to open the file in an external editor.

By default `e` launches gvim at `C:\Vim\vim90\gvim.exe`. Set the `MDVIEW_EDITOR`
environment variable to use a different editor (the file path is passed as the
first argument):

```bat
set MDVIEW_EDITOR=C:\Program Files\Notepad++\notepad++.exe
```

### Finding Text

Press `/` to open the find bar in the top-right corner. Type your text, then
press `Enter` to jump to the next match, `Shift+Enter` for the previous match,
and `Esc` to close the bar. Search uses the browser's built-in text search.

### Menu Options (GUI mode)

| Menu | Item | Action |
|------|------|--------|
| File | Open | Open a Markdown file |
| File | Register as .md Viewer... | Add MDView to the Open With list |
| File | Unregister as .md Viewer | Remove MDView from Open With |
| Help | About MDView | Version and license info |

## Building from Source

### Prerequisites

- Rust 1.75 or later
- Windows 10/11 with WebView2 Runtime

### Build Commands

```bash
# Build release version (x64)
cargo build --release

# Build x86 version (for 32-bit Total Commander)
cargo build --release --target i686-pc-windows-msvc

# Run tests
cargo test
```

#### Cross-compiling from Linux / WSL

The Windows binaries can be built from Linux/WSL with
[cargo-xwin](https://github.com/rust-cross/cargo-xwin):

```bash
cargo install cargo-xwin
XWIN_ARCH=x86,x86_64 cargo xwin build --release --target x86_64-pc-windows-msvc
XWIN_ARCH=x86,x86_64 cargo xwin build --release --target i686-pc-windows-msvc
```

`XWIN_ARCH=x86,x86_64` is required so the 32-bit import libraries are downloaded
for the `i686` (32-bit Total Commander) target.

### Output Files

After building, copy the following files for distribution:

| File | Description |
|------|-------------|
| `target/release/mdview.exe` | Standalone viewer (x64) |
| `target/release/mdview_wlx.dll` | Rename to `mdview.wlx64` for TC plugin (x64) |
| `target/i686-pc-windows-msvc/release/mdview_wlx.dll` | Rename to `mdview.wlx` for TC plugin (x86) |

## Troubleshooting

### Debug Logging

If the plugin hangs or doesn't work correctly, enable debug logging:

1. Set environment variable: `set MDVIEW_DEBUG=1`
2. Start Total Commander from that command prompt
3. Try to view a markdown file
4. Check log file: `%TEMP%\mdview_debug.log`

The log shows WebView2 initialization steps and helps identify where issues occur.
With `MDVIEW_DEBUG=1` set, the plugin also enables the WebView2 developer tools
(press `F12`) so you can inspect the rendered page.

### Common Issues

| Issue | Solution |
|-------|----------|
| Plugin hangs | WebView2 has a 30-second timeout; check debug log |
| Blank display | Ensure WebView2 Runtime is installed |
| F3 opens WebView search | Update to latest version (F3 now passed to TC) |
| Total Commander crashes when updating the plugin | Close all Lister / Quick View windows (or exit TC) before installing the new ZIP; a loaded plugin DLL cannot be overwritten in place |
| Table of contents is empty | The document has no headings, or you are on an old build — update to the latest version |

## Requirements

- Windows 10 version 1803 or later
- [WebView2 Runtime](https://developer.microsoft.com/en-us/microsoft-edge/webview2/) (usually pre-installed on Windows 10/11)

## License

This project is licensed under the Mozilla Public License 2.0 - see the [LICENSE](LICENSE) file for details.

## Author

Remko Weijnen

## Acknowledgments

- [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark) - Markdown parsing
- [webview2-com](https://github.com/nicksenger/webview2-com) - WebView2 bindings for Rust
