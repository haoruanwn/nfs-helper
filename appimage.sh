#!/bin/bash

# --- NFS Helper AppImage 打包脚本 ---
# 使用方法:
# 1. 在终端中，首先给脚本添加执行权限: chmod +x appimage.sh
# 2. 然后运行脚本: ./appimage.sh
#
# set -e: 如果任何命令执行失败，脚本将立即退出
set -e

# --- 定义一些颜色变量，让输出更美观 ---
COLOR_BLUE='\033[0;34m'
COLOR_GREEN='\033[0;32m'
COLOR_YELLOW='\033[1;33m'
COLOR_NC='\033[0m' # No Color

# --- 开始打包流程 ---
echo -e "${COLOR_BLUE}=====================================================${COLOR_NC}"
echo -e "${COLOR_BLUE}🚀 开始为 NFS Helper 打包 AppImage...${COLOR_NC}"
echo -e "${COLOR_BLUE}=====================================================${COLOR_NC}"
echo ""

echo -e "${COLOR_BLUE}▶️  正在执行 'pnpm tauri build'...${COLOR_NC}"
echo "   (此过程可能需要几分钟，特别是首次编译时，请耐心等待)"
echo ""

# 执行核心打包命令
pnpm tauri build

# --- 打包完成 ---
echo ""
echo -e "${COLOR_GREEN}=====================================================${COLOR_NC}"
echo -e "${COLOR_GREEN}✅ AppImage 打包成功!${COLOR_NC}"
echo ""
echo -e "   你可以在 src-tauri/target/release/bundle/appimage/ 目录下找到你的 AppImage 文件。"
echo -e "${COLOR_GREEN}=====================================================${COLOR_NC}"

# 脚本正常结束
exit 0