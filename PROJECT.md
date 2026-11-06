# Wallpaper Switcher

A Tauri v2 desktop app for browsing, downloading, and setting wallpapers on Wayland via the `awww` daemon.

## Tech Stack

| Layer | Technology |
|---|---|
| Desktop shell | Tauri v2 (Rust backend + webview frontend) |
| Frontend | Svelte 5 (runes mode) + SvelteKit (SPA/adapter-static) |
| Styling | Tailwind CSS v4 |
| Icons | Phosphor via `@iconify/svelte` + `@iconify-json/ph` (offline) |
| Font | Poppins via `@fontsource/poppins` (offline, weights 400/500/600) |
| Build system | Vite 8 + rolldown |
| Nix | flake.nix dev shell with all system deps |

## Architecture

```
┌─────────────────────────────────────────┐
│  Svelte Frontend (webview)              │
│  +page.svelte  ── orchestrates state    │
│  TopBar         ── header + buttons     │
│  WallpaperGrid  ── thumbnail grid       │
│  PreviewModal   ── full-size preview    │
│  DownloadModal  ── URL input + preview  │
├─────────────────────────────────────────┤
│  Tauri IPC (invoke)                     │
├─────────────────────────────────────────┤
│  Rust Backend                           │
│  commands.rs    ── 8 Tauri commands     │
│  utils/fs.rs    ── wallpapers_dir()     │
│  utils/image.rs ── ext_to_mime, base64  │
│  utils/download.rs ── filename, validate│
│  wallpaper/awww.rs ── awww CLI backend  │
│  wallpaper/mod.rs ── WallpaperBackend   │
│           trait                         │
└─────────────────────────────────────────┘
```

## File Structure

```
src-tauri/src/
├── lib.rs              # AppState, run(), mod declarations
├── main.rs             # Binary entry point
├── models.rs           # WallpaperEntry struct
├── commands.rs         # All #[tauri::command] functions
├── utils/
│   ├── mod.rs
│   ├── download.rs     # derive_filename, validate_image, mime_to_ext
│   ├── fs.rs           # wallpapers_dir() → $HOME/Pictures/Wallpapers
│   └── image.rs        # ext_to_mime, encode_data_url (base64)
└── wallpaper/
    ├── mod.rs          # WallpaperBackend trait
    └── awww.rs         # AwwwBackend (calls `awww img`, `awww query`)

src/
├── app.css             # Tailwind import, dark theme, Poppins font
├── app.html            # Root HTML
├── routes/
│   ├── +layout.svelte  # Icon registration, font imports, h-full wrapper
│   ├── +layout.ts      # ssr = false (SPA mode)
│   └── +page.svelte    # Main page — state management, modal orchestration
└── lib/components/
    ├── TopBar.svelte       # Header: logo, "From Web", "Open folder"
    ├── WallpaperGrid.svelte # Responsive thumbnail grid with snippet
    ├── PreviewModal.svelte  # Full-size preview + Set/Open/Close actions
    └── DownloadModal.svelte # URL input, live preview, download button
```

## Tauri Commands

| Command | Args | Returns | Notes |
|---|---|---|---|
| `list_wallpapers` | — | `WallpaperEntry[]` | Reads `$HOME/Pictures/Wallpapers`, filters images |
| `set_wallpaper` | `path: String` | `String` | Calls `awww img <path> --transition-type center` |
| `get_current_wallpaper` | — | `Option<String>` | Queries awww, falls back to in-memory state |
| `download_wallpaper` | `url: String` | `WallpaperEntry` | Fetches URL, validates image, saves with correct extension |
| `read_image` | `path: String` | `String` | Reads local file, returns `data:<mime>;base64,...` |
| `open_wallpapers_folder` | — | `()` | `xdg-open` on wallpapers dir |
| `open_in_folder` | `path: String` | `()` | `xdg-open` on parent dir |
| `preview_url` | `url: String` | `String` | Fetches URL, validates, returns base64 data URL |

## Key Design Decisions

- **Images as base64**: WebKitGTK version on NixOS doesn't support `asset://` protocol for images. All images are read by Rust and returned as base64 `data:` URLs to bypass cross-origin restrictions.
- **Offline icons & fonts**: `@iconify-json/ph` and `@fontsource/poppins` bundle everything locally — no CDN requests at runtime.
- **Wallpaper backend trait**: `WallpaperBackend` trait in `wallpaper/mod.rs` makes it easy to add new backends (hyprpaper, swaybg, etc.). Currently only `awww` is implemented.
- **MIME-aware downloads**: `download_wallpaper` detects the correct file extension from `Content-Type` header or magic bytes, stripping any misleading extension from the URL.
- **Svelte 5 snippets**: `WallpaperGrid` uses `{#snippet wallpaperItem(entry)}` for reusable tile markup.

## NixOS Dev Environment

```sh
nix develop    # enters shell with cargo, nodejs, webkitgtk, etc.
```

System deps required by Tauri on NixOS:
- `webkitgtk_4_1` (provides `javascriptcoregtk-4.1`, `webkit2gtk-4.1`)
- `dbus`
- `librsvg`, `gst_all_1.gst-plugins-base`

## Build & Run

```sh
# Ensure awww daemon is running
awww-daemon &

# Dev mode
nix develop
npm run tauri dev

# Production build
nix develop
npm run tauri build
```

## Styling Conventions

- Dark mode only — neutral palette: `neutral-925` (bg), `neutral-850` (surfaces), `neutral-750` (inputs/buttons), `neutral-650` (borders)
- Custom Tailwind colors defined in `app.css` `@theme` block
- Default font: Poppins 500, antialiased
- Scrollbar: custom dark-themed WebKit scrollbar
- Window: transparent, always-on-top, resizable