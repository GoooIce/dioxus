# 测试文件：subscribe.rs

## 测试 1：reading_subscribes

### 目的
验证信号（signal）的读取订阅机制是否正确工作。当父组件中的信号被读取时，所有订阅该信号的子组件应该在信号变化时被正确重新渲染。

### 前置条件
- `tracing_subscriber` 需要初始化（用于调试输出）
- 测试环境支持 `dioxus` 和 `dioxus_signals` crate

### 测试步骤
1. 创建一个 `RunCounter` 结构体，用于追踪父组件和各子组件的运行次数
2. 使用 `VirtualDom::new_with_props()` 创建虚拟 DOM，传入 `counter` 作为 props
3. 在父组件中：
   - 使用 `use_signal()` 创建一个初始值为 0 的信号
   - 检查当前 generation 是否为 1，如果是则将信号值加 1
   - 递增父组件运行计数器
   - 使用 `rsx!` 渲染 10 个子组件，将信号和 counter 作为 props 传递
4. 定义 `ChildProps` 结构体并实现 `PartialEq` trait（基于 signal 比较）
5. 定义 `Child` 子组件：
   - 在子组件中读取并显示传入的信号值
   - 记录每个子组件 scope ID 的运行次数
6. 调用 `dom.rebuild_in_place()` 重建 DOM
7. 验证初始状态：父组件运行 1 次，每个子组件运行 1 次
8. 调用 `dom.mark_dirty(ScopeId::APP)` 标记应用 scope 为脏状态
9. 连续调用 `dom.render_immediate(&mut NoOpMutations)` 两次
10. 验证最终状态：父组件运行 2 次，每个子组件运行 2 次

### 预期结果
- 初始渲染后，父组件和所有 10 个子组件都只运行了 1 次
- 标记为脏并重新渲染后，父组件和所有子组件都运行了 2 次
- 这证明了子组件通过读取父组件传递的 signal 正确建立了订阅关系

### 涉及的 API
- `use_signal()` - 创建响应式信号
- `VirtualDom::new_with_props()` - 创建带 props 的虚拟 DOM
- `current_scope_id()` - 获取当前作用域 ID
- `generation()` - 获取当前渲染代数
- `mark_dirty()` - 标记作用域为需要重新渲染
- `render_immediate()` - 立即执行渲染
- `rebuild_in_place()` - 原地重建 DOM
- `Signal<T>` - 响应式信号类型

### 关键断言
```rust
// 初始状态
assert_eq!(current_counter.parent, 1);
assert_eq!(rerun_count, &1); // 每个子组件

// 重新渲染后
assert_eq!(current_counter.parent, 2);
assert_eq!(rerun_count, &2); // 每个子组件
```
