FROM scratch
ARG TARGETPLATFORM
COPY target/$TARGETPLATFORM/advent-of-code-server /advent-of-code-server
EXPOSE 8080
ENTRYPOINT ["/advent-of-code-server"]
