
CARGO_BUILD_STYLUS := \
	cargo build \
		-Z build-std=std,panic_abort \
		-Z build-std-features=panic_immediate_abort \
		-Z unstable-options \
		--release \
		--target wasm32-unknown-unknown \
		--artifact-dir .

.PHONY: build

build:
	@${CARGO_BUILD_STYLUS}
