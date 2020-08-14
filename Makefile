DOCKER_IMAGE_NAME=fredrikfornwall/advent-of-code-2019-rs
.PHONY: check docker-image publish-docker publish-html publish-npm publish-all

check:
	cargo fmt --all
	cargo clippy --all-targets --all-features -- -D warnings
	cargo test

docker-image:
	docker build --no-cache --tag $(DOCKER_IMAGE_NAME) .

publish-docker: docker-image
	docker login
	docker push $(DOCKER_IMAGE_NAME)

install-wasm-target:
	rustup target add wasm32-unknown-unknown

create-html:
	wasm-pack build --target browser --out-dir target/browser
	cd wasm/html && \
		rm -Rf dist node_modules package-lock.json && \
		npm install && \
		webpack --config webpack.config.js
	rm -Rf docs/
	mkdir -p docs/
	cp wasm/html/dist/* docs/
	cp run.ts docs/
	cd docs/ && ln -s *.module.wasm module.wasm
	@echo docs/ folder has been generated

serve-html: create-html
	cd wasm/html/dist && python3 -m http.server

publish-npm:
	./wasm/publish-npm-module.sh

publish-all: publish-docker publish-html publish-npm
	@echo Everything published
