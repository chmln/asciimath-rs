# Set the base image
FROM rustlang/rust:nightly-slim
# Dockerfile author / maintainer
MAINTAINER Name <gregory.mkv@gmail.com>

RUN cargo install cargo-tarpaulin
