# Contributing to tapes-deck

Thanks for wanting to help. tapes-deck is a small project, so the rules are short.

Not sure where to start? Ask in the [Discord](https://discord.gg/TVjsGzFAur) or look at the
[roadmap](README.md#roadmap). For anything bigger than a small fix, open an issue or ask
first, so you don't spend time on something that doesn't fit.

## Reporting bugs and requesting features

Use [GitHub Issues](https://github.com/LilaccFlowerr/tapes-deck/issues/new). For a bug, include:

- what you did, what you expected and what happened instead
- your distro and how you installed tapes-deck (install script, AppImage, built from source)
- your `tapes-v2 --version`
- the text from the output view in the app, if there is any

## AI policy

You may use AI tools, but you are the author of what you submit.

- **You must understand it.** If you can't explain what a change does and why, don't submit it.
  Reviewers will ask, and "the AI wrote it" is not an answer.
- **Review it yourself first.** Run it, read the diff, remove what doesn't belong. Unreviewed AI
  output wastes the maintainers' time and will be closed.
- **Write your own words.** Commit messages, PR descriptions, issue text and review comments
  must be written by you, not generated. This includes `Co-Authored-By` trailers from AI tools.
- **Say so in the PR** if a meaningful part of the code came from an AI tool, so reviewers know
  to look closely.

If a PR looks like it was submitted without reading it, it will be closed without a review.

## Setting up

You need Rust (edition 2024, so a recent stable toolchain), `pkg-config` and the fontconfig
development files (`libfontconfig1-dev` on Debian/Ubuntu, `fontconfig` on Arch).
[tapes-v2](https://github.com/futurekismo-oss/tapes-v2) must be installed, on `PATH` or in
`~/.local/bin`.

```
git clone https://github.com/LilaccFlowerr/tapes-deck
cd tapes-deck
cargo run
```

## Making a change

1. Fork the repo and branch off `main`. Name the branch after what it does: `feat/tape-editor`,
   `fix/empty-view`, `chore/readme`.
2. Keep a PR to one thing. Two unrelated fixes are two PRs.
3. Run `cargo fmt` and `cargo clippy` before you push, and make sure `cargo build` is clean.
4. Follow the style of the code around your change: naming, comment density, structure.
5. Open the PR against `main`. Say what changed and why, and link the issue if there is one.
   For UI changes, add a screenshot.
6. Be ready for feedback. Review is about the change, not about you.

### Commit messages

Short, in the imperative, with a prefix like the existing history:

```
feat: add inline tape.toml editor
fix: don't crash when ~/.tapes is missing
chore: update dependencies
docs: explain the install script
```

## Project layout

```
tapes-deck/
├── build.rs          compiles ui/app.slint
├── assets/           images, icons and fonts
├── packaging/        .desktop file and the PKGBUILD for Arch
├── install.sh        the one-line installer
├── ui/               everything you see (Slint)
│   ├── app.slint     the window; picks which view to show
│   ├── theme.slint   shared colors and sizes
│   ├── types.slint   data types shared with Rust
│   ├── views/        whole screens
│   └── components/   small reusable pieces
└── src/              everything it does (Rust)
    ├── main.rs       creates the window, wires things up, runs
    ├── ui.rs         glue between src/tapes and the UI
    └── tapes/        talks to tapes-v2
        ├── mod.rs    the actions: show, status, insert, eject, install
        ├── cli.rs    finds and runs the tapes-v2 command
        ├── parse.rs  turns its output into Rust types
        └── types.rs  those types
```

### What goes where

**`ui/`** holds only presentation. Slint files get data in through properties and send user
actions out through callbacks. They contain no knowledge of tapes-v2.

- **`ui/app.slint`**: the `AppWindow`. It holds the top-level properties and callbacks that Rust
  talks to, and decides which view is visible (for example the empty view when there are no tapes).
  Keep it thin: it composes views, it doesn't draw much itself.
- **`ui/theme.slint`**: the `Theme` global with colors, font sizes and spacing. Use
  `Theme.text` instead of writing a hex color in a component, so the look can change in one place.
- **`ui/views/`**: one file per screen, such as `empty.slint` (no tapes yet) and `library.slint`
  (your tapes). A view arranges components into a full page and exposes the callbacks that
  screen needs.
- **`ui/components/`**: small building blocks that are used in more than one place or are big
  enough to deserve their own file, such as a cassette, a button or a text field. A component
  shouldn't know which view it's in; give it what it needs through properties.

Rule of thumb: if you copy a piece of Slint into a second view, move it to `components/`.

**`src/`** holds behaviour.

- **`src/tapes/`**: everything that touches tapes-v2: running the command and parsing its output
  into plain Rust types. No Slint in here. That keeps it easy to test, and it's the place to
  change if tapes-v2's output changes.
- **`src/ui.rs`**: turns data from `src/tapes` into Slint models and properties, and connects
  callbacks from the UI to calls in `src/tapes`. Slow commands should run off the UI thread so
  the window doesn't freeze.
- **`src/main.rs`**: as short as possible: create the `AppWindow`, set it up through `ui.rs`,
  call `run()`.

**`assets/`** holds files loaded by the UI, referenced from Slint with `@image-url(...)` relative
to the `.slint` file. `add-tape.svg` is a public-domain cassette drawing from Openclipart, and `icon.svg` is a copy of it used as a placeholder app icon.
`fonts/Caveat.ttf` is the handwriting font (SIL Open Font License, see `fonts/OFL.txt`).

## Releases

Maintainers only. Pushing a tag that starts with `v` runs the release workflow, which builds
the binary, the `.tgz` and the AppImage and publishes them as a GitHub release.

## License

tapes-deck is GPL-3.0. By contributing you agree that your contribution is licensed under the
same terms (see [LICENSE](LICENSE)).
