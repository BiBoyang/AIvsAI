# AIvsAI

A dual-AI collaborative terminal tool built with Rust.

## Introduction

**AIvsAI** is an interesting command-line tool that leverages the collaboration of two AI models (Moonshot AI and DeepSeek AI) to provide you with higher quality answers.

Workflow:
1.  **User Question**: You type your question in the terminal.
2.  **Moonshot Answers**: Moonshot AI (Kimi) attempts to answer your question first.
3.  **DeepSeek Review**: DeepSeek AI acts as a "technical reviewer", reading your question and Moonshot's answer, then providing rigorous review, corrections, and additions.

## Features

*   🚀 **Dual-AI Collaboration**: Combines Moonshot's general capabilities with DeepSeek's deep reasoning/code review abilities.
*   🦀 **Written in Rust**: High performance, fast startup, low resource usage.
*   💾 **Auto Configuration**: Automatically prompts for API Keys on first run and persists them to local config.
*   🖥️ **Terminal Friendly**: Colored output, clear interactive experience.
*   💬 **Conversation History**: Save conversations with `/save` command.

## Installation

### Quick Install (macOS)

1. Clone the project:
   ```bash
   git clone https://github.com/BiBoyang/AIvsAI.git
   ```

2. Double-click to install:
   - Open the `AIvsAI` folder
   - **Double-click** `Install AIvsAI.app`
   - Wait for installation to complete

3. Usage:
   ```bash
   aivsai        # Start the program
   aivsai-cd     # View conversation history
   ```

### Manual Install

#### Prerequisites
*   Rust environment (Cargo)

#### Source Install
```bash
git clone https://github.com/BiBoyang/AIvsAI.git
cd AIvsAI
cargo install --path .
```

After installation, you can run `ai_vs_ai` from anywhere in the terminal.

## Usage

1.  Run in terminal:
    ```bash
    ai_vs_ai
    # or
    aivsai  # if using the alias
    ```

2.  **First Run Configuration**:
    The program will prompt you to enter API Keys:
    *   `Moonshot API Key`: [Get it here](https://platform.moonshot.cn/)
    *   `DeepSeek API Key`: [Get it here](https://platform.deepseek.com/)
    
    *Keys are automatically saved to `~/.ai_vs_ai_config`, no need to re-enter.*

3.  **Start Chatting**:
    Type your question and watch the two AIs collaborate.

4.  **Save Conversation**:
    Type `/save` to save the current conversation to `conversations/` directory.

## Example

```text
User > Explain Rust's ownership system

--- Moonshot AI Answer ---
(Detailed explanation from Moonshot...)

--- DeepSeek AI Review ---
(DeepSeek highlights the strengths of Moonshot's answer and adds details about lifetimes...)

Type /save to save this conversation
```

## Development

```bash
# Clone the project
git clone https://github.com/BiBoyang/AIvsAI.git

# Run development version
cargo run

# Build Release version
cargo build --release
```

## License

MIT
