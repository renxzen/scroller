# scroller

A dependency-free CLI that renders every image in the current directory as a
single scrollable HTML page and opens it in the default browser. Built as a
practice Rust port of a Python original.

## Usage

```
scroller
```

Run it from any directory containing images (`.png`, `.jpg`, `.jpeg`,
`.webp`, `.gif`, `.svg`, `.bmp`, `.tiff`, `.ico`). It generates a hidden
`.scroller_view.html` in that directory, opens it (`open` on macOS,
`xdg-open` on Linux), and deletes it again ~45 seconds later via a detached
child process (`scroller --cleanup-after <path>`).

The viewer supports adjustable image width/gap, dark/light theme, scroll
snapping, a sticky header, keyboard navigation, and per-image copy/open
actions — all client-side, no server involved.

## Development

Requires [just](https://github.com/casey/just).

```
just run      # cargo run (accepts extra args)
just build    # release build
just test     # cargo test
just check    # fmt + clippy
just install  # cargo install --path .
```
