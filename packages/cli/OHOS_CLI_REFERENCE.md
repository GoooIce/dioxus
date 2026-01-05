# Dioxus CLI - OHOS 平台命令参考

本文档详细介绍了使用 Dioxus CLI 构建、运行和部署 OHOS (OpenHarmony) 应用程序的命令。

## 目录

- [快速参考](#快速参考)
- [平台特定命令](#平台特定命令)
- [配置选项](#配置选项)
- [环境变量](#环境变量)
- [构建输出](#构建输出)
- [设备管理](#设备管理)

## 快速参考

### 最常用命令

```bash
# 创建新项目
dx new my-app

# 开发模式运行
dx serve --platform ohos

# 构建发布版本
dx build --platform ohos --release

# 打包应用
dx bundle --platform ohos --release
```

## 平台特定命令

### `--platform ohos`

所有 Dioxus CLI 命令都支持 `--platform ohos` 标志来指定 OHOS 作为目标平台。

### `dx serve`

在 OHOS 设备或模拟器上运行应用,支持热重载。

#### 语法

```bash
dx serve --platform ohos [OPTIONS]
```

#### 选项

| 选项 | 简写 | 描述 | 默认值 |
|------|------|------|--------|
| `--platform ohos` | `-p ohos` | 目标平台为 OHOS | - |
| `--target <TRIPLE>` | `-t` | 指定目标架构 | 自动检测 |
| `--device <ID>` | `-d` | 指定设备 ID | 第一个可用设备 |
| `--release` | `-r` | 使用发布模式 (更快,无热重载) | false |
| `--verbose` | `-v` | 详细输出 | false |

#### 目标架构

支持的目标三元组:

- `aarch64-unknown-linux-ohos` - ARM64 设备 (大多数现代设备)
- `armv7a-unknown-linux-ohos` - ARM32 设备
- `x86_64-unknown-linux-ohos` - x86_64 模拟器

#### 示例

```bash
# 基本用法 - 在第一个可用设备上运行
dx serve --platform ohos

# 在指定设备上运行
dx serve --platform ohos --device OHOS12345678

# 指定目标架构
dx serve --platform ohos --target aarch64-unknown-linux-ohos

# 发布模式 (不使用热重载)
dx serve --platform ohos --release

# 详细输出 (用于调试)
dx serve --platform ohos --verbose
```

### `dx build`

构建 OHOS 应用但不运行。

#### 语法

```bash
dx build --platform ohos [OPTIONS]
```

#### 选项

| 选项 | 简写 | 描述 | 默认值 |
|------|------|------|--------|
| `--platform ohos` | `-p ohos` | 目标平台为 OHOS | - |
| `--target <TRIPLE>` | `-t` | 指定目标架构 | 自动检测 |
| `--release` | `-r` | 发布构建 | false |
| `--out-dir <PATH>` | `-o` | 输出目录 | `target/ohos/[debug|release]` |

#### 示例

```bash
# 开发构建
dx build --platform ohos

# 发布构建
dx build --platform ohos --release

# 指定输出目录
dx build --platform ohos --out-dir ./my-build

# 构建特定架构
dx build --platform ohos --target armv7a-unknown-linux-ohos
```

### `dx bundle`

将应用打包为 `.hap` 文件 (HarmonyOS Ability Package)。

#### 语法

```bash
dx bundle --platform ohos [OPTIONS]
```

#### 选项

| 选项 | 简写 | 描述 | 默认值 |
|------|------|------|--------|
| `--platform ohos` | `-p ohos` | 目标平台为 OHOS | - |
| `--release` | `-r` | 发布构建 | false |
| `--out-dir <PATH>` | `-o` | 输出目录 | `target/ohos/release/app/build/outputs/hap/` |

#### 示例

```bash
# 创建开发包
dx bundle --platform ohos

# 创建发布包
dx bundle --platform ohos --release

# 指定输出目录
dx bundle --platform ohos --release --out-dir ./dist
```

### `dx new`

创建新的 Dioxus 项目,自动配置 OHOS 支持。

#### 语法

```bash
dx new <PROJECT_NAME> [OPTIONS]
```

#### 选项

| 选项 | 描述 | 默认值 |
|------|------|--------|
| `--template <TEMPLATE>` | 使用指定模板 | `default` |
| `--platform <PLATFORM>` | 默认平台 | `web` |

#### 示例

```bash
# 创建新项目
dx new my-ohos-app

# 创建并设置 OHOS 为默认平台
dx new my-ohos-app --platform ohos
```

### `dx device`

管理连接的 OHOS 设备。

#### `dx device list`

列出所有连接的 OHOS 设备。

```bash
dx device list --platform ohos
```

#### 输出示例

```
Available OHOS devices:
  1. OHOS12345678 - Huawei Mate 60 (API 11)
  2. OHOS87654321 - Emulator (x86_64)
```

### `dx config`

管理 Dioxus 配置。

#### `dx config init`

初始化 `Dioxus.toml` 配置文件。

```bash
dx config init my-app
```

生成的配置包含 OHOS 特定设置:

```toml
[application]
name = "my-app"
default_platform = "ohos"

[ohos]
min_api_level = 9
package = "com.example.myapp"
version_code = 1
version_name = "1.0.0"
```

## 配置选项

### Dioxus.toml

`Dioxus.toml` 是 Dioxus 项目的配置文件。对于 OHOS,你可以在 `[ohos]` 部分添加特定配置。

#### 完整配置示例

```toml
[application]
# 应用名称
name = "My OHOS App"

# 默认平台
default_platform = "ohos"

# 静态资源目录
public_dir = "public"

[web.app]
# HTML 标题
title = "My OHOS App"

[web.resource.dev]
# 开发模式资源
style = []
script = []

[web.resource.release]
# 发布模式资源
style = []
script = []

# OHOS 特定配置
[ohos]
# 最小 API 级别
min_api_level = 9

# 应用包名 (反向域名表示法)
package = "com.example.myapp"

# 版本代码 (整数)
version_code = 1

# 版本名称 (字符串)
version_name = "1.0.0"

# 应用图标路径 (可选)
icon = "assets/icon.png"

# 应用权限 (可选)
# permissions = [
#     "ohos.permission.INTERNET",
#     "ohos.permission.CAMERA"
# ]
```

#### OHOS 配置字段

| 字段 | 类型 | 必需 | 描述 |
|------|------|------|------|
| `min_api_level` | integer | 否 | 最小 API 级别 (默认: 9) |
| `package` | string | 是 | 应用包名 |
| `version_code` | integer | 是 | 版本代码 |
| `version_name` | string | 是 | 版本名称 |
| `icon` | string | 否 | 应用图标路径 |
| `permissions` | array | 否 | 应用权限列表 |

## 环境变量

Dioxus CLI 使用以下环境变量来配置 OHOS 构建环境。

### 必需的环境变量

| 变量 | 描述 | 示例 |
|------|------|------|
| `OHOS_SDK_HOME` | OHOS SDK 根目录 | `/Users/developer/HarmonyOS/Sdk` |
| `OHOS_NDK_HOME` | OHOS NDK 目录 | `/Users/developer/HarmonyOS/Sdk/ohos-ndk` |

### 可选的环境变量

| 变量 | 描述 | 默认值 |
|------|------|--------|
| `HDC_PORT` | hdc (HarmonyOS Device Connector) 端口 | `5037` |
| `OHOS_API_LEVEL` | 覆盖 API 级别 | 从配置读取 |

### 设置环境变量

#### macOS / Linux

在 `~/.bashrc`, `~/.zshrc` 或 `~/.profile` 中:

```bash
export OHOS_SDK_HOME=~/HarmonyOS/Sdk
export OHOS_NDK_HOME=~/HarmonyOS/Sdk/ohos-ndk
export PATH="$PATH:$OHOS_SDK_HOME/toolchains/hdc"
```

#### Windows (PowerShell)

```powershell
$env:OHOS_SDK_HOME="C:\Users\YourName\HarmonyOS\Sdk"
$env:OHOS_NDK_HOME="C:\Users\YourName\HarmonyOS\Sdk\ohos-ndk"
```

或设置系统环境变量。

## 构建输出

### 目录结构

OHOS 构建生成的目录结构:

```
target/
├── ohos/
│   ├── debug/          # 开发构建
│   │   └── app/
│   │       ├── src/
│   │       │   └── main/
│   │       │       ├── assets/     # 应用资源
│   │       │       ├── libs/       # 本地库 (.so)
│   │       │       └── ets/        # ArkTS 代码
│   │       └── build/
│   │           └── outputs/
│   │               └── hap/        # .hap 文件
│   └── release/        # 发布构建
│       └── app/
└── aarch64-unknown-linux-ohos/
    └── debug/
        └── libdioxusmain.so        # Rust 动态库
```

### 输出文件

| 文件 | 位置 | 描述 |
|------|------|------|
| `libdioxusmain.so` | `target/<triple>/debug/` | Rust 代码编译的动态库 |
| `app.hap` | `target/ohos/release/app/build/outputs/hap/` | 可安装的应用包 |

## 设备管理

### 列出设备

使用 `hdc` 工具列出连接的设备:

```bash
hdc list targets
```

或使用 Dioxus CLI:

```bash
dx device list --platform ohos
```

### 检查设备信息

```bash
# 获取设备详细信息
hdc shell "getprop | grep ro.build.version"

# 检查设备架构
hdc shell "uname -m"

# 查看运行中的应用
hdc shell aa dump -l
```

### 安装应用

```bash
# 安装 .hap 文件
hdc install app.hap

# 卸载应用
hdc uninstall com.example.myapp
```

### 查看日志

```bash
# 查看 Dioxus 日志
hdc shell hilog -T Dioxus

# 查看所有日志
hdc shell hilog

# 清除日志
hdc shell hilog -r
```

## 常见工作流程

### 首次设置

```bash
# 1. 创建新项目
dx new my-app
cd my-app

# 2. 配置 OHOS
# 编辑 Dioxus.toml 设置 default_platform = "ohos"

# 3. 运行应用
dx serve --platform ohos
```

### 日常开发

```bash
# 启动开发服务器 (自动热重载)
dx serve --platform ohos

# 如果有多个设备,选择特定设备
dx serve --platform ohos --device <device-id>
```

### 发布应用

```bash
# 1. 构建发布版本
dx build --platform ohos --release

# 2. 打包应用
dx bundle --platform ohos --release

# 3. 在设备上测试
hdc install target/ohos/release/app/build/outputs/hap/release/app.hap

# 4. 提交到应用商店
# (使用华为开发者联盟的上传工具)
```

### 多架构构建

```bash
# 为所有支持的架构构建
for target in aarch64-unknown-linux-ohos armv7a-unknown-linux-ohos x86_64-unknown-linux-ohos; do
    dx build --platform ohos --target $target --release
    dx bundle --platform ohos --target $target --release
done
```

## 故障排除

### 常见错误

#### `error: OHOS_SDK_HOME not set`

```bash
# 解决方案: 设置环境变量
export OHOS_SDK_HOME=/path/to/sdk
```

#### `error: Linker command not found`

```bash
# 解决方案: 确保 NDK 路径正确
echo $OHOS_NDK_HOME

# 或在配置中指定
# Dioxus.toml:
# [ohos]
# ndk_path = "/path/to/ndk"
```

#### `error: Device not found`

```bash
# 检查设备连接
hdc list targets

# 重启 hdc
hdc kill
hdc start
```

### 调试模式

使用 `--verbose` 标志获取详细输出:

```bash
dx serve --platform ohos --verbose
```

### 清理构建

```bash
# 清理所有构建产物
cargo clean

# 清理特定平台
rm -rf target/ohos
```

## 更多信息

- [完整 Dioxus CLI 文档](https://dioxuslabs.com/learn/0.7/cli)
- [OHOS 快速开始指南](./OHOS_GUIDE.md)
- [GitHub Issues](https://github.com/DioxusLabs/dioxus/issues)
- [Discord 社区](https://discord.gg/XgGxMSkvUM)

---

*最后更新: 2026-01-05*
