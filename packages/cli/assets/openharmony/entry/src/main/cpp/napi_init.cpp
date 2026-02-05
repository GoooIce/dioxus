// Dioxus OpenHarmony NAPI Entry Point
// This file provides the native bridge for the Dioxus application.
// The actual Dioxus runtime is loaded via @ohos-rs/ability's RustAbility.

#include "napi/native_api.h"

// Placeholder NAPI function - the actual Dioxus logic runs through RustAbility
static napi_value GetVersion(napi_env env, napi_callback_info info)
{
    napi_value result;
    napi_create_string_utf8(env, "Dioxus 0.7", NAPI_AUTO_LENGTH, &result);
    return result;
}

EXTERN_C_START
static napi_value Init(napi_env env, napi_value exports)
{
    napi_property_descriptor desc[] = {
        { "getVersion", nullptr, GetVersion, nullptr, nullptr, nullptr, napi_default, nullptr }
    };
    napi_define_properties(env, exports, sizeof(desc) / sizeof(desc[0]), desc);
    return exports;
}
EXTERN_C_END

static napi_module dioxusModule = {
    .nm_version = 1,
    .nm_flags = 0,
    .nm_filename = nullptr,
    .nm_register_func = Init,
    .nm_modname = "entry",
    .nm_priv = ((void*)0),
    .reserved = { 0 },
};

extern "C" __attribute__((constructor)) void RegisterDioxusModule(void)
{
    napi_module_register(&dioxusModule);
}
