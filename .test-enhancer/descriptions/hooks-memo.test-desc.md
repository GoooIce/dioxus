# 测试文件：hooks/tests/memo.rs

## 测试 1：memo_updates

### 目的
验证 `use_memo` 在多线程环境下的响应性和更新机制，确保：
1. Memo 能够正确响应来自其他线程的信号变化
2. Memo 在组件渲染过程中保持最新状态
3. 子组件中的 memo 在元素移除后不再重新运行

### 前置条件
- 使用 `tokio` 异步运行时
- 初始化 `thread_local!` 静态变量用于跨线程信号访问
- VirtualDom 和异步等待环境可用

### 测试步骤
1. 定义 `thread_local!` 静态变量 `VEC_SIGNAL`，持有 `RefCell<Option<Signal<Vec<usize>>>>`
2. 创建 `app()` 组件：
   - 使用 `use_signal_sync()` 创建跨线程同步信号 `vec`，初始值为 `vec![0, 1, 2]`
   - 使用 `use_hook` 注册回调，将 vec 存入 thread_local 并启动后台线程
   - 后台线程在 100ms 后调用 `vec.push(5)`
   - 创建 `len_memo = use_memo(move || vec.len())`
   - 在 generation < 2 时调用 `vec.push(len)`
   - 断言 `vec.len()` 等于 `len_memo()`
   - 渲染多个 `Child` 组件，传入 index 和 vec
3. 创建 `Child` 子组件：
   - 使用 `use_memo(move || vec.read()[index])` 创建 item memo
   - 渲染 `div { "Item: {item}" }`
4. 创建 VirtualDom 并执行测试流程：
   - 调用 `rebuild_in_place()` 初始化
   - 循环调用 `wait_for_work().await` 和 `render_immediate()` 处理更新
   - 断言 signal 最终值为 `vec![0, 1, 2, 3, 4, 5]`
   - 循环 6 次，每次调用 `signal.pop()` 并等待渲染完成
5. 使用 `tokio::select!` 设置 1 秒超时保护

### 预期结果
- 后台线程能够成功修改信号
- Memo 始终反映 vec 的最新长度
- 子元素移除后，对应的 memo 不再重新运行
- 测试在 1 秒内完成，不会超时

### 涉及的 API
- `use_signal_sync()` - 创建跨线程同步信号
- `use_memo()` - 创建派生信号
- `use_hook()` - 注册 hook 回调
- `thread_local!` - 线程局部存储
- `VirtualDom::new()` - 创建虚拟 DOM
- `rebuild_in_place()` - 原地重建 DOM
- `wait_for_work()` - 等待待处理的工作
- `render_immediate()` - 立即渲染
- `generation()` - 获取当前渲染代数

---

## 测试 2：use_memo_only_triggers_one_update

### 目的
验证 `use_memo` 的更新传播机制，确保：
1. 多次连续写入依赖信号时，memo 只触发一次更新
2. 嵌套 memo 的依赖关系正确处理
3. 不会因多次读取导致重复更新订阅者

### 前置条件
- 使用 `tokio` 异步运行时
- 初始化 `thread_local!` 静态变量用于追踪更新次数

### 测试步骤
1. 定义 `thread_local!` 静态变量 `VEC_SIGNAL`，持有 `Vec<usize>` 用于记录更新
2. 创建 `app()` 组件：
   - 使用 `use_signal(|| 0)` 创建 count 信号
   - 创建第一个 memo：`memorized = use_memo(move || count() * 2)`
   - 创建第二个 memo，读取 `memorized` 并将值推入 `VEC_SIGNAL`
   - 使用 `use_hook` 循环 10 次：
     - 每次将 count 加 1
     - 读取 `memorized` 值
3. 创建 VirtualDom 并重建
4. 使用 `tokio::select!` 等待工作完成或超时 100ms
5. 执行 `render_immediate()`
6. 断言 `VEC_SIGNAL` 的值为 `vec![0, 20]`

### 预期结果
- 初始化时 VEC_SIGNAL 记录初始值 0
- 连续 10 次写入后，只触发一次 memo 更新
- 最终值为 20（10 * 2）
- 中间值（2, 4, 6, ... 18）不会被记录

### 涉及的 API
- `use_signal()` - 创建响应式信号
- `use_memo()` - 创建派生信号
- `use_hook()` - 注册 hook 回调
- `thread_local!` - 线程局部存储
- `VirtualDom::new()` - 创建虚拟 DOM
- `rebuild_in_place()` - 原地重建 DOM
- `wait_for_work()` - 等待待处理的工作
- `render_immediate()` - 立即渲染

---

## 测试文件摘要

该测试文件包含 2 个测试用例，主要验证 `dioxus-hooks` 中 `use_memo` 的高级行为：

1. **多线程响应性**：memo 能够正确响应来自其他线程的信号变化，并在组件移除后停止更新
2. **批量更新优化**：多次连续写入依赖信号时，memo 只触发一次更新，避免不必要的重复计算

这两个测试覆盖了 memo 在并发场景和性能优化场景下的核心行为。
