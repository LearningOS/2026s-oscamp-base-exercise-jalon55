FROM rust:latest

RUN curl -fsSL https://code-server.dev/install.sh | sh \
  && code-server --install-extension cnbcool.cnb-welcome \
  && code-server --install-extension rust-lang.rust-analyzer \
  && echo done

RUN apt-get update && apt-get install -y \
    git wget unzip openssh-server \
    jq gcc-riscv64-linux-gnu qemu-user-static

RUN rustup target add riscv64gc-unknown-linux-gnu

ENV LANG C.UTF-8
ENV LANGUAGE C.UTF-8
