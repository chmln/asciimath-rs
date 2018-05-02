# Set the base image
FROM rustlang/rust:nightly
# Dockerfile author / maintainer
MAINTAINER Name <gregory.mkv@gmail.com>

RUN cargo install cargo-tarpaulin
