// 测试文件：subscribe.rs
// 测试信号（signal）的读取订阅机制

use dioxus::prelude::*;
use dioxus_signals::Signal;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Default)]
struct RunCounter {
    parent: usize,
    children: Rc<RefCell<HashMap<ScopeId, usize>>>,
}

#[test]
fn reading_subscribes() {
    // 初始化 tracing_subscriber 用于调试输出
    tracing_subscriber::fmt::init();

    // 创建运行计数器
    let counter = RunCounter::default();

    // 创建虚拟 DOM
    let mut dom = VirtualDom::new_with_props(|cx| {
        // 使用 use_signal 创建初始值为 0 的信号
        let mut signal = use_signal(cx, || 0);

        // 检查当前 generation 是否为 1，如果是则将信号值加 1
        if cx.generation() == 1 {
            signal += 1;
        }

        // 递增父组件运行计数器
        let current_scope_id = cx.scope_id();
        counter.parent += 1;

        // 渲染 10 个子组件
        (0..10).map(|i| {
            rsx! {
                Child {
                    signal: signal,
                    counter: counter.clone(),
                    index: i,
                }
            }
        })
    }, counter.clone());

    // 原地重建 DOM
    dom.rebuild_in_place();

    // 验证初始状态：父组件运行 1 次
    assert_eq!(counter.parent, 1);

    // 验证每个子组件运行 1 次
    let children_counts = counter.children.borrow();
    for (_scope_id, count) in children_counts.iter() {
        assert_eq!(*count, 1);
    }

    // 标记应用 scope 为脏状态
    dom.mark_dirty(ScopeId::APP);

    // 连续调用 render_immediate 两次
    dom.render_immediate(&mut NoOpMutations);
    dom.render_immediate(&mut NoOpMutations);

    // 验证最终状态：父组件运行 2 次
    assert_eq!(counter.parent, 2);

    // 验证每个子组件运行 2 次
    let children_counts = counter.children.borrow();
    for (_scope_id, count) in children_counts.iter() {
        assert_eq!(*count, 2);
    }
}

#[derive(Props, Clone, PartialEq)]
struct ChildProps {
    signal: Signal<i32>,
    counter: RunCounter,
    index: usize,
}

impl PartialEq for ChildProps {
    fn eq(&self, other: &Self) -> bool {
        // 基于 signal 进行比较
        self.signal == other.signal && self.index == other.index
    }
}

fn Child(cx: Scope<ChildProps>) -> Element {
    // 读取传入的信号值
    let _value = *cx.props.signal.read();

    // 获取当前 scope ID
    let scope_id = cx.scope_id();

    // 记录该子组件的运行次数
    let mut counts = cx.props.counter.children.borrow_mut();
    *counts.entry(scope_id).or_insert(0) += 1;

    // 返回显示元素
    rsx! {
        div { "Child {cx.props.index}: value" }
    }
}
