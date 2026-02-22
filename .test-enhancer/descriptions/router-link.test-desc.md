# Router Link 测试描述

**文件路径**: `packages/router/tests/via_ssr/link.rs`

## 测试概述

本文件包含 Dioxus Router `Link` 组件的服务端渲染(SSR)测试，验证 Link 组件在不同配置下的 HTML 输出是否正确。

---

## 测试函数列表

### 1. `href_internal`

**位置**: 第 62-98 行

**目的**: 测试内部路由链接的 `href` 属性生成

**描述**:
- 定义两个路由：`/` (Root) 和 `/test` (Test)
- 在 Root 组件中创建一个指向 `Route::Test` 的 Link
- 验证生成的 `<a>` 标签包含正确的 `href="/test"` 属性
- 验证当使用 base_path (如 `/deeply/nested/path`) 时，内部链接会自动添加 base_path 前缀
- 预期输出: `<h1>App</h1><a href="/test">Link</a>`
- 带 base_path 时预期: `<h1>App</h1><a href="/deeply/nested/path/test">Link</a>`

---

### 2. `href_external`

**位置**: 第 101-137 行

**目的**: 测试外部链接的 `href` 和 `rel` 属性生成

**描述**:
- 创建指向外部 URL (`https://dioxuslabs.com/`) 的 Link
- 验证外部链接包含 `href` 属性指向完整 URL
- 验证外部链接自动添加 `rel="noopener noreferrer"` 安全属性
- 验证 base_path 不会影响外部链接
- 预期输出: `<h1>App</h1><a href="https://dioxuslabs.com/" rel="noopener noreferrer">Link</a>`

---

### 3. `with_class`

**位置**: 第 140-172 行

**目的**: 测试 Link 组件的自定义 CSS class 属性

**描述**:
- 创建带有 `class: "test_class"` 属性的 Link
- 验证生成的 `<a>` 标签包含正确的 class 属性
- 预期输出: `<h1>App</h1><a href="/test" class="test_class">Link</a>`

---

### 4. `with_active_class_active`

**位置**: 第 175-202 行

**目的**: 测试当前激活路由的 active_class 样式应用

**描述**:
- 当前路由为 `/` (Root)
- 创建一个指向当前路由 `Route::Root` 的 Link
- 设置 `active_class: "active_class"` 和基础 `class: "test_class"`
- 验证激活状态下:
  - class 列表包含 active_class
  - 添加 `aria-current="page"` 无障碍属性
- 预期输出: `<h1>App</h1><a href="/" class="test_class active_class" aria-current="page">Link</a>`

---

### 5. `with_active_class_inactive`

**位置**: 第 205-238 行

**目的**: 测试非激活路由的 active_class 样式不应用

**描述**:
- 当前路由为 `/` (Root)
- 创建一个指向其他路由 `Route::Test` 的 Link
- 设置 `active_class: "active_class"` 和基础 `class: "test_class"`
- 验证非激活状态下:
  - class 列表不包含 active_class
  - 不添加 aria-current 属性
- 预期输出: `<h1>App</h1><a href="/test" class="test_class">Link</a>`

---

### 6. `with_id`

**位置**: 第 241-273 行

**目的**: 测试 Link 组件的自定义 id 属性

**描述**:
- 创建带有 `id: "test_id"` 属性的 Link
- 验证生成的 `<a>` 标签包含正确的 id 属性
- 预期输出: `<h1>App</h1><a href="/test" id="test_id">Link</a>`

---

### 7. `with_new_tab`

**位置**: 第 276-308 行

**目的**: 测试内部链接的新标签页打开功能

**描述**:
- 创建带有 `new_tab: true` 属性的内部 Link
- 验证生成的 `<a>` 标签包含 `target="_blank"` 属性
- 预期输出: `<h1>App</h1><a href="/test" target="_blank">Link</a>`

---

### 8. `with_new_tab_external`

**位置**: 第 311-337 行

**目的**: 测试外部链接的新标签页打开功能

**描述**:
- 创建带有 `new_tab: true` 属性的外部 Link
- 验证生成的 `<a>` 标签同时包含:
  - `href` 指向外部 URL
  - `rel="noopener noreferrer"` 安全属性
  - `target="_blank"` 新标签页属性
- 预期输出: `<h1>App</h1><a href="https://dioxuslabs.com/" rel="noopener noreferrer" target="_blank">Link</a>`

---

### 9. `with_rel`

**位置**: 第 340-372 行

**目的**: 测试 Link 组件的自定义 rel 属性

**描述**:
- 创建带有 `rel: "test_rel"` 属性的 Link
- 验证生成的 `<a>` 标签包含正确的 rel 属性
- 预期输出: `<h1>App</h1><a href="/test" rel="test_rel">Link</a>`

---

### 10. `with_child_route`

**位置**: 第 375-445 行

**目的**: 测试嵌套路由(Nested Routes)的 Link 生成

**描述**:
- 定义父路由 `Route` 包含 Root、Test 和 Nested 子路由
- 定义子路由 `ChildRoute` 包含 ChildRoot 和动态参数路由
- 测试三种场景的 Link 生成:
  1. 在根路由 `/` 访问父级链接和子级链接
  2. 在子路由 `/child` 中访问父级链接和子级链接
- 验证嵌套路由的路径正确拼接
- 在 `/` 路由预期输出: `<h1>App</h1><a href="/test">Parent Link</a><a href="/child/this-is-a-child-route">Child Link</a>`
- 在 `/child` 路由预期输出: `<h1>App</h1><a href="/test">Parent Link</a><a href="/child/this-is-a-child-route">Child Link 1</a><a href="/child/this-is-a-child-route">Child Link 2</a>`

---

### 11. `with_hash_segment`

**位置**: 第 448-473 行

**目的**: 测试包含 hash 片段的路由链接生成

**描述**:
- 定义带有 hash 参数的路由 `/#:data`
- 测试两种情况:
  1. 带有 hash 值 `"test"` 的链接
  2. 空 hash 值的链接
- 验证当前路由激活时正确添加 `aria-current="page"` 属性
- 预期输出: `<h1>App</h1><a href="/#test" aria-current="page">Link</a><a href="/">Empty</a>`

---

## 辅助函数

### `prepare<R: Routable>()`
在根路径 `/` 初始化 VirtualDom 并渲染 SSR 输出

### `prepare_at<R: Routable>(at: impl ToString)`
在指定路径初始化 VirtualDom 并渲染 SSR 输出

### `prepare_at_with_base_path<R: Routable>(at: impl ToString, base_path: impl ToString)`
在指定路径和 base_path 下初始化 VirtualDom 并渲染 SSR 输出

### `App<R: Routable>(props: AppProps<R>)`
测试用的根组件，包装 HistoryProvider 和 Router

---

## 测试覆盖的功能点

1. **内部路由链接**: href 属性正确生成，base_path 前缀处理
2. **外部链接**: 完整 URL、安全属性 rel、不受 base_path 影响
3. **HTML 属性**: class、id、rel 自定义
4. **激活状态**: active_class 应用、aria-current 属性
5. **新标签页**: target="_blank" 属性
6. **嵌套路由**: 子路由路径正确拼接
7. **Hash 片段**: URL hash 参数处理
