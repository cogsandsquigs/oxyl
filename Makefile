build:
	cargo build --release

test:
	cargo test

fuzz:
ifeq (,shell which cargo-afl)
	echo "This needs `cargo-afl` to run the fuzzer!"
else
	cargo afl build
	cargo afl fuzz -i ./oxylc-fuzzer/corpus -o ./oxylc-fuzzer/out target/debug/oxylc-fuzzer
endif

doc:
	cargo doc --no-deps
	mdbook build ./spec

doc-open:
	cargo doc --no-deps --open
	mdbook build spec --open

ifneq (,shell which cargo-watch)
	cargo watch -s 'cargo doc; mdbook build spec'
endif

