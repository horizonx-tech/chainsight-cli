FROM --platform=linux/amd64 ubuntu:22.04

ENV RUSTUP_HOME=/opt/rustup
ENV CARGO_HOME=/opt/cargo
ENV RUST_VERSION=1.73.0
ENV DFX_VERSION=0.14.1
# NOTE: fail when deployment with ver >0.14.1 - ~0.15
ENV NODE_VERSION=20.8.0
ENV YARN_VERSION=1.22.19

## dfx
# Install a basic environment needed for our build tools
RUN apt -yq update && \
    apt -yqq install --no-install-recommends curl ca-certificates \
        build-essential pkg-config libssl-dev llvm-dev liblmdb-dev clang cmake rsync \
        wget \
        libunwind8

# Install Rust and Cargo
ENV PATH=/opt/cargo/bin:${PATH}
RUN curl --fail https://sh.rustup.rs -sSf \
        | sh -s -- -y --default-toolchain ${RUST_VERSION}-x86_64-unknown-linux-gnu --no-modify-path
RUN rustup default ${RUST_VERSION}-x86_64-unknown-linux-gnu
RUN rustup target add wasm32-unknown-unknown

# Install dfx
RUN sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"

## nodejs
# install nodejs,yarn
RUN curl --compressed "https://nodejs.org/dist/v$NODE_VERSION/node-v$NODE_VERSION-linux-x64.tar.xz" > /tmp/node.tar.xz && \
  tar -xJf /tmp/node.tar.xz -C /usr/local --strip-components=1 && \
  ln -s /usr/local/bin/node /usr/local/bin/nodejs && \
  rm /tmp/node.tar.xz
RUN curl -L --compressed "https://yarnpkg.com/downloads/$YARN_VERSION/yarn-v$YARN_VERSION.tar.gz" > /tmp/yarn.tar.gz && \
  tar -xzf /tmp/yarn.tar.gz -C /opt && \
  ln -s /opt/yarn-v$YARN_VERSION/bin/yarn /usr/local/bin/yarn && \
  ln -s /opt/yarn-v$YARN_VERSION/bin/yarnpkg /usr/local/bin/yarnpkg && \
  rm /tmp/yarn.tar.gz

# Bring in codes, manifests etc
COPY ./workspace /workspace
WORKDIR /workspace

# Install hardhat
RUN yarn add --dev hardhat

EXPOSE 8545
EXPOSE 4943

RUN chmod 744 ./bootstrap.sh
CMD ["./bootstrap.sh"]
