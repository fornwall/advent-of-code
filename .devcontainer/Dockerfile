ARG DOCKER_IMAGE="rustlang/rust:nightly-slim"
FROM ${DOCKER_IMAGE}

RUN apt-get update && apt-get install -y bash-completion curl git libssl-dev make pkg-config vim

RUN rustup component add clippy rustfmt rust-src

RUN cargo install cargo-benchcmp devserver

RUN rustup target add wasm32-unknown-unknown

RUN curl --silent --show-error --fail https://rustwasm.github.io/wasm-pack/installer/init.sh | sh -

# Install rust-analyzer binary so that vscode doesn't need to prompt for installation:
RUN mkdir -p /root/.vscode-server/data/User/globalStorage/matklad.rust-analyzer && curl --silent --show-error --fail --location https://github.com/rust-analyzer/rust-analyzer/releases/latest/download/rust-analyzer-linux -o /root/.vscode-server/data/User/globalStorage/matklad.rust-analyzer/rust-analyzer-linux && chmod +x /root/.vscode-server/data/User/globalStorage/matklad.rust-analyzer/rust-analyzer-linux
