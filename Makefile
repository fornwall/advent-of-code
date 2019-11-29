.PHONY: static-linux-executable docker-image publish-docker

check:
	cargo fmt --all
	cargo clippy --all-targets --all-features -- -D warnings
	cargo test

static-linux-executable:
	docker run --rm -v $(PWD):/build fredrikfornwall/rust-static-builder

docker-image: static-linux-executable
	docker build -t fredrikfornwall/advent-of-code-2019-rs .

publish-docker: docker-image
	docker push fredrikfornwall/advent-of-code-2019-rs

install-wasm-target:
	rustup target add wasm32-unknown-unknown
