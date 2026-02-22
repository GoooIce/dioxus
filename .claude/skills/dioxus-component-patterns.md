# Skill: Dioxus 组件模式

## 适用场景
- 编写 Dioxus 组件测试
- 定义组件 Props
- 理解组件生命周期

## 组件定义模式

### 函数组件（推荐）

```rust
// 简单组件（无 Props）
fn MyComponent() -> Element {
    let count = use_signal(|| 0);

    rsx! {
        div { "Count: {count}" }
    }
}

// 带 Props 的组件
#[derive(Props, Clone)]
struct MyComponentProps {
    name: String,
    count: Signal<usize>,
}

fn MyComponent(props: MyComponentProps) -> Element {
    rsx! {
        div { "Hello, {props.name}! Count: {props.count}" }
    }
}
```

### ⚠️ 避免的模式

```rust
// ❌ 旧版 Scope 模式（已弃用）
fn MyComponent(cx: Scope<MyComponentProps>) -> Element {
    let value = cx.props.value;  // 通过 cx.props 访问
}

// ❌ 在函数内嵌套定义组件
fn test_something() {
    let dom = VirtualDom::new(|| {
        fn Child() -> Element { ... }  // 组件定义在闭包内！
        rsx! { Child {} }
    });
}
```

## Props 模式

### 自动派生 PartialEq

```rust
#[derive(Props, Clone)]
struct ChildProps {
    signal: Signal<usize>,
    counter: Rc<RefCell<RunCounter>>,
}

// 手动实现 PartialEq 以控制比较逻辑
impl PartialEq for ChildProps {
    fn eq(&self, other: &Self) -> bool {
        self.signal == other.signal  // 只比较 signal，不比较 counter
    }
}
```

### Props 传递

```rust
// 在父组件中
rsx! {
    Child {
        signal: signal,           // 直接传递 Signal
        counter: props.clone(),   // 克隆 Rc
    }
}
```

## generation() 函数

```rust
use dioxus_core::generation;

fn MyComponent() -> Element {
    match generation() {
        0 => rsx! { "First render" },
        1 => rsx! { "Second render" },
        _ => rsx! { "Later renders" },
    }
}
```

## 组件位置规则

```rust
// ✅ 正确 - 组件定义在测试函数外部
fn Child() -> Element {
    rsx! { "Child" }
}

#[test]
fn test_component() {
    let dom = VirtualDom::new(|| {
        rsx! { Child {} }
    });
}

// ❌ 错误 - 组件定义在测试函数内部
#[test]
fn test_component() {
    fn Child() -> Element {  // 会导致作用域问题
        rsx! { "Child" }
    }
    let dom = VirtualDom::new(|| {
        rsx! { Child {} }
    });
}
```

## to_owned! 宏

```rust
// 在闭包中捕获变量
let counter = Rc::new(RefCell::new(0));

let memo = use_memo({
    to_owned![counter];  // 克隆 counter 到闭包
    move || {
        counter.borrow_mut().increment();
    }
});
```

## 相关文件
- packages/core/src/component.rs
- packages/core-macro/src/props.rs
