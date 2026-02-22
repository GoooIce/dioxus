# SSR Hydration 测试描述

## 文件信息
- **文件路径**: `packages/ssr/tests/hydration.rs`
- **测试数量**: 6 个测试函数
- **测试主题**: SSR (服务端渲染) 的 hydration 功能

---

## 测试详情

### 1. `root_ids` (第 4-16 行)

**测试目的**: 验证根节点的 hydration ID 生成

**测试行为**:
- 创建一个简单的组件，包含一个带有静态样式属性的 `div` 元素
- 使用 VirtualDom 重建 DOM
- 验证 SSR 预渲染结果包含正确的 `data-node-hydration` 属性

**预期输出**:
```html
<div style="width:100px;" data-node-hydration="0"></div>
```

**关键点**: 确保根节点获得 hydration ID "0"

---

### 2. `dynamic_attributes` (第 19-34 行)

**测试目的**: 验证动态属性的 hydration 处理

**测试行为**:
- 创建包含嵌套 `div` 的组件
- 父 div 使用静态属性，子 div 使用动态属性（`"{dynamic}px"`）
- 验证动态属性被正确解析并包含在渲染输出中

**预期输出**:
```html
<div style="width:100px;" data-node-hydration="0">
  <div style="width:123px;" data-node-hydration="1"></div>
</div>
```

**关键点**:
- 动态属性值 (123) 被正确解析
- 每个节点都有唯一的 hydration ID

---

### 3. `listeners` (第 37-66 行)

**测试目的**: 验证事件监听器的 hydration 标记

**测试行为**:
- **子测试 1**: 创建包含静态属性 div 和带 onclick 监听器 div 的组件
- **子测试 2**: 创建同时包含动态属性和事件监听器的组件

**预期输出**:

子测试 1:
```html
<div style="width:100px;" data-node-hydration="0">
  <div data-node-hydration="1,click:1"></div>
</div>
```

子测试 2:
```html
<div style="width:100px;" data-node-hydration="0">
  <div style="width:123px;" data-node-hydration="1,click:1"></div>
</div>
```

**关键点**:
- 事件监听器被标记在 `data-node-hydration` 属性中
- 格式为 `"id,eventType:listenerCount"`
- 静态属性不包含在 hydration 数据中

---

### 4. `text_nodes` (第 69-99 行)

**测试目的**: 验证动态文本节点的 hydration 处理

**测试行为**:
- **子测试 1**: 单个动态文本变量插入
- **子测试 2**: 多个动态文本表达式（变量和字面量）

**预期输出**:

子测试 1:
```html
<div data-node-hydration="0">
  <!--node-id1-->hello<!--#-->
</div>
```

子测试 2:
```html
<div data-node-hydration="0">
  <!--node-id1-->123<!--#-->
  <!--node-id2-->1234<!--#-->
</div>
```

**关键点**:
- 动态文本使用 HTML 注释标记包裹
- 格式为 `<!--node-idX-->content<!--#-->`
- 每个动态文本节点都有唯一 ID

---

### 5. `components_hydrate` (第 103-174 行)

**测试目的**: 验证组件嵌套时的 hydration 行为

**测试行为**:
- **子测试 1**: 静态内容的子组件
- **子测试 2**: 包含动态文本的子组件
- **子测试 3**: 包含动态属性的子组件
- **子测试 4**: 使用循环生成多个节点

**预期输出**:

子测试 1:
```html
<div data-node-hydration="0">hello</div>
```

子测试 2:
```html
<div data-node-hydration="0">
  <!--node-id1-->hello<!--#-->
</div>
```

子测试 3:
```html
<div style="width:1;" data-node-hydration="0"></div>
```

子测试 4:
```html
<!--node-id0-->1<!--#-->
<!--node-id1-->1<!--#-->
```

**关键点**:
- 组件边界不影响 hydration ID 的连续性
- 循环生成的节点从 ID 0 开始
- 动态内容正确标记

---

### 6. `hello_world_hydrates` (第 177-197 行)

**测试目的**: 验证完整应用的 hydration 功能（综合测试）

**测试行为**:
- 创建一个包含 hook (`use_signal`) 的计数器应用
- 包含动态文本显示和两个带点击事件的按钮
- 验证完整的 hydration 标记生成

**预期输出**:
```html
<h1 data-node-hydration="0">
  <!--node-id1-->High-Five counter: 0<!--#-->
</h1>
<button data-node-hydration="2,click:1">Up high!</button>
<button data-node-hydration="3,click:1">Down low!</button>
```

**关键点**:
- Hook 使用不会影响 hydration
- 动态文本内容正确标记
- 多个事件监听器正确标记
- 节点 ID 按顺序递增 (0, 1, 2, 3)

---

## 测试覆盖范围

| 功能 | 测试覆盖 |
|------|----------|
| 根节点 ID 生成 | ✅ `root_ids` |
| 动态属性解析 | ✅ `dynamic_attributes` |
| 事件监听器标记 | ✅ `listeners` |
| 动态文本节点 | ✅ `text_nodes` |
| 组件嵌套 | ✅ `components_hydrate` |
| Hooks 集成 | ✅ `hello_world_hydrates` |
| 循环渲染 | ✅ `components_hydrate` 子测试 4 |
| 多动态内容 | ✅ `text_nodes`, `components_hydrate` |

## Hydration 数据格式总结

1. **节点 ID**: `data-node-hydration="N"`
2. **事件监听器**: `data-node-hydration="N,eventType:count"`
3. **动态文本**: `<!--node-idN-->content<!--#-->`
