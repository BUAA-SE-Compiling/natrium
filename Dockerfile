FROM rust:1.48-alpine
RUN if [ -z "$CI" ]; then sed -i 's/dl-cdn.alpinelinux.org/mirrors.tuna.tsinghua.edu.cn/g' /etc/apk/repositories; fi
RUN apk add --no-cache gcc libgcc build-base
WORKDIR /app
RUN if [ -z "$CI" ]; then \
    mkdir -p ./.cargo && \
    echo -e '[source.crates-io]\nreplace-with = "ustc"\n[source.ustc]\nregistry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"' > ./.cargo/config.toml;\
    fi
RUN mkdir -p src/bin && \
    echo "fn main() {println!(\"if you see this, the build broke\")}" > src/lib.rs 
COPY crates ./crates
COPY web ./web
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch --locked
RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs && \
    echo "fn main() {println!(\"if you see this, the build broke\")}" > src/bin/r0vm.rs 
RUN cargo build --release --locked
COPY src ./
RUN cargo build --release --locked --frozen
