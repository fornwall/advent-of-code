DOCKER_IMAGE_NAME=fredrikfornwall/advent-of-code-2019-rs
.PHONY: check docker-image publish-docker publish-html publish-npm publish-all

check:
	cargo fmt --all
	cargo clippy --all-targets --all-features -- -D warnings
	cargo test

docker-image:
	docker build --no-cache --tag $(DOCKER_IMAGE_NAME) crates/core

publish-docker: docker-image
	docker login
	docker push $(DOCKER_IMAGE_NAME)

install-wasm-target:
	rustup target add wasm32-unknown-unknown

create-html:
	cd crates/wasm && wasm-pack build --target browser --out-dir target/browser
	cd crates/wasm/wasm/html && \
		rm -Rf dist node_modules package-lock.json && \
		npm install && \
		webpack --config webpack.config.js
	rm -Rf docs/
	mkdir -p docs/
	cp crates/wasm/wasm/html/dist/* docs/
	cp crates/wasm/run.ts docs/
	cd docs/ && ln -s *.module.wasm module.wasm
	@echo docs/ folder has been generated

serve-html: create-html
	cd crates/wasm/wasm/html/dist && python3 -m http.server

publish-npm:
	./wasm/publish-npm-module.sh

test-python:
	cd crates/python && \
		python3 -m venv env && \
		. env/bin/activate && \
		pip install -r requirements.txt && \
		python setup.py develop && \
		python tests/test_solve.py

publish-all: publish-docker publish-html publish-npm
	@echo Everything published
