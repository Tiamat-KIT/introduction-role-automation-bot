# ベースイメージとしてRustの公式イメージを使用
FROM rust:latest

# 必要なパッケージのインストール
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    git \
    curl \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# cargo-shuttleのインストール
RUN cargo install cargo-shuttle
WORKDIR /app
