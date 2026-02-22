# Hook: Dioxus Hooks API Migration Guide

## 触发条件
- 使用 `use_signal(cx, ...)` 或其他 hooks 传入 `cx` 参数
- 使用 `cx.generation()` 或 `cx.scope_id()`
- 使用旧版 `Scope<T>` 组件签名

## 问题模式

### 错误：旧版 Hooks API（需要 cx 参数）
```rust
// ❌ 错误 - 旧版 API
fn app(cx: Scope) -> Element {
    let mut signal = use_signal(cx, || 0);
    let memo = use_memo(cx, move |_| signal.get());
}
```

### 正确：新版 Hooks API（无需 cx 参数）
```rust
// ✅ 正确 - 新版 API
fn app() -> Element {
    let mut signal = use_signal(|| 0);
    let memo = use_memo(move || signal());
}
```

## 关键迁移点

| 旧版 API | 新版 API |
|---------|---------|
| `use_signal(cx, \|\| default)` | `use_signal(\|\| default)` |
| `use_memo(cx, \|\| ...)` | `use_memo(\|\| ...)` |
| `use_effect(cx, \|\| ...)` | `use_effect(\|\| ...)` |
| `cx.generation()` | `generation()` (从 dioxus_core 导入) |
| `cx.scope_id()` | `current_scope_id()` (从 dioxus_core 导入) |

## 必要导入

```rust
use dioxus::prelude::*;
use dioxus_core::{generation, current_scope_id, NoOpMutations};
use dioxus_signals::*;
```

## 组件签名变更

```rust
// ❌ 旧版
fn Child(cx: Scope<ChildProps>) -> Element {
    let value = cx.props.signal;
}

// ✅ 新版
fn Child(props: ChildProps) -> Element {
    let value = props.signal;
}
```

## 相关文件
- packages/signals/tests/*.rs
- packages/core/tests/*.rs
