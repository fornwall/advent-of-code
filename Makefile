DOCKER_IMAGE_NAME=fredrikfornwall/advent-of-code-2019-rs
.PHONY: check docker-image publish-docker publish-html publish-npm publish-all

ifeq ($(DEBUG),1)
  wasm_pack_profile=--dev
else
  wasm_pack_profile=--release
endif

check:
	cargo fmt --all
	cargo clippy --all-targets --all-features -- -D warnings
	cargo test

docker-image:
	docker build --no-cache --tag $(DOCKER_IMAGE_NAME) crates/core

publish-docker: docker-image
	docker push $(DOCKER_IMAGE_NAME)

install-wasm-target:
	rustup target add wasm32-unknown-unknown

create-html:
	cd crates/wasm && \
		wasm-pack build $(wasm_pack_profile) --target browser --out-dir target/browser
	cd crates/wasm/site && \
		rm -Rf dist package-lock.json && \
		npm install && \
		webpack --config webpack.config.js
	cd && ln -f -s *.module.wasm module.wasm

node-library:
	cd crates/wasm && wasm-pack build --target nodejs --out-dir target/nodejs

serve-html: create-html
	cd crates/wasm/site/dist && python3 -m http.server

publish-npm:
	cd crates/wasm && ./publish-npm-module.sh

test-python:
	cd crates/python && \
		python3 -m venv env && \
		. env/bin/activate && \
		pip install -r requirements.txt && \
		python setup.py develop && \
		python tests/test_solve.py

publish-all: publish-docker publish-html publish-npm
	@echo Everything published

home:
	echo $(HOME)

netlify:
	curl -sSf -o /tmp/rustup.sh https://sh.rustup.rs && \
		sh /tmp/rustup.sh -y && \
		. $(HOME)/.cargo/env && \
		curl -sSf -o /tmp/setup-wasm-pack.sh https://rustwasm.github.io/wasm-pack/installer/init.sh && \
		sh /tmp/setup-wasm-pack.sh && \
		rustup target add wasm32-unknown-unknown && \
		npm install --save-dev webpack webpack-cli && \
		make create-html && \
		make node-library && \
		cd crates/wasm/functions && npm install

.PHONY: netlify
