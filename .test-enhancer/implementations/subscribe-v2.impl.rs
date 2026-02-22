// Test: reading_subscribes
// Purpose: 验证信号（signal）的读取订阅机制是否正确工作。
// 当父组件中的信号被读取时，所有订阅该信号的子组件应该在信号变化时被正确重新渲染。

#![allow(unused, non_upper_case_globals, non_snake_case)]

use dioxus::prelude::*;
use dioxus_core::{current_scope_id, generation, NoOpMutations, ScopeId};
use dioxus_signals::*;
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn reading_subscribes() {
    // 初始化 tracing_subscriber（用于调试输出）
    tracing_subscriber::fmt::init();

    /// 用于追踪父组件和各子组件运行次数的结构体
    #[derive(Default)]
    struct RunCounter {
        parent: usize,
        children: HashMap<ScopeId, usize>,
    }

    let counter = Rc::new(RefCell::new(RunCounter::default()));

    /// 子组件的 Props
    #[derive(Props, Clone)]
    struct ChildProps {
        signal: Signal<usize>,
        counter: Rc<RefCell<RunCounter>>,
    }

    // 子组件需要实现 PartialEq（基于 signal 比较）
    impl PartialEq for ChildProps {
        fn eq(&self, other: &Self) -> bool {
            self.signal == other.signal
        }
    }

    /// 子组件：读取并显示传入的信号值，记录运行次数
    fn Child(props: ChildProps) -> Element {
        println!("Child: {:?}", current_scope_id());

        // 记录该 scope 的运行次数
        *props
            .counter
            .borrow_mut()
            .children
            .entry(current_scope_id())
            .or_default() += 1;

        // 在 rsx 中直接使用 signal - 这会建立订阅关系
        rsx! {
            "{props.signal}"
        }
    }

    // 创建虚拟 DOM，传入 counter 作为 props
    let mut dom = VirtualDom::new_with_props(
        |props: Rc<RefCell<RunCounter>>| {
            // 使用 use_signal 创建初始值为 0 的信号
            let mut signal = use_signal(|| 0);

            println!("Parent: {:?}", current_scope_id());

            // 检查 current generation 是否为 1，如果是则将信号值加 1
            // 注意：generation() 从 0 开始，这里检查 1 意味着在第二次渲染时触发
            if generation() == 1 {
                signal += 1;
            }

            // 递增父组件运行计数器
            props.borrow_mut().parent += 1;

            // 渲染 10 个子组件
            rsx! {
                for id in 0..10 {
                    Child {
                        signal: signal,
                        counter: props.clone()
                    }
                }
            }
        },
        counter.clone(),
    );

    // 重建 DOM
    dom.rebuild_in_place();

    // 验证初始状态：父组件运行 1 次，每个子组件运行 1 次
    {
        let current_counter = counter.borrow();
        assert_eq!(current_counter.parent, 1, "父组件应该运行 1 次");

        for (scope_id, rerun_count) in current_counter.children.iter() {
            assert_eq!(rerun_count, &1, "子组件 {:?} 应该运行 1 次", scope_id);
        }
    }

    // 标记应用 scope 为脏状态
    dom.mark_dirty(ScopeId::APP);

    // 连续调用 render_immediate 两次
    dom.render_immediate(&mut NoOpMutations);
    dom.render_immediate(&mut NoOpMutations);

    // 验证最终状态：父组件运行 2 次，每个子组件运行 2 次
    {
        let current_counter = counter.borrow();
        assert_eq!(current_counter.parent, 2, "父组件应该运行 2 次");

        for (scope_id, rerun_count) in current_counter.children.iter() {
            assert_eq!(rerun_count, &2, "子组件 {:?} 应该运行 2 次", scope_id);
        }
    }
}
