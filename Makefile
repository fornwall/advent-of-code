ifeq ($(DEBUG),1)
  wasm_pack_profile=--dev
else
  wasm_pack_profile=--release
endif

CLIPPY_PARAMS = --all-features -- -D warnings -W clippy::cargo -W clippy::nursery -D clippy::expect_used -D clippy::unwrap_used -D clippy::items_after_statements -D clippy::if_not_else -D clippy::trivially_copy_pass_by_ref -D clippy::match_same_arms
ifeq ($(CLIPPY_PEDANTIC),1)
  CLIPPY_PARAMS += -W clippy::pedantic
endif

check:
	cargo fmt --all
	cargo clippy --all-targets $(CLIPPY_PARAMS)
	cargo clippy $(CLIPPY_PARAMS) -D clippy::panic
	cargo test

site:
	cd crates/wasm && \
		wasm-pack build $(wasm_pack_profile) --target web --out-dir target/web && \
		ln -f site/index.html target/web/index.html && \
		curl https://unpkg.com/picnic@6.5.3/picnic.min.css > target/web/picnic-6.5.3.min.css && \
		curl https://adventofcode.com/favicon.ico > target/web/favicon.ico && \
		ln -f site/index.js target/web/index.js && \
		ln -f site/openapi.json target/web/openapi.json

wasm-size: site
	ls -la crates/wasm/target/web/advent_of_code_wasm_bg.wasm

serve-site: site
	cd crates/wasm/target/web && devserver

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
	cd crates/fuzzing-afl/ && ./fuzz.sh

netlify:
	curl -sSf -o /tmp/rustup.sh https://sh.rustup.rs && \
		sh /tmp/rustup.sh -y && \
		. $(HOME)/.cargo/env && \
		make install-wasm-pack && \
		make site && \
		make node-package && \
		cd crates/wasm/functions && npm install

.PHONY: check site wasm-size serve-site serve-api node-package npm-publish test-python install-wasm-pack fuzz-afl netlify
