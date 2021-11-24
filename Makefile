NIGHTLY_DATE = 2021-10-22 # Update versions in .github/workflows as well.
NIGHTLY_TOOLCHAIN = nightly-${NIGHTLY_DATE}

CARGO_COMMAND = cargo
CLIPPY_PARAMS = --all-targets -- \
	-W clippy::cargo \
	-W clippy::cast_lossless \
	-W clippy::dbg_macro \
	-W clippy::expect_used \
	-W clippy::manual_filter_map \
	-W clippy::if_not_else \
	-W clippy::items_after_statements \
	-W clippy::linkedlist \
	-W clippy::match_same_arms \
	-W clippy::nursery \
	-W clippy::option_if_let_else \
	-W clippy::redundant_closure_for_method_calls \
	-W clippy::needless_continue \
	-W clippy::needless_pass_by_value \
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

WASM_DIR = debug
WASM_OPT = wasm-opt -all
WASM_BINDGEN = wasm-bindgen --target no-modules
ifeq ($(WASM_RELEASE),1)
  WASM_BUILD_PROFILE = --release
  WASM_DIR = release
  WASM_OPT += -O3
else
  WASM_OPT += -O0
endif

check:
	$(CARGO_COMMAND) fmt --all
	$(CARGO_COMMAND) clippy --tests $(CLIPPY_PARAMS)
	$(CARGO_COMMAND) clippy --lib --bins $(CLIPPY_PARAMS) -D clippy::panic
	if [ -n "${COUNT_ALLOCATIONS}" ]; then $(CARGO_COMMAND) test --features count-allocations; else $(CARGO_COMMAND) test; fi

site-wasm:
	cd crates/wasm && \
	RUSTFLAGS="-C target-feature=+bulk-memory,+mutable-globals,+nontrapping-fptoint,+sign-ext" cargo build $(WASM_BUILD_PROFILE) --target wasm32-unknown-unknown && \
	rm -Rf site/generated && \
	$(WASM_BINDGEN) --out-dir site/generated ../../target/wasm32-unknown-unknown/$(WASM_DIR)/advent_of_code_wasm.wasm && \
	cd site/generated && \
	$(WASM_OPT) -o advent_of_code_wasm_bg.wasm advent_of_code_wasm_bg.wasm && \
	cd ../.. && \
	RUSTFLAGS="-C target-feature=+atomics,+bulk-memory,+mutable-globals,+nontrapping-fptoint,+sign-ext" rustup run $(NIGHTLY_TOOLCHAIN) \
		cargo build $(WASM_BUILD_PROFILE) --target wasm32-unknown-unknown --features visualization -Z build-std=std,panic_abort && \
	$(WASM_BINDGEN) --out-dir site/show/generated ../../target/wasm32-unknown-unknown/$(WASM_DIR)/advent_of_code_wasm.wasm && \
	cd site/show/generated && \
	$(WASM_OPT) -o advent_of_code_wasm_bg.wasm advent_of_code_wasm_bg.wasm

site-pack: site-wasm
	cd crates/wasm/site && \
		rm -Rf dist && \
		npm i && npm run webpack --mode=production && \
		cd show && \
		rm -Rf dist && \
		npm i && npm run webpack --mode=production

wasm-size:
	make WASM_RELEASE=1 site-wasm && \
	ls -la crates/wasm/site/generated/advent_of_code_wasm_bg.wasm

--run-devserver:
	cd crates/wasm/site && NODE_ENV=development npx webpack serve --https

--watch-and-build-wasm:
	cargo watch --ignore crates/wasm/site --shell 'make site-wasm'

serve-site: --run-devserver --watch-and-build-wasm ;

node-package:
	cd crates/wasm && ./build-package.sh

npm-publish: node-package
	cd crates/wasm/target/nodejs && npm publish

test-python:
	cd crates/python && ./run-tests.sh

fuzz-afl:
	cargo install afl
	cd crates/fuzzing-afl/ && \
		cargo afl build && \
		rm -Rf target/fuzz-findings && \
		mkdir -p target/fuzz-findings && \
		cargo afl fuzz -i testcase-dir -o target/fuzz-findings target/debug/advent-of-code-fuzzing-afl && \
		./process-files-for-upload.sh target/fuzz-findings/default/crashes

fuzz-hfuzz:
	cargo install honggfuzz
	cd crates/fuzzing-hfuzz/ && cargo hfuzz run advent-of-code-fuzzing-hfuzz

fuzz-libfuzzer:
	cargo install cargo-fuzz
	cd crates/fuzzing-libfuzzer/ && cargo +$(NIGHTLY_TOOLCHAIN) fuzz run fuzz_target

install-cargo-deps:
	cargo install cargo-benchcmp cargo-watch devserver

install-nightly:
	rustup toolchain install $(NIGHTLY_TOOLCHAIN) && \
	rustup component add --toolchain $(NIGHTLY_TOOLCHAIN) rust-src

install-wasm-bindgen:
	rustup target add wasm32-unknown-unknown
	cargo install wasm-bindgen-cli

netlify: node-package
	cd crates/wasm/functions && \
		npm install && \
		cd .. && \
		netlify deploy --prod

deploy-site:
	make WASM_RELEASE=1 site-pack && \
		curl https://adventofcode.com/favicon.ico > crates/wasm/site/dist/favicon_1.ico && \
		cd crates/wasm && \
		rm -Rf aoc.fornwall.net && \
		git clone -b site git@github.com:fornwall/aoc.fornwall.net.git && \
		cd aoc.fornwall.net && \
		rm -Rf * && \
		cp -Rf ../site/dist/* . && \
		mv ../site/show/dist/ show/ && \
		cp ../site/show/*.mp4 ../site/show/*.svg show/ && \
		cp -Rf ../site/api/ api/ && \
		cp -Rf ../site/benchmark/ benchmark/ && \
		git add . && \
		git commit -m "Update site: ${GITHUB_SHA}" && \
		git push

.PHONY: check install-cargo-deps site-wasm site-pack wasm-size --run-devserver --watch-and-build-wasm serve-site node-package npm-publish test-python install-wasm-bindgen fuzz-afl fuzz-hfuzz fuzz-libfuzzer install-nightly netlify deploy-site
