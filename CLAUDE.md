# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Dioxus 是一个跨平台的 Rust UI 框架，支持 Web、桌面、移动端和服务器端渲染。它使用 VirtualDom 架构，通过不同的渲染器实现多平台支持。

## Common Development Commands

### Building and Testing

```bash
# 构建所有包
cargo build

# 构建特定包
cargo build -p dioxus-cli

# 运行测试
cargo test

# 运行单个测试
cargo test --package dioxus-core test_name
```

### Running Examples

```bash
# 使用 cargo 直接运行示例
cargo run --example hello_world

# 使用 CLI 运行示例（推荐，支持热重载）
dx serve --example hello_world

# 在 Web 平台运行示例
dx serve --example hello_world --platform web -- --no-default-features

# 在桌面平台运行
dx serve --example hello_world --platform desktop
```

### CLI Development

```bash
# 从源码构建并安装 CLI
cargo install --path packages/cli

# 使用特定功能集运行 CLI
cargo run --package dioxus-cli -- serve --platform web
```

### Platform-Specific Builds

```bash
# Web (wasm32-unknown-unknown)
cargo build --target wasm32-unknown-unknown --package dioxus-web

# Desktop (当前主机平台)
cargo build --package dioxus-desktop

# Server
cargo build --package dioxus-fullstack
```

## Architecture Overview

### Workspace Structure

```
packages/
├── Core Layer:
│   ├── dioxus-core          # VirtualDom 和渲染核心
│   ├── dioxus-html          # HTML 元素定义
│   ├── dioxus-hooks         # React 风格的 hooks
│   └── dioxus-signals       # 响应式状态管理
│
├── Renderers:
│   ├── dioxus-web           # Web 平台（WebAssembly）
│   ├── dioxus-desktop       # 桌面平台（WebView）
│   ├── dioxus-native        # 原生渲染器（WGPU）
│   ├── dioxus-ssr           # 服务器端渲染
│   └── dioxus-liveview      # Liveview 渲染
│
├── Tooling:
│   ├── dioxus-cli           # 命令行工具
│   ├── dioxus-devtools      # 开发者工具
│   └── dioxus-autofmt       # RSX 格式化
│
├── Fullstack:
│   ├── dioxus-fullstack     # 全栈框架
│   ├── dioxus-fullstack-server
│   └── dioxus-fullstack-core
│
└── Assets/Build:
    ├── manganis             # 资源打包
    ├── asset-resolver       # 资源解析
    └── wasm-split           # WASM 优化
```

### Key Architectural Concepts

1. **VirtualDom**: `dioxus-core` 实现了 VirtualDom，负责差异计算和状态管理
2. **Renderer Abstraction**: 不同的渲染器实现 `dioxus-core::Renderer` trait
3. **RSX Macro**: `dioxus-rsx` 将 RSX 标记编译为 Rust 代码
4. **Interpreter**: `dioxus-interpreter` 在运行时解释虚拟 Dom 指令

### Feature System

Dioxus 使用 Cargo features 管理平台依赖：
- `web`: 启用 Web 平台支持
- `desktop`: 启用桌面平台支持（WebView）
- `mobile`: 启用移动端支持（iOS/Android）
- `server`: 启用全栈服务器功能

## Platform Support Details

### HarmonyOS (OHOS) Support

Dioxus 对 HarmonyOS 的支持主要通过条件编译实现：

1. **目标三元组**: 使用 `target_env = "ohos"` 检测
2. **Desktop 适配**: `packages/desktop/src/launch_ohos.rs` 包含 OHOS 特定的事件循环
3. **功能限制**: 在 OHOS 环境下，以下功能被禁用：
   - 全局热键 (`global-hotkey`)
   - 菜单系统 (`muda`)
   - 托盘图标 (`tray-icon`)
   - 文件对话框 (`rfd`)

查看 `packages/desktop/Cargo.toml:59-68` 了解条件编译配置。

### Target Triples

常用目标平台：
- Web: `wasm32-unknown-unknown`
- macOS: `x86_64-apple-darwin`, `aarch64-apple-darwin`
- Linux: `x86_64-unknown-linux-gnu`
- Windows: `x86_64-pc-windows-msvc`
- iOS: `aarch64-apple-ios`
- Android: `aarch64-linux-android`

## Development Workflow

1. 修改代码后，使用 `dx serve` 启动热重载开发服务器
2. 对于 Web 平台，使用 `--platform web` 标志
3. 对于桌面平台，直接使用 `dx serve` 或 `--platform desktop`
4. 测试更改时使用 `cargo test` 运行相关测试

## Important Notes

- 主分支的示例需要 git 版本的 Dioxus 和 CLI
- 稳定版示例在 [v0.6 分支](https://github.com/DioxusLabs/dioxus/tree/v0.6/examples)
- `packages/desktop` 使用 `tao` (Winit 分支) 和 `wry` (WebView 封装)
- 热重载功能需要 CLI 支持
- 全栈应用使用 Axum 作为后端框架
