# router/site_map.rs 测试描述

## 测试文件概述
- **文件路径**: `packages/router/tests/site_map.rs`
- **测试数量**: 1
- **主要功能**: 测试路由的静态站点地图生成功能

---

## 测试详情

### 1. `with_class`

**位置**: 第 3-53 行

**测试目的**:
验证 `Routable` trait 的 `static_routes()` 方法能够正确生成静态路由列表，特别是当路由定义中包含嵌套的子路由（child routes）时。

**测试场景**:
- 定义一个嵌套路由结构 `Route`，包含一个子路由 `ChildRoute`
- `Route` 包含三个路由：
  - `/` - 根路径
  - `/test` - 静态路径
  - `/child` - 嵌套子路由的父路径
- `ChildRoute` 包含：
  - `/` - 子路由根路径
  - `/:not_static` - 动态参数路径

**预期行为**:
`static_routes()` 方法应返回所有静态路由的变体，即：
- `Route::Root {}`
- `Route::Test {}`
- `Route::Nested { child: ChildRoute::ChildRoot {} }`

**断言验证**:
- 验证返回的静态路由列表包含三个路由变体
- 动态路由（`/:not_static`）不应出现在静态路由列表中

---

## 依赖的类型和 trait

- **`Routable`**: Dioxus router 的核心 trait，用于定义路由结构
- **`#[route]`**: 用于标记路由路径的属性宏
- **`#[child]`**: 用于标记嵌套子路由的属性宏
- **`#[component]`**: 用于定义组件的属性宏

---

## 关键验证点

1. 静态路由识别正确性
2. 嵌套路由的静态变体展开
3. 动态路由被正确排除
