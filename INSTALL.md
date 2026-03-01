# AIvsAI Installation Guide

## Quick Install (Recommended)

### macOS

1. **Download the project**
   ```bash
   git clone https://github.com/BiBoyang/AIvsAI.git
   ```

2. **Double-click to install**
   - Open the `AIvsAI` folder
   - **Double-click** `Install AIvsAI.app`
   - Wait for installation to complete

3. **Usage**
   ```bash
   aivsai        # Start the program
   aivsai-cd     # View conversation history
   ```

### Linux / Manual Install

```bash
# 1. Clone the project
git clone https://github.com/BiBoyang/AIvsAI.git
cd AIvsAI

# 2. Run the install script
./setup

# 3. Usage
aivsai
```

## Features

- ✅ Auto-detect and install Rust (if not installed)
- ✅ One-click compile and install
- ✅ Auto-configure shortcuts
- ✅ Conversation history saved in project directory

## File Reference

| File/Directory | Description |
|----------------|-------------|
| `Install AIvsAI.app` | macOS double-click installer |
| `setup` | Command-line install script |
| `conversations/` | Conversation history directory |
