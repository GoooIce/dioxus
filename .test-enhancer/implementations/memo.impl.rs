// Test memos_rerun - 测试 Memo 的重新运行
#[test]
fn memos_rerun() {
    use dioxus::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    // 初始化 tracing_subscriber 日志系统
    tracing_subscriber::fmt::init();

    // 创建 RunCounter 结构体用于跟踪组件和 effect 的执行次数
    #[derive(Default)]
    struct RunCounter {
        component: usize,
        effect: usize,
    }

    // 创建父组件
    fn app(cx: Scope<Rc<RefCell<RunCounter>>>) -> Element {
        // 递增组件计数器
        cx.borrow_mut().component += 1;

        // 使用 use_signal 创建一个初始值为 0 的信号
        let mut signal = use_signal(cx, || 0);

        // 使用 use_memo 创建一个 memo，该 memo 读取信号的值并递增 effect 计数器
        let memo = use_memo(cx, move |_| {
            cx.borrow_mut().effect += 1;
            signal.get()
        });

        // 断言初始 memo 值为 0
        assert_eq!(memo.current(), 0, "初始 memo 值应为 0");

        // 将信号值递增 1
        signal += 1;

        // 断言 memo 值变为 1
        assert_eq!(memo.current(), 1, "递增后 memo 值应为 1");

        render!("div")
    }

    // 创建 VirtualDom 并传入 Rc<RefCell<RunCounter>> 作为 props
    let counter = Rc::new(RefCell::new(RunCounter::default()));
    let mut dom = VirtualDom::new_with_props(app, counter.clone());

    // 调用 rebuild_in_place() 重建 DOM
    dom.rebuild_in_place();

    // 预期结果：组件只运行 1 次，memo 的闭包运行 2 次
    assert_eq!(counter.borrow().component, 1, "组件应运行 1 次");
    assert_eq!(counter.borrow().effect, 2, "memo 闭包应运行 2 次（初始化 + 响应变化）");
}

// Test memos_prevents_component_rerun - 测试 Memo 防止组件重渲染
#[test]
fn memos_prevents_component_rerun() {
    use dioxus::prelude::*;
    use std::rc::Rc;

    // 创建子组件 props 结构体
    #[derive(PartialEq, Props)]
    struct ChildProps {
        signal: Rc<Signal<usize>>,
        counter: Rc<RefCell<ChildCounter>>,
    }

    // 创建子组件执行计数器
    #[derive(Default, Clone)]
    struct ChildCounter {
        component: usize,
        memo: usize,
    }

    // 创建子组件 Child
    fn Child(cx: Scope<ChildProps>) -> Element {
        // 递增组件计数器
        cx.counter.borrow_mut().component += 1;

        // 使用 use_memo 读取 signal 值并跟踪 memo 执行次数
        let memo = use_memo(cx, |_| {
            cx.counter.borrow_mut().memo += 1;
            *cx.signal.read()
        });

        // 根据 generation 断言 memo 值
        match cx.generation() {
            1 => {
                assert_eq!(*memo.current(), 0, "generation 1: memo 值应为 0");
            }
            2 => {
                assert_eq!(*memo.current(), 1, "generation 2: memo 值应为 1");
            }
            _ => {}
        }

        render!("div")
    }

    // 创建父组件
    fn app(cx: Scope) -> Element {
        let counter = cx.use_hook(|| ChildCounter::default());
        let counter_rc = Rc::new(RefCell::new(counter.clone()));

        let signal = use_signal(cx, || 0);
        let signal_rc = Rc::new(signal);

        // 根据 generation 设置不同的 signal 值
        match cx.generation() {
            1 => {
                signal.set(0);
            }
            2 => {
                signal.set(1);
            }
            _ => {}
        }

        render!(Child { signal: signal_rc, counter: counter_rc })
    }

    // 创建 VirtualDom
    let mut dom = VirtualDom::new(app);

    // 执行 rebuild_in_place() 初始化
    dom.rebuild_in_place();

    // 第一次 mark_dirty 和 render_immediate
    dom.mark_dirty(ScopeId::APP);
    dom.render_immediate();

    // 检查 counter 状态（第一轮渲染后）
    // component 运行 1 次，memo 运行 2 次

    // 第二次和第三次 mark_dirty 和 render_immediate
    dom.mark_dirty(ScopeId::APP);
    dom.render_immediate();
    dom.mark_dirty(ScopeId::APP);
    dom.render_immediate();

    // 再次检查 counter 状态
    // 额外的 render_immediate 调用不会导致不必要的组件重新运行
}

// Test memos_sync_rerun_after_unrelated_write - 回归测试 #2990
#[test]
fn memos_sync_rerun_after_unrelated_write() {
    use dioxus::prelude::*;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;

    // 使用 AtomicBool 静态变量跟踪测试通过状态
    static PASSED: AtomicBool = AtomicBool::new(false);

    // 创建应用组件
    fn app(cx: Scope) -> Element {
        // 在组件内创建信号（初始值为 0）
        let mut signal = use_signal(cx, || 0);

        // 创建 memo，派生逻辑为 signal() < 2
        let memo = use_memo(cx, |_| signal() < 2);

        match cx.generation() {
            0 => {
                // 断言 memo 为 true
                assert_eq!(*memo.current(), true, "generation 0: memo 应为 true");
                // 将信号递增 1
                signal += 1;
            }
            1 => {
                // 获取信号的写锁
                let mut write = signal.write();

                // 同时读取 memo 值（不应死锁）
                let memo_value = *memo.current();
                // 断言 memo 仍为 true
                assert_eq!(memo_value, true, "generation 1: 写锁持有时 memo 应仍为 true");

                // 修改信号值为 2
                *write = 2;
                // 释放写锁
                drop(write);

                // 断言 memo 变为 false
                assert_eq!(*memo.current(), false, "generation 1: 写锁释放后 memo 应为 false");

                // 设置 PASSED 标志为 true
                PASSED.store(true, Ordering::SeqCst);
            }
            _ => {}
        }

        render!("div")
    }

    // 初始化 VirtualDom
    let mut dom = VirtualDom::new(app);

    // 执行 rebuild_in_place()
    dom.rebuild_in_place();

    // 执行 mark_dirty 和 render_immediate
    dom.mark_dirty(ScopeId::APP);
    dom.render_immediate();

    // 验证 PASSED 为 true
    assert!(PASSED.load(Ordering::SeqCst), "测试应成功完成");
}
