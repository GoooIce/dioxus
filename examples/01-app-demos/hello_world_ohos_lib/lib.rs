//! Dioxus Hello World for OHOS
//!
//! 这是一个 OHOS 动态库，通过 N-API 暴露给 OHOS 应用调用
//!
//! 使用方法：
//! 1. 编译为动态库: cargo build --target aarch64-unknown-linux-ohos
//! 2. 将生成的 libhello_world_ohos.so 放入 OHOS 项目的 libs 目录
//! 3. 在 OHOS 应用中通过 N-API 调用 start_dioxus_app 函数

use dioxus::prelude::*;

/// Dioxus 应用函数
fn app() -> Element {
    rsx! {
        div {
            style: "color: white; font-size: 30px; text-align: center; padding: 50px;",
            "Hello, world from Dioxus on OHOS!"
        }
    }
}

/// OHOS N-API 绑定 - 启动 Dioxus 应用
///
/// 这个函数可以被 OHOS 通过 N-API 调用
#[cfg(target_env = "ohos")]
#[no_mangle]
pub extern "C" fn start_dioxus_app() {
    println!("Dioxus OHOS: start_dioxus_app called");

    // 对于桌面平台（用于测试）
    #[cfg(not(target_env = "ohos"))]
    {
        dioxus::launch(app);
    }
}

/// OHOS N-API 绑定 - 获取应用的 HTML 内容
///
/// 返回 Dioxus 应用的 HTML 表示，可以用于在 OHOS WebView 中加载
/// 返回的指针必须通过 free_dioxus_html 释放
#[cfg(target_env = "ohos")]
#[no_mangle]
pub extern "C" fn get_dioxus_html() -> *const i8 {
    // 渲染为 HTML 字符串（简化版本）
    // TODO: 使用正确的渲染 API，将 VirtualDom 渲染为 HTML
    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <style>
        body {{
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            display: flex;
            justify-content: center;
            align-items: center;
            height: 100vh;
            margin: 0;
        }}
        .container {{
            color: white;
            font-size: 30px;
            text-align: center;
            padding: 50px;
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
        }}
    </style>
</head>
<body>
    <div class="container">
        Hello, world from Dioxus on OHOS!
    </div>
</body>
</html>"#
    );

    // 将 HTML 转换为 C 字符串并返回指针
    // 使用 CString 确保 null 终止
    use std::ffi::CString;
    let c_string = CString::new(html).unwrap();
    c_string.into_raw() as *const i8
}

/// OHOS N-API 绑定 - 释放 HTML 字符串内存
///
/// 释放由 get_dioxus_html 分配的内存
#[cfg(target_env = "ohos")]
#[no_mangle]
pub extern "C" fn free_dioxus_html(ptr: *mut i8, _len: usize) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        // 将指针转换回 CString 并释放
        let _ = std::ffi::CString::from_raw(ptr as *mut std::ffi::c_char);
    }
}

// N-API 模块定义（OHOS 使用 N-API 2.0）
#[cfg(target_env = "ohos")]
mod napi_binding {
    /// N-API 模块注册函数
    #[no_mangle]
    pub extern "C" fn napi_register_module(
        _env: *mut std::ffi::c_void,
        _exports: *mut std::ffi::c_void,
    ) -> *mut std::ffi::c_void {
        // TODO: 实现完整的 N-API 模块注册
        // 这需要使用 napi-ohos crate
        std::ptr::null_mut()
    }
}

// 为了让库能正常编译，在非 OHOS 平台也提供一个 dummy 实现
#[cfg(not(target_env = "ohos"))]
#[no_mangle]
pub extern "C" fn napi_register_module(
    _env: *mut std::ffi::c_void,
    _exports: *mut std::ffi::c_void,
) -> *mut std::ffi::c_void {
    std::ptr::null_mut()
}
