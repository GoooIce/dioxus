// Test file: create.rs
// Tests for signal creation, dereferencing, and dropping behavior

use dioxus::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::cell::Cell;

/// Test 1: create_signals_global
/// Verifies that signals can be created without a Dioxus context
#[test]
fn create_signals_global() {
    // Function that creates a signal without any Dioxus context
    fn create_without_cx() -> Signal<i32> {
        Signal::new(42)
    }

    // Create a VirtualDom with multiple Child components
    let dom = VirtualDom::new(|| {
        // Render 10 Child components
        (0..10).map(|_| rsx! { Child {} })
    });

    // Define the Child component that creates signals without context
    #[component]
    fn Child() -> Element {
        // Create a signal without using the context
        let signal = create_without_cx();

        // Render the signal value in RSX
        rsx! {
            div { "Signal value: {signal}" }
        }
    }

    // Rebuild the virtual DOM in place
    dom.rebuild_in_place();

    // If we reach here, signals were successfully created without context
    // and the program did not crash
}

/// Test 2: deref_signal
/// Verifies that signals can be dereferenced using function call syntax
#[test]
fn deref_signal() {
    // Create a VirtualDom with 10 Child components
    let dom = VirtualDom::new(|| {
        (0..10).map(|_| rsx! { Child {} })
    });

    #[component]
    fn Child() -> Element {
        // Create a signal with a string value
        let signal = Signal::new("hello world");

        // Use function call syntax to get Ref<T>
        let _ref = signal();

        // Dereference to get the actual value
        let value = &*signal();

        // Verify the value equals "hello world"
        assert_eq!(value, "hello world");

        rsx! {
            div { "Signal value: {signal}" }
        }
    }

    // Rebuild the virtual DOM in place
    dom.rebuild_in_place();
}

/// Test 3: drop_signals
/// Verifies that signals and their contained values are properly dropped
#[test]
fn drop_signals() {
    // Static atomic counter to track drops
    static SIGNAL_DROP_COUNT: AtomicUsize = AtomicUsize::new(0);

    // Custom type that implements Drop to track when it's dropped
    #[derive(Clone)]
    struct TracksDrops;

    impl Drop for TracksDrops {
        fn drop(&mut self) {
            SIGNAL_DROP_COUNT.fetch_add(1, Ordering::SeqCst);
        }
    }

    // Use a cell to track the current generation
    let generation = Cell::new(0usize);

    // Create VirtualDom with conditional rendering based on generation
    let dom = VirtualDom::new(|| {
        // Get the current generation value
        let gen = generation.get();

        // Render 10 Child components when generation is even, 0 when odd
        if gen % 2 == 0 {
            (0..10).map(|_| rsx! { Child {} })
        } else {
            rsx! { "No children" }
        }
    });

    #[component]
    fn Child() -> Element {
        // Create a signal containing TracksDrops using use_signal
        // This tracks the signal lifecycle within the component
        let _signal = use_signal(|| TracksDrops);

        rsx! {
            div { "Child with tracked signal" }
        }
    }

    // Initial build - creates 10 children with signals
    dom.rebuild_in_place();

    // Change generation to odd (will cause children to be removed)
    generation.set(1);

    // Mark APP as dirty and re-render
    ScopeId::APP.mark_dirty();
    dom.render_immediate();

    // Verify that SIGNAL_DROP_COUNT equals 10
    // This proves all 10 signals were properly dropped when children were removed
    assert_eq!(SIGNAL_DROP_COUNT.load(Ordering::SeqCst), 10);
}
