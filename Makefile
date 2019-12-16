.PHONY: check static-linux-executable docker-image publish-docker publish-html publish-npm publish-all

check:
	cargo fmt --all
	cargo clippy --all-targets --all-features -- -D warnings
	cargo test

static-linux-executable:
	docker run --rm -v $(PWD):/build fredrikfornwall/rust-static-builder

docker-image: static-linux-executable
	docker build -t fredrikfornwall/advent-of-code-2019-rs .

publish-docker: docker-image
	docker login
	docker push fredrikfornwall/advent-of-code-2019-rs

install-wasm-target:
	rustup target add wasm32-unknown-unknown

create-html:
	wasm-pack build --target browser --out-dir target/browser
	cd wasm/html && \
		rm -Rf dist node_modules package-lock.json && \
		npm install && \
		webpack --config webpack.config.js
	@echo wasm/html/dist/ has been created

serve-html: create-html
	cd wasm/html/dist && python3 -m http.server

publish-html: create-html
	scp -r wasm/html/dist/* fornwall.net:www/advent-of-code-2019/
	scp wasm/html/dist/*.wasm fornwall.net:www/advent-of-code-2019/solution.wasm

publish-npm:
	./wasm/publish-npm-module.sh

publish-all: publish-docker publish-html publish-npm
	@echo Everything published
