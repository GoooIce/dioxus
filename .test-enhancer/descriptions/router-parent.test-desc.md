# 测试文件：parent.rs

## 文件概述

本测试文件主要测试 Dioxus Router 的父子路由关系功能。测试涵盖了嵌套路由（nest）、布局（layout）以及路由之间的层级关系。

### 路由结构

测试使用 `#[derive(Routable)]` 定义了一个复杂的路由树，包含：
- 根路由 `/` (RootIndex)
- 固定嵌套 `/fixed` (Fixed 布局)
  - `/fixed/` (FixedIndex)
  - `/fixed/fixed` (FixedFixed)
- 参数化嵌套 `/:id` (Parameter 布局)
  - `/:id/` (ParameterIndex)
  - `/:id/fixed` (ParameterFixed)
- Hash 嵌套 `/hash`
  - `/hash/` (HashIndex)
  - `/hash/:id` 带查询参数 `?:query` (HashId)
  - `/hash/:id/path` 带 query 和 hash (HashQuery)

---

## 测试 1：get_parent

### 目的
验证 `parent()` 方法能正确返回每个路由的父级路由。

### 测试步骤

1. **根路由无父级测试**
   - 调用 `Route::RootIndex {}.parent()`
   - 验证返回 `None`

2. **固定嵌套父级测试**
   - 测试 `FixedIndex` 的父级应为 `RootIndex`
   - 测试 `FixedFixed` 的父级应为 `FixedIndex`

3. **参数化嵌套父级测试**
   - 测试 `ParameterIndex { id: 0 }` 的父级应为 `RootIndex`
   - 测试 `ParameterFixed { id: 0 }` 的父级应为 `ParameterIndex { id: 0 }`

4. **Hash 和 Query 嵌套父级测试**
   - 测试 `HashQuery` 的父级应为 `HashId`（query 为空字符串）
   - 测试 `HashId` 的父级应为 `HashIndex`
   - 测试 `HashIndex` 的父级应为 `RootIndex`

### 预期结果

- 根路由没有父级（返回 `None`）
- 所有子路由都正确返回其直接父级路由
- 带参数的路由在获取父级时保留参数值

### 涉及的 API

- `#[derive(Routable)]` - 路由派生宏
- `#[route]` - 路由定义属性
- `#[nest]` / `#[end_nest]` - 嵌套路由定义
- `#[layout]` / `#[end_layout]` - 布局定义
- `Route::parent()` - 获取父级路由的方法

### 关键断言

```rust
assert_eq!(Route::RootIndex {}.parent(), None);
assert_eq!(Route::FixedIndex {}.parent(), Some(Route::RootIndex {}));
assert_eq!(Route::FixedFixed {}.parent(), Some(Route::FixedIndex {}));
```

---

## 测试 2：is_child

### 目的
验证 `is_child_of()` 方法能正确判断两个路由之间的父子关系。

### 测试步骤

1. **自反性测试（路由不是自己的子级）**
   - 验证 `RootIndex` 不是 `RootIndex` 的子级
   - 验证 `FixedIndex` 不是自己的子级
   - 验证 `FixedFixed` 不是自己的子级

2. **固定嵌套父子关系测试**
   - 验证 `FixedIndex` 是 `RootIndex` 的子级
   - 验证 `FixedFixed` 是 `FixedIndex` 的子级

3. **参数化嵌套父子关系测试**
   - 验证 `ParameterIndex { id: 0 }` 是 `RootIndex` 的子级
   - 验证 `ParameterIndex` 不是自己的子级
   - 验证 `ParameterFixed { id: 0 }` 是 `ParameterIndex { id: 0 }` 的子级

4. **Hash 和 Query 嵌套父子关系测试**
   - 验证 `HashQuery` 是 `HashId` 的子级（相同 id 和 query）
   - 验证 `HashQuery` 不是自己的子级
   - 验证 `HashId` 是 `HashIndex` 的子级
   - 验证 `HashId` 不是自己的子级
   - 验证 `HashIndex` 是 `RootIndex` 的子级
   - 验证 `HashIndex` 不是自己的子级

### 预期结果

- 路由永远不会是自己的子级（返回 `false`）
- 直接子路由正确返回 `true`
- 父子关系考虑路由参数（相同参数值才认为是父子关系）

### 涉及的 API

- `Route::is_child_of(&Route)` - 判断当前路由是否为指定路由的子级

### 关键断言

```rust
// 自反性：路由不是自己的子级
assert!(!Route::RootIndex {}.is_child_of(&Route::RootIndex {}));
assert!(!Route::FixedIndex {}.is_child_of(&Route::FixedIndex {}));

// 正常父子关系
assert!(Route::FixedIndex {}.is_child_of(&Route::RootIndex {}));
assert!(Route::FixedFixed {}.is_child_of(&Route::FixedIndex {}));
```

---

## 辅助组件

本文件还定义了多个辅助组件，用于支持路由的渲染：

### 组件列表

1. **RootIndex** - 根索引页组件
2. **Fixed** - 固定布局组件（包含 `<Outlet<Route>>`）
3. **FixedIndex** - 固定嵌套索引页组件
4. **FixedFixed** - 固定嵌套固定路径组件
5. **Parameter(id: u8)** - 参数化布局组件（包含 `<Outlet<Route>>`）
6. **ParameterIndex(id: u8)** - 参数化嵌套索引页组件
7. **ParameterFixed(id: u8)** - 参数化嵌套固定路径组件
8. **HashQuery(id, query, hash)** - Hash 查询页组件
9. **HashIndex** - Hash 索引页组件
10. **HashId(id, query)** - Hash ID 页组件

### 关键 API

- `#[component]` - 组件定义宏
- `Outlet::<Route>` - 嵌套路由出口，用于渲染子路由

---

## 测试覆盖范围

- ✅ 根路由的父级获取
- ✅ 固定路径嵌套的父子关系
- ✅ 动态参数嵌套的父子关系
- ✅ 带查询参数和 hash 的路由父子关系
- ✅ 多层嵌套的父子关系
- ✅ 路由自反性验证（不是自己的子级）
