#!/bin/bash

# --- NFS Helper AppImage 打包脚本 ---
#
# 这个脚本会自动设置解决 'strip' 命令兼容性问题所需的环境变量，
# 然后执行 Tauri 的构建和打包流程。
#
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

# --- 1. 开始打包流程 ---
echo -e "${COLOR_BLUE}=====================================================${COLOR_NC}"
echo -e "${COLOR_BLUE}🚀 开始为 NFS Helper 打包 AppImage...${COLOR_NC}"
echo -e "${COLOR_BLUE}=====================================================${COLOR_NC}"
echo ""

# --- 2. 设置并导出环境变量 ---
# 这是解决在现代Linux发行版 (如Fedora 42) 上打包失败的关键
export NO_STRIP=true
echo -e "${COLOR_YELLOW}[INFO] 已设置环境变量: NO_STRIP=true (跳过 strip 步骤)${COLOR_NC}"
echo ""

# --- 3. 执行 Tauri 构建命令 ---
echo -e "${COLOR_BLUE}▶️  正在执行 'pnpm tauri build'...${COLOR_NC}"
echo "   (此过程可能需要几分钟，特别是首次编译时，请耐心等待)"
echo ""

# 执行核心打包命令
pnpm tauri build

# --- 4. 打包完成 ---
echo ""
echo -e "${COLOR_GREEN}=====================================================${COLOR_NC}"
echo -e "${COLOR_GREEN}✅ AppImage 打包成功!${COLOR_NC}"
echo ""
echo -e "   你可以在 src-tauri/target/release/bundle/appimage/ 目录下找到你的 AppImage 文件。"
echo -e "${COLOR_GREEN}=====================================================${COLOR_NC}"

# 脚本正常结束
exit 0