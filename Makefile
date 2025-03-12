build:
	cargo build --release

test:
	cargo test

doc:
	cargo doc --no-deps
	mdbook build ./spec

doc-open:
	cargo doc --no-deps --open
	mdbook build spec --open

ifneq (,shell which cargo-watch)
	cargo watch -s 'cargo doc; mdbook build spec'
endif

