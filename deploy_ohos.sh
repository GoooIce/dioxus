#!/bin/bash

# Dioxus OHOS 部署脚本
# 此脚本将 Dioxus 二进制文件部署到 OHOS 模拟器/设备

set -e

# 配置
DIOXUS_BINARY="target/aarch64-unknown-linux-ohos/debug/examples/hello_world"
OHOS_TMP_DIR="/data/local/tmp"
BINARY_NAME="hello_world"
HDC_TOOL="/Users/devel0per/Library/OpenHarmony/Sdk/20/toolchains/hdc"

echo "🚀 Dioxus OHOS 部署脚本"
echo "========================"

# 检查二进制文件是否存在
if [ ! -f "$DIOXUS_BINARY" ]; then
    echo "❌ 错误: 二进制文件不存在: $DIOXUS_BINARY"
    echo "请先运行: cargo run --example hello_world --target aarch64-unknown-linux-ohos"
    exit 1
fi

echo "✅ 找到二进制文件: $DIOXUS_BINARY"

# 检查 HDC 工具
if [ ! -f "$HDC_TOOL" ]; then
    echo "❌ 错误: HDC 工具不存在: $HDC_TOOL"
    exit 1
fi

echo "✅ HDC 工具: $HDC_TOOL"

# 检查设备连接
echo "📱 检查 OHOS 设备..."
DEVICES=$($HDC_TOOL list targets)
if [ -z "$DEVICES" ]; then
    echo "❌ 错误: 没有连接的 OHOS 设备或模拟器"
    echo "请确保:"
    echo "  1. OHOS 模拟器正在运行"
    echo "  2. 或者 OHOS 设备已通过 USB 连接"
    exit 1
fi

echo "✅ 设备已连接: $DEVICES"

# 推送二进制文件
echo "📤 推送二进制文件到设备..."
$HDC_TOOL file send "$DIOXUS_BINARY" "$OHOS_TMP_DIR/$BINARY_NAME"

# 给予执行权限
echo "🔐 设置执行权限..."
$HDC_TOOL shell chmod 755 "$OHOS_TMP_DIR/$BINARY_NAME"

# 验证文件
echo "✅ 验证文件..."
$HDC_TOOL shell ls -lh "$OHOS_TMP_DIR/$BINARY_NAME"

echo ""
echo "⚠️  重要提示:"
echo "由于 OHOS 的安全限制，直接运行原生二进制文件受到限制。"
echo ""
echo "现在您有两个选择:"
echo ""
echo "选项 1: 在 DevEco Studio 中构建 HAP 包"
echo "  1. 打开项目: helloohos"
echo "  2. Build > Build Hap(s) / APP(s) > Build Hap(s)"
echo "  3. 使用以下命令安装:"
echo "     $HDC_TOOL install entry/build/default/outputs/default/entry-default-signed.hap"
echo ""
echo "选项 2: 尝试直接运行（可能失败）"
echo "  $HDC_TOOL shell $OHOS_TMP_DIR/$BINARY_NAME"
echo ""
echo "选项 3: 查看 DIOXUS_SETUP.md 获取更多详细说明"
echo ""
echo "📝 注意: Dioxus 需要图形界面，因此可能需要 OHOS WebView 支持"
