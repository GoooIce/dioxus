# Dioxus Desktop (webview)

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![Build Status][actions-badge]][actions-url]
[![Discord chat][discord-badge]][discord-url]

[crates-badge]: https://img.shields.io/crates/v/dioxus-desktop.svg
[crates-url]: https://crates.io/crates/dioxus-desktop
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/dioxuslabs/dioxus/blob/main/LICENSE-MIT
[actions-badge]: https://github.com/dioxuslabs/dioxus/actions/workflows/main.yml/badge.svg
[actions-url]: https://github.com/dioxuslabs/dioxus/actions?query=workflow%3ACI+branch%3Amaster
[discord-badge]: https://img.shields.io/discord/899851952891002890.svg?logo=discord&style=flat-square
[discord-url]: https://discord.gg/XgGxMSkvUM

[Website](https://dioxuslabs.com) |
[Guides](https://dioxuslabs.com/learn/0.7/) |
[API Docs](https://docs.rs/dioxus-desktop/latest/dioxus_desktop) |
[Chat](https://discord.gg/XgGxMSkvUM)

## Overview

`dioxus-desktop` provides a webview-based desktop renderer for the Dioxus VirtualDom.

This requires that webview is installed on the target system. WebView is installed by default on macOS and iOS devices, but might not come preinstalled on Windows or Linux devices. To fix these issues, follow the [instructions in the guide](guide-url).

### Platform Support

`dioxus-desktop` supports the following platforms:

- **Desktop**: macOS, Linux, Windows
- **Mobile**: iOS, Android
- **OpenHarmony (OHOS)**: Experimental support for HarmonyOS devices

#### OpenHarmony (OHOS) Support

Dioxus now includes experimental support for OpenHarmony (HarmonyOS), allowing you to build and run Dioxus applications on HarmonyOS devices.

**Supported Architectures:**
- `aarch64-unknown-linux-ohos` (ARM64 devices)
- `armv7a-unknown-linux-ohos` (ARM32 devices)
- `x86_64-unknown-linux-ohos` (x86_64 emulator)

**Requirements:**
- DevEco Studio (latest version)
- OpenHarmony SDK and NDK
- Rust with OHOS target support
- `hdc` (HarmonyOS Device Connector) for device deployment

**Quick Start:**
```bash
# Add the OHOS target
rustup target add aarch64-unknown-linux-ohos

# Set environment variables
export OHOS_SDK_HOME=/path/to/ohos/sdk
export OHOS_NDK_HOME=/path/to/ohos/ndk

# Build your app for OHOS
dx build --platform ohos

# Run on device/emulator
dx serve --platform ohos
```

For detailed OHOS setup instructions, see the [OHOS Documentation](#ohos-documentation).

[guide-url]: https://dioxuslabs.com/learn/0.7/getting_started

## Features

- Simple, one-line launch for desktop apps
- Dioxus VirtualDom running on a native thread
- Full HTML/CSS support via `wry` and `tao`
- Exposed `window` and `Proxy` types from tao for direct window manipulation
- Helpful hooks for accessing the window, WebView, and running javascript.

## Contributing

- Report issues on our [issue tracker](https://github.com/dioxuslabs/dioxus/issues).
- Join the discord and ask questions!

## License

This project is licensed under the [MIT license].

[mit license]: https://github.com/dioxuslabs/dioxus/blob/main/LICENSE-MIT

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Dioxus by you shall be licensed as MIT without any additional
terms or conditions.
