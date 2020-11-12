ifeq ($(DEBUG_WASM),1)
  wasm_pack_profile=--dev
else
  wasm_pack_profile=--release
endif

CLIPPY_PARAMS =  --all-features -- -W clippy::cargo -W clippy::nursery -W clippy::expect_used -W clippy::unwrap_used -W clippy::items_after_statements -W clippy::if_not_else -W clippy::trivially_copy_pass_by_ref -W clippy::match_same_arms
CLIPPY_CARGO = cargo
ifeq ($(CLIPPY_NIGHTLY),1)
  CLIPPY_CARGO += +nightly
  CLIPPY_PARAMS := --benches $(CLIPPY_PARAMS)
else
  CLIPPY_PARAMS += -D warnings
endif
ifeq ($(CLIPPY_PEDANTIC),1)
  CLIPPY_PARAMS += -W clippy::pedantic
endif

WASM_PACK_COMMAND := wasm-pack build $(wasm_pack_profile) --target no-modules --out-dir site

check:
	$(CLIPPY_CARGO) fmt --all
	$(CLIPPY_CARGO) clippy --tests $(CLIPPY_PARAMS)
	$(CLIPPY_CARGO) clippy --lib --bins $(CLIPPY_PARAMS) -D clippy::panic
	$(CLIPPY_CARGO) test

install-cargo-deps:
	cargo install cargo-benchcmp cargo-watch devserver

bench:
	cargo +nightly bench

site-downloads:
	cd crates/wasm && \
		curl https://adventofcode.com/favicon.ico > site/favicon.ico

site-wasmpack:
	cd crates/wasm && $(WASM_PACK_COMMAND)

wasm-size: site-wasmpack
	ls -la crates/wasm/site/advent_of_code_wasm_bg.wasm

run-devserver:
	cd crates/wasm/site && devserver

watch-and-build-wasm:
	cargo watch -s 'cd crates/wasm && $(WASM_PACK_COMMAND)'

serve-site: site-wasmpack
	make DEBUG_WASM=1 -j run-devserver watch-and-build-wasm

serve-api:
	cd crates/server && cargo run

node-package:
	cd crates/wasm && ./build-package.sh

npm-publish: node-package
	cd crates/wasm/target/nodejs && npm publish

test-python:
	cd crates/python && ./run-tests.sh

install-wasm-pack:
	rustup target add wasm32-unknown-unknown
	curl -sSf -o /tmp/setup-wasm-pack.sh https://rustwasm.github.io/wasm-pack/installer/init.sh && \
		sh /tmp/setup-wasm-pack.sh

fuzz-afl:
	cargo install afl
	cd crates/fuzzing-afl/ && \
		cargo afl build && \
		rm -Rf target/fuzz-findings && \
		mkdir -p target/fuzz-findings && \
		cargo afl fuzz -i testcase-dir -o target/fuzz-findings ../../target/debug/advent-of-code-fuzzing-afl && \
		./process-files-for-upload.sh target/fuzz-findings/crashes

fuzz-hfuzz:
	cargo install honggfuzz
	cd crates/fuzzing-hfuzz/ && cargo hfuzz run advent-of-code-fuzzing-hfuzz

fuzz-libfuzzer:
	cargo install cargo-fuzz
	cd crates/fuzzing-libfuzzer/ && cargo +nightly fuzz run fuzz_target

netlify:
	curl -sSf -o /tmp/rustup.sh https://sh.rustup.rs && \
		sh /tmp/rustup.sh -y && \
		. $(HOME)/.cargo/env && \
		make install-wasm-pack && \
		make site-wasmpack && \
		make site-downloads && \
		make node-package && \
		mkdir crates/wasm/site/cached-${CACHED_COMMIT_REF} && cp crates/wasm/site/*.* crates/wasm/site/cached-${CACHED_COMMIT_REF} && \
		cd crates/wasm/functions && \
		npm install

.PHONY: check install-cargo-deps bench site-downloads site-wasmpack wasm-size run-devserver watch-and-build-wasm serve-site serve-api node-package npm-publish test-python install-wasm-pack fuzz-afl netlify

