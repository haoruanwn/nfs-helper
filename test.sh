#!/bin/bash

# --- 自动化构建和开发启动脚本 ---

# set -e: 确保任何命令执行失败时，脚本会立即停止
# 这可以防止在一个步骤失败后，后续错误的步骤继续执行
set -e

echo "🚀 [Step 1/5] 清理旧的构建产物..."
# 使用 -f 选项，即使文件不存在也不会报错
rm -f src-tauri/resources/nfs_helper_cli
echo "✅ 旧产物清理完成。"

echo "🐍 [Step 2/5] 激活Python虚拟环境..."
# 检查虚拟环境是否存在
if [ -f "src-tauri/sidecar-venv/bin/activate" ]; then
    source src-tauri/sidecar-venv/bin/activate
    echo "✅ 虚拟环境已激活。"
else
    echo "❌ 错误：找不到虚拟环境 'src-tauri/sidecar-venv/bin/activate'。"
    echo "请先在 'src-tauri' 目录下创建名为 'sidecar-venv' 的虚拟环境。"
    exit 1
fi


echo "📦 [Step 3/5] 使用PyInstaller打包Python脚本..."
# 进入 src-tauri 目录执行打包
(cd src-tauri && pyinstaller --name nfs_helper_cli --onefile sidecar.py)
echo "✅ Python脚本打包完成。"

echo "🚚 [Step 4/5] 移动新的构建产物到资源目录..."
# 检查打包产物是否存在
if [ -f "src-tauri/dist/nfs_helper_cli" ]; then
    mv src-tauri/dist/nfs_helper_cli src-tauri/resources/
    echo "✅ 新产物已移动到 'src-tauri/resources/'。"
else
    echo "❌ 错误：打包失败，未在 'src-tauri/dist/' 目录下找到 'nfs_helper_cli'。"
    exit 1
fi


echo "🖥️  [Step 5/5] 启动Tauri开发服务器..."
# 设置环境变量并启动开发服务器
# 这是脚本的最后一步，会占用当前终端
export WEBKIT_DISABLE_COMPOSITING_MODE=1
pnpm tauri dev

echo "👋 脚本执行完毕。"