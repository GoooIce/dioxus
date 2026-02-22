# Dioxus 异步 Hooks 使用指南

## 触发条件
当编写涉及 `use_future`、`use_effect`、`use_coroutine` 等 Dioxus 异步 hooks 的测试或代码时触发。

## 关键模式

### 1. use_future 闭包语法

**错误写法**:
```rust
use_future({
    to_owned![counter, mut signal];
    async move {
        // ...
    }
});
```

**正确写法**:
```rust
use_future(move || async move {
    for i in 0..10 {
        signal += 1;
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
});
```

**关键点**:
- `use_future` 接受一个返回 Future 的闭包
- 闭包语法是 `move || async move { }`，不是带 `to_owned!` 的块
- 内部的 async 块也需要 `move` 关键字

### 2. use_effect 中的 Signal 订阅

**错误写法**:
```rust
use_effect(move || {
    let _ = signal();  // 显式调用但无实际使用
    counter.borrow_mut().effect += 1;
});
```

**正确写法**:
```rust
use_effect(move || {
    counter.borrow_mut().effect += 1;
    println!("Signal: {:?}", signal);  // Debug 格式化会建立依赖
    dioxus_core::needs_update();
});
```

**关键点**:
- 在 effect 闭包中使用 Signal（任何方式）都会自动建立依赖
- `Debug` 格式化（`{:?}`）也能建立依赖
- 调用 `needs_update()` 标记组件需要重新渲染

### 3. 等待异步操作完成

**错误写法**:
```rust
vdom.wait_for_work().timeout(Duration::from_millis(500)).run();
```

**正确写法**:
```rust
tokio::select! {
    _ = dom.wait_for_work() => {}
    _ = tokio::time::sleep(Duration::from_millis(500)) => panic!("timed out")
};
```

**关键点**:
- `wait_for_work()` 返回 `impl Future`，不是带 `.timeout()` 方法的构建器
- 使用 `tokio::select!` 宏来处理超时
- 需要在 `#[tokio::test]` 标记的测试中使用

### 4. 异步测试属性

**错误写法**:
```rust
#[test]
fn effects_rerun() {
    // ...
}
```

**正确写法**:
```rust
#[tokio::test]
async fn effects_rerun() {
    // ...
}
```

**关键点**:
- 使用 `.await` 的测试必须是 `async` 函数
- 必须使用 `#[tokio::test]` 属性而非 `#[test]`

## 常见错误对照表

| 错误 | 正确 | 说明 |
|------|------|------|
| `use_future({ ... async move })` | `use_future(move \|\| async move)` | 闭包语法 |
| `wait_for_work().timeout().run()` | `tokio::select! { ... }` | 超时处理 |
| `#[test]` 异步函数 | `#[tokio::test]` | 测试属性 |
| 显式 `signal()` 不使用 | 自然使用 Signal 值 | 依赖建立 |

## 测试模板

```rust
#[tokio::test]
async fn test_async_hook() {
    let mut dom = VirtualDom::new(app);

    dom.rebuild_in_place();

    // 触发异步操作
    dom.handle_effectful_events();

    // 等待工作完成或超时
    tokio::select! {
        _ = dom.wait_for_work() => {}
        _ = tokio::time::sleep(Duration::from_millis(500)) => {
            panic!("Test timed out")
        }
    };
}

fn app() -> Element {
    let mut count = use_signal(|| 0);

    use_future(move || async move {
        count += 1;
    });

    rsx! { "{count}" }
}
```

## 相关资源
- [Dioxus Async Documentation](https://dioxuslabs.com/learn/0.5/reference/async)
- [Tokio Select Macro](https://tokio.rs/tokio/tutorial/select)
