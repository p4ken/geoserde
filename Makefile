all: build test lint doc

build: build-all-features build-no-default-features

build-%: FORCE
	cargo build --workspace --$*

test: FORCE
	cargo test --workspace --all-features
	cargo test --workspace --all-features --release

lint: FORCE
	cargo clippy --workspace --all-targets --all-features -- -D warnings
	cargo fmt --all -- --check

doc: FORCE
	RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --workspace --no-deps --all-features -Zrustdoc-map

PKG ?= geoserde

version: FORCE
	@cargo pkgid --package $(PKG) | sed 's/.*[#@]//'

.PHONY: FORCE
FORCE:
