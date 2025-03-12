# oxyl

My silly little programming language.

## Building

### Requirements

- [Rust toolchain](https://www.rust-lang.org/tools/install)
- [mdBook](https://github.com/rust-lang/mdBook) (For documentation)

### Building

Run `make build` or `cargo build --release`

### Testing

Run `make test` or `cargo test`

### Documentation

Run `make doc`. You can also run `make doc-open` to automatically open the documentation.

> [!NOTE]
> If [cargo-watch](https://crates.io/crates/cargo-watch), is installed, the latter will automatically watch the
> directory and rebuild the documentation on changes.
