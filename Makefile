NIGHTLY_TOOLCHAIN ?= nightly

CARGO_COMMAND = cargo

ifeq ($(NIGHTLY),1)
  CARGO_COMMAND += +$(NIGHTLY_TOOLCHAIN)
endif

WASM_DIR = debug
WASM_OPT = wasm-opt --all-features --disable-gc
WASM_BINDGEN = wasm-bindgen --target web --weak-refs --reference-types
WASM_TARGET_FEATURES = "+bulk-memory,+mutable-globals,+nontrapping-fptoint,+sign-ext,+simd128,+reference-types"
ifeq ($(WASM_RELEASE),1)
  WASM_BUILD_PROFILE = --release
  WASM_DIR = release
  WASM_OPT += -O3
else
  WASM_OPT += -O0
endif

ifeq ($(SITE_DEV_RENDER),1)
  WASM_MAKE_TARGET = site-renderer-wasm
else
  WASM_MAKE_TARGET = site-compute-wasm
endif

check:
	$(CARGO_COMMAND) fmt --all
	$(CARGO_COMMAND) clippy --all-targets
	$(CARGO_COMMAND) clippy --all-targets --features webgpu-compute
	cd crates/core && $(CARGO_COMMAND) clippy --features visualization --tests
	$(CARGO_COMMAND) clippy --lib --bins --all-targets -- -D clippy::panic
	if [ -n "${COUNT_ALLOCATIONS}" ]; then $(CARGO_COMMAND) test --features count-allocations; else $(CARGO_COMMAND) test; fi

check-simd:
	cd crates/core && \
		cargo +nightly clippy --features simd --lib --bins && \
		cargo +nightly test --features simd

check-site:
	cd crates/wasm && npx prettier --write . && npx eslint . --ext .js && npx prettier --check .

site-compute-wasm:
	cd crates/wasm && \
	RUSTFLAGS="-C target-feature=$(WASM_TARGET_FEATURES)" cargo +nightly build $(WASM_BUILD_PROFILE) --target wasm32-unknown-unknown --features simd && \
	rm -Rf site/generated && \
	$(WASM_BINDGEN) --out-dir site/generated ../../target/wasm32-unknown-unknown/$(WASM_DIR)/advent_of_code_wasm.wasm && \
	cd site/generated && \
	$(WASM_OPT) -o advent_of_code_wasm_bg.wasm advent_of_code_wasm_bg.wasm

site-renderer-wasm:
	cd crates/wasm && \
	RUSTFLAGS="-C target-feature=$(WASM_TARGET_FEATURES)" \
		cargo +nightly build $(WASM_BUILD_PROFILE) --target wasm32-unknown-unknown --features visualization && \
	$(WASM_BINDGEN) --out-dir site/show/generated ../../target/wasm32-unknown-unknown/$(WASM_DIR)/advent_of_code_wasm.wasm && \
	cd site/show/generated && \
	$(WASM_OPT) -o advent_of_code_wasm_bg.wasm advent_of_code_wasm_bg.wasm

--download-bootstrap-css:
	cd crates/wasm/site/static/ && curl -O https://cdnjs.cloudflare.com/ajax/libs/twitter-bootstrap/5.3.2/css/bootstrap.min.css

site-pack: site-compute-wasm site-renderer-wasm --download-bootstrap-css
	cd crates/wasm/site && \
		rm -Rf dist && \
		npm i && NODE_ENV=production npm run webpack -- --mode=production

wasm-size:
	$(MAKE) WASM_RELEASE=1 site-compute-wasm && \
	ls -la crates/wasm/site/generated/advent_of_code_wasm_bg.wasm

--run-devserver: --download-bootstrap-css
	cd crates/wasm/site && NODE_ENV=development npx webpack serve --server-type http --open

--watch-and-build-wasm:
	cargo watch --ignore crates/wasm/site --shell '$(MAKE) $(WASM_MAKE_TARGET)'

serve-site: --run-devserver --watch-and-build-wasm ;

node-package:
	cd crates/wasm && ./build-package.sh

npm-publish: node-package
	cd crates/wasm/target/nodejs && npm publish

test-python:
	cd crates/python && ./run-tests.sh

fuzz-afl:
	cargo install cargo-afl
	cd crates/fuzzing-afl/ && \
		cargo afl build && \
		rm -Rf target/fuzz-findings && \
		mkdir -p target/fuzz-findings && \
		cargo afl fuzz -i testcase-dir -o target/fuzz-findings target/debug/advent-of-code-fuzzing-afl --max_total_time=1200 && \
		./process-files-for-upload.sh target/fuzz-findings/default/crashes

fuzz-hfuzz:
	cargo install honggfuzz
	cd crates/fuzzing-hfuzz/ && cargo hfuzz run advent-of-code-fuzzing-hfuzz

fuzz-libfuzzer:
	cargo install cargo-fuzz
	cd crates/fuzzing-libfuzzer/ && cargo +$(NIGHTLY_TOOLCHAIN) fuzz run fuzz_target -- -max_total_time=1200 -only_ascii=1 -report_slow_units=120

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
	$(MAKE) WASM_RELEASE=1 site-pack && \
		curl https://adventofcode.com/favicon.ico > crates/wasm/site/dist/favicon.ico && \
		cd crates/wasm && \
		rm -Rf aoc.fornwall.net && \
		mkdir aoc.fornwall.net && \
		cd aoc.fornwall.net && \
		cp -Rf ../site/dist/* . && \
		cp -Rf ../site/api/ api/ && \
		cp -Rf ../site/benchmark/ benchmark/ && \
		touch robots.txt

test-c-and-cxx-bindings:
	cd crates/cbindings && ./test-example.sh
	cd crates/cxxbindings && ./test-example.sh

man-page:
	mkdir -p target/man
	pandoc --standalone --to man crates/core/MANPAGE.md -o target/man/advent-of-code.1

.PHONY: check install-cargo-deps site-compute-wasm site-renderer-wasm site-pack wasm-size --run-devserver --watch-and-build-wasm serve-site node-package npm-publish test-python install-wasm-bindgen fuzz-afl fuzz-hfuzz fuzz-libfuzzer install-nightly netlify deploy-site test-cbindings manpage
