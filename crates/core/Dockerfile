FROM fredrikfornwall/rust-static-builder:latest
ADD . /build/
RUN sed -i"" 's/visualization =/#/' /build/Cargo.toml
RUN sed -i"" 's/advent-of-code-painter =/#/' /build/Cargo.toml
RUN /root/build.sh
RUN ls -lha /build/target/x86_64-unknown-linux-musl
RUN ls -lha /build/target/x86_64-unknown-linux-musl/release/

FROM scratch
COPY --from=0 /build/target/x86_64-unknown-linux-musl/release/advent-of-code /
ENTRYPOINT ["/advent-of-code"]
