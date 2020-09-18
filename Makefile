ifeq ($(DEBUG),1)
  wasm_pack_profile=--dev
else
  wasm_pack_profile=--release
endif

CLIPPY_COMMAND = cargo clippy --all-targets --all-features -- -D warnings -W clippy::cargo -W clippy::nursery
ifeq ($(CLIPPY_PEDANTIC),1)
  CLIPPY_COMMAND += -W clippy::pedantic
endif

check:
	cargo fmt --all
	$(CLIPPY_COMMAND)
	cargo test

install-wasm-target:
	rustup target add wasm32-unknown-unknown

site:
	cd crates/wasm && \
		wasm-pack build $(wasm_pack_profile) --target web --out-dir target/web && \
		ln -f index.html target/web/index.html && \
		ln -f index.js target/web/index.js

serve-site: site
	cd crates/wasm/target/web && devserver

node-library:
	cd crates/wasm && wasm-pack build --target nodejs --out-dir target/nodejs

publish-npm:
	cd crates/wasm && ./publish-npm-module.sh

test-python:
	cd crates/python && \
		python3 -m venv env && \
		. env/bin/activate && \
		pip install -r requirements.txt && \
		python setup.py develop && \
		python tests/test_solve.py

netlify:
	curl -sSf -o /tmp/rustup.sh https://sh.rustup.rs && \
		sh /tmp/rustup.sh -y && \
		. $(HOME)/.cargo/env && \
		curl -sSf -o /tmp/setup-wasm-pack.sh https://rustwasm.github.io/wasm-pack/installer/init.sh && \
		sh /tmp/setup-wasm-pack.sh && \
		rustup target add wasm32-unknown-unknown && \
		make site && \
		make node-library && \
		cd crates/wasm/functions && npm install

.PHONY: check install-wasm-target create-html node-library serve-html publish-npm test-python netlify
