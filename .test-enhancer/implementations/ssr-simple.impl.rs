// SSR 简单功能测试实现
// 基于 Dioxus SSR 最新 API

use dioxus::prelude::*;

#[test]
fn simple() {
    // 测试 1：验证基本 SSR 功能
    // 测试 VirtualDom 和直接渲染 rsx! 元素

    // 定义一个简单的 App 组件
    fn App() -> Element {
        rsx! {
            div { "hello!" }
        }
    }

    // 创建 VirtualDom 实例
    let mut dom = VirtualDom::new(App);

    // 使用 NoOpMutations 重建虚拟 DOM
    dom.rebuild(&mut dioxus_core::NoOpMutations);

    // 使用 dioxus_ssr::render() 渲染整个 VirtualDom
    let html_from_dom = dioxus_ssr::render(&dom);

    // 使用 dioxus_ssr::render_element() 直接渲染 rsx! 宏创建的元素
    let html_from_element = dioxus_ssr::render_element(rsx! {
        div { "hello!" }
    });

    // 验证两种渲染方式产生相同的 HTML 输出
    assert_eq!(html_from_dom, "<div>hello!</div>");
    assert_eq!(html_from_element, "<div>hello!</div>");
}

#[test]
fn lists() {
    // 测试 2：验证列表渲染
    // 验证 SSR 能正确渲染动态生成的列表元素

    // 使用迭代器创建包含 5 个 li 元素的 ul
    let element = rsx! {
        ul {
            (0..5).map(|i| rsx! {
                li { "item {i}" }
            })
        }
    };

    // 渲染元素
    let html = dioxus_ssr::render_element(element);

    // 验证所有 5 个列表项都被正确渲染
    assert_eq!(
        html,
        "<ul><li>item 0</li><li>item 1</li><li>item 2</li><li>item 3</li><li>item 4</li></ul>"
    );
}

#[test]
fn dynamic() {
    // 测试 3：验证动态值插入和 HTML 特殊字符转义
    // 验证 SSR 能正确处理动态值插入，并对 HTML 特殊字符进行转义

    // 创建一个动态变量
    let dynamic = 123;

    // 创建混合静态文本和动态值的元素
    let element = rsx! {
        div { "Hello world 1 --" {dynamic} "<-- Hello world 2" }
    };

    // 渲染元素
    let html = dioxus_ssr::render_element(element);

    // 验证动态值被正确插入，HTML 特殊字符被正确转义
    assert_eq!(html, "<div>Hello world 1 --&#62;123&#60;-- Hello world 2</div>");
}

#[test]
fn components() {
    // 测试 4：验证带 Props 的自定义组件渲染
    // 验证 SSR 能正确渲染带 Props 的自定义组件

    // 定义 Props 结构体
    #[derive(Props, Clone, PartialEq)]
    struct MyComponentProps {
        name: i32,
    }

    // 实现组件函数
    fn MyComponent(MyComponentProps { name }: MyComponentProps) -> Element {
        rsx! {
            div { "component {name}" }
        }
    }

    // 创建包含 5 个组件实例的结构
    let element = rsx! {
        div {
            (0..5).map(|name| rsx! {
                MyComponent { name }
            })
        }
    };

    // 渲染元素
    let html = dioxus_ssr::render_element(element);

    // 验证 5 个组件实例被正确渲染
    assert_eq!(
        html,
        "<div><div>component 0</div><div>component 1</div><div>component 2</div><div>component 3</div><div>component 4</div></div>"
    );
}

#[test]
fn fragments() {
    // 测试 5：验证空 Fragment 处理
    // 验证 SSR 能正确处理空 Fragment（不产生额外 DOM 节点）

    // 创建一个包含 5 个空 Fragment 的 div
    let element = rsx! {
        div {
            (0..5).map(|_| rsx! {})
        }
    };

    // 渲染元素
    let html = dioxus_ssr::render_element(element);

    // 验证空 Fragment 不产生任何 DOM 节点
    assert_eq!(html, "<div></div>");
}
