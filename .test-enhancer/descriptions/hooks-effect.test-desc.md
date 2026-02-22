# 测试文件：effect.rs

## 测试 1：effects_rerun

### 目的
验证 `use_effect` 能够正确地响应依赖信号的变化并重新运行。

### 前置条件
- 创建 `RunCounter` 结构体用于跟踪组件和 effect 的执行次数
- 使用 `Rc<RefCell<>>` 包装计数器以在闭包间共享状态

### 测试步骤
1. 创建 `VirtualDom` 并传入 `Rc<RefCell<RunCounter>>` 作为 props
2. 在组件内部：
   - 使用 `use_signal` 创建一个初始值为 0 的信号
   - 使用 `use_effect` 创建一个 effect，该 effect 读取信号值并递增 `effect` 计数器
   - 将信号值递增 1
   - 使用 `dioxus_core::needs_update()` 手动停止等待
3. 调用 `rebuild_in_place()` 重建 DOM
4. 等待工作完成（超时 500ms）

### 预期结果
- 组件只运行 1 次（`component == 1`）
- effect 运行 1 次（`effect == 1`）

### 涉及的 API
- `use_effect()` - 创建响应式副作用
- `use_signal()` - 创建响应式信号
- `VirtualDom::new_with_props()` - 创建带 props 的虚拟 DOM
- `rebuild_in_place()` - 重建 DOM
- `wait_for_work()` - 等待异步工作完成
- `to_owned![]` - 捕获变量的宏
- `dioxus_core::needs_update()` - 手动触发更新

---

## 测试 2：effects_rerun_without_rerender

### 目的
回归测试 [issue #2347](https://github.com/DioxusLabs/dioxus/issues/2347)。验证 effect 能够在组件不重新渲染的情况下响应信号变化并重新运行。

### 前置条件
- 创建 `RunCounter` 结构体用于跟踪组件和 effect 的执行次数
- 使用 `tokio` 运行时处理异步操作

### 测试步骤
1. 创建 `VirtualDom` 并传入 `Rc<RefCell<RunCounter>>` 作为 props
2. 在组件内部：
   - 使用 `use_signal` 创建一个初始值为 0 的信号
   - 使用 `use_effect` 创建一个 effect，该 effect 读取信号值并递增 `effect` 计数器
   - 使用 `use_future` 创建一个 future，在循环中：
     - 等待 10ms
     - 递增信号值 10 次（从 0 到 10）
3. 调用 `rebuild_in_place()` 重建 DOM
4. 等待工作完成或超时（500ms）

### 预期结果
- 组件只运行 1 次（`component == 1`），证明组件没有重新渲染
- effect 运行 11 次（`effect == 1`）：第一次初始化 + 10 次信号变化

### 涉及的 API
- `use_effect()` - 创建响应式副作用
- `use_signal()` - 创建响应式信号
- `use_future()` - 创建异步 future
- `VirtualDom::new_with_props()` - 创建带 props 的虚拟 DOM
- `rebuild_in_place()` - 重建 DOM
- `wait_for_work()` - 等待异步工作完成
- `to_owned![]` - 捕获变量的宏

---

## 测试文件摘要

该测试文件包含 2 个测试用例，主要验证 `dioxus-hooks` 中 `use_effect` 的核心行为：

1. **基本响应性**：effect 能够响应依赖信号的变化
2. **独立运行**：effect 可以在组件不重新渲染的情况下独立重新运行

这两个测试都强调了 Dioxus 中 effect 的重要特性：它们是响应式的副作用机制，可以订阅信号变化并独立于组件渲染周期运行。第二个测试特别解决了 issue #2347，确认 effect 在没有组件重新渲染的情况下仍然能够正确响应信号变化。
