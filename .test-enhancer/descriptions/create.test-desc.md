# 测试文件：create.rs

## 测试 1：create_signals_global

### 目的
验证在没有 Dioxus 上下文（`Scope`）的情况下创建和使用全局信号的功能，确保信号可以在组件外部创建。

### 前置条件
- Dioxus signals 库已正确导入
- VirtualDom 环境可用

### 测试步骤
1. 创建一个 VirtualDom，包含 10 个 `Child` 组件
2. 在 `Child` 组件内部调用 `create_without_cx()` 函数
3. `create_without_cx()` 函数在没有 Dioxus 上下文的情况下调用 `Signal::new()` 创建信号
4. 在 RSX 中渲染信号的值
5. 执行 `dom.rebuild_in_place()` 重建虚拟 DOM

### 预期结果
- 信号成功在无上下文环境中创建
- 信号值能正确渲染到组件中
- 程序不会因缺少上下文而崩溃

### 涉及的 API
- `Signal::new()` - 创建新信号
- `VirtualDom::new()` - 创建虚拟 DOM
- `rebuild_in_place()` - 原地重建虚拟 DOM

---

## 测试 2：deref_signal

### 目的
验证信号的解引用（dereference）功能，确保可以通过函数调用语法获取信号内部的值。

### 前置条件
- 已创建的信号实例
- Signal 实现了 `Fn` trait 以支持函数调用语法

### 测试步骤
1. 创建 VirtualDom，包含 10 个 `Child` 组件
2. 在 `Child` 组件中使用 `Signal::new()` 创建字符串信号
3. 使用 `signal()` 函数调用语法获取信号的 `Ref`
4. 使用 `&*signal()` 解引用并验证值是否为 "hello world"
5. 执行 `dom.rebuild_in_place()`

### 预期结果
- `signal()` 调用成功返回 `Ref<T>`
- `&*signal()` 解引用后的值等于 "hello world"
- 断言通过，证明信号值正确存储和访问

### 涉及的 API
- `Signal::new()` - 创建新信号
- `signal()` - 函数调用语法获取 Ref
- `&*signal()` - 解引用信号值
- `VirtualDom::new()` - 创建虚拟 DOM
- `rebuild_in_place()` - 原地重建虚拟 DOM

---

## 测试 3：drop_signals

### 目的
验证信号及其包含的值在适当的时机被正确释放（drop），确保内存管理正确且没有资源泄漏。

### 前置条件
- 原子计数器 `SIGNAL_DROP_COUNT` 用于跟踪释放次数
- 自定义类型 `TracksDrops` 实现 `Drop` trait

### 测试步骤
1. 定义静态原子计数器 `SIGNAL_DROP_COUNT` 初始值为 0
2. 定义 `TracksDrops` 结构体，实现 `Drop` trait 来增加计数器
3. 创建 VirtualDom，根据 `generation` 值决定渲染 10 个或 0 个 `Child` 组件
4. 在 `Child` 组件中使用 `use_signal(|| TracksDrops)` 创建包含 `TracksDrops` 的信号
5. 执行 `dom.rebuild_in_place()` 初始构建
6. 标记 `ScopeId::APP` 为脏并执行 `render_immediate()`
7. 验证 `SIGNAL_DROP_COUNT` 的值等于 10

### 预期结果
- 当 generation 为偶数时创建 10 个子组件
- 重新渲染时（generation 变为奇数），子组件被移除
- 信号包含的 `TracksDrops` 值被正确释放
- `SIGNAL_DROP_COUNT` 最终值为 10，证明所有信号都被正确释放

### 涉及的 API
- `use_signal()` - 创建信号并跟踪其生命周期
- `AtomicUsize` - 原子计数器
- `Drop` trait - 自定义释放逻辑
- `VirtualDom::new()` - 创建虚拟 DOM
- `generation()` - 获取当前渲染代数
- `mark_dirty()` - 标记作用域为脏
- `render_immediate()` - 立即重新渲染
