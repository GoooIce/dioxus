#include "napi/native_api.h"

// Placeholder NAPI init for Dioxus OpenHarmony
// The actual Rust library is loaded by @ohos-rs/ability

EXTERN_C_START
static napi_value Init(napi_env env, napi_value exports)
{
    // Dioxus uses the webview/xcomponent bridge
    // No additional NAPI exports needed here
    return exports;
}
EXTERN_C_END

static napi_module demoModule = {
    .nm_version = 1,
    .nm_flags = 0,
    .nm_filename = nullptr,
    .nm_register_func = Init,
    .nm_modname = "entry",
    .nm_priv = ((void*)0),
    .reserved = { 0 },
};

extern "C" __attribute__((constructor)) void RegisterEntryModule(void)
{
    napi_module_register(&demoModule);
}
