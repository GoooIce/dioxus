//! SSR Hydration 测试
//!
//! 测试服务端渲染的 hydration 功能，验证节点 ID、动态属性、事件监听器
//! 和动态文本节点的正确标记和生成。

use dioxus::prelude::*;

#[test]
fn root_ids() {
    // 验证根节点的 hydration ID 生成
    // 根节点应获得 hydration ID "0"
    let mut vdom = VirtualDom::new(|| {
        rsx! {
            div { style: "width:100px;" }
        }
    });
    vdom.rebuild();

    let html = dioxus_ssr::render(&vdom);

    // 验证渲染结果包含正确的 data-node-hydration 属性
    assert!(html.contains(r#"data-node-hydration="0""#));
}

#[test]
fn dynamic_attributes() {
    // 验证动态属性的 hydration 处理
    // 动态属性值应被正确解析，每个节点都有唯一的 hydration ID
    let dynamic = 123;
    let mut vdom = VirtualDom::new(|| {
        rsx! {
            div { style: "width:100px;",
                div { style: "{dynamic}px" }
            }
        }
    });
    vdom.rebuild();

    let html = dioxus_ssr::render(&vdom);

    // 验证动态属性值 (123) 被正确解析
    assert!(html.contains("123px"));
    // 验证每个节点都有唯一的 hydration ID
    assert!(html.contains(r#"data-node-hydration="0""#));
    assert!(html.contains(r#"data-node-hydration="1""#));
}

#[test]
fn listeners() {
    // 子测试 1: 验证事件监听器的 hydration 标记
    let mut vdom = VirtualDom::new(|| {
        rsx! {
            div { style: "width:100px;",
                div { onclick: |_| {} }
            }
        }
    });
    vdom.rebuild();

    let html = dioxus_ssr::render(&vdom);

    // 验证事件监听器被标记在 data-node-hydration 属性中
    // 格式为 "id,eventType:listenerCount"
    assert!(html.contains(r#"data-node-hydration="1,click:1""#));

    // 子测试 2: 同时包含动态属性和事件监听器的组件
    let dynamic = 123;
    let mut vdom = VirtualDom::new(|| {
        rsx! {
            div { style: "width:100px;",
                div { style: "{dynamic}px", onclick: |_| {} }
            }
        }
    });
    vdom.rebuild();

    let html = dioxus_ssr::render(&vdom);

    // 验证动态属性和事件监听器都被正确处理
    assert!(html.contains("123px"));
    assert!(html.contains(r#"data-node-hydration="1,click:1""#));
}

#[test]
fn text_nodes() {
    // 子测试 1: 单个动态文本变量插入
    let name = "hello";
    let mut vdom = VirtualDom::new(|| {
        rsx! {
            div { "{name}" }
        }
    });
    vdom.rebuild();

    let html = dioxus_ssr::render(&vdom);

    // 验证动态文本使用 HTML 注释标记包裹
    // 格式为 <!--node-idX-->content<!--#-->
    assert!(html.contains("<!--node-id1-->"));
    assert!(html.contains("hello"));
    assert!(html.contains("<!--#-->"));

    // 子测试 2: 多个动态文本表达式
    let a = 123;
    let b = 1234;
    let mut vdom = VirtualDom::new(|| {
        rsx! {
            div {
                "{a}"
                "{b}"
            }
        }
    });
    vdom.rebuild();

    let html = dioxus_ssr::render(&vdom);

    // 验证每个动态文本节点都有唯一 ID
    assert!(html.contains("<!--node-id1-->123<!--#-->"));
    assert!(html.contains("<!--node-id2-->1234<!--#-->"));
}

#[test]
fn components_hydrate() {
    // 子测试 1: 静态内容的子组件
    #[component]
    fn Child() -> Element {
        rsx! {
            div { "hello" }
        }
    }

    let mut vdom = VirtualDom::new(|| {
        rsx! {
            Child {}
        }
    });
    vdom.rebuild();

    let html = dioxus_ssr::render(&vdom);

    // 组件边界不影响 hydration ID 的连续性
    assert!(html.contains(r#"data-node-hydration="0""#));
    assert!(html.contains("hello"));

    // 子测试 2: 包含动态文本的子组件
    #[component]
    fn DynamicChild(text: String) -> Element {
        rsx! {
            div { "{text}" }
        }
    }

    let mut vdom = VirtualDom::new(|| {
        rsx! {
            DynamicChild { text: "hello".to_string() }
        }
    });
    vdom.rebuild();

    let html = dioxus_ssr::render(&vdom);

    // 验证动态内容正确标记
    assert!(html.contains(r#"data-node-hydration="0""#));
    assert!(html.contains("<!--node-id1-->hello<!--#-->"));

    // 子测试 3: 包含动态属性的子组件
    #[component]
    fn AttrChild(width: i32) -> Element {
        rsx! {
            div { style: "width:{width}px;" }
        }
    }

    let mut vdom = VirtualDom::new(|| {
        rsx! {
            AttrChild { width: 1 }
        }
    });
    vdom.rebuild();

    let html = dioxus_ssr::render(&vdom);

    // 验证动态属性正确渲染
    assert!(html.contains(r#"style="width:1px;""#));
    assert!(html.contains(r#"data-node-hydration="0""#));

    // 子测试 4: 使用循环生成多个节点
    let mut vdom = VirtualDom::new(|| {
        rsx! {
            div {
                (0..2).map(|_| {
                    rsx! {
                        div { "1" }
                    }
                })
            }
        }
    });
    vdom.rebuild();

    let html = dioxus_ssr::render(&vdom);

    // 循环生成的节点从 ID 0 开始（在动态文本节点 ID 之后）
    // 验证包含节点标记
    assert!(html.contains("<!--node-id0-->"));
    assert!(html.contains("<!--node-id1-->"));
}

#[test]
fn hello_world_hydrates() {
    // 综合测试：完整应用的 hydration 功能
    // 包含 hook、动态文本和事件监听器
    #[component]
    fn Counter() -> Element {
        let mut count = use_signal(|| 0);

        rsx! {
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
        }
    }

    let mut vdom = VirtualDom::new(|| {
        rsx! {
            Counter {}
        }
    });
    vdom.rebuild();

    let html = dioxus_ssr::render(&vdom);

    // 验证节点 ID 按顺序递增 (0, 1, 2, 3)
    assert!(html.contains(r#"data-node-hydration="0""#));
    assert!(html.contains(r#"data-node-hydration="1""#));
    assert!(html.contains(r#"data-node-hydration="2,click:1""#));
    assert!(html.contains(r#"data-node-hydration="3,click:1""#));

    // 验证动态文本内容正确标记
    assert!(html.contains("<!--node-id1-->"));
    assert!(html.contains("High-Five counter:"));

    // 验证按钮文本存在
    assert!(html.contains("Up high!"));
    assert!(html.contains("Down low!"));
}
