# AIvsAI 安装指南

## 超简单安装（推荐）

### macOS

1. **下载项目**
   ```bash
   git clone https://github.com/BiBoyang/AIvsAI.git
   ```

2. **双击安装**
   - 打开 `AIvsAI` 文件夹
   - **双击** `安装 AIvsAI.app`
   - 等待安装完成

3. **使用**
   ```bash
   aivsai        # 启动程序
   aivsai-cd     # 查看对话记录
   ```

### Linux / 手动安装

```bash
# 1. 克隆项目
git clone https://github.com/BiBoyang/AIvsAI.git
cd AIvsAI

# 2. 运行安装脚本
./setup

# 3. 使用
aivsai
```

## 功能特点

- ✅ 自动检测并安装 Rust（如未安装）
- ✅ 一键编译安装
- ✅ 自动配置快捷命令
- ✅ 对话记录保存在项目目录

## 文件说明

| 文件/目录 | 说明 |
|-----------|------|
| `安装 AIvsAI.app` | macOS 双击安装程序 |
| `setup` | 命令行安装脚本 |
| `conversations/` | 对话记录保存目录 |
