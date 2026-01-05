# Dioxus OHOS 端到端测试指南

本指南提供了完整的 OHOS (OpenHarmony) 应用端到端测试流程，确保从构建到部署的所有功能正常工作。

## 目录

- [测试环境准备](#测试环境准备)
- [测试场景](#测试场景)
- [自动化测试](#自动化测试)
- [手动测试流程](#手动测试流程)
- [功能验证清单](#功能验证清单)
- [性能验证](#性能验证)
- [回归测试](#回归测试)
- [测试报告](#测试报告)

## 测试环境准备

### 必需环境

1. **DevEco Studio** 4.0+ （包含 OHOS SDK 和 NDK）
2. **Rust** 工具链和 OHOS 目标
3. **HDC** (HarmonyOS Device Connector) 工具
4. **OHOS 模拟器** 或 **真实设备**

### 环境变量验证

在开始测试前，确保以下环境变量已正确设置：

```bash
# 验证 OHOS SDK
echo $OHOS_SDK_HOME
# 应输出: /path/to/HarmonyOS/Sdk

# 验证 OHOS NDK
echo $OHOS_NDK_HOME
# 应输出: /path/to/HarmonyOS/Sdk/ohos-ndk

# 验证 hdc 工具
hdc --version
# 应输出版本号

# 验证 Rust 目标
rustup target list | grep ohos
# 应包含: aarch64-unknown-linux-ohos, armv7a-unknown-linux-ohos, x86_64-unknown-linux-ohos
```

### 设备连接验证

```bash
# 列出所有连接的设备
hdc list targets

# 预期输出至少一个设备 ID，例如：
# <device_id>
```

如果没有设备，启动 DevEco Studio 中的模拟器。

## 测试场景

### 场景 1: 创建测试应用

**目标**: 验证可以使用 CLI 创建新的 OHOS 应用

**步骤**:

```bash
# 1. 在 examples 目录中创建测试应用
cd examples
dx new test_ohos_app

# 2. 进入项目目录
cd test_ohos_app

# 3. 检查生成的文件结构
ls -la
# 应包含: Cargo.toml, Dioxus.toml, src/
```

**预期结果**:
- 项目目录成功创建
- `Cargo.toml` 包含 dioxus 依赖
- `Dioxus.toml` 包含 OHOS 配置

### 场景 2: 构建 OHOS 应用

**目标**: 验证应用可以成功构建为 OHOS 格式

**步骤**:

```bash
# 开发构建
dx build --platform mobile --target ohos

# 发布构建
dx build --platform mobile --target ohos --release
```

**预期结果**:
- 构建成功，无编译错误
- 生成的文件位于: `target/aarch64-unknown-linux-ohos/` 或 `target/x86_64-unknown-linux-ohos/`
- 生成了 `.so` 动态库文件

### 场景 3: 部署到模拟器/设备

**目标**: 验证应用可以部署到 OHOS 设备并运行

**步骤**:

```bash
# 启动开发服务器并部署
dx serve --platform mobile --target ohos

# 如果有多个设备，指定设备
dx serve --platform mobile --target ohos --device <device_id>
```

**预期结果**:
- 应用自动安装到设备
- 应用自动启动
- 终端显示 "App successfully started"

## 自动化测试

### 单元测试

Dioxus CLI 包含了 OHOS 的构建级别单元测试：

```bash
# 运行所有 CLI 测试
cd packages/cli
cargo test run_harness

# 运行特定的 OHOS 平台测试
cargo test platform_ohos
```

**测试覆盖**:
- ✅ 平台识别 (`--platform ohos`)
- ✅ 目标三元组 (模拟器: `x86_64-unknown-linux-ohos`, 设备: `aarch64-unknown-linux-ohos`)
- ✅ Bundle 格式 (`BundleFormat::Ohos`)
- ✅ 全栈项目中的 OHOS 支持
- ✅ 默认特性禁用时的 OHOS 构建

### CI/CD 集成

可以将这些测试集成到 CI/CD 流程中：

```yaml
# .github/workflows/ohos-test.yml 示例
name: OHOS Tests
on: [push, pull_request]
jobs:
  test:
    runs-on: macos-latest
    steps:
      - uses: actions/checkout@v3
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: aarch64-unknown-linux-ohos
      - name: Run CLI tests
        run: cargo test --package dx-cli run_harness
```

## 手动测试流程

### 预测试检查

在开始手动测试前，完成以下检查清单：

- [ ] DevEco Studio 已安装并可启动
- [ ] OHOS SDK 路径正确配置
- [ ] hdc 工具可以检测到设备
- [ ] Rust 目标已安装
- [ ] 环境变量已设置

### 测试执行

#### 1. 创建测试应用

```bash
cd examples
dx new test_ohos_app
cd test_ohos_app
```

#### 2. 验证项目配置

检查 `Dioxus.toml`:

```toml
[application]
name = "test_ohos_app"

[ohos]
min_api_level = 9
package = "com.example.test_ohos_app"
version_code = 1
version_name = "1.0.0"
```

#### 3. 构建应用

```bash
# 开发构建
dx build --platform mobile --target ohos

# 检查构建产物
ls -la target/*/debug/libdioxusmain.so
```

#### 4. 部署和运行

```bash
# 启动开发服务器
dx serve --platform mobile --target ohos
```

观察终端输出，确认：
- 设备检测成功
- 应用安装成功
- 应用启动成功
- 没有运行时错误

#### 5. 修改代码并验证热重载

在 `src/main.rs` 中进行简单修改，观察：

```rust
// 修改前
rsx! { "hello world!" }

// 修改后
rsx! { "hello from OHOS!" }
```

保存文件后，应用应该自动重新加载并显示新文本。

## 功能验证清单

在运行的应用中验证以下功能：

### 核心 Dioxus 功能

- [ ] **WebView 渲染**
  - 应用启动后显示内容
  - 没有空白屏幕
  - 内容正确显示

- [ ] **虚拟 DOM**
  - 组件正确渲染
  - DOM 结构符合预期
  - 嵌套组件正确显示

- [ ] **用户交互**
  - [ ] 点击按钮响应
  - [ ] 文本输入接受输入
  - [ ] 滚动工作正常
  - [ ] 手势识别（如果实现）

- [ ] **状态管理**
  - [ ] `use_signal` 状态更新
  - [ ] `use_resource` 异步数据加载
  - [ ] `use_context` 跨组件状态
  - [ ] 状态更新触发重新渲染

- [ ] **IPC 通信**
  - Rust 到 JS 的消息传递
  - JS 到 Rust 的消息传递
  - 没有消息丢失

- [ ] **资源加载**
  - [ ] 图片加载和显示
  - [ ] CSS 样式应用
  - [ ] 字体加载
  - [ ] 公共资源访问

### OHOS 特定功能

- [ ] **应用生命周期**
  - 应用启动
  - 应用暂停/恢复
  - 应用关闭

- [ ] **权限处理**
  - 如果需要权限，正确请求
  - 权限被拒绝时的处理

- [ ] **深链接**（如果实现）
  - 可以通过 URL 启动应用
  - 参数正确解析

## 性能验证

### 测试指标

记录以下性能指标：

#### 启动时间

```bash
# 使用 hdc 测量应用启动时间
hdc shell am start -W com.example.test_ohos_app/.EntryAbility
```

记录从命令执行到应用完全加载的时间。

**基准**:
- 冷启动: < 5 秒
- 热启动: < 2 秒

#### 渲染性能

观察应用的流畅度：
- 滚动帧率应接近 60fps
- 复杂列表应流畅滚动
- 动画应平滑

#### 内存使用

```bash
# 检查应用内存使用
hdc shell ps -A | grep test_ohos_app
```

记录以下指标：
- 应用启动后内存占用
- 运行 10 分钟后的内存占用
- 是否有内存泄漏

#### APK/HAP 大小

```bash
# 检查构建产物大小
ls -lh target/ohos/release/app/build/outputs/hap/
```

**基准**:
- 空白应用: < 10 MB
- 包含资源的应用: 根据资源大小

### 性能测试脚本

创建自动化性能测试：

```bash
#!/bin/bash
# performance_test.sh

echo "=== OHOS Performance Test ==="

# 启动应用
START_TIME=$(date +%s%N)
dx serve --platform mobile --target ohos --release &
SERVE_PID=$!

# 等待应用启动
sleep 30

# 测量内存
echo "Memory usage:"
hdc shell "dumpsys meminfo com.example.test_ohos_app | grep TOTAL"

# 清理
kill $SERVE_PID

echo "=== Test Complete ==="
```

## 回归测试

在添加 OHOS 支持后，必须验证其他平台仍然工作。

### Android 测试

```bash
# 构建 Android
dx build --platform mobile --target android --device

# 运行 Android
dx serve --platform mobile --target android --device
```

**验证**:
- [ ] Android 应用可以构建
- [ ] Android 应用可以运行
- [ ] Android 热重载工作

### iOS 测试

```bash
# 构建 iOS
dx build --platform mobile --target ios

# 运行 iOS
dx serve --platform mobile --target ios
```

**验证**:
- [ ] iOS 应用可以构建
- [ ] iOS 应用可以运行
- [ ] iOS 热重载工作

### Desktop 测试

```bash
# 构建 Desktop
dx build --platform desktop

# 运行 Desktop
dx serve --platform desktop
```

**验证**:
- [ ] Desktop 应用可以构建
- [ ] Desktop 应用可以运行
- [ ] Desktop 热重载工作

### Web 测试

```bash
# 构建 Web
dx build --platform web

# 运行 Web
dx serve --platform web
```

**验证**:
- [ ] Web 应用可以构建
- [ ] Web 应用可以运行
- [ ] Web 热重载工作

## 测试报告

完成测试后，创建详细的测试报告。

### 报告模板

```markdown
# OHOS E2E 测试报告

**测试日期**: YYYY-MM-DD
**测试人员**: Your Name
**Dioxus 版本**: 0.7.x
**OHOS SDK 版本**: API Level XX
**测试设备**: Emulator/Device Model

## 测试摘要

- 总测试项: XX
- 通过: XX
- 失败: XX
- 跳过: XX

## 构建测试

| 测试项 | 状态 | 备注 |
|--------|------|------|
| 创建项目 | ✅/❌ | |
| 开发构建 | ✅/❌ | |
| 发布构建 | ✅/❌ | |
| 目标三元组 | ✅/❌ | |

## 功能测试

### 核心功能

| 功能 | 状态 | 备注 |
|------|------|------|
| WebView 渲染 | ✅/❌ | |
| 虚拟 DOM | ✅/❌ | |
| 用户交互 | ✅/❌ | |
| 状态管理 | ✅/❌ | |
| IPC 通信 | ✅/❌ | |
| 资源加载 | ✅/❌ | |

## 性能测试

| 指标 | 测量值 | 基准 | 状态 |
|------|--------|------|------|
| 冷启动时间 | X 秒 | < 5 秒 | ✅/❌ |
| 内存使用 | X MB | < 100 MB | ✅/❌ |
| 应用大小 | X MB | < 10 MB | ✅/❌ |

## 回归测试

| 平台 | 构建 | 运行 | 备注 |
|------|------|------|------|
| Android | ✅/❌ | ✅/❌ | |
| iOS | ✅/❌ | ✅/❌ | |
| Desktop | ✅/❌ | ✅/❌ | |
| Web | ✅/❌ | ✅/❌ | |

## 发现的问题

1. **[问题标题]**
   - 描述: ...
   - 复现步骤: ...
   - 预期行为: ...
   - 实际行为: ...
   - 严重性: High/Medium/Low

## 建议

- [建议 1]
- [建议 2]

## 结论

[总体评估和下一步行动]
```

## 常见问题排查

### 问题: 构建失败

**错误**: `OHOS_SDK_HOME not set`

**解决方案**:
```bash
export OHOS_SDK_HOME=/path/to/HarmonyOS/Sdk
export OHOS_NDK_HOME=/path/to/HarmonyOS/Sdk/ohos-ndk
```

### 问题: 设备未检测到

**错误**: `No devices found`

**解决方案**:
```bash
# 检查设备连接
hdc list targets

# 重启 hdc 服务
hdc kill
hdc start

# 在 DevEco Studio 中启动模拟器
```

### 问题: 应用启动但显示空白

**可能原因**:
1. WebView 初始化失败
2. 网络权限缺失
3. 资源加载失败

**排查步骤**:
```bash
# 查看应用日志
hdc shell hilog -T Dioxus

# 检查应用权限
hdc shell bm dump -n com.example.test_ohos_app
```

## 下一步

完成测试后：

1. **提交测试报告**: 将报告添加到项目的文档
2. **修复发现的问题**: 创建 GitHub Issues 跟踪
3. **更新文档**: 如果发现文档不足
4. **庆祝**: 🎉 你完成了 OHOS E2E 测试！

---

*最后更新: 2026-01-05*
