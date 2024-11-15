FROM --platform=$BUILDPLATFORM rust:1.81.0 AS buildbase

WORKDIR /src

RUN <<EOT bash
    set -ex
    apt-get update
    apt-get install -y \
        git \
        clang
    rustup target add wasm32-wasi
EOT

FROM buildbase AS build

COPY Cargo.toml .

COPY src ./src

RUN cargo build --target wasm32-wasi --release

FROM scratch

COPY --from=build /src/target/wasm32-wasi/release/file_handling_rust.wasm .

ENTRYPOINT [ "./file_handling_rust.wasm" ]
