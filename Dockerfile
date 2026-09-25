FROM rust:1.98-bookworm AS build
WORKDIR /src
COPY Cargo.toml Cargo.lock rust-toolchain.toml ./
COPY src ./src
RUN cargo build --release --locked

FROM debian:bookworm-slim
# ca-certificates is not in slim, and the server verifies TLS to TypeSafe and the
# optional LLM host with the system trust store (rustls-native-certs).
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --system --home /var/lib/jev-tree --create-home jev
WORKDIR /var/lib/jev-tree
COPY --from=build /src/target/release/jev-tree /usr/local/bin/jev-tree
COPY data/seed.json /opt/jev-tree/data/seed.json
COPY static /opt/jev-tree/static
ENV JEV_TREE_DB=/var/lib/jev-tree/demo.db \
    JEV_TREE_SEED=/opt/jev-tree/data/seed.json \
    JEV_TREE_STATIC_DIR=/opt/jev-tree/static \
    JEV_TREE_BIND=0.0.0.0 \
    PORT=8768
# The container binds every interface, so the server refuses to start in open mode.
# Supply a key, or accept the risk explicitly:
#   docker run -e JEV_TREE_INIT_SERVER_KEY=<6+ chars> -v jev-tree:/var/lib/jev-tree -p 8768:8768 jev-tree
#   docker run -e JEV_TREE_ALLOW_OPEN=1 ...
# The volume must persist both demo.db and the sibling .jev-tree.key/.salt files.
VOLUME ["/var/lib/jev-tree"]
USER jev
EXPOSE 8768
CMD ["jev-tree"]
