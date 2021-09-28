FROM scratch
ARG TARGETPLATFORM
COPY target/$TARGETPLATFORM/advent-of-code-grpc-server /advent-of-code-grpc-server
EXPOSE 50051
ENTRYPOINT ["/advent-of-code-grpc-server"]
