# Dioxus Router 模式指南

## 触发条件
当编写涉及 `dioxus-router`、路由定义、嵌套路由、`Routable` trait 的测试或代码时触发。

## 路由定义语法

### 1. 嵌套路由语法

**使用 #[child] 语法**:
```rust
#[derive(Routable, Clone, PartialEq, Debug)]
enum Route {
    #[route("/")]
    Home {},

    #[route("/blog/:id")]
    BlogPost { id: usize },

    #[child("/admin")]
    AdminRoute {
        #[route("/")]
        Dashboard {},

        #[route("/users")]
        Users {},
    },
}
```

**注意**: 不要混淆 `#[child]` 和 `#[nest]`

| 语法 | 用途 | 子路由前缀 |
|------|------|-----------|
| `#[child("/path")]` | 内联定义嵌套路由 | 继承父路径 |
| `#[nest("/path")]...#[end_nest]` | 代码块嵌套 | 需要显式闭合 |

### 2. 组件与 Props 的关系

**正确的 App 组件模式**:
```rust
#[component]
fn App<R: Routable + Clone + 'static>(route: Option<R>) -> Element {
    rsx! {
        Router::<R>::new(route)
    }
}

#[derive(Props, Clone, PartialEq)]
struct AppProps<R: Routable + Clone + 'static> {
    route: Option<R>,
}
```

**关键点**:
- 使用 `#[component]` 宏标记组件函数
- 使用 `#[derive(Props)]` 派生 Props trait
- 泛型参数需要适当的 trait 约束

### 3. PhantomData 类型标记

**使用场景**: 当需要在运行时持有类型信息但不存储实际值时

```rust
use std::marker::PhantomData;

struct RouterConfig<R: Routable> {
    prefix: String,
    _marker: PhantomData<R>,  // 持有 R 的类型信息
}

impl<R: Routable> RouterConfig<R> {
    fn new(prefix: &str) -> Self {
        Self {
            prefix: prefix.to_string(),
            _marker: PhantomData,
        }
    }
}
```

**为什么需要 PhantomData**:
- Rust 会移除未使用的泛型参数
- PhantomData 告诉编译器 "这个类型很重要"
- 用于类型安全检查和 trait 实现

## Routable Trait 约束

### 基本约束
```rust
where
    R: Routable + Clone + PartialEq + Debug
```

### 关于 'static 约束
- 不是所有场景都需要 `'static`
- 只在需要存储类型或跨越异步边界时添加
- 原始测试可能不添加，Agent 不应过度约束

## 常用测试模式

### SSR 路由测试
```rust
#[test]
fn test_route_rendering() {
    #[derive(Routable, Clone, PartialEq, Debug)]
    enum Route {
        #[route("/")]
        Home {},
        #[route("/about")]
        About {},
    }

    #[component]
    fn App(route: Option<Route>) -> Element {
        rsx! {
            Router::<Route> {
                route: route,
                Home => rsx! { "Home" },
                About => rsx! { "About" },
            }
        }
    }

    let mut dom = VirtualDom::new_with_props(App, AppProps {
        route: Some(Route::Home {}),
    });
    dom.rebuild(&mut NoOpMutations);

    let html = dioxus_ssr::render(&dom);
    assert!(html.contains("Home"));
}
```

### 嵌套路由测试
```rust
#[test]
fn test_nested_routes() {
    #[derive(Routable, Clone, PartialEq, Debug)]
    enum Route {
        #[route("/")]
        Home {},

        #[child("/admin")]
        Admin {
            #[route("/")]
            Dashboard {},
            #[route("/users")]
            Users {},
        },
    }

    // 测试嵌套路由
    let admin_route = Route::Admin { child: AdminRoute::Dashboard {} };
    // ...
}
```

## 常见错误对照表

| 错误 | 正确 | 说明 |
|------|------|------|
| `#[nest]...#[end_nest]` | `#[child("/path")]` | 语法选择 |
| 无 PhantomData | 添加 `_marker: PhantomData<R>` | 类型安全 |
| 过度添加 `'static` | 按需添加 | 约束精确性 |
| 组件无 Props 派生 | `#[derive(Props)]` | 组件规范 |

## Outlet 组件用法

```rust
#[component]
fn Layout() -> Element {
    rsx! {
        header { "Navigation" }
        main {
            Outlet::<Route> {}  // 渲染子路由
        }
        footer { "Footer" }
    }
}
```

## 相关资源
- [Dioxus Router Documentation](https://dioxuslabs.com/learn/0.5/router)
- [Routable Trait](https://docs.rs/dioxus-router/latest/dioxus_router/trait.Routable.html)
- [PhantomData Pattern](https://doc.rust-lang.org/std/marker/struct.PhantomData.html)
