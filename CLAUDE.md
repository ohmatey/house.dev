# house.dev - Claude Code Instructions

A Tauri desktop app for finding and cleaning large folders in developer project directories.

## Quick Reference

```bash
# Development
pnpm tauri dev      # Run with hot reload

# Build
pnpm tauri build    # Production build (outputs to src-tauri/target/release/bundle/)

# Type check only
pnpm build          # Runs tsc && vite build
```

## Architecture

```
house.dev/
├── src/                    # Frontend (TypeScript)
│   ├── main.ts            # App logic, state, rendering
│   └── style.css          # Dark theme styles
├── src-tauri/             # Backend (Rust)
│   ├── src/lib.rs         # Folder scanning, deletion commands
│   ├── Cargo.toml         # Rust dependencies
│   ├── tauri.conf.json    # App config, permissions
│   └── capabilities/      # Tauri permissions
├── docs/ux/               # UX documentation
└── index.html             # Entry point
```

## Key Files

### Backend (Rust)

**`src-tauri/src/lib.rs`** - Core logic:
- `scan_folders(path)` - Recursively finds heavy folders (node_modules, .git, target, etc.)
- `delete_folder(path)` - Moves folder to trash (safe delete)
- `HEAVY_FOLDERS` - List of folder names to detect

### Frontend (TypeScript)

**`src/main.ts`** - App state and UI:
- `selectFolder()` - Opens native folder picker
- `scan()` - Calls Rust backend to scan
- `deleteFolder(path)` - Calls Rust backend to delete
- Uses `@tauri-apps/plugin-store` for persistence

### Config

**`src-tauri/tauri.conf.json`** - Window size, permissions, build config
**`src-tauri/capabilities/default.json`** - Runtime permissions (dialog, fs, store)

## Design Constraints

- **Minimal** - One feature: find and delete large folders
- **Dark mode only** - No light theme
- **JetBrains Mono** - Coding font
- **Safe delete** - Always moves to trash, never permanent delete
- **No extras** - No settings, no config, no complexity

## Adding New Folder Types

Edit `HEAVY_FOLDERS` in `src-tauri/src/lib.rs`:

```rust
const HEAVY_FOLDERS: &[&str] = &[
    "node_modules",
    ".git",
    // Add new folder name here
];
```

## Persistence

User's selected folder is saved to `settings.json` via Tauri's store plugin. Located at:
- macOS: `~/Library/Application Support/dev.housedev.folder-cleaner/settings.json`

## Testing

Currently manual testing only. Launch with `pnpm tauri dev` and:
1. Select a projects folder
2. Verify folders are scanned
3. Verify sizes are accurate
4. Test delete (check item moves to trash)
5. Verify folder persists after restart
