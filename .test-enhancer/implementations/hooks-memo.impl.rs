#![allow(unused, non_upper_case_globals, non_snake_case)]
use std::cell::RefCell;

use dioxus::prelude::*;
use dioxus_signals::*;

thread_local! {
    static VEC_SIGNAL: RefCell<Option<Signal<Vec<usize>>>> = RefCell::new(None);
}

/// 测试 1：memo_updates
///
/// 验证 `use_memo` 在多线程环境下的响应性和更新机制
#[tokio::test]
async fn memo_updates() {
    // 子组件：使用 memo 获取 vec 中的特定元素
    fn Child(index: usize, vec: Signal<Vec<usize>>) -> Element {
        // 创建 item memo，读取 vec 中的特定索引
        let item = use_memo(move || vec()[index]);

        rsx! {
            div { "Item: {item}" }
        }
    }

    // 主组件：使用同步信号和 memo
    let mut dom = VirtualDom::new(|| {
        // 创建跨线程同步信号
        let mut vec = use_signal_sync(|| vec![0, 1, 2]);

        // 注册 hook：设置 thread_local 并启动后台线程
        use_hook(|| {
            // 将 vec 存入 thread_local
            VEC_SIGNAL.set(Some(vec));

            // 启动后台线程，100ms 后添加元素
            std::thread::spawn(move || {
                std::thread::sleep(std::time::Duration::from_millis(100));
                vec.write().push(5);
            });
        });

        // 创建 len_memo，追踪 vec 的长度
        let len_memo = use_memo(move || vec().len());

        // 在前两次渲染时，添加 len 到 vec
        if dioxus_core::internal::generation() < 2 {
            vec.write().push(len_memo());
        }

        // 断言 vec.len() 等于 len_memo
        assert_eq!(vec().len(), len_memo());

        // 渲染多个子组件
        rsx! {
            Child { index: 0, vec }
            Child { index: 1, vec }
            Child { index: 2, vec }
        }
    });

    // 重建 DOM
    dom.rebuild_in_place();

    // 处理初始更新
    tokio::select! {
        _ = dom.wait_for_work() => {}
        _ = tokio::time::sleep(tokio::time::Duration::from_secs(1)) => panic!("timed out")
    }
    dom.render_immediate();

    // 等待后台线程的更新
    tokio::select! {
        _ = dom.wait_for_work() => {}
        _ = tokio::time::sleep(tokio::time::Duration::from_secs(1)) => panic!("timed out")
    }
    dom.render_immediate();

    // 获取最终 signal 值并断言
    let vec = VEC_SIGNAL.with(|inner| inner.borrow().clone().unwrap());
    assert_eq!(*vec.read(), vec![0, 1, 2, 3, 4, 5]);

    // 循环 6 次，每次弹出一个元素
    for _ in 0..6 {
        vec.write().pop();

        tokio::select! {
            _ = dom.wait_for_work() => {}
            _ = tokio::time::sleep(tokio::time::Duration::from_secs(1)) => panic!("timed out")
        }
        dom.render_immediate();
    }
}

/// 测试 2：use_memo_only_triggers_one_update
///
/// 验证 `use_memo` 的批量更新优化机制
#[tokio::test]
async fn use_memo_only_triggers_one_update() {
    // thread_local 用于追踪 memo 更新
    thread_local! {
        static VEC_SIGNAL: RefCell<Vec<usize>> = RefCell::new(Vec::new());
    }

    let mut dom = VirtualDom::new(|| {
        // 创建 count 信号
        let mut count = use_signal(|| 0);

        // 创建第一个 memo：count * 2
        let memorized = use_memo(move || count() * 2);

        // 创建第二个 memo：读取 memorized 并记录到 VEC_SIGNAL
        use_memo(move || {
            let value = memorized();
            VEC_SIGNAL.with(|v| v.borrow_mut().push(value));
            value
        });

        // 注册 hook：连续 10 次写入 count
        use_hook(|| {
            for _ in 0..10 {
                count += 1;
                // 读取 memorized 值（触发订阅）
                let _ = memorized();
            }
        });

        rsx! {
            div { "Count: {count}" }
        }
    });

    // 重建 DOM
    dom.rebuild_in_place();

    // 等待工作完成或超时
    tokio::select! {
        _ = dom.wait_for_work() => {}
        _ = tokio::time::sleep(tokio::time::Duration::from_millis(100)) => {}
    }

    // 立即渲染
    dom.render_immediate();

    // 断言 VEC_SIGNAL 只记录了初始值和最终值
    VEC_SIGNAL.with(|v| {
        assert_eq!(*v.borrow(), vec![0, 20]);
    });
}
