FROM artichoke-ci-base:latest

COPY --chown=artichoke:artichoke . /app

RUN cd /app && cargo build --target wasm32-unknown-unknown
