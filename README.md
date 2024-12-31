# xcap-screenshot-example

Demonstrates capturing screenshots of all screens using the `xcap` crate in Rust.

Run with `cargo run`. However, a release build is **recommended** as debug builds are significantly slower:

``bash
cargo run --release
``

Captures screenshots of all connected screens and saves them to the current directory.

Captures screenshots of all available screens, similar to how `screenshots::capture_area()` can be used to capture a specific area.

Provides functionality comparable to `mss.mss()` in Python's `mss` library, capturing full screenshots of each display.

