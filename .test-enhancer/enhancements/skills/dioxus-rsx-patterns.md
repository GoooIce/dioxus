# Skill: RSX 宏使用模式

## 适用场景
- 在 Dioxus 测试中使用 rsx! 宏
- 理解 RSX 中的控制流语法
- 避免常见的 RSX 使用错误

## 基本语法

### 元素渲染

```rust
rsx! {
    div {
        class: "container",
        "Hello, World!"
    }
}
```

### 信号插值

```rust
let signal = use_signal(|| "Hello".to_string());

rsx! {
    // ✅ 正确 - 直接使用信号
    "{signal}"

    // 也正确 - 显式调用
    "{signal()}"
}
```

## for 循环语法

### ✅ 正确用法

```rust
rsx! {
    for id in 0..10 {
        Child {
            key: "{id}",
            index: id,
        }
    }
}

// 或使用迭代器
rsx! {
    for item in items.iter() {
        Item { data: item.clone() }
    }
}
```

### ❌ 错误用法

```rust
// ❌ 错误 - map 返回迭代器，不是 Element
(0..10).map(|id| rsx! {
    Child { index: id }
})

// ❌ 错误 - 在 rsx! 外部构建元素列表
let children: Vec<_> = (0..10).map(|i| {
    rsx! { Child { index: i } }
}).collect();

rsx! {
    div { {children} }  // 语法不正确
}
```

## 条件渲染

```rust
rsx! {
    if show_header {
        header { "Title" }
    }

    if let Some(user) = user {
        div { "Hello, {user.name}" }
    }
}
```

## 组件嵌套

```rust
rsx! {
    Parent {
        header: rsx! {
            h1 { "Title" }
        },
        Child {
            data: value,
        }
    }
}
```

## Props 语法

```rust
// 传递 props
rsx! {
    Child {
        signal: signal,           // 简单传递
        count: 42,                // 字面值
        on_click: move |_| {},    // 闭包
    }
}
```

## 关键规则

1. **for 循环必须在 rsx! 内部** - 不能在 rsx! 外部构建迭代器
2. **信号自动订阅** - 在 rsx! 中直接使用 `{signal}` 会自动建立订阅
3. **key 属性推荐** - 在列表中使用 key 优化 diff 算法

## 测试中的常见模式

```rust
#[test]
fn test_list_rendering() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            for i in 0..10 {
                div { key: "{i}", "Item {i}" }
            }
        }
    });

    dom.rebuild_in_place();
}
```

## 相关文件
- packages/rsx/src/lib.rs
- packages/core-macro/src/rsx.rs
