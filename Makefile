NIGHTLY_DATE = 2021-03-01 # Update versions in .github/workflows as well.
NIGHTLY_TOOLCHAIN = nightly-${NIGHTLY_DATE}

CARGO_COMMAND = cargo
CLIPPY_PARAMS = --all-targets -- \
	-W clippy::cargo \
	-W clippy::cast_lossless \
	-W clippy::expect_used \
	-W clippy::filter_map \
	-W clippy::if_not_else \
	-W clippy::items_after_statements \
	-W clippy::linkedlist \
	-W clippy::match_same_arms \
	-W clippy::nursery \
	-W clippy::option_if_let_else \
	-W clippy::redundant_closure_for_method_calls \
	-W clippy::similar_names \
	-W clippy::trivially_copy_pass_by_ref \
	-W clippy::unreadable-literal \
	-W clippy::unseparated-literal-suffix \
	-W clippy::unwrap_used
ifeq ($(NIGHTLY),1)
  CARGO_COMMAND += +$(NIGHTLY_TOOLCHAIN)
endif
ifeq ($(CLIPPY_PEDANTIC),1)
  CLIPPY_PARAMS += -W clippy::pedantic
else ifneq ($(NIGHTLY),1)
  CLIPPY_PARAMS += -D warnings
endif

ifeq ($(WASM_RELEASE),1)
  wasm_pack_profile = --release
else
  wasm_pack_profile = --dev
endif
WASM_PACK_COMMAND = wasm-pack \
	build $(wasm_pack_profile) \
	--target no-modules \
	--out-dir site
WASM_PACK_COMMAND_VISUALIZER = RUSTFLAGS="-C target-feature=+atomics,+bulk-memory,+mutable-globals" \
	rustup run $(NIGHTLY_TOOLCHAIN) \
	wasm-pack build \
	$(wasm_pack_profile) \
	--target no-modules \
	--out-dir site/show \
	-- --features visualization -Z build-std=std,panic_abort

check:
	$(CARGO_COMMAND) fmt --all
	$(CARGO_COMMAND) clippy --tests $(CLIPPY_PARAMS)
	$(CARGO_COMMAND) clippy --lib --bins $(CLIPPY_PARAMS) -D clippy::panic
	$(CARGO_COMMAND) test

install-cargo-deps:
	cargo install cargo-benchcmp cargo-watch devserver

bench:
	cargo +$(NIGHTLY_TOOLCHAIN) bench

site-downloads:
	cd crates/wasm && \
		curl https://adventofcode.com/favicon.ico > site/favicon.ico

site-pack:
	cd crates/wasm && \
		$(WASM_PACK_COMMAND) && \
		cd site && \
		webpack --mode=production

site-pack-visualization:
	cd crates/wasm && $(WASM_PACK_COMMAND_VISUALIZER)

wasm-size: site-pack
	ls -la crates/wasm/site/advent_of_code_wasm_bg.wasm

run-devserver:
	cd crates/wasm/site && NODE_ENV=development webpack serve

watch-and-build-wasm:
	cargo watch -s 'cd crates/wasm && $(WASM_PACK_COMMAND)'

watch-and-build-wasm-visualization:
	cargo watch -s 'cd crates/wasm && $(WASM_PACK_COMMAND_VISUALIZER)'

serve-site:
	make -j run-devserver watch-and-build-wasm

serve-site-visualization:
	make -j run-devserver watch-and-build-wasm-visualization

serve-api:
	cd crates/server && cargo run

node-package:
	which wasm-opt
	wasm-opt --version
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
		cargo afl fuzz -i testcase-dir -o target/fuzz-findings target/debug/advent-of-code-fuzzing-afl && \
		./process-files-for-upload.sh target/fuzz-findings/crashes

fuzz-hfuzz:
	cargo install honggfuzz
	cd crates/fuzzing-hfuzz/ && cargo hfuzz run advent-of-code-fuzzing-hfuzz

fuzz-libfuzzer:
	cargo install cargo-fuzz
	cd crates/fuzzing-libfuzzer/ && cargo +$(NIGHTLY_TOOLCHAIN) fuzz run fuzz_target

install-nightly:
	rustup toolchain install $(NIGHTLY_TOOLCHAIN) && \
	rustup component add --toolchain $(NIGHTLY_TOOLCHAIN) rust-src

netlify:
	npm install -g webpack webpack-cli && \
		make WASM_RELEASE=1 site-pack && \
		make WASM_RELEASE=1 site-pack-visualization && \
		make site-downloads && \
		make node-package && \
		cd crates/wasm/functions && \
		npm install && \
		cd .. && \
		netlify deploy --prod

.PHONY: check install-cargo-deps bench site-downloads site-pack wasm-size run-devserver watch-and-build-wasm serve-site serve-api node-package npm-publish test-python install-wasm-pack fuzz-afl netlify

