// OHOS 端到端测试应用
// 这个应用包含所有需要测试的核心功能

use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        style { {include_str!("../assets/test_styles.css")} }
        div { class: "container",
            h1 { "OHOS E2E 测试应用" }
            p { "这个应用用于验证 Dioxus 在 OHOS 平台上的所有核心功能" }

            TestSections {}
        }
    }
}

#[component]
fn TestSections() -> Element {
    rsx! {
        // 基础渲染测试
        RenderingTest {}

        // 用户交互测试
        InteractionTest {}

        // 状态管理测试
        StateManagementTest {}

        // 资源加载测试
        AssetLoadingTest {}

        // 列表渲染测试
        ListRenderingTest {}
    }
}

// ===== 基础渲染测试 =====
#[component]
fn RenderingTest() -> Element {
    rsx! {
        div { class: "test-section",
            h2 { "1. WebView 渲染测试" }
            p { "如果你能看到这段文字，WebView 渲染正常" }

            div { class: "test-box",
                "这是一个测试框，用于验证样式和布局"
            }

            div { class: "result",
                span { class: "label", "状态: " }
                span { class: "status success", "✓ 渲染正常" }
            }
        }
    }
}

// ===== 用户交互测试 =====
#[component]
fn InteractionTest() -> Element {
    let mut click_count = use_signal(|| 0);
    let mut input_value = use_signal(|| String::new());
    let mut checkbox_checked = use_signal(|| false);
    let mut radio_selection = use_signal(|| String::from("option1"));

    rsx! {
        div { class: "test-section",
            h2 { "2. 用户交互测试" }

            // 点击测试
            div { class: "test-item",
                button {
                    class: "btn",
                    onclick: move |_| {
                        click_count += 1;
                    },
                    "点击我 (已点击: {click_count} 次)"
                }
            }

            // 输入测试
            div { class: "test-item",
                label { "文本输入: " }
                input {
                    r#type: "text",
                    value: "{input_value}",
                    oninput: move |event| {
                        input_value.set(event.value());
                    }
                }
                p { class: "result", "你输入了: {input_value}" }
            }

            // 复选框测试
            div { class: "test-item",
                label {
                    input {
                        r#type: "checkbox",
                        checked: checkbox_checked,
                        onchange: move |event| {
                            checkbox_checked.set(event.checked());
                        }
                    }
                    "复选框 (当前: {if *checkbox_checked { "已选中" } else { "未选中" }})"
                }
            }

            // 单选框测试
            div { class: "test-item",
                fieldset {
                    legend { "单选框测试" }
                    label {
                        input {
                            r#type: "radio",
                            name: "radio-test",
                            value: "option1",
                            checked: radio_selection == "option1",
                            onchange: move |_| radio_selection.set("option1".to_string())
                        }
                        "选项 1"
                    }
                    label {
                        input {
                            r#type: "radio",
                            name: "radio-test",
                            value: "option2",
                            checked: radio_selection == "option2",
                            onchange: move |_| radio_selection.set("option2".to_string())
                        }
                        "选项 2"
                    }
                    p { class: "result", "选择了: {radio_selection}" }
                }
            }
        }
    }
}

// ===== 状态管理测试 =====
#[component]
fn StateManagementTest() -> Element {
    let mut count = use_signal(|| 0);

    // 使用 use_resource 测试异步加载
    let async_data = use_resource(|| async move {
        // 模拟异步操作
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        "异步数据加载成功!".to_string()
    });

    // Context 测试
    rsx! {
        div { class: "test-section",
            h2 { "3. 状态管理测试" }

            // use_signal 测试
            div { class: "test-item",
                button {
                    class: "btn",
                    onclick: move |_| count += 1,
                    "增加计数 (当前: {count})"
                }
                p { class: "result", "状态更新后自动重新渲染" }
            }

            // use_resource 测试
            div { class: "test-item",
                h3 { "异步数据加载" }
                match &*async_data.read() {
                    Some(data) => rsx! {
                        p { class: "status success", "✓ {data}" }
                    },
                    None => rsx! {
                        p { class: "status loading", "加载中..." }
                    }
                }
            }

            // ContextProvider 测试
            div { class: "test-item",
                h3 { "Context 测试" }
                TestContextChild { current_count: count }
            }
        }
    }
}

// Context 测试子组件
#[component]
fn TestContextChild(current_count: Signal<i32>) -> Element {
    rsx! {
        p { class: "result", "从父组件接收的计数: {current_count}" }
    }
}

// ===== 资源加载测试 =====
#[component]
fn AssetLoadingTest() -> Element {
    rsx! {
        div { class: "test-section",
            h2 { "4. 资源加载测试" }

            // 图片加载测试
            div { class: "test-item",
                h3 { "图片加载" }
                p { "如果看到下面的图标，SVG 图片加载正常" }
                div { class: "icon-test",
                    "🎨"
                }
                p { class: "note", "（使用 emoji 作为图片替代，实际应用中应使用真实图片）" }
            }

            // CSS 样式测试
            div { class: "test-item",
                h3 { "CSS 样式测试" }
                div { class: "style-test-box primary",
                    "主色框"
                }
                div { class: "style-test-box secondary",
                    "次色框"
                }
                div { class: "style-test-box success",
                    "成功框"
                }
            }
        }
    }
}

// ===== 列表渲染测试 =====
#[component]
fn ListRenderingTest() -> Element {
    let items = use_signal(|| vec![
        "项目 1".to_string(),
        "项目 2".to_string(),
        "项目 3".to_string(),
        "项目 4".to_string(),
        "项目 5".to_string(),
    ]);

    let mut new_item = use_signal(|| String::new());

    rsx! {
        div { class: "test-section",
            h2 { "5. 列表渲染测试" }

            div { class: "test-item",
                h3 { "项目列表" }
                ul { class: "item-list",
                    items.iter().enumerate().map(|(index, item)| {
                        rsx! {
                            li { key: "{index}",
                                "{item}"
                            }
                        }
                    })
                }
            }

            div { class: "test-item",
                h3 { "添加新项目" }
                input {
                    r#type: "text",
                    placeholder: "输入新项目",
                    value: "{new_item}",
                    oninput: move |event| new_item.set(event.value())
                }
                button {
                    class: "btn",
                    onclick: move |_| {
                        if !new_item.is_empty() {
                            items.with_mut(|v| v.push(new_item.clone()));
                            new_item.set(String::new());
                        }
                    },
                    "添加"
                }
            }
        }
    }
}
