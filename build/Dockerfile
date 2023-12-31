FROM ubuntu:24.04 AS base
RUN apt update
RUN apt install -y make
RUN apt install -y curl

# https://stackoverflow.com/a/52445962
# install gcc toolchain for rust linker `cc` not found
RUN apt install -y build-essential

FROM base AS environment
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

FROM environment AS workspace
WORKDIR /app