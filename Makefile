all: build test doc

build: build-all-features build-no-default-features

build-%: FORCE
	cargo build --workspace --$*

test: FORCE
	cargo test --workspace --all-features
	cargo test --workspace --all-features --release

doc: FORCE
	cargo +nightly doc --workspace --all-features

PKG ?= geoserde

version: FORCE
	@cargo pkgid --package $(PKG) | sed 's/.*[#@]//'

.PHONY: FORCE
FORCE:
