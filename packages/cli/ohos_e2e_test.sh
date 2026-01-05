#!/bin/bash

# Dioxus OHOS 端到端测试脚本
# 这个脚本自动化测试的构建部分，生成测试清单供人工验证

set -e  # 遇到错误立即退出

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 配置
PROJECT_NAME="test_ohos_app"
TEST_DIR="examples/${PROJECT_NAME}"
DX_CMD="cargo run --package dx --"

# 日志函数
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

# 清理函数
cleanup() {
    log_info "清理测试环境..."
    if [ -d "$TEST_DIR" ]; then
        log_info "删除测试项目: $TEST_DIR"
        rm -rf "$TEST_DIR"
    fi
}

# 设置陷阱，确保脚本退出时清理
trap cleanup EXIT

# 打印标题
print_header() {
    echo ""
    echo "========================================"
    echo "  Dioxus OHOS 端到端测试"
    echo "========================================"
    echo ""
}

# 检查环境
check_environment() {
    log_info "检查测试环境..."

    # 检查 Rust
    if ! command -v rustc &> /dev/null; then
        log_error "Rust 未安装"
        exit 1
    fi
    log_success "Rust 已安装: $(rustc --version)"

    # 检查 OHOS SDK
    if [ -z "$OHOS_SDK_HOME" ]; then
        log_error "OHOS_SDK_HOME 环境变量未设置"
        exit 1
    fi
    log_success "OHOS SDK: $OHOS_SDK_HOME"

    if [ -z "$OHOS_NDK_HOME" ]; then
        log_error "OHOS_NDK_HOME 环境变量未设置"
        exit 1
    fi
    log_success "OHOS NDK: $OHOS_NDK_HOME"

    # 检查 hdc
    if ! command -v hdc &> /dev/null; then
        log_warning "hdc 工具未找到，请确保已安装并添加到 PATH"
    else
        log_success "hdc 工具: $(hdc --version)"
    fi

    # 检查 OHOS 目标
    if ! rustup target list --installed | grep -q "ohos"; then
        log_warning "OHOS 目标未安装，尝试安装..."
        rustup target add aarch64-unknown-linux-ohos
        rustup target add x86_64-unknown-linux-ohos
    fi
    log_success "OHOS 目标已安装"

    # 检查设备
    log_info "检查 OHOS 设备..."
    if command -v hdc &> /dev/null; then
        DEVICE_COUNT=$(hdc list targets 2>/dev/null | wc -l | tr -d ' ')
        if [ "$DEVICE_COUNT" -gt 0 ]; then
            log_success "找到 $DEVICE_COUNT 个 OHOS 设备"
            hdc list targets
        else
            log_warning "未找到 OHOS 设备，请在 DevEco Studio 中启动模拟器"
        fi
    fi

    echo ""
}

# 测试 1: 创建项目
test_create_project() {
    log_info "测试 1: 创建 OHOS 项目"

    cd examples

    # 删除旧项目（如果存在）
    if [ -d "$PROJECT_NAME" ]; then
        log_warning "测试项目已存在，删除..."
        rm -rf "$PROJECT_NAME"
    fi

    # 创建新项目
    log_info "创建项目: $PROJECT_NAME"
    $DX_CMD new "$PROJECT_NAME"

    if [ -d "$PROJECT_NAME" ]; then
        log_success "项目创建成功"
    else
        log_error "项目创建失败"
        exit 1
    fi

    cd "$PROJECT_NAME"

    # 验证文件结构
    log_info "验证文件结构..."
    if [ -f "Cargo.toml" ]; then
        log_success "  ✓ Cargo.toml 存在"
    else
        log_error "  ✗ Cargo.toml 不存在"
        exit 1
    fi

    if [ -f "Dioxus.toml" ]; then
        log_success "  ✓ Dioxus.toml 存在"
    else
        log_error "  ✗ Dioxus.toml 不存在"
        exit 1
    fi

    if [ -d "src" ]; then
        log_success "  ✓ src 目录存在"
    else
        log_error "  ✗ src 目录不存在"
        exit 1
    fi

    cd ../..
    echo ""
}

# 测试 2: 构建项目
test_build() {
    log_info "测试 2: 构建 OHOS 应用"

    cd "$TEST_DIR"

    # 开发构建
    log_info "执行开发构建..."
    if $DX_CMD build --platform mobile --target ohos; then
        log_success "开发构建成功"

        # 检查构建产物
        if [ -f "target/x86_64-unknown-linux-ohos/debug/libdioxusmain.so" ] || \
           [ -f "target/aarch64-unknown-linux-ohos/debug/libdioxusmain.so" ]; then
            log_success "  ✓ 找到动态库文件"
        else
            log_warning "  ✗ 未找到动态库文件（可能正常，取决于目标架构）"
        fi
    else
        log_error "开发构建失败"
        exit 1
    fi

    # 发布构建
    log_info "执行发布构建..."
    if $DX_CMD build --platform mobile --target ohos --release; then
        log_success "发布构建成功"
    else
        log_error "发布构建失败"
        exit 1
    fi

    cd ../..
    echo ""
}

# 测试 3: 运行单元测试
test_unit_tests() {
    log_info "测试 3: 运行单元测试"

    cd packages/cli

    log_info "运行 OHOS 平台测试..."
    if cargo test platform_ohos; then
        log_success "OHOS 平台测试通过"
    else
        log_error "OHOS 平台测试失败"
        exit 1
    fi

    log_info "运行构建测试套件..."
    if cargo test run_harness; then
        log_success "构建测试套件通过"
    else
        log_error "构建测试套件失败"
        exit 1
    fi

    cd ../..
    echo ""
}

# 生成测试清单
generate_checklist() {
    log_info "生成功能验证清单..."

    cat > "$TEST_DIR/E2E_TEST_CHECKLIST.md" << 'EOF'
# OHOS E2E 测试清单

测试时间: ___________
测试人员: ___________
设备信息: ___________

## 构建验证

- [ ] 项目创建成功
- [ ] 开发构建成功
- [ ] 发布构建成功
- [ ] 单元测试通过

## 部署验证

- [ ] 应用可以安装到设备
- [ ] 应用可以启动
- [ ] 没有启动时的崩溃

## 核心功能验证

### WebView 渲染

- [ ] 应用启动后显示内容
- [ ] 没有空白屏幕
- [ ] 内容正确显示
- [ ] 样式正确应用

### 虚拟 DOM

- [ ] 组件正确渲染
- [ ] DOM 结构符合预期
- [ ] 嵌套组件正确显示
- [ ] 列表渲染正确

### 用户交互

- [ ] 点击按钮有响应
- [ ] 文本输入可以输入
- [ ] 滚动工作正常
- [ ] 复选框/单选框可以选中

### 状态管理

- [ ] use_signal 状态更新正常
- [ ] use_resource 异步数据加载正常
- [ ] use_context 跨组件状态正常
- [ ] 状态更新触发重新渲染

### IPC 通信

- [ ] Rust 到 JS 消息传递正常
- [ ] JS 到 Rust 消息传递正常
- [ ] 没有消息丢失

### 资源加载

- [ ] 图片加载和显示正常
- [ ] CSS 样式应用正常
- [ ] 公共资源可访问

## 热重载验证

- [ ] 代码修改后自动重新加载
- [ ] 热重载后状态保持
- [ ] 没有热重载导致的崩溃

## 性能验证

### 启动时间

- 冷启动时间: _____ 秒（基准: < 5 秒）
- 热启动时间: _____ 秒（基准: < 2 秒）

### 运行性能

- 滚动流畅度: _____ / 5（1=卡顿, 5=非常流畅）
- 动画流畅度: _____ / 5

### 资源使用

- 内存使用: _____ MB
- CPU 使用: _____ %
- 应用大小: _____ MB

## 已知问题

1.
2.
3.

## 备注

_______________________________________________________________
_______________________________________________________________
_______________________________________________________________

## 测试结论

- [ ] 通过所有测试，可以发布
- [ ] 有小问题，但不影响使用
- [ ] 有重要问题，需要修复

签名: ___________ 日期: ___________
EOF

    log_success "测试清单已生成: $TEST_DIR/E2E_TEST_CHECKLIST.md"
    echo ""
}

# 打印下一步
print_next_steps() {
    log_info "自动化测试部分已完成！"
    echo ""
    echo "接下来的步骤:"
    echo ""
    echo "1. 部署应用到设备:"
    echo "   cd $TEST_DIR"
    echo "   $DX_CMD serve --platform mobile --target ohos"
    echo ""
    echo "2. 在设备上验证功能（使用测试清单）:"
    echo "   打开 $TEST_DIR/E2E_TEST_CHECKLIST.md"
    echo ""
    echo "3. 运行回归测试（验证其他平台）:"
    echo "   - Android: $DX_CMD build --platform mobile --target android"
    echo "   - iOS:     $DX_CMD build --platform mobile --target ios"
    echo "   - Desktop: $DX_CMD build --platform desktop"
    echo "   - Web:     $DX_CMD build --platform web"
    echo ""
    echo "4. 填写测试报告"
    echo ""
}

# 主函数
main() {
    print_header

    log_info "开始 OHOS E2E 测试..."
    echo ""

    # 检查环境
    check_environment

    # 运行测试
    test_unit_tests
    test_create_project
    test_build

    # 生成清单
    generate_checklist

    # 打印下一步
    print_next_steps

    log_success "测试准备完成！"
    echo ""
}

# 运行主函数
main
