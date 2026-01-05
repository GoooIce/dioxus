# Dioxus OHOS (OpenHarmony) 快速开始指南

本指南将帮助你设置开发环境并创建你的第一个 Dioxus OHOS 应用。

## 目录

- [系统要求](#系统要求)
- [环境设置](#环境设置)
- [创建第一个 OHOS 应用](#创建第一个-ohos-应用)
- [构建和运行](#构建和运行)
- [故障排除](#故障排除)
- [已知限制](#已知限制)

## 系统要求

### 开发环境

- **操作系统**: macOS, Linux, 或 Windows
- **DevEco Studio**: 4.0 或更高版本
- **OpenHarmony SDK**: API Level 9 或更高
- **OpenHarmony NDK**: 包含在 SDK 中
- **Rust**: 1.70 或更高版本
- **Node.js** (可选,用于某些构建工具)

### 支持的目标架构

| 架构 | 目标三元组 | 设备类型 |
|------|-----------|----------|
| ARM64 | `aarch64-unknown-linux-ohos` | 真实设备 (大多数现代设备) |
| ARM32 | `armv7a-unknown-linux-ohos` | 较老的设备 |
| x86_64 | `x86_64-unknown-linux-ohos` | 模拟器 |

### Beta 状态说明

⚠️ **重要**: OHOS 支持目前处于 **Beta** 阶段。这意味着:

- 核心功能已经可用,但可能会有 bug
- 性能优化尚未完成
- 某些高级功能可能不支持
- API 可能会在未来版本中发生变化

## 环境设置

### 1. 安装 DevEco Studio 和 SDK

1. 从 [华为开发者联盟](https://developer.harmonyos.com/cn/develop/deveco-studio) 下载并安装 DevEco Studio
2. 启动 DevEco Studio 并完成初始设置向导
3. 在 SDK Manager 中安装:
   - OpenHarmony SDK (API Level 9+)
   - OpenHarmony NDK

### 2. 设置环境变量

设置以下环境变量,将 Dioxus CLI 指向你的 OHOS SDK:

#### macOS / Linux

在你的 `~/.bashrc`, `~/.zshrc` 或 `~/.profile` 中添加:

```bash
# OHOS SDK 路径
export OHOS_SDK_HOME=~/HarmonyOS/Sdk
export OHOS_NDK_HOME=~/HarmonyOS/Sdk/ohos-ndk

# 或者如果 NDK 在不同的位置
# export OHOS_NDK_HOME=/path/to/ohos/ndk
```

然后运行 `source ~/.bashrc` (或相应的配置文件)。

#### Windows

在系统环境变量中添加:

```
OHOS_SDK_HOME=C:\Users\YourName\HarmonyOS\Sdk
OHOS_NDK_HOME=C:\Users\YourName\HarmonyOS\Sdk\ohos-ndk
```

或在 PowerShell 中:

```powershell
$env:OHOS_SDK_HOME="C:\Users\YourName\HarmonyOS\Sdk"
$env:OHOS_NDK_HOME="C:\Users\YourName\HarmonyOS\Sdk\ohos-ndk"
```

### 3. 安装 Rust 工具链

1. 安装 Rust (如果还没有安装):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. 添加 OHOS 目标架构:
   ```bash
   # 对于 ARM64 设备 (推荐)
   rustup target add aarch64-unknown-linux-ohos

   # 对于 ARM32 设备
   rustup target add armv7a-unknown-linux-ohos

   # 对于 x86_64 模拟器
   rustup target add x86_64-unknown-linux-ohos
   ```

3. 安装 Dioxus CLI:
   ```bash
   cargo install dioxus-cli
   ```

### 4. 配置 hdc (HarmonyOS Device Connector)

`hdc` 是用于与 HarmonyOS 设备通信的工具。它通常包含在 DevEco Studio 的 SDK 中。

确保 `hdc` 在你的 PATH 中:

```bash
# macOS/Linux
export PATH="$PATH:$OHOS_SDK_HOME/toolchains/hdc"

# Windows
# 将 %OHOS_SDK_HOME%\toolchains\hdc 添加到 PATH
```

验证安装:

```bash
hdc --version
```

## 创建第一个 OHOS 应用

### 方法 1: 使用 Dioxus CLI 创建新项目

```bash
# 创建新的 Dioxus 项目
dx new my-ohos-app

# 进入项目目录
cd my-ohos-app

# 编辑 Dioxus.toml 以配置 OHOS 设置
```

### 方法 2: 添加到现有项目

如果你已经有一个 Dioxus 项目,可以添加 OHOS 支持:

1. 确保你的 `Cargo.toml` 包含必要的依赖:

```toml
[dependencies]
dioxus = { version = "0.7", features = ["mobile"] }
```

2. 在 `Dioxus.toml` 中添加 OHOS 配置:

```toml
[application]
name = "My App"
default_platform = "ohos"

[ohos]
# 最小 API 级别 (默认: 9)
min_api_level = 9

# 应用包名
package = "com.example.myapp"

# 应用版本
version_code = 1
version_name = "1.0.0"
```

## 构建和运行

### 开发模式 (热重载)

在开发过程中,使用热重载可以显著提高开发效率:

```bash
# 启动开发服务器
dx serve --platform ohos

# 如果有多个设备,可以指定设备
dx serve --platform ohos --device <device-id>
```

这将:
1. 编译你的 Rust 代码为 OHOS 动态库
2. 启动开发服务器
3. 在连接的设备或模拟器上安装并运行应用
4. 监控文件更改并自动重新加载

### 生产构建

构建用于发布的 `.hap` (HarmonyOS Ability Package) 文件:

```bash
# 构建发布版本
dx build --platform ohos --release

# 生成的文件位于:
# target/ohos/release/app/build/outputs/hap/
```

### CLI 命令参考

#### `dx serve`

在 OHOS 设备/模拟器上运行应用:

```bash
# 基本用法
dx serve --platform ohos

# 指定设备
dx serve --platform ohos --device <device-id>

# 指定目标架构
dx serve --platform ohos --target aarch64-unknown-linux-ohos

# 发布模式 (更快,但没有热重载)
dx serve --platform ohos --release
```

#### `dx build`

构建应用但不运行:

```bash
# 开发构建
dx build --platform ohos

# 发布构建
dx build --platform ohos --release

# 指定输出目录
dx build --platform ohos --out-dir ./build
```

#### `dx bundle`

打包应用为 `.hap` 文件:

```bash
# 创建发布包
dx bundle --platform ohos --release
```

### 列出连接的设备

```bash
# 列出所有可用的 OHOS 设备
hdc list targets

# 使用 Dioxus CLI
dx device list --platform ohos
```

## 配置选项

### Dioxus.toml 配置

完整的 `Dioxus.toml` 示例:

```toml
[application]
name = "My OHOS App"
default_platform = "ohos"

[web.app]
title = "My OHOS App"

[web.resource.dev]
style = []
script = []

[web.resource.release]
style = []
script = []

[ohos]
# OHOS 特定配置
min_api_level = 9
package = "com.example.myapp"
version_code = 1
version_name = "1.0.0"

# 应用图标 (可选)
icon = "assets/icon.png"

# 应用权限 (可选)
# permissions = ["ohos.permission.INTERNET"]
```

### 环境变量

| 变量 | 描述 | 必需 |
|------|------|------|
| `OHOS_SDK_HOME` | OHOS SDK 路径 | 是 |
| `OHOS_NDK_HOME` | OHOS NDK 路径 | 是* |
| `HDC_PORT` | hdc 端口 (默认: 5037) | 否 |

*如果 NDK 在 SDK 的标准位置,可以省略。

## 故障排除

### 常见问题

#### 1. "OHOS_SDK_HOME not set" 错误

**问题**: Dioxus CLI 找不到 OHOS SDK。

**解决方案**:
```bash
# 验证环境变量
echo $OHOS_SDK_HOME
echo $OHOS_NDK_HOME

# 如果未设置,按"环境设置"部分的说明设置
```

#### 2. "Failed to find OHOS toolchains" 错误

**问题**: NDK 工具链路径不正确。

**解决方案**:
```bash
# 检查 NDK 是否存在
ls $OHOS_NDK_HOME/toolchains/llvm/prebuilt

# 手动指定 NDK 位置
export OHOS_NDK_HOME=/correct/path/to/ndk
```

#### 3. 编译错误: "target not found"

**问题**: Rust 目标架构未安装。

**解决方案**:
```bash
# 安装所需的目标
rustup target add aarch64-unknown-linux-ohos
```

#### 4. 设备连接问题

**问题**: `hdc` 无法连接到设备。

**解决方案**:
```bash
# 检查设备连接
hdc list targets

# 重启 hdc 服务
hdc kill
hdc start

# 在设备上启用 USB 调试:
# 设置 > 系统 > 开发者选项 > USB 调试
```

#### 5. WebView 不显示内容

**问题**: 应用启动但显示空白屏幕。

**解决方案**:
- 检查 `dioxus` 是否启用了 `mobile` feature
- 确保应用有正确的网络权限
- 查看日志: `hdc shell hilog -T Dioxus`

### 获取帮助

如果遇到问题:

1. 检查 [GitHub Issues](https://github.com/DioxusLabs/dioxus/issues)
2. 加入 [Discord 社区](https://discord.gg/XgGxMSkvUM)
3. 查看 [Dioxus 文档](https://dioxuslabs.com/learn/0.7/)

## 已知限制

### 当前不支持的功能

- **某些系统 API**: 某些 HarmonyOS 特定的 API 可能尚不可用
- **自定义协议处理**: HTTPS scheme 已启用,但某些高级协议功能可能受限
- **菜单栏和托盘图标**: 移动平台不支持桌面特有功能
- **原生插件**: 需要手动集成 HarmonyOS 原生代码

### 性能注意事项

- **首次启动**: 首次启动可能较慢,因为需要加载 WebView
- **内存使用**: WebView 内存使用高于原生渲染器
- **动画性能**: 复杂动画可能不如原生渲染器流畅

### Beta 限制

- API 可能发生变化
- 某些边缘情况可能导致崩溃
- 文档和示例可能不完整

## 下一步

- 探索 [Dioxus 指南](https://dioxuslabs.com/learn/0.7/)
- 查看 [示例项目](https://github.com/DioxusLabs/dioxus/tree/main/examples)
- 阅读 [API 文档](https://docs.rs/dioxus-desktop/latest/dioxus_desktop/)
- 为项目做贡献!

## 贡献

OHOS 支持仍在开发中。欢迎贡献!

- 报告 bug: [GitHub Issues](https://github.com/DioxusLabs/dioxus/issues)
- 提交 PR: [GitHub Pull Requests](https://github.com/DioxusLabs/dioxus/pulls)
- 加入讨论: [Discord](https://discord.gg/XgGxMSkvUM)

---

*最后更新: 2026-01-05*
