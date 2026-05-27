all: build test doc

build: build-all-features build-no-default-features

build-%: FORCE
	cargo build --$*

test: FORCE
	cargo test --all-features
	cargo test --all-features --release

doc: FORCE
	cargo +nightly doc --all-features

version: FORCE
	@grep '^version =' Cargo.toml | cut -d '"' -f 2

.PHONY: FORCE
FORCE:
