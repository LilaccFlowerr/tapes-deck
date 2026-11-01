# tapesgui

A desktop GUI for [tapes-v2](https://github.com/futurekismo-oss/tapes-v2), the per-target dotfile switcher.
Tapes are dotfile setups (rices) installed in `~/.tapes`; tapesgui shows them and lets you
insert, eject and install them without the terminal.

Built with Rust and [Slint](https://slint.dev). It doesn't reimplement tapes-v2: it runs the
`tapes-v2` command and reads its output.

## Running

```
cargo run
```

`tapes-v2` must be installed (on `PATH` or in `~/.local/bin`).

## Folder structure

```
tapesgui/
├── build.rs          compiles ui/app.slint
├── assets/           images and icons
├── ui/               everything you see (Slint)
│   ├── app.slint     the window; picks which view to show
│   ├── theme.slint   shared colors and sizes
│   ├── views/        whole screens
│   └── components/   small reusable pieces
└── src/              everything it does (Rust)
    ├── main.rs       creates the window, wires things up, runs
    ├── tapes.rs      talks to tapes-v2
    └── ui.rs         glue between tapes.rs and the UI
```

### What goes where

**`ui/`** holds only presentation. Slint files get data in through properties and send user
actions out through callbacks. They contain no knowledge of tapes-v2.

- **`ui/app.slint`**: the `AppWindow`. It holds the top-level properties and callbacks that Rust
  talks to, and decides which view is visible (for example the empty view when there are no tapes).
  Keep it thin: it composes views, it doesn't draw much itself.
- **`ui/theme.slint`**: the `Theme` global with colors, font sizes and spacing. Use
  `Theme.text` instead of writing `#ebe5d9` in a component, so the look can change in one place.
- **`ui/views/`**: one file per screen, such as `empty.slint` (no tapes yet) and `library.slint`
  (your tapes). A view arranges components into a full page and exposes the callbacks that
  screen needs.
- **`ui/components/`**: small building blocks that are used in more than one place or are big
  enough to deserve their own file, such as a cassette, a button or a text field. A component
  shouldn't know which view it's in; give it what it needs through properties.

Rule of thumb: if you copy a piece of Slint into a second view, move it to `components/`.

**`src/`** holds behaviour.

- **`src/tapes.rs`**: everything that touches tapes-v2: running the command (`show`, `status`,
  `insert`, `eject`, `install`) and parsing its output into plain Rust types. No Slint in here.
  That keeps it easy to test, and it's the only file to change if tapes-v2's output changes.
- **`src/ui.rs`**: turns data from `tapes.rs` into Slint models and properties, and connects
  callbacks from the UI to calls in `tapes.rs`. Slow commands should run off the UI thread so
  the window doesn't freeze.
- **`src/main.rs`**: as short as possible: create the `AppWindow`, set it up through `ui.rs`,
  call `run()`.

**`assets/`** holds files loaded by the UI, referenced from Slint with `@image-url(...)` relative
to the `.slint` file. `add-tape.svg` is a public-domain cassette drawing from Openclipart.
