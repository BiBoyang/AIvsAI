#!/bin/bash

# AIvsAI 安装脚本
# 在别的电脑上运行此脚本，可完全复刻原电脑的环境

set -e

# 颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}=== AIvsAI 安装脚本 ===${NC}"

# 获取项目绝对路径
PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
echo -e "${YELLOW}项目目录: $PROJECT_DIR${NC}"

# 检查 Rust/Cargo 是否安装
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}错误: 未找到 Cargo，请先安装 Rust${NC}"
    echo "访问: https://rustup.rs/"
    exit 1
fi

echo -e "${GREEN}✓ Rust/Cargo 已安装${NC}"

# 编译并安装
echo -e "${YELLOW}正在编译安装...${NC}"
cargo install --path "$PROJECT_DIR"

echo -e "${GREEN}✓ 编译安装完成${NC}"

# 检测 shell 类型
SHELL_TYPE=""
if [[ "$SHELL" == *"zsh"* ]]; then
    SHELL_TYPE="zsh"
    SHELL_RC="$HOME/.zshrc"
elif [[ "$SHELL" == *"bash"* ]]; then
    SHELL_TYPE="bash"
    SHELL_RC="$HOME/.bashrc"
else
    SHELL_TYPE="sh"
    SHELL_RC="$HOME/.profile"
fi

echo -e "${YELLOW}检测到 Shell: $SHELL_TYPE${NC}"

# 添加 alias
ALIAS_FILE="$SHELL_RC"

# 检查是否已存在 aivsai alias
if grep -q "alias aivsai=" "$ALIAS_FILE" 2>/dev/null; then
    echo -e "${YELLOW}警告: 已存在 aivsai alias，跳过添加${NC}"
else
    echo "" >> "$ALIAS_FILE"
    echo "# AIvsAI aliases" >> "$ALIAS_FILE"
    echo "alias aivsai='cd $PROJECT_DIR && ai_vs_ai'" >> "$ALIAS_FILE"
    echo "alias aivsai-cd='cd $PROJECT_DIR/conversations && ls -la'" >> "$ALIAS_FILE"
    echo -e "${GREEN}✓ Alias 已添加到 $ALIAS_FILE${NC}"
fi

# 创建 conversations 目录
CONV_DIR="$PROJECT_DIR/conversations"
if [ ! -d "$CONV_DIR" ]; then
    mkdir -p "$CONV_DIR"
    echo -e "${GREEN}✓ 创建 conversations 目录${NC}"
fi

echo ""
echo -e "${GREEN}=== 安装完成! ===${NC}"
echo ""
echo "请运行以下命令使 alias 生效:"
echo -e "${YELLOW}  source $ALIAS_FILE${NC}"
echo ""
echo "然后可以使用以下命令:"
echo -e "  ${YELLOW}aivsai${NC}     - 启动 AIvsAI"
echo -e "  ${YELLOW}aivsai-cd${NC}  - 查看保存的对话记录"
echo ""
echo -e "${GREEN}对话记录将保存在: $CONV_DIR${NC}"
