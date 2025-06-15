# Markdown Tasks 🤖

A smart, command-line task manager that uses the power of Large Language Models (LLMs) to refine and improve your raw task ideas into clear, actionable items in your markdown files.

---

## ✨ Key Features

- **LLM-Powered Task Improvement:** Simply provide a rough idea for a task, and it will be sent to an LLM to be clarified, expanded, and formatted as a proper markdown task item.
- **Simple and Fast:** A lightweight, compiled binary that runs instantly from your terminal.
- **Configurable:** Set a global task file or specify one on the fly for different projects.
- **Standard Markdown:** Works directly with `.md` files, making your tasks portable and easy to use with other tools like Obsidian, Notion, or VS Code.

---

## 🚀 Getting Started

### Prerequisites

- You need to have Rust and Cargo installed on your system. You can get them from [rustup.rs](https://rustup.rs/).
- An API key from an LLM provider that is compatible with the OpenAI completions API format (e.g., OpenRouter, OpenAI, etc.). This guide uses OpenRouter.ai.

### 1. Installation & Setup

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/gabrielazevedo76/markdown-tasks
    cd markdown-tasks
    ```

2.  **Set Your API Key:**
    This application securely reads your API key from an environment variable. You must set this variable in your terminal session for the application to work.

    **On macOS / Linux:**
    ```bash
    export OPENROUTER_API_KEY="your-api-key-here"
    ```
    *(You can add this line to your `.zshrc`, `.bashrc`, or `.profile` file to make it permanent.)*

    **On Windows (PowerShell):**
    ```powershell
    $env:OPENROUTER_API_KEY="your-api-key-here"
    ```

### 2. How to Use

All commands are run through `cargo run -- ` during development.

**Step 1: Configure a global task file**
This tells the application where to save your tasks by default.

```bash
cargo run -- config --global-file ~/Documents/tasks.md
```

**Step 2: Create a task**
Provide your task idea as a string. The application will contact the LLM and save the improved version to your file.

```bash
cargo run -- create "meeting about the Q3 budget"
```

**Output:**
```
🤖 Calling LLM to improve the task... please wait.

✅ Successfully added improved task to "/Users/YourUser/Documents/tasks.md"
   > - [ ] Schedule and prepare for a meeting to discuss the Q3 budget projections.
```

**Use a different file for a specific task:**
You can override the global configuration with the `--file` flag.

```bash
cargo run -- create "write the project proposal" --file ./project-specific-tasks.md
```

---

## 📦 Building for Production

When you are ready to use the application without `cargo`, you can build an optimized, production-ready executable.

1.  **Build with the `--release` flag:**
    ```bash
    cargo build --release
    ```
    This will create a highly optimized binary named `markdown-tasks` in the `target/release/` directory.

2.  **Install the binary (Optional, Recommended):**
    For easy access, copy the executable to a directory in your system's `PATH` and rename it to `mt` for a shorter command.

    **On macOS / Linux:**
    ```bash
    sudo cp target/release/markdown-tasks /usr/local/bin/mt
    ```

    **On Windows:**
    Copy `target\release\markdown-tasks.exe` to a folder included in your `Path` environment variable, and rename it to `mt.exe`.

3.  **Run from anywhere!**
    Once installed, you can call the application directly from any terminal window using `mt`.

    ```bash
    # Don't forget your API key must still be set!
    mt create "call the supplier tomorrow"
    ```
