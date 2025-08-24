build: FORCE
	cargo build --all-features

test: FORCE
	cargo test --no-default-features --tests
	cargo test --all-features

doc: FORCE
	cargo +nightly doc --all-features

version: FORCE
	@grep '^version =' Cargo.toml | cut -d '"' -f 2

.PHONY: FORCE
FORCE:
