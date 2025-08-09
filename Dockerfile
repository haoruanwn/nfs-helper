FROM ivangabriele/tauri:debian-bookworm-20

USER root

# 更新软件源，安装缺失的包，并清理缓存
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        # 之前添加的桌面工具
        xdg-utils \
        # 本次新增: Python 虚拟环境和开发包
        python3-venv \
        python3-dev \
    && \
    rm -rf /var/lib/apt/lists/*