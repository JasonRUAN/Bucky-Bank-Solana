# 使用多阶段构建来减小镜像大小
FROM rust:latest AS builder

# 安装构建依赖
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# 设置工作目录
WORKDIR /app

# 复制Cargo文件以利用Docker缓存
COPY Cargo.toml Cargo.lock ./

# 创建虚拟main.rs来预编译依赖
RUN mkdir src && echo "fn main() {}" > src/main.rs

# 构建依赖
RUN cargo build --release

# 复制源代码
COPY src ./src
# COPY migrations ./migrations
# COPY config.toml ./

# 构建应用
RUN cargo build --release --package bucky_bank_indexer --bin bucky_bank_indexer 

# 运行时镜像
# FROM debian:bullseye-slim
FROM  ubuntu:25.10

# 安装运行时依赖
RUN apt update && apt install -y --no-install-recommends \
    ca-certificates tzdata vim jq curl telnet iputils-ping && \
    rm -rf /var/lib/apt/lists/*

# 创建非root用户
RUN useradd -m -u 10000 appuser

# 设置工作目录
WORKDIR /app

# 从构建阶段复制二进制文件
COPY --from=builder /app/target/release/bucky_bank_indexer /app/
#COPY --from=builder /app/migrations /app/migrations
#COPY --from=builder /app/config.toml /app/

# 复制启动脚本
COPY run.sh /app/

# 设置权限
RUN chown -R appuser:appuser /app && \
    chmod +x /app/bucky_bank_indexer

# 切换到非root用户
USER appuser

# 暴露端口
EXPOSE 8080

# 健康检查
# HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
#     CMD curl -f http://localhost:8080/health || exit 1

# 设置环境变量
ENV RUST_LOG=info

# 启动应用
# CMD ["./bucky_bank_indexer", "--config", "config.toml", "index"]
CMD ["./bucky_bank_indexer", "index"]