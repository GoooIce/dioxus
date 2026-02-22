#![allow(non_snake_case)]

use dioxus::prelude::*;

/// 测试 1: 验证静态属性值中的危险字符是否被正确转义
///
/// 测试场景: 创建一个包含危险字符 `"><div>` 的静态属性值
/// 预期结果: `"` → `&#34;`, `>` → `&#62;`, `<` → `&#60;`
/// 最终输出: `<input disabled="&#34;&#62;&#60;div&#62;" data-node-hydration="0"/>`
#[test]
fn escape_static_values() {
    fn app() -> Element {
        rsx! { input { disabled: "\"><div>" } }
    }

    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);

    assert_eq!(
        dioxus_ssr::pre_render(&dom),
        "<input disabled=\"&#34;&#62;&#60;div&#62;\" data-node-hydration=\"0\"/>"
    );
}

/// 测试 2: 验证动态属性值中的危险字符是否被正确转义
///
/// 测试场景: 创建一个包含危险字符的动态变量 `disabled = "\"><div>"`
/// 预期结果: 与静态值相同，危险字符应被转义
/// 最终输出: `<input disabled="&#34;&#62;&#60;div&#62;" data-node-hydration="0"/>`
#[test]
fn escape_dynamic_values() {
    fn app() -> Element {
        let disabled = "\"><div>";
        rsx! { input { disabled } }
    }

    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);

    assert_eq!(
        dioxus_ssr::pre_render(&dom),
        "<input disabled=\"&#34;&#62;&#60;div&#62;\" data-node-hydration=\"0\"/>"
    );
}

/// 测试 3: 验证静态 style 属性值中的危险字符是否被正确转义
///
/// 测试场景: 创建一个包含危险字符的静态 style 属性 `width: "\"><div>"`
/// 预期结果: 危险字符应被转义
/// 最终输出: `<div style="width:&#34;&#62;&#60;div&#62;;" data-node-hydration="0"></div>`
#[test]
fn escape_static_style() {
    fn app() -> Element {
        rsx! { div { width: "\"><div>" } }
    }

    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);

    assert_eq!(
        dioxus_ssr::pre_render(&dom),
        "<div style=\"width:&#34;&#62;&#60;div&#62;;\" data-node-hydration=\"0\"></div>"
    );
}

/// 测试 4: 验证动态 style 属性值中的危险字符是否被正确转义
///
/// 测试场景: 创建一个包含危险字符的动态变量 `width = "\"><div>"`
/// 预期结果: 与静态 style 相同，危险字符应被转义
/// 最终输出: `<div style="width:&#34;&#62;&#60;div&#62;;" data-node-hydration="0"></div>`
#[test]
fn escape_dynamic_style() {
    fn app() -> Element {
        let width = "\"><div>";
        rsx! { div { width } }
    }

    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);

    assert_eq!(
        dioxus_ssr::pre_render(&dom),
        "<div style=\"width:&#34;&#62;&#60;div&#62;;\" data-node-hydration=\"0\"></div>"
    );
}

/// 测试 5: 验证静态文本内容中的危险字符是否被正确转义
///
/// 测试场景: 在 div 元素中包含危险字符作为文本内容 `"\"><div>"`
/// 预期结果: 危险字符应被转义
/// 最终输出: `<div data-node-hydration="0">&#34;&#62;&#60;div&#62;</div>`
#[test]
fn escape_static_text() {
    fn app() -> Element {
        rsx! {
            div {
                "\"><div>"
            }
        }
    }

    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);

    assert_eq!(
        dioxus_ssr::pre_render(&dom),
        "<div data-node-hydration=\"0\">&#34;&#62;&#60;div&#62;</div>"
    );
}

/// 测试 6: 验证动态文本内容中的危险字符是否被正确转义
///
/// 测试场景: 创建一个包含危险字符的动态变量 `text = "\"><div>"`
/// 预期结果: 危险字符应被转义，包含水合注释标记
/// 最终输出: `<div data-node-hydration="0"><!--node-id1-->&#34;&#62;&#60;div&#62;<!--#--></div>`
#[test]
fn escape_dynamic_text() {
    fn app() -> Element {
        let text = "\"><div>";
        rsx! {
            div {
                {text}
            }
        }
    }

    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);

    assert_eq!(
        dioxus_ssr::pre_render(&dom),
        "<div data-node-hydration=\"0\"><!--node-id1-->&#34;&#62;&#60;div&#62;<!--#--></div>"
    );
}

/// 测试 7: 验证静态 `<script>` 标签内容不应被转义
///
/// 测试场景: 创建一个包含 JavaScript 代码的静态 script 标签
/// 代码内容: `console.log('hello world');`
/// 预期结果: JavaScript 代码应保持原样，不被转义
/// 最终输出: `<script data-node-hydration="0">console.log('hello world');</script>`
#[test]
fn don_t_escape_static_scripts() {
    fn app() -> Element {
        rsx! {
            script {
                "console.log('hello world');"
            }
        }
    }

    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);

    assert_eq!(
        dioxus_ssr::pre_render(&dom),
        "<script data-node-hydration=\"0\">console.log('hello world');</script>"
    );
}

/// 测试 8: 验证动态 `<script>` 标签内容不应被转义
///
/// 测试场景: 创建一个包含 JavaScript 代码的动态变量
/// 预期结果: JavaScript 代码应保持原样，不被转义，包含水合注释标记
/// 最终输出: `<script data-node-hydration="0"><!--node-id1-->console.log('hello world');<!--#--></script>`
#[test]
fn don_t_escape_dynamic_scripts() {
    fn app() -> Element {
        let script = "console.log('hello world');";
        rsx! {
            script {
                {script}
            }
        }
    }

    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);

    assert_eq!(
        dioxus_ssr::pre_render(&dom),
        "<script data-node-hydration=\"0\"><!--node-id1-->console.log('hello world');<!--#--></script>"
    );
}

/// 测试 9: 验证静态 `<style>` 标签内容不应被转义
///
/// 测试场景: 创建一个包含 CSS 代码的静态 style 标签
/// CSS 内容: `body { background-color: red; }`
/// 注意: 双大括号 `{{` 在 rsx 中会被转换为单大括号 `{`
/// 最终输出: `<style data-node-hydration="0">body { background-color: red; }</style>`
#[test]
fn don_t_escape_static_styles() {
    fn app() -> Element {
        rsx! {
            style {
                "body {{ background-color: red; }}"
            }
        }
    }

    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);

    assert_eq!(
        dioxus_ssr::pre_render(&dom),
        "<style data-node-hydration=\"0\">body { background-color: red; }</style>"
    );
}

/// 测试 10: 验证动态 `<style>` 标签内容不应被转义
///
/// 测试场景: 创建一个包含 CSS 代码的动态变量
/// CSS 包含引号: `body { font-family: "sans-serif"; }`
/// 预期结果: CSS 代码应保持原样，引号应保持不变，包含水合注释标记
/// 最终输出: `<style data-node-hydration="0"><!--node-id1-->body { font-family: "sans-serif"; }<!--#--></style>`
#[test]
fn don_t_escape_dynamic_styles() {
    fn app() -> Element {
        let style = "body { font-family: \"sans-serif\"; }";
        rsx! {
            style {
                {style}
            }
        }
    }

    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);

    assert_eq!(
        dioxus_ssr::pre_render(&dom),
        "<style data-node-hydration=\"0\"><!--node-id1-->body { font-family: \"sans-serif\"; }<!--#--></style>"
    );
}

/// 测试 11: 验证静态 fragment 作为 `<style>` 标签内容时不应被转义
///
/// 测试场景: 创建一个 rsx fragment 作为 style 标签的内容
/// 预期结果: CSS 代码应保持原样，不被转义，包含水合注释标记
/// 最终输出: `<style data-node-hydration="0"><!--node-id1-->body { font-family: "sans-serif"; }<!--#--></style>`
#[test]
fn don_t_escape_static_fragment_styles() {
    fn app() -> Element {
        let style_element = rsx! { "body {{ font-family: \"sans-serif\"; }}" };
        rsx! {
            style {
                {style_element}
            }
        }
    }

    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);

    assert_eq!(
        dioxus_ssr::pre_render(&dom),
        "<style data-node-hydration=\"0\"><!--node-id1-->body { font-family: \"sans-serif\"; }<!--#--></style>"
    );
}

/// 测试 12: 验证组件返回的静态 fragment 作为 `<div>` 内容时应被转义
///
/// 测试场景: 创建一个返回静态 fragment 的组件 `StyleContents`
/// 将该组件放在 div 中（非 style 标签）
/// 预期结果: 因为不在 `<style>` 标签内，内容应被转义，引号应被转义为 `&#34;`
/// 最终输出: `<div data-node-hydration="0"><!--node-id1-->body { font-family: &#34;sans-serif&#34;; }<!--#--></div>`
#[test]
fn escape_static_component_fragment_div() {
    #[component]
    fn StyleContents() -> Element {
        rsx! { "body {{ font-family: \"sans-serif\"; }}" }
    }

    fn app() -> Element {
        rsx! {
            div {
                StyleContents {}
            }
        }
    }

    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);

    assert_eq!(
        dioxus_ssr::pre_render(&dom),
        "<div data-node-hydration=\"0\"><!--node-id1-->body { font-family: &#34;sans-serif&#34;; }<!--#--></div>"
    );
}

/// 测试 13: 验证组件返回的动态 fragment 作为 `<div>` 内容时应被转义
///
/// 测试场景: 创建一个返回动态 fragment 的组件 `StyleContents`
/// 将该组件放在 div 中（非 style 标签）
/// 预期结果: 因为不在 `<style>` 标签内，内容应被转义，引号应被转义为 `&#34;`
/// 最终输出: `<div data-node-hydration="0"><!--node-id1-->body { font-family: &#34;sans-serif&#34;; }<!--#--></div>`
#[test]
fn escape_dynamic_component_fragment_div() {
    #[component]
    fn StyleContents() -> Element {
        let dynamic = "body { font-family: \"sans-serif\"; }";
        rsx! { "{dynamic}" }
    }

    fn app() -> Element {
        rsx! {
            div {
                StyleContents {}
            }
        }
    }

    let mut dom = VirtualDom::new(app);
    dom.rebuild(&mut dioxus_core::NoOpMutations);

    assert_eq!(
        dioxus_ssr::pre_render(&dom),
        "<div data-node-hydration=\"0\"><!--node-id1-->body { font-family: &#34;sans-serif&#34;; }<!--#--></div>"
    );
}
