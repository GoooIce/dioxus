# Hook: VirtualDom 测试 API

## 触发条件
- 使用 `dom.render_immediate()` 无参数调用
- 使用 `ScopeId::APP.mark_dirty()` 实例方法
- 测试中缺少 `NoOpMutations` 导入

## 问题模式

### 错误：render_immediate 缺少参数
```rust
// ❌ 错误
dom.render_immediate();
```

### 正确：传入 NoOpMutations
```rust
// ✅ 正确
use dioxus_core::NoOpMutations;
dom.render_immediate(&mut NoOpMutations);
```

### 错误：mark_dirty 调用方式
```rust
// ❌ 错误 - ScopeId 没有 mark_dirty 方法
ScopeId::APP.mark_dirty();
```

### 正确：通过 VirtualDom 调用
```rust
// ✅ 正确
dom.mark_dirty(ScopeId::APP);
```

## VirtualDom 测试完整模式

```rust
use dioxus::prelude::*;
use dioxus_core::{generation, NoOpMutations, ScopeId};

#[test]
fn test_signal_behavior() {
    let mut dom = VirtualDom::new(|| {
        let mut signal = use_signal(|| 0);

        if generation() == 0 {
            signal += 1;
        }

        rsx! { div { "{signal}" } }
    });

    // 初始构建
    dom.rebuild_in_place();

    // 标记需要重新渲染
    dom.mark_dirty(ScopeId::APP);

    // 执行渲染
    dom.render_immediate(&mut NoOpMutations);
}
```

## 关键 API 签名

```rust
// VirtualDom 方法
fn rebuild_in_place(&mut self);
fn mark_dirty(&mut self, scope: ScopeId);
fn render_immediate(&mut self, mutations: &mut impl Mutations);
```

## NoOpMutations 用途
- 在测试中捕获 DOM 变更但不执行实际操作
- 实现 `Mutations` trait 的空实现
- 允许测试验证组件行为而不需要真实 DOM

## 相关文件
- packages/core/src/virtual_dom.rs
- packages/core/src/mutations.rs
