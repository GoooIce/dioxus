# Dioxus SSR 测试指南

## 触发条件
当编写涉及服务器端渲染 (SSR)、hydration、或 `dioxus_ssr` crate 的测试时触发。

## 关键 API

### 1. NoOpMutations - 用于收集 mutations

**错误写法**:
```rust
dom.rebuild();  // 无参数
```

**正确写法**:
```rust
dom.rebuild(&mut dioxus_core::NoOpMutations);
```

**说明**:
- `NoOpMutations` 是一个特殊的 MutationObserver 实现
- 它收集所有 mutations 但不实际应用
- 用于测试场景中验证渲染差异

### 2. SSR 渲染函数选择

| 函数 | 用途 | 输出 |
|------|------|------|
| `dioxus_ssr::pre_render()` | Hydration 预渲染 | 带 data-dioxus-id 属性的 HTML |
| `dioxus_ssr::render()` | 纯 SSR 渲染 | 干净的 HTML |

**Hydration 测试使用 pre_render**:
```rust
let html = dioxus_ssr::pre_render(&dom);
assert!(html.contains("data-dioxus-id"));
```

**纯 SSR 测试使用 render**:
```rust
let html = dioxus_ssr::render(&dom);
assert!(html.contains("<div>"));
```

### 3. rsx 属性简写语法

**错误写法**:
```rust
rsx! {
    div {
        style: "width:100px;height:50px;",
        "Content"
    }
}
```

**正确写法**:
```rust
rsx! {
    div {
        width: "100px",
        height: "50px",
        "Content"
    }
}
```

**关键点**:
- Dioxus 支持常用 CSS 属性作为直接属性
- `width`, `height`, `color`, `background` 等都可以直接使用
- 动态值使用花括号：`width: "{dynamic}px"`

### 4. 组件定义方式

**测试中的简单组件**:
```rust
// 可以不使用 #[component] 宏
fn Child() -> Element {
    rsx! { "Child content" }
}
```

**需要 Props 的组件**:
```rust
#[component]
fn Child(name: String) -> Element {
    rsx! { "Hello, {name}" }
}
```

**关键点**:
- 简单组件可以定义为普通函数
- 需要 Props 时使用 `#[component]` 宏
- 测试中可以在函数内部定义嵌套组件

## 完整测试模板

### Hydration 测试
```rust
#[test]
fn test_hydration() {
    fn app() -> Element {
        let count = use_signal(|| 0);
        rsx! {
            div {
                width: "100px",
                "Count: {count}"
            }
        }
    }

    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);

    let html = dioxus_ssr::pre_render(&dom);

    // 验证 hydration 标记存在
    assert!(html.contains("data-dioxus-id"));
    // 验证内容正确
    assert!(html.contains("Count: 0"));
}
```

### SSR 转义测试
```rust
#[test]
fn test_escape() {
    fn app() -> Element {
        let dangerous = "<script>alert('xss')</script>";
        rsx! {
            div {
                "{dangerous}"  // 应该被转义
            }
        }
    }

    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);

    let html = dioxus_ssr::render(&dom);

    // 尖括号应该被转义
    assert!(html.contains("&lt;script&gt;"));
    assert!(!html.contains("<script>"));
}
```

### 精确匹配 vs 包含检查

**推荐：精确匹配（用于关键输出）**:
```rust
assert_eq!(
    dioxus_ssr::pre_render(&dom),
    r#"<div data-dioxus-id="1">Hello</div>"#
);
```

**包含检查（用于部分验证）**:
```rust
assert!(html.contains("Hello"));
```

## 常见错误对照表

| 错误 | 正确 | 说明 |
|------|------|------|
| `dom.rebuild()` | `dom.rebuild(&mut NoOpMutations)` | 测试需要收集 mutations |
| `render()` for hydration | `pre_render()` | hydration 需要特殊标记 |
| `style: "width:100px"` | `width: "100px"` | 属性简写 |
| 忘记转义验证 | 检查 `&lt;` `&gt;` | XSS 防护 |

## 相关资源
- [Dioxus SSR Guide](https://dioxuslabs.com/learn/0.5/guides/ssr)
- [Hydration Documentation](https://dioxuslabs.com/learn/0.5/reference/hydration)
