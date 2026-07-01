# gptray

Instant, global-hotkey access to [ChatGPT](https://chatgpt.com) from anywhere on Windows — lives quietly in the system tray, opens as a small dropdown panel, and stays out of your way the rest of the time.

Inspired by [ik190/macos-chatgpt-overlay-bar](https://github.com/ik190/macos-chatgpt-overlay-bar), the macOS menu-bar equivalent of this idea.

## Why

Alt-tabbing to a browser tab (or opening a whole new one) every time you want to ask ChatGPT something is friction. `gptray` sits in the tray, does nothing until you summon it, and gets out of the way the instant you're done — without the RAM cost of running Electron or keeping a browser tab pinned all day.

## Features

- **Global hotkey** — `Ctrl+Shift+Space` toggles the panel open/closed from any app, any time.
- **Tray-only presence** — no taskbar entry, no window flash on launch, just an icon in the system tray.
- **Dropdown panel** — borderless, always-on-top, appears bottom-right above the taskbar, dismisses when you click away.
- **Persistent login** — log in to ChatGPT once; the session survives app restarts and reboots.
- **Low idle footprint** — the ChatGPT page isn't loaded until you open the panel for the first time, so the app sits at ~12-27MB RAM until you actually use it.
- **Launch at login** — optional, off by default, toggle it from the tray menu.

## Install

Grab the installer from [Releases](../../releases) and run it — per-user install, no admin rights needed.

> The installer is unsigned, so Windows SmartScreen will warn once on first run. Click "More info" → "Run anyway."

## Usage

- Press `Ctrl+Shift+Space` to open or close the panel.
- Left-click the tray icon to toggle it directly.
- Right-click the tray icon for the menu: **Toggle**, **Launch at Login**, **Quit**.

## Building from source

**Prerequisites:**
- [Node.js](https://nodejs.org) (v18+)
- [Rust](https://www.rust-lang.org/tools/install) (`rustup default stable-msvc`)
- MSVC C++ Build Tools ([Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022), "Desktop development with C++" workload)
- Windows 11 or Windows 10 with the [WebView2 Runtime](https://developer.microsoft.com/microsoft-edge/webview2/) installed (present by default on most modern Windows installs)

**Build:**

```powershell
npm install
npm run tauri build
```

Output:
- Executable: `src-tauri/target/release/chatgpt-overlay-bar.exe`
- Installer: `src-tauri/target/release/bundle/nsis/chatgpt-overlay-bar_0.1.0_x64-setup.exe`

For development, `npm run tauri dev` runs it without bundling.

## Tech stack

Built with [Tauri v2](https://v2.tauri.app/) — a Rust host process rendering through Windows' built-in WebView2 (Edge/Chromium) runtime — instead of Electron, specifically to avoid bundling a second full browser+Node.js runtime and keep idle memory usage low.

See [ARCHITECTURE.md](./ARCHITECTURE.md) for how it's put together.
