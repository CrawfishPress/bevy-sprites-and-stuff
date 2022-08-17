### Docs

    https://bevy-cheatbook.github.io/platforms/wasm.html
    https://rustwasm.github.io/docs/book/game-of-life/setup.html
    https://rustwasm.github.io/wasm-pack/installer/ # needed?
    https://rustwasm.github.io/wasm-bindgen/
    https://rustwasm.github.io/wasm-bindgen/reference/cli.html

### Setup
Added this to `~/.cargo/config.toml`

    [target.wasm32-unknown-unknown]
    runner = "wasm-server-runner"

Building:

    cargo build --release --target wasm32-unknown-unknown

### Webserving

    wasm-bindgen  --out-dir ./web/ --target web target/wasm32-unknown-unknown/release/bevy-sprites-and-stuff.wasm
    - add an `index.html` file for serving
    python -m http.server --directory web

### Issues
Note: I had to remove bevy-feature "dynamic", as `dynlib` doesn't support WASM.

Note: the "window" turns out to be larger than my browser's default - gotta use
< CTRL>- a few times, to shrink it down.
