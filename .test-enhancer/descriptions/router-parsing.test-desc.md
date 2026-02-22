# 测试文件：parsing.rs

## 测试 1：trailing_slashes_parse

### 目的
验证路由定义中带有尾部斜杠（trailing slash）的路径能够正确解析，无论输入 URL 是否包含尾部斜杠。

### 前置条件
- 定义 `Routable` 枚举 `Route`
- 路由定义使用尾部斜杠：`/`、`/test/`、`/:id/test/`

### 测试步骤
1. 定义带有 `#[derive(Routable)]` 的 `Route` 枚举，包含三个路由变体：
   - `Root {}` 对应路径 `/`
   - `Test {}` 对应路径 `/test/`
   - `Dynamic { id: usize }` 对应路径 `/:id/test/`
2. 使用 `Route::from_str()` 解析各种路径：
   - 解析根路径 `/`，期望得到 `Route::Root {}`
   - 解析带斜杠的路径 `/test/`，期望得到 `Route::Test {}`
   - 解析不带斜杠的路径 `/test`，期望得到 `Route::Test {}`
   - 解析带斜杠的动态路径 `/123/test/`，期望得到 `Route::Dynamic { id: 123 }`
   - 解析不带斜杠的动态路径 `/123/test`，期望得到 `Route::Dynamic { id: 123 }`

### 预期结果
- 带有尾部斜杠定义的路由能够匹配带或不带尾部斜杠的 URL
- 动态参数能够正确解析为指定类型（`usize`）
- 所有断言通过，证明路由解析器对尾部斜杠的处理是灵活的

### 涉及的 API
- `#[derive(Routable)]` - 路由派生宏
- `#[route("...")]` - 路由定义属性宏
- `FromStr::from_str()` 或 `str::parse::<Route>()` - 路由解析

---

## 测试 2：without_trailing_slashes_parse

### 目的
验证路由定义中不带有尾部斜杠的路径能够正确解析，无论输入 URL 是否包含尾部斜杠。这与 `trailing_slashes_parse` 测试互补。

### 前置条件
- 定义 `Routable` 枚举 `RouteWithoutTrailingSlash`
- 路由定义不使用尾部斜杠：`/`、`/test`、`/:id/test`

### 测试步骤
1. 定义带有 `#[derive(Routable)]` 的 `RouteWithoutTrailingSlash` 枚举：
   - `Root {}` 对应路径 `/`
   - `Test {}` 对应路径 `/test`
   - `Dynamic { id: usize }` 对应路径 `/:id/test`
2. 使用 `RouteWithoutTrailingSlash::from_str()` 解析各种路径：
   - 解析根路径 `/`，期望得到 `RouteWithoutTrailingSlash::Root {}`
   - 解析带斜杠的路径 `/test/`，期望得到 `RouteWithoutTrailingSlash::Test {}`
   - 解析不带斜杠的路径 `/test`，期望得到 `RouteWithoutTrailingSlash::Test {}`
   - 解析带斜杠的动态路径 `/123/test/`，期望得到 `RouteWithoutTrailingSlash::Dynamic { id: 123 }`
   - 解析不带斜杠的动态路径 `/123/test`，期望得到 `RouteWithoutTrailingSlash::Dynamic { id: 123 }`

### 预期结果
- 不带尾部斜杠定义的路由能够匹配带或不带尾部斜杠的 URL
- 动态参数能够正确解析为指定类型（`usize`）
- 所有断言通过，证明路由解析器对尾部斜杠的处理是双向兼容的

### 涉及的 API
- `#[derive(Routable)]` - 路由派生宏
- `#[route("...")]` - 路由定义属性宏
- `FromStr::from_str()` 或 `str::parse::<Route>()` - 路由解析

---

## 测试 3：query_segments_parse

### 目的
回归测试 [issue #2984](https://github.com/DioxusLabs/dioxus/issues/2984)。验证查询参数段（query segments）能够正确解析和序列化，特别是使用自定义类型作为查询参数。

### 前置条件
- 定义自定义类型 `Query` 实现 `From<&str>` 和 `Display` traits
- 路由定义使用查询参数段语法 `?:..query`

### 测试步骤
1. 定义 `Query` 枚举：
   - 实现 `From<&str>` 用于解析查询字符串（测试中固定返回 `Query::Id(10)`）
   - 实现 `Display` 用于序列化（固定返回 `"id=10"`）
2. 定义 `Index` 组件，接收 `Query` 类型的参数
3. 定义带有 `#[derive(Routable)]` 的 `Route` 枚举：
   - `Index { query: Query }` 对应路径 `/?:..query`
4. 创建 `Route::Index { query: Query::Id(10) }` 实例
5. 使用 `route.to_string()` 序列化路由，期望得到 `"/?id=10"`
6. 使用 `"/?id=10".parse::<Route>()` 解析路由
7. 断言解析后的路由与原始路由相等

### 预期结果
- 自定义类型的查询参数能够正确序列化
- 查询字符串能够正确解析为自定义类型
- 序列化和解析是可逆的（往返转换保持相等）

### 涉及的 API
- `#[derive(Routable)]` - 路由派生宏
- `#[route("/?:..query")]` - 查询参数段定义（捕获整个查询字符串）
- `From<&str>` trait - 自定义解析逻辑
- `Display` trait - 自定义序列化逻辑
- `route.to_string()` - 路由序列化
- `str::parse::<Route>()` - 路由解析

---

## 测试 4：optional_query_segments_parse

### 目的
验证可选查询参数段（optional query segments）的功能，确保可选参数在存在和不存在的情况下都能正确处理。

### 前置条件
- 路由定义使用多个查询参数，其中一个是 `Option<T>` 类型
- 路由定义使用查询参数段语法 `?:query&:other`

### 测试步骤
1. 定义带有 `#[derive(Routable)]` 的 `Route` 枚举：
   - `Index { query: Option<u64>, other: u64 }` 对应路径 `/?:query&:other`
2. 定义 `Index` 组件，接收 `query: Option<u64>` 和 `other: u64` 参数
3. 测试场景一：两个参数都存在
   - 创建 `Route::Index { query: Some(10), other: 20 }`
   - 序列化期望得到 `"/?query=10&other=20"`
   - 解析 `"/?query=10&other=20"` 期望得到相等的路由
4. 测试场景二：可选参数不存在
   - 创建 `Route::Index { query: None, other: 20 }`
   - 序列化期望得到 `"/?other=20"`（`query` 被省略）
   - 解析 `"/?other=20"` 期望得到相等的路由
5. 测试场景三：可选参数不存在且 `other` 为默认值
   - 创建 `Route::Index { query: None, other: 0 }`
   - 序列化期望得到 `"/?other=0"`
   - 解析 `"/"` 期望得到 `Route::Index { query: None, other: 0 }`（`other` 使用默认值 0）

### 预期结果
- 可选查询参数（`Option<T>`）在 `Some` 值时正确序列化和解析
- 可选查询参数在 `None` 时从 URL 中省略
- 解析空查询字符串时，可选参数正确解析为 `None`
- 必需查询参数可以使用默认值

### 涉及的 API
- `#[derive(Routable)]` - 路由派生宏
- `#[route("/?:query&:other")]` - 多个查询参数段定义
- `Option<T>` - 可选参数类型
- `route.to_string()` - 路由序列化
- `str::parse::<Route>()` - 路由解析

---

## 测试文件摘要

该测试文件包含 4 个测试用例，主要验证 `dioxus-router` 中路由解析的核心行为：

1. **尾部斜杠兼容性**：路由定义无论是否使用尾部斜杠，都能正确匹配带或不带尾部斜杠的 URL
2. **查询参数支持**：支持自定义类型的查询参数，通过 `From` 和 `Display` traits 实现自定义解析和序列化
3. **可选查询参数**：支持 `Option<T>` 类型的查询参数，能够正确处理参数存在和不存在的情况
4. **动态参数解析**：支持路径中的动态参数（如 `/:id`），能够自动解析为指定类型

所有测试都使用 `#[derive(Routable)]` 宏生成路由逻辑，并通过 `FromStr::from_str()` / `str::parse()` 进行解析，通过 `Display::to_string()` 进行序列化。
