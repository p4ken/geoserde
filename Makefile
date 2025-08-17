build: FORCE
	cargo build --all-features

test: FORCE
	cargo test --no-default-features --tests
	cargo test --all-features

doc: FORCE
	cargo +nightly doc --all-features

.PHONY: FORCE
FORCE:
