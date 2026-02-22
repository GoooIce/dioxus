// Router Link SSR Tests
// 测试 Dioxus Router Link 组件的服务端渲染输出

use std::rc::Rc;
use dioxus::prelude::*;
use dioxus_history::{History, MemoryHistory};
use dioxus_router::components::{HistoryProvider, Link};
use dioxus_router::Routable;

// 辅助函数：在指定路径初始化 VirtualDom 并渲染 SSR 输出
fn prepare_at<R: Routable + Clone + 'static>(at: R) -> VirtualDom {
    let mut vdom = VirtualDom::new_with_props(
        || {
            rsx! {
                h1 { "App" }
                HistoryProvider {
                    history: move |_| {
                        Rc::new(MemoryHistory::with_initial_path(at.clone().to_string())) as Rc<dyn History>
                    },
                    Router::<R> {}
                }
            }
        },
        (),
    );
    vdom.rebuild_in_place();
    vdom
}

// 辅助函数：在指定路径和 base_path 下初始化 VirtualDom
fn prepare_at_with_base_path<R: Routable + Clone + 'static>(
    at: R,
    base_path: &str,
) -> VirtualDom {
    let base_path = base_path.to_string();
    let mut vdom = VirtualDom::new_with_props(
        || {
            rsx! {
                h1 { "App" }
                HistoryProvider {
                    history: move |_| {
                        let mut history = MemoryHistory::with_initial_path(at.clone().to_string());
                        history.set_prefix(base_path.clone());
                        Rc::new(history) as Rc<dyn History>
                    },
                    Router::<R> {}
                }
            }
        },
        (),
    );
    vdom.rebuild_in_place();
    vdom
}

// ============ 测试 1: 内部路由链接 ============
#[test]
fn href_internal() {
    #[derive(Routable, Clone, PartialEq)]
    enum Route {
        #[route("/")]
        Root {},
        #[route("/test")]
        Test {},
    }

    #[component]
    fn Root() -> Element {
        rsx! {
            Link {
                to: Route::Test {},
                "Link"
            }
        }
    }

    #[component]
    fn Test() -> Element {
        rsx! { "Test Page" }
    }

    let vdom = prepare_at(Route::Root {});
    let html = dioxus_ssr::render(&vdom);
    assert_eq!(html, r#"<h1>App</h1><a href="/test">Link</a>"#);
}

#[test]
fn href_internal_with_base_path() {
    #[derive(Routable, Clone, PartialEq)]
    enum Route {
        #[route("/")]
        Root {},
        #[route("/test")]
        Test {},
    }

    #[component]
    fn Root() -> Element {
        rsx! {
            Link {
                to: Route::Test {},
                "Link"
            }
        }
    }

    #[component]
    fn Test() -> Element {
        rsx! { "Test Page" }
    }

    let vdom = prepare_at_with_base_path(Route::Root {}, "/deeply/nested/path");
    let html = dioxus_ssr::render(&vdom);
    assert_eq!(
        html,
        r#"<h1>App</h1><a href="/deeply/nested/path/test">Link</a>"#
    );
}

// ============ 测试 2: 外部链接 ============
#[test]
fn href_external() {
    #[derive(Routable, Clone, PartialEq)]
    enum Route {
        #[route("/")]
        Root {},
    }

    #[component]
    fn Root() -> Element {
        rsx! {
            Link {
                to: "https://dioxuslabs.com/",
                "Link"
            }
        }
    }

    let vdom = prepare_at(Route::Root {});
    let html = dioxus_ssr::render(&vdom);
    assert_eq!(
        html,
        r#"<h1>App</h1><a href="https://dioxuslabs.com/" rel="noopener noreferrer">Link</a>"#
    );
}

#[test]
fn href_external_with_base_path() {
    #[derive(Routable, Clone, PartialEq)]
    enum Route {
        #[route("/")]
        Root {},
    }

    #[component]
    fn Root() -> Element {
        rsx! {
            Link {
                to: "https://dioxuslabs.com/",
                "Link"
            }
        }
    }

    let vdom = prepare_at_with_base_path(Route::Root {}, "/deeply/nested/path");
    let html = dioxus_ssr::render(&vdom);
    // 外部链接不应受 base_path 影响
    assert_eq!(
        html,
        r#"<h1>App</h1><a href="https://dioxuslabs.com/" rel="noopener noreferrer">Link</a>"#
    );
}

// ============ 测试 3: 自定义 CSS class ============
#[test]
fn with_class() {
    #[derive(Routable, Clone, PartialEq)]
    enum Route {
        #[route("/")]
        Root {},
        #[route("/test")]
        Test {},
    }

    #[component]
    fn Root() -> Element {
        rsx! {
            Link {
                to: Route::Test {},
                class: "test_class",
                "Link"
            }
        }
    }

    #[component]
    fn Test() -> Element {
        rsx! { "Test Page" }
    }

    let vdom = prepare_at(Route::Root {});
    let html = dioxus_ssr::render(&vdom);
    assert_eq!(
        html,
        r#"<h1>App</h1><a href="/test" class="test_class">Link</a>"#
    );
}

// ============ 测试 4: 激活状态的 active_class ============
#[test]
fn with_active_class_active() {
    #[derive(Routable, Clone, PartialEq)]
    enum Route {
        #[route("/")]
        Root {},
        #[route("/test")]
        Test {},
    }

    #[component]
    fn Root() -> Element {
        rsx! {
            Link {
                to: Route::Root {},
                class: "test_class",
                active_class: "active_class",
                "Link"
            }
        }
    }

    #[component]
    fn Test() -> Element {
        rsx! { "Test Page" }
    }

    let vdom = prepare_at(Route::Root {});
    let html = dioxus_ssr::render(&vdom);
    assert_eq!(
        html,
        r#"<h1>App</h1><a href="/" class="test_class active_class" aria-current="page">Link</a>"#
    );
}

// ============ 测试 5: 非激活状态的 active_class ============
#[test]
fn with_active_class_inactive() {
    #[derive(Routable, Clone, PartialEq)]
    enum Route {
        #[route("/")]
        Root {},
        #[route("/test")]
        Test {},
    }

    #[component]
    fn Root() -> Element {
        rsx! {
            Link {
                to: Route::Test {},
                class: "test_class",
                active_class: "active_class",
                "Link"
            }
        }
    }

    #[component]
    fn Test() -> Element {
        rsx! { "Test Page" }
    }

    let vdom = prepare_at(Route::Root {});
    let html = dioxus_ssr::render(&vdom);
    // 非激活状态不应包含 active_class 和 aria-current
    assert_eq!(
        html,
        r#"<h1>App</h1><a href="/test" class="test_class">Link</a>"#
    );
}

// ============ 测试 6: 自定义 id 属性 ============
#[test]
fn with_id() {
    #[derive(Routable, Clone, PartialEq)]
    enum Route {
        #[route("/")]
        Root {},
        #[route("/test")]
        Test {},
    }

    #[component]
    fn Root() -> Element {
        rsx! {
            Link {
                to: Route::Test {},
                id: "test_id",
                "Link"
            }
        }
    }

    #[component]
    fn Test() -> Element {
        rsx! { "Test Page" }
    }

    let vdom = prepare_at(Route::Root {});
    let html = dioxus_ssr::render(&vdom);
    assert_eq!(
        html,
        r#"<h1>App</h1><a href="/test" id="test_id">Link</a>"#
    );
}

// ============ 测试 7: 内部链接新标签页打开 ============
#[test]
fn with_new_tab() {
    #[derive(Routable, Clone, PartialEq)]
    enum Route {
        #[route("/")]
        Root {},
        #[route("/test")]
        Test {},
    }

    #[component]
    fn Root() -> Element {
        rsx! {
            Link {
                to: Route::Test {},
                new_tab: true,
                "Link"
            }
        }
    }

    #[component]
    fn Test() -> Element {
        rsx! { "Test Page" }
    }

    let vdom = prepare_at(Route::Root {});
    let html = dioxus_ssr::render(&vdom);
    assert_eq!(
        html,
        r#"<h1>App</h1><a href="/test" target="_blank">Link</a>"#
    );
}

// ============ 测试 8: 外部链接新标签页打开 ============
#[test]
fn with_new_tab_external() {
    #[derive(Routable, Clone, PartialEq)]
    enum Route {
        #[route("/")]
        Root {},
    }

    #[component]
    fn Root() -> Element {
        rsx! {
            Link {
                to: "https://dioxuslabs.com/",
                new_tab: true,
                "Link"
            }
        }
    }

    let vdom = prepare_at(Route::Root {});
    let html = dioxus_ssr::render(&vdom);
    assert_eq!(
        html,
        r#"<h1>App</h1><a href="https://dioxuslabs.com/" rel="noopener noreferrer" target="_blank">Link</a>"#
    );
}

// ============ 测试 9: 自定义 rel 属性 ============
#[test]
fn with_rel() {
    #[derive(Routable, Clone, PartialEq)]
    enum Route {
        #[route("/")]
        Root {},
        #[route("/test")]
        Test {},
    }

    #[component]
    fn Root() -> Element {
        rsx! {
            Link {
                to: Route::Test {},
                rel: "test_rel",
                "Link"
            }
        }
    }

    #[component]
    fn Test() -> Element {
        rsx! { "Test Page" }
    }

    let vdom = prepare_at(Route::Root {});
    let html = dioxus_ssr::render(&vdom);
    assert_eq!(
        html,
        r#"<h1>App</h1><a href="/test" rel="test_rel">Link</a>"#
    );
}

// ============ 测试 10: 嵌套路由链接 ============
#[test]
fn with_child_route() {
    #[derive(Routable, Clone, PartialEq)]
    enum Route {
        #[route("/")]
        Root {},
        #[route("/test")]
        Test {},
        #[nest("/child")]
            #[route("/")]
            ChildRoot {},
            #[route("/:id")]
            ChildId { id: String },
        #[end_nest]
    }

    #[component]
    fn Root() -> Element {
        rsx! {
            Link {
                to: Route::Test {},
                "Parent Link"
            }
            Link {
                to: Route::ChildId {
                    id: "this-is-a-child-route".to_string()
                },
                "Child Link"
            }
        }
    }

    #[component]
    fn Test() -> Element {
        rsx! { "Test Page" }
    }

    #[component]
    fn ChildRoot() -> Element {
        rsx! {
            Link {
                to: Route::Test {},
                "Parent Link"
            }
            Link {
                to: Route::ChildId {
                    id: "this-is-a-child-route".to_string()
                },
                "Child Link 1"
            }
            Link {
                to: Route::ChildId {
                    id: "this-is-a-child-route".to_string()
                },
                "Child Link 2"
            }
        }
    }

    #[component]
    fn ChildId(id: String) -> Element {
        rsx! { "Child {id}" }
    }

    // 在根路由测试
    let vdom = prepare_at(Route::Root {});
    let html = dioxus_ssr::render(&vdom);
    assert_eq!(
        html,
        r#"<h1>App</h1><a href="/test">Parent Link</a><a href="/child/this-is-a-child-route">Child Link</a>"#
    );

    // 在子路由测试
    let vdom = prepare_at(Route::ChildRoot {});
    let html = dioxus_ssr::render(&vdom);
    assert_eq!(
        html,
        r#"<h1>App</h1><a href="/test">Parent Link</a><a href="/child/this-is-a-child-route">Child Link 1</a><a href="/child/this-is-a-child-route">Child Link 2</a>"#
    );
}

// ============ 测试 11: Hash 片段路由 ============
#[test]
fn with_hash_segment() {
    #[derive(Routable, Clone, PartialEq)]
    enum Route {
        #[route("/")]
        Root {},
        #[route("/#:data")]
        Hash { data: String },
    }

    #[component]
    fn Root() -> Element {
        rsx! {
            Link {
                to: Route::Hash {
                    data: "test".to_string()
                },
                "Link"
            }
            Link {
                to: Route::Hash {
                    data: "".to_string()
                },
                "Empty"
            }
        }
    }

    #[component]
    fn Hash(data: String) -> Element {
        rsx! { "Hash {data}" }
    }

    // 在 Hash 路由测试
    let vdom = prepare_at(Route::Hash {
        data: "test".to_string(),
    });
    let html = dioxus_ssr::render(&vdom);
    // 当前路由激活，应包含 aria-current="page"
    assert_eq!(
        html,
        r#"<h1>App</h1><a href="/#test" aria-current="page">Link</a><a href="/">Empty</a>"#
    );
}
