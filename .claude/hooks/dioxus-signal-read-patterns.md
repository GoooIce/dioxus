# Hook: Dioxus Signal 读取模式

## 触发条件
- 使用 `signal.get()` 读取信号值
- 使用 `memo.current()` 读取 memo 值
- 使用 `signal.read()` 读取信号
- 使用 `signal.write()` 写入信号时未正确解引用

## 问题模式

### 错误：使用 .get() / .current() / .read()
```rust
// ❌ 错误
let value = signal.get();
let memo_value = memo.current();
let val = *signal.read();
```

### 正确：直接调用信号/memo
```rust
// ✅ 正确
let value = signal();      // Signal<T> 实现了 Fn trait
let memo_value = memo();   // Memo<T> 同样实现了 Fn trait
```

## Signal 写入模式

```rust
// ✅ 正确的写入方式
*signal.write() = new_value;  // 通过解引用赋值
signal += 1;                   // 使用运算符重载

// ❌ 错误 - set() 方法不存在
signal.set(new_value);
```

## 在 RSX 中使用

```rust
// ✅ 正确 - 直接在 rsx! 中使用 signal
rsx! {
    "{signal}"  // 自动订阅并显示值
}

// ❌ 错误 - 不需要手动读取
rsx! {
    "{signal()}"  // 多余的函数调用
}
```

## 读写同时持有的注意事项

```rust
// ✅ 正确 - 可以同时持有写锁和读取 memo
let mut write = signal.write();
let memo_value = memo();  // 不会死锁
*write = 2;
drop(write);
let new_memo = memo();    // 现在反映新值
```

## 类型说明
- `Signal<T>` 实现了 `Fn() -> ReadRef<T>`
- `Memo<T>` 实现了 `Fn() -> ReadRef<T>`
- `signal()` 返回 `ReadRef<T>`，可以解引用获取 `&T`
- `&*signal()` 获取 `&T` 引用

## 相关文件
- packages/signals/src/signal.rs
- packages/signals/src/memo.rs
