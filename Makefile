all: build test doc

build: build-all-features build-no-default-features

build-%: FORCE
	cargo build --workspace --$*

test: FORCE
	cargo test --workspace --all-features
	cargo test --workspace --all-features --release

doc: FORCE
	cargo +nightly doc --workspace --all-features

version: FORCE
	@grep '^version =' Cargo.toml | cut -d '"' -f 2

.PHONY: FORCE
FORCE:
