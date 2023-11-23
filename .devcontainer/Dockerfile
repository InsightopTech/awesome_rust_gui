# 使用Rust官方提供的Docker镜像作为基础镜像
FROM rust:latest

# 安装常用的构建工具和依赖项
RUN apt-get update && \
    apt-get install -y \
    build-essential \
    cmake \
    git \
    libssl-dev \
    pkg-config

# 设置工作目录
WORKDIR /app

# 将本地代码复制到容器中
COPY . .

# 构建应用程序
RUN cargo build

# 运行应用程序
CMD ["cargo", "run"]

