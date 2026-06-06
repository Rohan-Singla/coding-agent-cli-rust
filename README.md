***

# open-code 🚀

`open-code` is a lightweight, CLI-based Coding Agent that acts as an assistant for your Rust projects. It reads all `.rs` files in your current working directory, constructs a global context of your codebase, and sends it alongside your prompt to Google Gemini AI models to generate highly relevant, context-aware coding solutions.

---

## 📦 Features

- **Codebase Context-Aware**: Automatically scans your directory recursively for `.rs` files and attaches them to your queries.
- **Provider Management**: Connect and switch between different AI providers easily.
- **Model Selection**: Switch between various supported models on the fly.
- **Secure Configuration**: Stores credentials and settings locally in your system's default configuration directory (e.g., `~/.config/open-code/config.json`).

---

## 🛠️ Installation & Setup

1. **Prerequisites**: Ensure you have [Rust and Cargo](https://rustup.rs/) installed.
2. **Build the project**:
   ```bash
   cargo build --release
   ```
3. **Run the binary**:
   The compiled binary will be located in `target/release/open-code`. You can move it to your path or run it directly:
   ```bash
   ./target/release/open-code --help
   ```

---

## 📖 Command Guide

### 1. Managing Providers

Before using the agent, you need to register and select an AI provider (e.g., Google Gemini).

#### **List available providers**
Fetches and lists all compatible providers from the central registry.
```bash
open-code providers list
```

#### **Log in to a provider**
Save your API key for a selected provider.
```bash
open-code providers login --provider <PROVIDER_NAME> --api_key <YOUR_API_KEY>
```
*Example:*
```bash
open-code providers login --provider gemini --api_key AIzaSyD...
```

#### **Set the active provider**
Choose which logged-in provider to use for queries.
```bash
open-code providers set --provider <PROVIDER_NAME>
```

#### **Log out of a provider**
Remove saved credentials for a provider.
```bash
open-code providers logout --provider <PROVIDER_NAME>
```

---

### 2. Managing Models

Once an active provider is set, you can configure which model should process your prompts.

#### **List available models**
Lists all models available under your currently active provider.
```bash
open-code models
```

#### **Set the active model**
Set your active model.
```bash
open-code models set --model <MODEL_NAME>
```
*Example:*
```bash
open-code models set --model gemini-1.5-flash
```

---

### 3. Running the Coding Agent 🤖

Run the agent within any Rust project directory. The agent recursively collects all `.rs` files, combines them with your custom prompt, and contacts the active AI model.

#### **Execute a query**
```bash
open-code agent -p "<your prompt/question here>"
```

*Example:*
```bash
open-code agent -p "Refactor the load_config function to use better error handling"
```

---

## 📂 Configuration Storage

All credentials and configurations are stored locally on your machine at:
- **Linux/macOS**: `~/.config/open-code/config.json`
- **Windows**: `%APPDATA%\open-code\config.json`