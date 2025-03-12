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
