FROM rust:1.67.1@sha256:02a53e734724bef4a58d856c694f826aa9e7ea84353516b76d9a6d241e9da60e as builder

ENV TARGET=x86_64-unknown-linux-musl
RUN rustup target add ${TARGET}

RUN rm -f /etc/apt/apt.conf.d/docker-clean; echo 'Binary::apt::APT::Keep-Downloaded-Packages "true";' > /etc/apt/apt.conf.d/keep-cache

# borrowed (Ba Dum Tss!) from
# https://github.com/pablodeymo/rust-musl-builder/blob/7a7ea3e909b1ef00c177d9eeac32d8c9d7d6a08c/Dockerfile#L48-L49
RUN --mount=type=cache,target=/var/cache/apt --mount=type=cache,target=/var/lib/apt \
    apt-get update && \
    apt-get --no-install-recommends install -y \
    build-essential \
    musl-dev \
    musl-tools

# The following block
# creates an empty app, and we copy in Cargo.toml and Cargo.lock as they represent our dependencies
# This allows us to copy in the source in a different layer which in turn allows us to leverage Docker's layer caching
# That means that if our dependencies don't change rebuilding is much faster
WORKDIR /build
RUN cargo new jwt-decode
WORKDIR /build/jwt-decode
COPY Cargo.toml Cargo.lock ./
RUN --mount=type=cache,id=cargo-only,target=/build/jwt-decode/target \
    cargo build --release --target ${TARGET}

# now we copy in the source which is more prone to changes and build it
COPY src ./src
# --release not needed, it is implied with install
RUN --mount=type=cache,id=full-build,target=/build/jwt-decode/target \
    cargo install --path . --target ${TARGET} --root /output

FROM alpine:3.17.2@sha256:8a81a7c2af9caf25aa960c6ef70e198b2d7194841546d3b26c9a1eb308a360b3

RUN addgroup -S appgroup && adduser -S appuser -G appgroup
USER appuser

WORKDIR /app
COPY --from=builder /output/bin/jwt-decode /app
ENTRYPOINT ["/app/jwt-decode"]
