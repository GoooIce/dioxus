use dioxus::prelude::*;

/// 测试 `Routable` trait 的 `static_routes()` 方法能够正确生成静态路由列表
///
/// 验证点：
/// 1. 静态路由识别正确性
/// 2. 嵌套路由的静态变体展开
/// 3. 动态路由被正确排除
#[test]
fn with_class() {
    // 定义子路由结构
    #[derive(Routable, Clone, PartialEq, Debug)]
    enum ChildRoute {
        // 子路由根路径（静态）
        #[route("/")]
        ChildRoot {},
        // 动态参数路径（不应出现在静态路由列表中）
        #[route("/:not_static")]
        NotStatic { not_static: String },
    }

    // 定义父路由结构，包含嵌套的子路由
    #[derive(Routable, Clone, PartialEq, Debug)]
    enum Route {
        // 根路径（静态）
        #[route("/")]
        Root {},
        // 静态路径
        #[route("/test")]
        Test {},
        // 嵌套子路由的父路径
        #[child("/child")]
        Nested { child: ChildRoute },
    }

    // 组件实现（测试所需）
    #[component]
    fn Root() -> Element {
        unimplemented!()
    }

    #[component]
    fn Test() -> Element {
        unimplemented!()
    }

    #[component]
    fn ChildRoot() -> Element {
        unimplemented!()
    }

    #[component]
    fn NotStatic(not_static: String) -> Element {
        unimplemented!()
    }

    // 验证静态路由列表
    // 预期：
    // 1. Route::Root {} - 根路径是静态的
    // 2. Route::Test {} - /test 是静态路径
    // 3. Route::Nested { child: ChildRoute::ChildRoot {} } - 子路由的根路径是静态的
    //
    // 不应包含：
    // - Route::Nested { child: ChildRoute::NotStatic { .. } } - 因为 /:not_static 是动态路径
    assert_eq!(
        Route::static_routes(),
        vec![
            Route::Root {},
            Route::Test {},
            Route::Nested {
                child: ChildRoute::ChildRoot {}
            },
        ],
    );

    // 验证静态路由数量为 3
    assert_eq!(Route::static_routes().len(), 3);

    // 验证动态路由确实不在静态路由列表中
    let dynamic_route = Route::Nested {
        child: ChildRoute::NotStatic {
            not_static: "some_value".to_string(),
        },
    };
    assert!(!Route::static_routes().contains(&dynamic_route));
}
