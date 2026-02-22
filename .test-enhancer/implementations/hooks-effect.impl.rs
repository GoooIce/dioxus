// 测试：effect hooks 的响应式行为

use dioxus::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Default)]
struct RunCounter {
    component: usize,
    effect: usize,
}

/// 测试 1：effects_rerun
///
/// 验证 `use_effect` 能够正确地响应依赖信号的变化并重新运行。
#[test]
fn effects_rerun() {
    // 创建计数器
    let counter = Rc::new(RefCell::new(RunCounter::default()));

    // 定义组件
    let app = |counter: Rc<RefCell<RunCounter>>| -> Element {
        // 递增组件计数器
        counter.borrow_mut().component += 1;

        // 创建信号
        let mut signal = use_signal(|| 0);

        // 创建 effect，订阅信号变化
        use_effect({
            to_owned![counter];
            move || {
                // 读取信号值以建立依赖
                let _ = signal();
                // 递增 effect 计数器
                counter.borrow_mut().effect += 1;
            }
        });

        // 递增信号值
        signal += 1;

        // 手动停止等待
        dioxus_core::needs_update();

        rsx! { div { "Signal: {signal}" } }
    };

    // 创建 VirtualDom
    let mut vdom = VirtualDom::new_with_props(app, counter.clone());

    // 重建 DOM
    vdom.rebuild_in_place();

    // 等待工作完成
    vdom.wait_for_work().timeout(std::time::Duration::from_millis(500)).run();

    // 验证结果
    let counter = counter.borrow();
    // 组件只运行 1 次
    assert_eq!(counter.component, 1, "组件应该只运行 1 次");
    // effect 运行 1 次
    assert_eq!(counter.effect, 1, "effect 应该运行 1 次");
}

/// 测试 2：effects_rerun_without_rerender
///
/// 回归测试 issue #2347。
/// 验证 effect 能够在组件不重新渲染的情况下响应信号变化并重新运行。
#[tokio::test]
async fn effects_rerun_without_rerender() {
    // 创建计数器
    let counter = Rc::new(RefCell::new(RunCounter::default()));

    // 定义组件
    let app = |counter: Rc<RefCell<RunCounter>>| -> Element {
        // 递增组件计数器
        counter.borrow_mut().component += 1;

        // 创建信号
        let mut signal = use_signal(|| 0);

        // 创建 effect，订阅信号变化
        use_effect({
            to_owned![counter];
            move || {
                // 读取信号值以建立依赖
                let _ = signal();
                // 递增 effect 计数器
                counter.borrow_mut().effect += 1;
            }
        });

        // 创建 future，异步修改信号值
        use_future({
            to_owned![counter, mut signal];
            async move {
                // 循环 10 次，每次递增信号
                for _ in 0..10 {
                    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
                    signal += 1;
                }
            }
        });

        rsx! { div { "Signal: {signal}" } }
    };

    // 创建 VirtualDom
    let mut vdom = VirtualDom::new_with_props(app, counter.clone());

    // 重建 DOM
    vdom.rebuild_in_place();

    // 等待工作完成
    vdom.wait_for_work().timeout(std::time::Duration::from_millis(500)).run();

    // 验证结果
    let counter = counter.borrow();
    // 组件只运行 1 次，证明没有重新渲染
    assert_eq!(counter.component, 1, "组件应该只运行 1 次（没有重新渲染）");
    // effect 运行 11 次：第一次初始化 + 10 次信号变化
    assert_eq!(counter.effect, 11, "effect 应该运行 11 次（初始化 + 10 次变化）");
}
