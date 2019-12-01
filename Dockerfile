FROM scratch
COPY target/x86_64-unknown-linux-musl/release/advent_of_code_rs /
ENTRYPOINT ["/advent_of_code_rs"]
