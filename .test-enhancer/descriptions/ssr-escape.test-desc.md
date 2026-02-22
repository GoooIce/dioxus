# SSR Escape Tests

文件路径: `packages/ssr/tests/escape.rs`

## 测试概述

此测试文件验证了 Dioxus SSR (Server-Side Rendering) 模块中的 HTML 转义行为。主要测试各种场景下危险字符（如 `<`, `>`, `"`）是否被正确转义，以及某些特殊标签（如 `<script>`, `<style>`）的内容是否不应被转义。

---

## 测试列表

### 1. `escape_static_values`

**测试目的**: 验证静态属性值中的危险字符是否被正确转义

**测试场景**:
- 创建一个包含危险字符 `"><div>` 的静态属性值
- 使用 `disabled` 属性作为测试目标

**预期结果**:
- 危险字符应被转义为 HTML 实体:
  - `"` → `&#34;`
  - `>` → `&#62;`
  - `<` → `&#60;`
- 最终输出: `<input disabled="&#34;&#62;&#60;div&#62;" data-node-hydration="0"/>`

**安全意义**: 防止 XSS 攻击，确保用户输入的静态字符串不会破坏 HTML 结构

---

### 2. `escape_dynamic_values`

**测试目的**: 验证动态属性值中的危险字符是否被正确转义

**测试场景**:
- 创建一个包含危险字符的动态变量 `disabled = "\"><div>"`
- 将该变量作为属性值传递

**预期结果**:
- 与静态值相同，危险字符应被转义
- 最终输出: `<input disabled="&#34;&#62;&#60;div&#62;" data-node-hydration="0"/>`

**安全意义**: 确保动态数据（如用户输入）被安全处理

---

### 3. `escape_static_style`

**测试目的**: 验证静态 style 属性值中的危险字符是否被正确转义

**测试场景**:
- 创建一个包含危险字符的静态 style 属性 `width: "\"><div>"`

**预期结果**:
- 危险字符应被转义
- 最终输出: `<div style="width:&#34;&#62;&#60;div&#62;;" data-node-hydration="0"></div>`

**安全意义**: 防止通过 style 属性注入恶意代码

---

### 4. `escape_dynamic_style`

**测试目的**: 验证动态 style 属性值中的危险字符是否被正确转义

**测试场景**:
- 创建一个包含危险字符的动态变量 `width = "\"><div>"`
- 将该变量作为 style 属性值传递

**预期结果**:
- 与静态 style 相同，危险字符应被转义
- 最终输出: `<div style="width:&#34;&#62;&#60;div&#62;;" data-node-hydration="0"></div>`

---

### 5. `escape_static_text`

**测试目的**: 验证静态文本内容中的危险字符是否被正确转义

**测试场景**:
- 在 div 元素中包含危险字符作为文本内容 `"\"><div>"`

**预期结果**:
- 危险字符应被转义
- 最终输出: `<div data-node-hydration="0">&#34;&#62;&#60;div&#62;</div>`

**安全意义**: 防止通过文本节点注入 HTML 标签

---

### 6. `escape_dynamic_text`

**测试目的**: 验证动态文本内容中的危险字符是否被正确转义

**测试场景**:
- 创建一个包含危险字符的动态变量 `text = "\"><div>"`
- 将该变量作为文本内容插入

**预期结果**:
- 危险字符应被转义
- 包含水合注释标记
- 最终输出: `<div data-node-hydration="0"><!--node-id1-->&#34;&#62;&#60;div&#62;<!--#--></div>`

---

### 7. `don_t_escape_static_scripts`

**测试目的**: 验证静态 `<script>` 标签内容不应被转义

**测试场景**:
- 创建一个包含 JavaScript 代码的静态 script 标签
- 代码内容: `console.log('hello world');`

**预期结果**:
- JavaScript 代码应保持原样，不被转义
- 最终输出: `<script data-node-hydration="0">console.log('hello world');</script>`

**安全意义**: Script 标签内容需要保持原始格式以便浏览器正确执行

---

### 8. `don_t_escape_dynamic_scripts`

**测试目的**: 验证动态 `<script>` 标签内容不应被转义

**测试场景**:
- 创建一个包含 JavaScript 代码的动态变量
- 将该变量作为 script 标签内容

**预期结果**:
- JavaScript 代码应保持原样，不被转义
- 包含水合注释标记
- 最终输出: `<script data-node-hydration="0"><!--node-id1-->console.log('hello world');<!--#--></script>`

---

### 9. `don_t_escape_static_styles`

**测试目的**: 验证静态 `<style>` 标签内容不应被转义

**测试场景**:
- 创建一个包含 CSS 代码的静态 style 标签
- CSS 内容: `body { background-color: red; }`

**预期结果**:
- CSS 代码应保持原样，不被转义
- 注意双大括号 `{{` 在 rsx 中会被转换为单大括号 `{`
- 最终输出: `<style data-node-hydration="0">body { background-color: red; }</style>`

**安全意义**: Style 标签内容需要保持原始格式以便浏览器正确解析

---

### 10. `don_t_escape_dynamic_styles`

**测试目的**: 验证动态 `<style>` 标签内容不应被转义

**测试场景**:
- 创建一个包含 CSS 代码的动态变量
- CSS 包含引号: `body { font-family: "sans-serif"; }`
- 将该变量作为 style 标签内容

**预期结果**:
- CSS 代码应保持原样，不被转义
- 引号应保持不变
- 包含水合注释标记
- 最终输出: `<style data-node-hydration="0"><!--node-id1-->body { font-family: "sans-serif"; }<!--#--></style>`

---

### 11. `don_t_escape_static_fragment_styles`

**测试目的**: 验证静态 fragment 作为 `<style>` 标签内容时不应被转义

**测试场景**:
- 创建一个 rsx fragment 作为 style 标签的内容
- Fragment 包含 CSS 代码

**预期结果**:
- CSS 代码应保持原样，不被转义
- 包含水合注释标记
- 最终输出: `<style data-node-hydration="0"><!--node-id1-->body { font-family: "sans-serif"; }<!--#--></style>`

---

### 12. `escape_static_component_fragment_div`

**测试目的**: 验证组件返回的静态 fragment 作为 `<div>` 内容时应被转义

**测试场景**:
- 创建一个返回静态 fragment 的组件 `StyleContents`
- Fragment 包含类似 CSS 的文本
- 将该组件放在 div 中（非 style 标签）

**预期结果**:
- 因为不在 `<style>` 标签内，内容应被转义
- 引号应被转义为 `&#34;`
- 最终输出: `<div data-node-hydration="0"><!--node-id1-->body { font-family: &#34;sans-serif&#34;; }<!--#--></div>`

**安全意义**: 只有在特定标签内才不转义，在其他位置仍需转义

---

### 13. `escape_dynamic_component_fragment_div`

**测试目的**: 验证组件返回的动态 fragment 作为 `<div>` 内容时应被转义

**测试场景**:
- 创建一个返回动态 fragment 的组件 `StyleContents`
- Fragment 包含动态变量和类似 CSS 的文本
- 将该组件放在 div 中（非 style 标签）

**预期结果**:
- 因为不在 `<style>` 标签内，内容应被转义
- 引号应被转义为 `&#34;`
- 最终输出: `<div data-node-hydration="0"><!--node-id1-->body { font-family: &#34;sans-serif&#34;; }<!--#--></div>`

---

## 测试模式总结

| 测试类型 | 静态值 | 动态值 | Style 属性 | 文本内容 |
|---------|--------|--------|-----------|---------|
| 普通转义 | ✅ | ✅ | ✅ | ✅ |
| Script 标签 | ❌ 不转义 | ❌ 不转义 | N/A | N/A |
| Style 标签 | ❌ 不转义 | ❌ 不转义 | N/A | N/A |
| Fragment 在 div | ✅ 转义 | ✅ 转义 | N/A | N/A |

**转义映射**:
- `"` → `&#34;`
- `<` → `&#60;`
- `>` → `&#62;`

---

## 关键安全考量

1. **默认转义**: 所有动态内容默认应被转义，除非在特定的安全上下文中（如 `<script>` 或 `<style>` 标签）

2. **上下文感知**: SSR 渲染器需要根据内容所在的 HTML 上下文决定是否转义

3. **水合标记**: 动态内容会包含 `<!--node-idX-->` 和 `<!--#-->` 注释用于客户端水合
