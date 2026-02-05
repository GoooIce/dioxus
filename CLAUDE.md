# Dioxus - Rust 跨平台 UI 框架

## 项目概述
- Rust 编写的跨平台 UI 框架（类似 React）
- 支持 Web (WASM)、桌面、移动端、LiveView
- 主仓库: https://github.com/DioxusLabs/dioxus

## 工作空间结构
- `packages/dioxus/` - 主导出 crate
- `packages/core/` - VirtualDOM、组件、diffing
- `packages/cli/` - `dx` 构建工具
- `packages/web/` - WASM 渲染器
- `packages/desktop/` - 桌面渲染器
- `packages/fullstack/` - SSR、hydration、#[server] 函数
- `packages/hooks/` - 内置 hooks
- `packages/router/` - 路由

## 关键概念
- **rsx!** - JSX 风格的 UI 声明宏
- **Signals** - 响应式状态管理 (`use_signal`)
- **#[component]** - 组件定义宏
- **#[server]** - 服务端函数宏

## 开发命令
- `cargo test --workspace` - 运行测试
- `dx serve` - 启动开发服务器
- MSRV: Rust 1.88.0

## 架构文档
详见 `notes/architecture/` 目录和 `AGENTS.md`
