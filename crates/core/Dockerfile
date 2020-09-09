FROM fredrikfornwall/rust-static-builder:1.46.0
ADD . /build/
RUN /root/build.sh
RUN ls -lha /build/target/x86_64-unknown-linux-musl
RUN ls -lha /build/target/x86_64-unknown-linux-musl/release/

FROM scratch
COPY --from=0 /build/target/x86_64-unknown-linux-musl/release/advent_of_code_rs /
ENTRYPOINT ["/advent_of_code_rs"]
