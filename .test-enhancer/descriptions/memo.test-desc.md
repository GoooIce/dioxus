# 测试文件：memo.rs

## 测试 1：memos_rerun

### 目的
验证 `use_memo` 能够正确地响应依赖信号的变化并重新计算。

### 前置条件
- 初始化 tracing_subscriber 日志系统
- 创建 `RunCounter` 结构体用于跟踪组件和 effect 的执行次数

### 测试步骤
1. 创建 `VirtualDom` 并传入 `Rc<RefCell<RunCounter>>` 作为 props
2. 在组件内部：
   - 使用 `use_signal` 创建一个初始值为 0 的信号
   - 使用 `use_memo` 创建一个 memo，该 memo 读取信号的值并递增 `effect` 计数器
   - 断言初始 memo 值为 0
   - 将信号值递增 1
   - 断言 memo 值变为 1
3. 调用 `rebuild_in_place()` 重建 DOM

### 预期结果
- 组件只运行 1 次（`component == 1`）
- memo 的闭包运行 2 次（`effect == 2`）：第一次初始化，第二次响应信号变化

### 涉及的 API
- `use_memo()` - 创建派生信号
- `use_signal()` - 创建响应式信号
- `VirtualDom::new_with_props()` - 创建带 props 的虚拟 DOM
- `rebuild_in_place()` - 重建 DOM
- `to_owned![]` - 捕获变量的宏

---

## 测试 2：memos_prevents_component_rerun

### 目的
验证 memo 能够防止组件不必要的重新渲染，当依赖未变化时组件不应重新运行。

### 前置条件
- 创建父组件和子组件 `Child`
- 子组件 props 实现 `PartialEq` 以比较 signal 是否相等

### 测试步骤
1. 创建 `VirtualDom`，父组件在不同 generation 设置不同的 signal 值：
   - generation 1: 设置 signal = 0
   - generation 2: 设置 signal = 1
2. 子组件 `Child`：
   - 使用 `use_memo` 读取 signal 值并跟踪 memo 执行次数
   - 根据 generation 断言 memo 值
3. 执行 `rebuild_in_place()` 初始化
4. 第一次 `mark_dirty` 和 `render_immediate`
5. 检查 counter 状态
6. 第二次和第三次 `mark_dirty` 和 `render_immediate`
7. 再次检查 counter 状态

### 预期结果
- 第一次渲染后：component 运行 1 次，memo 运行 2 次
- 第二轮渲染后：component 运行 2 次（因 signal 变化触发），memo 运行 3 次
- 额外的 render_immediate 调用不会导致不必要的组件重新运行

### 涉及的 API
- `use_memo()` - 创建派生信号
- `use_signal()` - 创建响应式信号
- `generation()` - 获取当前渲染代数
- `VirtualDom::mark_dirty()` - 标记 scope 为脏
- `VirtualDom::render_immediate()` - 立即渲染
- `PartialEq` trait - Props 比较实现

---

## 测试 3：memos_sync_rerun_after_unrelated_write

### 目的
回归测试 [issue #2990](https://github.com/DioxusLabs/dioxus/issues/2990)。验证在持有写锁的同时读取 memo 不会导致死锁或一致性问题，memo 能够在无关写入后同步重新运行。

### 前置条件
- 使用 `AtomicBool` 静态变量跟踪测试通过状态
- 初始化 `VirtualDom`

### 测试步骤
1. 在组件内创建信号（初始值为 0）
2. 创建 memo，派生逻辑为 `signal() < 2`
3. 在 generation 0：
   - 断言 memo 为 true
   - 将信号递增 1
4. 在 generation 1：
   - 获取信号的写锁
   - 同时读取 memo 值
   - 断言 memo 仍为 true
   - 修改信号值为 2
   - 释放写锁
   - 断言 memo 变为 false
   - 设置 PASSED 标志为 true
5. 执行 `rebuild_in_place()`
6. 执行 `mark_dirty` 和 `render_immediate`
7. 验证 PASSED 为 true

### 预期结果
- 持有写锁时读取 memo 不应死锁
- 写锁释放后 memo 应立即反映新值
- 测试成功完成（PASSED == true）

### 涉及的 API
- `use_memo()` - 创建派生信号
- `use_signal()` - 创建响应式信号
- `Signal::write()` - 获取写锁
- `generation()` - 获取当前渲染代数
- `AtomicBool` - 线程安全的状态标志
- `ScopeId::APP` - 应用根 scope ID

---

## 测试文件摘要

该测试文件包含 3 个测试用例，主要验证 `dioxus-signals` 中 `use_memo` 的核心行为：

1. **基本响应性**：memo 能够响应依赖信号的变化
2. **渲染优化**：memo 防止不必要的组件重新渲染
3. **并发安全**：在持有写锁时读取 memo 不会导致死锁或一致性问题

所有测试都使用 `VirtualDom` 作为测试环境，并通过 `rebuild_in_place()`、`mark_dirty()` 和 `render_immediate()` 等 API 模拟不同的渲染场景。
