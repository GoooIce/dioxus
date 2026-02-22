# Router Outlet SSR 测试描述

**文件路径**: `packages/router/tests/via_ssr/outlet.rs`

**测试目的**: 验证路由 Outlet 组件在服务端渲染(SSR)场景下的正确行为，包括嵌套路由、布局组件和动态参数的处理。

---

## 测试结构概述

### 路由定义
```
/                           -> RootIndex
/fixed/                     -> Fixed (layout) + FixedIndex
/fixed/fixed               -> Fixed (layout) + FixedFixed
/:id/                      -> Parameter (layout) + ParameterIndex
/:id/fixed                 -> Parameter (layout) + ParameterFixed
```

### 组件层次
- `App` (根组件) → `Router` → 嵌套布局 → `Outlet` → 子路由组件

---

## 测试用例

### 1. `root_index` (第94-99行)
**测试路径**: `/`

**测试目的**: 验证根路径的基本路由渲染

**预期行为**:
- 渲染 App 组件的 `<h1>App</h1>`
- 渲染 RootIndex 组件的 `<h2>Root Index</h2>`
- 不涉及任何布局或 Outlet

**断言**:
```rust
assert_eq!(html, "<h1>App</h1><h2>Root Index</h2>");
```

---

### 2. `fixed` (第102-107行)
**测试路径**: `/fixed`

**测试目的**: 验证固定路径的嵌套布局和 Outlet 渲染

**路由流程**:
1. 匹配 `/fixed` nest 路由
2. 应用 `Fixed` 布局组件
3. Outlet 渲染 `FixedIndex` 子路由

**预期行为**:
- 渲染 App 组件: `<h1>App</h1>`
- 渲染 Fixed 布局: `<h2>Fixed</h2>`
- Outlet 渲染 FixedIndex: `<h3>Fixed - Index</h3>`

**断言**:
```rust
assert_eq!(html, "<h1>App</h1><h2>Fixed</h2><h3>Fixed - Index</h3>");
```

---

### 3. `fixed_fixed` (第110-115行)
**测试路径**: `/fixed/fixed`

**测试目的**: 验证嵌套固定路径下布局与子路由的渲染

**路由流程**:
1. 匹配 `/fixed` nest 路由
2. 应用 `Fixed` 布局组件
3. Outlet 渲染 `FixedFixed` 子路由

**预期行为**:
- 渲染 App 组件: `<h1>App</h1>`
- 渲染 Fixed 布局: `<h2>Fixed</h2>`
- Outlet 渲染 FixedFixed: `<h3>Fixed - Fixed</h3>`

**断言**:
```rust
assert_eq!(html, "<h1>App</h1><h2>Fixed</h2><h3>Fixed - Fixed</h3>");
```

---

### 4. `parameter` (第118-126行)
**测试路径**: `/18`

**测试目的**: 验证动态参数路由在 SSR 中的正确处理

**路由流程**:
1. 匹配 `/:id` nest 路由，解析 id=18
2. 应用 `Parameter` 布局组件（接收 id 参数）
3. Outlet 渲染 `ParameterIndex` 子路由

**预期行为**:
- 渲染 App 组件: `<h1>App</h1>`
- 渲染 Parameter 布局（包含参数）: `<h2>Parameter 18</h2>`
- Outlet 渲染 ParameterIndex: `<h3>Parameter - Index</h3>`

**断言**:
```rust
assert_eq!(html, "<h1>App</h1><h2>Parameter 18</h2><h3>Parameter - Index</h3>");
```

**关键点**: 动态参数在服务端渲染时能正确传递到布局组件

---

### 5. `parameter_fixed` (第129-137行)
**测试路径**: `/18/fixed`

**测试目的**: 验证动态参数与固定路径组合时的路由渲染

**路由流程**:
1. 匹配 `/:id` nest 路由，解析 id=18
2. 应用 `Parameter` 布局组件（接收 id 参数）
3. Outlet 渲染 `ParameterFixed` 子路由

**预期行为**:
- 渲染 App 组件: `<h1>App</h1>`
- 渲染 Parameter 布局（包含参数）: `<h2>Parameter 18</h2>`
- Outlet 渲染 ParameterFixed: `<h3>Parameter - Fixed</h3>`

**断言**:
```rust
assert_eq!(html, "<h1>App</h1><h2>Parameter 18</h2><h3>Parameter - Fixed</h3>");
```

**关键点**: 动态参数在布局和嵌套子路由间正确传递

---

## 测试覆盖的关键功能

| 功能 | 涉及测试 |
|------|----------|
| 基础路由渲染 | `root_index` |
| 固定路径嵌套布局 | `fixed`, `fixed_fixed` |
| 动态参数路由 | `parameter`, `parameter_fixed` |
| Outlet 组件渲染 | `fixed`, `fixed_fixed`, `parameter`, `parameter_fixed` |
| SSR 渲染正确性 | 全部测试 |

---

## 依赖组件

- **dioxus**: 核心框架
- **dioxus_router**: 路由组件
- **dioxus_history**: HistoryProvider, MemoryHistory
- **dioxus_ssr**: 服务端渲染

---

## 辅助函数

### `prepare(path: impl Into<String>) -> VirtualDom`
创建并初始化 VirtualDom，配置指定路径的路由。
