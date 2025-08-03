test: FORCE
	cargo test --all-features

doc: FORCE
	cargo +nightly doc --all-features

.PHONY: FORCE
FORCE:
