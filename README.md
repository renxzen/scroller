# scroller

A CLI that renders every image in a directory as a single scrollable HTML page
and opens it in the default browser. Built as a practice Rust port of a Python
original.

## Usage

Run it from inside a directory that contains images:

```bash
cd ~/Pictures/some-folder
scroller
```

- Supported formats: `.png`, `.jpg`, `.jpeg`, `.webp`, `.gif`, `.svg`, `.bmp`,
  `.tiff`, `.ico` (extension match is case-insensitive).
- Only the directory itself is scanned, not subdirectories.
- Images are ordered naturally and case-insensitively (`img2` before `img10`).
- Exits with an error if the directory has no images.

It writes a hidden `.scroller_view.html` into that directory, opens it (`open`
on macOS, `xdg-open` on Linux), and deletes it ~45 seconds later via a detached
child process (`scroller --cleanup-after <path>`). The page references images
by filename, so a manual reload after the file is deleted is not possible.

The viewer is entirely client-side, with no server. It provides adjustable
image width and gap, dark/light theme, scroll snapping, a sticky header,
per-image copy name/path and open-original actions, and the hotkeys below.
Settings persist in the browser's local storage.

## Hotkeys

| Action                            | Keys                                 |
| --------------------------------- | ------------------------------------ |
| Next image                        | `Space`, `Down`, `J`                 |
| Previous image                    | `Up`, `K`                            |
| First / last image                | `Home` / `End`                       |
| Scroll slightly                   | `W` (up), `S` (down)                 |
| Narrower / wider images           | `Left`, `H`, `A` / `Right`, `L`, `D` |
| Reset width                       | `0`                                  |
| Fullscreen current image          | `F`                                  |
| Toggle scroll snap                | `N`                                  |
| Toggle theme                      | `T`                                  |
| Show shortcuts                    | `?`                                  |
| Close shortcuts / exit fullscreen | `Esc`                                |

## Nautilus integration (Linux)

`scroller` can be launched from the Nautilus right-click menu. It reads the
`NAUTILUS_SCRIPT_SELECTED_FILE_PATHS` variable set by Nautilus: if the first
selected path is a directory, that directory is used. Otherwise the current
working directory is used, which Nautilus sets to the open folder for local
paths.

1. Install the binary (see Development): `just install` places it in
   `~/.cargo/bin/scroller`.
2. Create the script. Nautilus does not load the shell profile, so use the
   absolute path:

   ```bash
   mkdir -p ~/.local/share/nautilus/scripts
   cat > ~/.local/share/nautilus/scripts/scroller <<'EOF'
   #!/usr/bin/env bash
   exec "$HOME/.cargo/bin/scroller"
   EOF
   chmod +x ~/.local/share/nautilus/scripts/scroller
   ```

3. In Nautilus, right-click a folder (or the empty background inside a folder)
   and choose **Scripts > scroller**. If the entry is missing, restart
   Nautilus with `nautilus -q`.

## Development

Requires [just](https://github.com/casey/just).

```
just run      # cargo run (accepts extra args)
just build    # release build
just test     # cargo test
just check    # fmt + clippy
just install  # cargo install --path .
```

Layout: `src/main.rs` holds the CLI logic. `web/` holds the viewer
(`scroller.html` Askama template, `scroller.css`, `scroller.js`), all embedded
into the binary at build time.
