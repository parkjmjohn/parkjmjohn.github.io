# johnjpark.com

Personal site, written in Rust and compiled to WebAssembly with
[Dioxus](https://dioxuslabs.com). Deployed to GitHub Pages.

## Prerequisites

```sh
rustup target add wasm32-unknown-unknown
cargo install dioxus-cli --version 0.7.10 --locked
```

## Develop

```sh
dx serve          # http://localhost:8080, hot reloads on save
```

## Build

```sh
dx bundle --platform web --release
```

Output lands in `target/dx/portfolio/release/web/public`.

## Checks

```sh
cargo fmt --check
cargo clippy --target wasm32-unknown-unknown -- -D warnings
```

## Editing content

All copy — name, bio, roles, projects, links — lives in
[`src/content.rs`](src/content.rs). Editing the site normally means editing
that file only; the components in `src/main.rs` render whatever it holds.

Styling is plain CSS in [`assets/main.css`](assets/main.css), organised with
custom properties at the top for colour and spacing.

## Deploying

Pushes to `main` trigger [`.github/workflows/deploy.yml`](.github/workflows/deploy.yml),
which builds the site and publishes it to GitHub Pages. The repository's Pages
source must be set to **GitHub Actions** (Settings → Pages).
