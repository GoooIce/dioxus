#![allow(unused, non_upper_case_globals, non_snake_case)]

use dioxus::prelude::*;
use dioxus_core::{generation, NoOpMutations, ScopeId};
use dioxus_signals::*;

/// 测试 1：create_signals_global
///
/// 目的：验证在没有 Dioxus 上下文（Scope）的情况下创建和使用全局信号的功能
///
/// 测试步骤：
/// 1. 创建一个 VirtualDom，包含 10 个 `Child` 组件
/// 2. 在 `Child` 组件内部调用 `create_without_cx()` 函数
/// 3. `create_without_cx()` 函数在没有 Dioxus 上下文的情况下调用 `Signal::new()` 创建信号
/// 4. 在 RSX 中渲染信号的值
/// 5. 执行 `dom.rebuild_in_place()` 重建虚拟 DOM
///
/// 预期结果：信号成功在无上下文环境中创建，信号值能正确渲染到组件中
#[test]
fn create_signals_global() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            for _ in 0..10 {
                Child {}
            }
        }
    });

    fn Child() -> Element {
        let signal = create_without_cx();

        rsx! {
            "{signal}"
        }
    }

    dom.rebuild_in_place();

    fn create_without_cx() -> Signal<String> {
        Signal::new("hello world".to_string())
    }
}

/// 测试 2：deref_signal
///
/// 目的：验证信号的解引用（dereference）功能
///
/// 测试步骤：
/// 1. 创建 VirtualDom，包含 10 个 `Child` 组件
/// 2. 在 `Child` 组件中使用 `Signal::new()` 创建字符串信号
/// 3. 使用 `signal()` 函数调用语法获取信号的 `Ref`
/// 4. 使用 `&*signal()` 解引用并验证值是否为 "hello world"
/// 5. 执行 `dom.rebuild_in_place()`
///
/// 预期结果：信号值正确存储和访问
#[test]
fn deref_signal() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            for _ in 0..10 {
                Child {}
            }
        }
    });

    fn Child() -> Element {
        let signal = Signal::new("hello world".to_string());

        // 可以通过函数调用语法获取信号的 Ref，然后解引用获取值
        assert_eq!(&*signal(), "hello world");

        rsx! {
            "hello world"
        }
    }

    dom.rebuild_in_place();
}

/// 测试 3：drop_signals
///
/// 目的：验证信号及其包含的值在适当的时机被正确释放（drop）
///
/// 测试步骤：
/// 1. 定义静态原子计数器 `SIGNAL_DROP_COUNT` 初始值为 0
/// 2. 定义 `TracksDrops` 结构体，实现 `Drop` trait 来增加计数器
/// 3. 创建 VirtualDom，根据 `generation` 值决定渲染 10 个或 0 个 `Child` 组件
/// 4. 在 `Child` 组件中使用 `use_signal(|| TracksDrops)` 创建包含 `TracksDrops` 的信号
/// 5. 执行 `dom.rebuild_in_place()` 初始构建
/// 6. 标记 `ScopeId::APP` 为脏并执行 `render_immediate()`
/// 7. 验证 `SIGNAL_DROP_COUNT` 的值等于 10
///
/// 预期结果：当 generation 为偶数时创建 10 个子组件，重新渲染时（generation 变为奇数），
/// 子组件被移除，信号包含的 `TracksDrops` 值被正确释放
#[test]
fn drop_signals() {
    use std::sync::atomic::{AtomicUsize, Ordering};

    static SIGNAL_DROP_COUNT: AtomicUsize = AtomicUsize::new(0);

    let mut dom = VirtualDom::new(|| {
        let gen = generation();

        let count = if gen % 2 == 0 { 10 } else { 0 };
        rsx! {
            for _ in 0..count {
                Child {}
            }
        }
    });

    fn Child() -> Element {
        struct TracksDrops;

        impl Drop for TracksDrops {
            fn drop(&mut self) {
                SIGNAL_DROP_COUNT.fetch_add(1, Ordering::Relaxed);
            }
        }

        use_signal(|| TracksDrops);

        rsx! {
            ""
        }
    }

    dom.rebuild_in_place();
    dom.mark_dirty(ScopeId::APP);
    dom.render_immediate(&mut NoOpMutations);

    assert_eq!(SIGNAL_DROP_COUNT.load(Ordering::Relaxed), 10);
}
