# AIvsAI - Agent Collaboration Guide

> This document guides AI Agents in collaborating on the AIvsAI project.

## Project Overview

- **Project Name**: AIvsAI
- **Project Type**: Rust CLI Tool
- **Core Function**: Dual-AI collaborative Q&A system (Moonshot + DeepSeek)
- **Project Path**: `/Users/boyang/Desktop/WebKit_build/AIvsAI`

## Development Standards

### Code Style

- Use standard Rust code style (`cargo fmt` auto-formatting)
- Follow Clippy suggestions (`cargo clippy` check)
- Error handling using `anyhow::Result` and `Context`
- Async code using `tokio` runtime

### Project Structure

```
AIvsAI/
├── Cargo.toml          # Project configuration
├── README.md           # User documentation
├── AGENTS.md           # This file - Agent collaboration guide
├── .gitignore
└── src/
    └── main.rs         # Main program (current single-file structure)
```

### Dependency Management

Current dependencies:
- `tokio` - Async runtime
- `reqwest` - HTTP client
- `serde` / `serde_json` - Serialization
- `anyhow` - Error handling
- `colored` - Terminal colored output
- `dotenvy` - Environment variable loading
- `chrono` - Date/time handling
- `rustyline` - Better terminal input with Unicode support

**Before adding new dependencies, confirm**:
1. Is it really needed?
2. Is it the latest stable version?
3. Will it increase binary size significantly?

### API Standards

- Use OpenAI-compatible API format
- Request structures: `ChatRequest` / `ChatMessage`
- Response structures: `ChatResponse` / `ChatChoice` / `MessageContent`
- Error handling: Use `anyhow` uniformly

### Configuration Management

- Config file path: `~/.ai_vs_ai_config`
- Environment variables:
  - `MOONSHOT_API_KEY` - Moonshot API key
  - `DEEPSEEK_API_KEY` - DeepSeek API key
- Auto-prompt for user input on first run and persist

## Development Workflow

### 1. Code Checks

Run before each modification:
```bash
cargo check      # Compilation check
cargo clippy     # Code style check
cargo fmt        # Formatting
```

### 2. Testing

Currently no tests. To add:
```bash
cargo test       # Run tests
```

### 3. Build

```bash
cargo build              # Development build
cargo build --release    # Release build
```

### 4. Run

```bash
cargo run        # Development run
```

## Feature Development Guide

### Checklist for Adding New Features

- [ ] Code passes `cargo check`
- [ ] Code passes `cargo clippy` (no warnings)
- [ ] Code is formatted with `cargo fmt`
- [ ] Error handling is complete (using `anyhow`)
- [ ] User prompts are clear (using `colored`)
- [ ] Configuration is properly persisted (if needed)
- [ ] README.md is updated (if needed)

### Code Organization Suggestions

If functionality becomes complex, consider splitting `main.rs` into:
```
src/
├── main.rs           # Program entry
├── config.rs         # Configuration management
├── api.rs            # API calls
├── models.rs         # Data structures
└── ui.rs             # Terminal interaction
```

## Todo List

> Record planned feature updates

- [x] **Conversation History Save Feature**
  - Trigger: User inputs `/save` command
  - Save location: `conversations/` subdirectory (auto-created)
  - File naming format: `YYYY-MM-DD_HH-MM-SS_user-question-summary.md`
    - Uses local time
    - Summary takes first 20 chars of user question (remove punctuation, spaces to underscores)
  - Save content:
    1. User question
    2. Moonshot's answer
    3. DeepSeek's review
    4. Metadata (timestamp, models used, etc.)
  - File format: Markdown (conversation style + YAML Front Matter metadata)
    ```markdown
    ---
    created_at: 2025-03-01 14:30:25
    moonshot_model: moonshot-v1-8k
    deepseek_model: deepseek-chat
    ---

    # AIvsAI Conversation

    > 💬 **User**: User question content

    ---

    > 🤖 **Moonshot** (moonshot-v1-8k)
    > 
    > Moonshot's answer content

    ---

    > 🔍 **DeepSeek** (deepseek-chat)
    > 
    > DeepSeek's review content
    ```
  - Privacy reminder: Conversation history may contain sensitive information

## Notes

1. **API Key Security**: Never hardcode API keys, always use config files or environment variables
2. **Friendly Error Messages**: Provide clear error info when API calls fail
3. **Terminal Experience**: Use `colored` for beautiful output, keep interaction clear
4. **Backward Compatibility**: Consider migration logic when changing config formats

## Related Links

- Moonshot API: https://platform.moonshot.cn/
- DeepSeek API: https://platform.deepseek.com/
- Rust Async Programming: https://rust-lang.github.io/async-book/
