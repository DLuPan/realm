FROM debian:bullseye-slim

COPY target/x86_64-unknown-linux-musl/release/xtunnel /usr/bin
RUN chmod +x /usr/bin/xtunnel

ENTRYPOINT ["/usr/bin/xtunnel"]