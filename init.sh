#!/bin/bash

# --- 初始化开发环境脚本 for nfs-helper ---

# set -e: 脚本中任何命令返回非零退出码时，立即退出。
# set -u: 尝试使用未设置的变量时，立即退出。
# set -o pipefail: 管道中的任何命令失败，整个管道的退出码为非零。
set -euo pipefail

# --- 步骤 1: 检查核心依赖 (Node.js 和 Rust/Cargo) ---
echo "--- Step 1: Checking for required dependencies (Node.js, Cargo)..."

# 检查 Node.js
if command -v node &> /dev/null; then
    echo "Node.js found. Version: $(node -v)"
else
    echo "错误：未找到 Node.js。请先安装 Node.js (推荐 v18 或更高版本)。"
    exit 1
fi

# 检查 Cargo
if command -v cargo &> /dev/null; then
    echo "Cargo (Rust) found. Version: $(cargo --version)"
else
    echo "错误：未找到 Cargo。请按照 https://rustup.rs/ 的指引安装 Rust 工具链。"
    exit 1
fi

echo "依赖检查通过！"
echo ""


# --- 步骤 2: 安装前端依赖 ---
echo "--- Step 2: Installing frontend dependencies using pnpm..."

# 检查 pnpm 是否存在
if ! command -v pnpm &> /dev/null; then
    echo "错误：未找到 pnpm。请先通过 'npm install -g pnpm' 安装。"
    exit 1
fi

pnpm install
echo "前端依赖安装完成！"
echo ""


# --- 步骤 3: 初始化Python虚拟环境并安装依赖 ---
echo "--- Step 3: Setting up Python virtual environment for backend scripts..."

# 切换到后端目录
cd src-tauri

# 检查 python3 是否存在
if ! command -v python3 &> /dev/null; then
    echo "错误：在 src-tauri/ 目录下未找到 python3 命令。请确保 Python 3 已安装并且在您的 PATH 中。"
    exit 1
fi

VENV_NAME="sidecar-venv"

if [ -d "$VENV_NAME" ]; then
    echo "虚拟环境 '$VENV_NAME' 已存在，跳过创建步骤。"
else
    echo "正在创建Python虚拟环境: $VENV_NAME..."
    python3 -m venv "$VENV_NAME"
    echo "虚拟环境创建成功。"
fi

echo "激活虚拟环境并安装Python包 (paramiko, pyinstaller)..."
# 直接使用虚拟环境中的pip来安装，无需手动激活
# 这种方式在脚本中更健壮
./"$VENV_NAME"/bin/pip install paramiko pyinstaller

echo "Python依赖安装完成！"
echo ""

# --- 完成 ---
echo "✅ ✅ ✅"
echo "开发环境初始化成功！"
echo "现在您可以运行 'pnpm tauri dev' 来启动应用。"
echo "请注意：如果您需要手动运行Python脚本，请先执行 'source src-tauri/sidecar-venv/bin/activate' 来激活虚拟环境。"