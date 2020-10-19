ifeq ($(DEBUG),1)
  wasm_pack_profile=--dev
else
  wasm_pack_profile=--release
endif

CLIPPY_PARAMS =  --all-features -- -D warnings -W clippy::cargo -W clippy::nursery -D clippy::expect_used -D clippy::unwrap_used -D clippy::items_after_statements -D clippy::if_not_else -D clippy::trivially_copy_pass_by_ref -D clippy::match_same_arms
CLIPPY_CARGO = cargo
ifeq ($(CLIPPY_NIGHTLY),1)
  CLIPPY_CARGO += +nightly
  CLIPPY_PARAMS := --benches $(CLIPPY_PARAMS)
endif
ifeq ($(CLIPPY_PEDANTIC),1)
  CLIPPY_PARAMS += -W clippy::pedantic
endif

check:
	$(CLIPPY_CARGO) fmt --all
	$(CLIPPY_CARGO) clippy --tests $(CLIPPY_PARAMS)
	$(CLIPPY_CARGO) clippy --lib --bins $(CLIPPY_PARAMS) -D clippy::panic
	$(CLIPPY_CARGO) test

bench:
	cargo +nightly bench

site:
	cd crates/wasm && \
		wasm-pack build $(wasm_pack_profile) --target no-modules --out-dir target/web && \
		curl https://unpkg.com/picnic@6.5.3/picnic.min.css > target/web/picnic-6.5.3.min.css && \
		curl https://adventofcode.com/favicon.ico > target/web/favicon.ico && \
		for i in site/*; do ln -f $$i target/web/`basename $$i`; done

wasm-size: site
	ls -la crates/wasm/target/web/advent_of_code_wasm_bg.wasm

serve-site: site
	cd crates/wasm/target/web && devserver --address 192.168.2.30:8080

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

.PHONY: bench check site wasm-size serve-site serve-api node-package npm-publish test-python install-wasm-pack fuzz-afl netlify
