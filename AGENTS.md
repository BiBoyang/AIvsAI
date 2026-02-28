# AIvsAI - Agent 协作指南

> 本文档用于指导 AI Agent 在 AIvsAI 项目中的协作开发。

## 项目概述

- **项目名称**: AIvsAI
- **项目类型**: Rust CLI 工具
- **核心功能**: 双 AI 协作问答系统（Moonshot + DeepSeek）
- **项目路径**: `/Users/boyang/Desktop/WebKit_build/AIvsAI`

## 开发规范

### 代码风格

- 使用标准 Rust 代码风格（`cargo fmt` 自动格式化）
- 遵循 Clippy 建议（`cargo clippy` 检查）
- 错误处理使用 `anyhow::Result` 和 `Context`
- 异步代码使用 `tokio` 运行时

### 项目结构

```
AIvsAI/
├── Cargo.toml          # 项目配置
├── README.md           # 用户文档
├── AGENTS.md           # 本文件 - Agent 协作指南
├── .gitignore
└── src/
    └── main.rs         # 主程序（当前单文件结构）
```

### 依赖管理

当前依赖：
- `tokio` - 异步运行时
- `reqwest` - HTTP 客户端
- `serde` / `serde_json` - 序列化
- `anyhow` - 错误处理
- `colored` - 终端彩色输出
- `dotenvy` - 环境变量加载

**添加新依赖前请确认**：
1. 是否确实需要？
2. 是否使用最新稳定版本？
3. 是否会增加二进制体积？

### API 规范

- 使用 OpenAI 兼容的 API 格式
- 请求结构：`ChatRequest` / `ChatMessage`
- 响应结构：`ChatResponse` / `ChatChoice` / `MessageContent`
- 错误处理：统一使用 `anyhow` 进行错误传递

### 配置管理

- 配置文件路径: `~/.ai_vs_ai_config`
- 环境变量：
  - `MOONSHOT_API_KEY` - Moonshot API 密钥
  - `DEEPSEEK_API_KEY` - DeepSeek API 密钥
- 首次运行时自动引导用户输入并持久化

## 开发流程

### 1. 代码检查

每次修改前运行：
```bash
cargo check      # 编译检查
cargo clippy     # 代码风格检查
cargo fmt        # 格式化
```

### 2. 测试

当前项目暂无测试，如需添加：
```bash
cargo test       # 运行测试
```

### 3. 构建

```bash
cargo build              # 开发构建
cargo build --release    # 发布构建
```

### 4. 运行

```bash
cargo run        # 开发运行
```

## 功能开发指南

### 添加新功能时的检查清单

- [ ] 代码能通过 `cargo check`
- [ ] 代码能通过 `cargo clippy`（无警告）
- [ ] 代码已格式化 `cargo fmt`
- [ ] 错误处理完善（使用 `anyhow`）
- [ ] 用户提示信息清晰（使用 `colored` 美化）
- [ ] 配置项正确持久化（如需要）
- [ ] README.md 已更新（如需要）

### 代码组织建议

如果功能复杂，考虑将 `main.rs` 拆分为：
```
src/
├── main.rs           # 程序入口
├── config.rs         # 配置管理
├── api.rs            # API 调用
├── models.rs         # 数据结构
└── ui.rs             # 终端交互
```

## 待办事项

> 记录计划中的功能更新

- [x] **对话历史保存功能**
  - 触发方式：用户输入 `/save` 命令时保存
  - 保存位置：`conversations/` 子目录（自动创建）
  - 文件命名格式：`YYYY-MM-DD_HH-MM-SS_用户问题摘要.md`
    - 时间使用本地时间
    - 摘要取用户问题的前 20 个字符（去除标点，空格替换为下划线）
  - 保存内容：
    1. 用户问题
    2. Moonshot 的回答
    3. DeepSeek 的审查
    4. 元信息（时间戳、使用的模型等）
  - 文件格式：Markdown（对话流版 + YAML Front Matter 元信息）
    ```markdown
    ---
    created_at: 2025-03-01 14:30:25
    moonshot_model: moonshot-v1-8k
    deepseek_model: deepseek-chat
    ---

    # AIvsAI 对话记录

    > 💬 **用户**：用户问题内容

    ---

    > 🤖 **Moonshot** (moonshot-v1-8k)
    > 
    > Moonshot 的回答内容

    ---

    > 🔍 **DeepSeek** (deepseek-chat)
    > 
    > DeepSeek 的审查内容
    ```
  - 隐私提醒：对话历史可能包含敏感信息，注意保护


## 注意事项

1. **API 密钥安全**：不要硬编码 API 密钥，始终通过配置文件或环境变量获取
2. **错误提示友好**：API 调用失败时给出清晰的错误信息
3. **终端体验**：使用 `colored` 美化输出，保持交互清晰
4. **向后兼容**：配置格式变更时考虑迁移逻辑

## 相关链接

- Moonshot API: https://platform.moonshot.cn/
- DeepSeek API: https://platform.deepseek.com/
- Rust 异步编程: https://rust-lang.github.io/async-book/
