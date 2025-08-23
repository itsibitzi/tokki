#/usr/bin/env bash
SCRIPT_PATH=$( cd "$(dirname "$0")" || exit ; pwd -P )

docker build -t tokki-linux-build -f "SCRIPT_PATH/../build/build.Dockerfile" .

docker run --rm \
    -e RUSTFLAGS="-C target-feature=+aes,+neon". \
    -v $(pwd):/out \
    tokki-linux-build \
    bash -c 'cargo deb && cp /app/target/debian/tokki_0.1.0_arm64.deb /out/.'
