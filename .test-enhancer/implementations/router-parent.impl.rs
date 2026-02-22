#![allow(unused)]

use dioxus::prelude::*;

// 定义路由结构，包含嵌套、布局和层级关系
#[derive(Routable, Clone, PartialEq, Debug)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    RootIndex {},

    #[nest("/fixed")]
        #[layout(Fixed)]
            #[route("/")]
            FixedIndex {},
            #[route("/fixed")]
            FixedFixed {},
        #[end_layout]
    #[end_nest]

    #[nest("/:id")]
        #[layout(Parameter)]
            #[route("/")]
            ParameterIndex { id: u8 },
            #[route("/fixed")]
            ParameterFixed { id: u8 },
        #[end_layout]
    #[end_nest]

    #[nest("/hash")]
        #[route("/")]
        HashIndex {},
        #[nest("/:id")]
            #[route("/?:query")]
            HashId { id: u8, query: String },
            #[layout(Parameter)]
                #[route("/path/?:query#:hash")]
                HashQuery { id: u8, query: String, hash: String },
            #[end_layout]
        #[end_nest]
    #[end_nest]
}

// 组件定义
#[component]
fn RootIndex() -> Element {
    rsx! { h2 { "Root Index" } }
}

#[component]
fn Fixed() -> Element {
    rsx! {
        h2 { "Fixed" }
        Outlet::<Route> {}
    }
}

#[component]
fn FixedIndex() -> Element {
    rsx! { h3 { "Fixed - Index" } }
}

#[component]
fn FixedFixed() -> Element {
    rsx! { h3 { "Fixed - Fixed"} }
}

#[component]
fn Parameter(id: u8) -> Element {
    rsx! {
        h2 { "Parameter {id}" }
        Outlet::<Route> {}
    }
}

#[component]
fn ParameterIndex(id: u8) -> Element {
    rsx! { h3 { "Parameter - Index" } }
}

#[component]
fn ParameterFixed(id: u8) -> Element {
    rsx! { h3 { "Parameter - Fixed" } }
}

#[component]
fn HashQuery(id: u8, query: String, hash: String) -> Element {
    rsx! {
        h2 { "Hash Query" }
        h3 { "id: {id}" }
        h3 { "query: {query}" }
        h3 { "hash: {hash}" }
    }
}

#[component]
fn HashIndex() -> Element {
    rsx! { h3 { "Hash Index" } }
}

#[component]
fn HashId(id: u8, query: String) -> Element {
    rsx! {
        h3 { "Hash Id {id}" }
        h3 { "query: {query}" }
    }
}

// 测试 1：get_parent
#[test]
fn get_parent() {
    // 根路由无父级测试
    assert_eq!(Route::RootIndex {}.parent(), None);

    // 固定嵌套父级测试
    assert_eq!(
        Route::FixedIndex {}.parent(),
        Some(Route::RootIndex {})
    );
    assert_eq!(
        Route::FixedFixed {}.parent(),
        Some(Route::FixedIndex {})
    );

    // 参数化嵌套父级测试
    assert_eq!(
        Route::ParameterIndex { id: 0 }.parent(),
        Some(Route::RootIndex {})
    );
    assert_eq!(
        Route::ParameterFixed { id: 0 }.parent(),
        Some(Route::ParameterIndex { id: 0 })
    );

    // Hash 和 Query 嵌套父级测试
    // HashQuery 的父级应为 HashId（query 为空字符串，因为在父路由中 query 被移除了）
    assert_eq!(
        Route::HashQuery {
            id: 0,
            query: "query".into(),
            hash: "hash".into()
        }
        .parent(),
        Some(Route::HashId {
            id: 0,
            query: "".into()
        })
    );
    // HashId 的父级应为 HashIndex
    assert_eq!(
        Route::HashId {
            id: 0,
            query: "query".into()
        }
        .parent(),
        Some(Route::HashIndex {})
    );
    // HashIndex 的父级应为 RootIndex
    assert_eq!(Route::HashIndex {}.parent(), Some(Route::RootIndex {}));
}

// 测试 2：is_child
#[test]
fn is_child() {
    // 自反性测试（路由不是自己的子级）
    assert!(!Route::RootIndex {}.is_child_of(&Route::RootIndex {}));
    assert!(Route::FixedIndex {}.is_child_of(&Route::RootIndex {}));
    assert!(!Route::FixedIndex {}.is_child_of(&Route::FixedIndex {}));
    assert!(Route::FixedFixed {}.is_child_of(&Route::FixedIndex {}));
    assert!(!Route::FixedFixed {}.is_child_of(&Route::FixedFixed {}));

    // 参数化嵌套父子关系测试
    assert!(Route::ParameterIndex { id: 0 }.is_child_of(&Route::RootIndex {}));
    assert!(!Route::ParameterIndex { id: 0 }.is_child_of(&Route::ParameterIndex { id: 0 }));
    assert!(Route::ParameterFixed { id: 0 }.is_child_of(&Route::ParameterIndex { id: 0 }));
    assert!(!Route::ParameterFixed { id: 0 }.is_child_of(&Route::ParameterFixed { id: 0 }));

    // Hash 和 Query 嵌套父子关系测试
    assert!(Route::HashQuery {
        id: 0,
        query: "query".into(),
        hash: "hash".into()
    }
    .is_child_of(&Route::HashId {
        id: 0,
        query: "query".into()
    }));
    assert!(!Route::HashQuery {
        id: 0,
        query: "query".into(),
        hash: "hash".into()
    }
    .is_child_of(&Route::HashQuery {
        id: 0,
        query: "query".into(),
        hash: "hash".into()
    }));
    assert!(Route::HashId {
        id: 0,
        query: "query".into()
    }
    .is_child_of(&Route::HashIndex {}));
    assert!(!Route::HashId {
        id: 0,
        query: "query".into()
    }
    .is_child_of(&Route::HashId {
        id: 0,
        query: "query".into()
    }));
    assert!(Route::HashIndex {}.is_child_of(&Route::RootIndex {}));
    assert!(!Route::HashIndex {}.is_child_of(&Route::HashIndex {}));
}
