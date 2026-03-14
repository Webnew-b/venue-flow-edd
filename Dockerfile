# syntax=docker/dockerfile:1

############################
# Builder
############################
FROM docker.io/library/rust:1.92 AS builder

ARG http_proxy
ARG https_proxy
ARG no_proxy

ENV http_proxy=${http_proxy} \
    https_proxy=${https_proxy} \
    no_proxy=${no_proxy} \
    HTTP_PROXY=${http_proxy} \
    HTTPS_PROXY=${https_proxy} \
    NO_PROXY=${no_proxy}

WORKDIR /app

# BIN_PKG:workspace 里要编译的 package 名（cargo -p 用的）
# BIN_NAME:最终产物二进制文件名（target/release/ 下的文件名）
# BIN_DIR:这个二进制 crate 的目录（用来造 dummy main.rs 以缓存依赖）
ARG BIN_PKG=app
ARG BIN_NAME=infra_app
ARG BIN_DIR=app

COPY Cargo.toml Cargo.lock ./

COPY app/Cargo.toml app/Cargo.toml
COPY domain/Cargo.toml domain/Cargo.toml
COPY domain_core/Cargo.toml domain_core/Cargo.toml
COPY event/Cargo.toml event/Cargo.toml
COPY infra/Cargo.toml infra/Cargo.toml
COPY util_macros/Cargo.toml util_macros/Cargo.toml

RUN mkdir -p ${BIN_DIR}/src \
 && printf "fn main() {}\n" > ${BIN_DIR}/src/main.rs

RUN cargo build -p ${BIN_PKG} --release || true

RUN rm -rf ${BIN_DIR}/src

COPY . .

RUN cargo build -p ${BIN_PKG} --release


############################
# Runtime
############################
FROM docker.io/library/ubuntu:24.04

RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

RUN useradd -m appuser
WORKDIR /app

# 让运行阶段也知道二进制名（与 builder 保持一致）
ARG BIN_NAME=infra_app
ENV BIN_NAME=${BIN_NAME}

# 拷贝二进制到 /app
COPY --from=builder /app/target/release/${BIN_NAME} /app/${BIN_NAME}

# 把 run/config 拷贝到 /app/config（与二进制同目录 /app 下）
COPY --from=builder /app/run/config /app/config

RUN chown -R appuser:appuser /app
RUN chmod 755 /app/${BIN_NAME}

EXPOSE 8090
USER appuser
ENV RUST_LOG=debug
CMD ["sh", "-c", "exec /app/$BIN_NAME"]
