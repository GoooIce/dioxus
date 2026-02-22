# 测试文件：simple.rs

## 测试 1：simple

### 目的
验证 Dioxus SSR（服务端渲染）的基本功能，包括渲染 VirtualDom 和直接渲染 rsx! 元素。

### 前置条件
- 导入 `dioxus::prelude::*`

### 测试步骤
1. 定义一个简单的 `App` 组件，返回包含文本 "hello!" 的 `div` 元素
2. 创建 `VirtualDom` 实例并传入 `App` 组件
3. 调用 `rebuild()` 方法构建虚拟 DOM（使用 `NoOpMutations`）
4. 使用 `dioxus_ssr::render()` 渲染整个 VirtualDom
5. 使用 `dioxus_ssr::render_element()` 直接渲染 rsx! 宏创建的元素

### 预期结果
- `dioxus_ssr::render(&dom)` 返回 `"<div>hello!</div>"`
- `dioxus_ssr::render_element()` 也返回 `"<div>hello!</div>"`
- 两种渲染方式产生相同的 HTML 输出

### 涉及的 API
- `VirtualDom::new()` - 创建虚拟 DOM 实例
- `VirtualDom::rebuild()` - 重建虚拟 DOM
- `dioxus_ssr::render()` - 渲染整个 VirtualDom 为 HTML 字符串
- `dioxus_ssr::render_element()` - 直接渲染单个元素为 HTML 字符串
- `rsx!` - Dioxus 的 UI 声明式宏

---

## 测试 2：lists

### 目的
验证 SSR 能正确渲染动态生成的列表元素。

### 前置条件
- 使用 `dioxus_ssr::render_element()` 进行渲染

### 测试步骤
1. 使用 `rsx!` 创建一个 `ul` 元素
2. 在 `ul` 内部使用迭代器 `(0..5).map(|i| ...)` 动态生成 5 个 `li` 元素
3. 每个 `li` 元素包含文本 "item {i}"，其中 i 为迭代索引
4. 使用 `dioxus_ssr::render_element()` 渲染整个结构

### 预期结果
- 输出完整的 HTML 列表结构：`<ul><li>item 0</li><li>item 1</li><li>item 2</li><li>item 3</li><li>item 4</li></ul>`
- 所有 5 个列表项都被正确渲染
- 没有 Fragment 或额外的包裹元素

### 涉及的 API
- `dioxus_ssr::render_element()` - 直接渲染元素
- `rsx!` - 声明式 UI 宏
- `Iterator::map()` - 动态生成子元素

---

## 测试 3：dynamic

### 目的
验证 SSR 能正确处理动态值插入，并对 HTML 特殊字符进行转义。

### 前置条件
- 定义一个动态变量（如 `let dynamic = 123;`）

### 测试步骤
1. 创建一个动态变量 `dynamic = 123`
2. 在 `rsx!` 中创建一个 `div` 元素
3. 在 `div` 内混合使用：
   - 静态文本 "Hello world 1 --"
   - 动态值 `{dynamic}`（通过 `{}` 插值）
   - 静态文本 "<-- Hello world 2"
4. 使用 `dioxus_ssr::render_element()` 渲染

### 预期结果
- 动态值 `123` 被正确插入到输出中
- HTML 特殊字符 `>` 和 `<` 被正确转义为 `&#62;` 和 `&#60;`
- 最终输出：`<div>Hello world 1 --&#62;123&#60;-- Hello world 2</div>`

### 涉及的 API
- `dioxus_ssr::render_element()` - 渲染元素
- `rsx!` - 带插值的声明式 UI 宏
- `{variable}` 语法 - 在 rsx! 中插入动态值

---

## 测试 4：components

### 目的
验证 SSR 能正确渲染带 Props 的自定义组件。

### 前置条件
- 使用 `#[derive(Props, Clone, PartialEq)]` 派生 Props trait

### 测试步骤
1. 定义 `MyComponentProps` 结构体，包含 `name: i32` 字段
2. 派生 `Props`、`Clone`、`PartialEq` trait
3. 实现 `MyComponent` 函数组件，接收 props 并渲染包含 `name` 的 `div`
4. 在外层组件中使用迭代器 `(0..5).map(|name| ...)` 动态创建 5 个 `MyComponent` 实例
5. 使用 `dioxus_ssr::render_element()` 渲染整个结构

### 预期结果
- 5 个组件实例被正确渲染
- 每个 props 值被正确传递和渲染
- 输出：`<div><div>component 0</div><div>component 1</div><div>component 2</div><div>component 3</div><div>component 4</div></div>`

### 涉及的 API
- `dioxus_ssr::render_element()` - 渲染元素
- `#[derive(Props)]` - 自动派生 Props trait
- `Props` trait - 组件属性标记
- `rsx!` - 组件调用语法
- 模式解构：`MyComponentProps { name }: MyComponentProps`

---

## 测试 5：fragments

### 目的
验证 SSR 能正确处理空 Fragment（不产生额外 DOM 节点）。

### 前置条件
- 使用 `rsx!` 创建 Fragment（通过 `({})` 或类似语法）

### 测试步骤
1. 创建一个 `div` 元素
2. 在 `div` 内部使用迭代器 `(0..5).map(|_| rsx! ({}))` 生成 5 个空 Fragment
3. 每个 Fragment 是空的（使用 `rsx! ({})` 语法）
4. 使用 `dioxus_ssr::render_element()` 渲染

### 预期结果
- 空 Fragment 不产生任何 DOM 节点
- 最终输出为一个空的 `div`：`<div></div>`
- Fragment 不会在 HTML 中留下任何痕迹

### 涉及的 API
- `dioxus_ssr::render_element()` - 渲染元素
- `rsx! ({})` - 创建空 Fragment
- Fragment - Dioxus 中用于组合多个子元素而不创建额外 DOM 节点的抽象

---

## 测试文件摘要

该测试文件包含 5 个测试用例，主要验证 `dioxus-ssr` 包的核心服务端渲染功能：

1. **基本渲染**：验证 VirtualDom 和单个元素的 SSR 渲染
2. **列表渲染**：验证动态生成列表的正确渲染
3. **动态值与转义**：验证动态值插入和 HTML 特殊字符转义
4. **组件渲染**：验证带 Props 的自定义组件的 SSR
5. **Fragment 处理**：验证空 Fragment 不产生额外 DOM 节点

所有测试都使用 `dioxus_ssr::render()` 或 `dioxus_ssr::render_element()` 作为渲染 API，并验证生成的 HTML 字符串是否符合预期。
