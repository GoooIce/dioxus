<div>
  <h1>📦✨ Dioxus CLI</h1>
  <p><strong>Tooling to supercharge Dioxus projects</strong></p>
</div>

The **dioxus-cli** (inspired by wasm-pack and webpack) is a tool for getting Dioxus projects up and running.
It handles building, bundling, development and publishing to simplify development.

## Installation

### Install the stable version (recommended)

```shell
cargo install dioxus-cli
```

### Install the latest development build through git

To get the latest bug fixes and features, you can install the development version from git.
However, this is not fully tested.
That means you're probably going to have more bugs despite having the latest bug fixes.

```shell
cargo install --git https://github.com/DioxusLabs/dioxus dioxus-cli
```

This will download the CLI from the master branch,
and install it in Cargo's global binary directory (`~/.cargo/bin/` by default).

### Install from local folder
Note: The CLI will fail to build projects in debug profile. This is currently under investigation.
```shell
cargo install --path .
```

## Get started

Use `dx new` to initialize a new Dioxus project.
It will be cloned from the [dioxus-template](https://github.com/DioxusLabs/dioxus-template) repository.

Alternatively, you can specify the template path:

```shell
dx new --template gh:dioxuslabs/dioxus-template
```

Run `dx --help` for a list of all the available commands.
Furthermore, you can run `dx <command> --help` to get help with a specific command.

## Dioxus config file

You can use the `Dioxus.toml` file for further configuration.
Some fields are mandatory, but the CLI tool will tell you which ones are missing.
You can create a `Dioxus.toml` with all fields already set using `dx config init project-name`,
or you can use this bare-bones template (only mandatory fields) to get started:

```toml
[application]
name = "project-name"
# Supported platforms: web, desktop, ios, android, ohos, server, liveview
default_platform = "web"

# Optional: enable copying from a static directory (e.g. "public")
public_dir = "public"

[web.app]
title = "Hello"

[web.resource.dev]
```

## Platform Support

The Dioxus CLI supports building for multiple platforms:

| Platform | Description | Status |
|----------|-------------|--------|
| `web` | WebAssembly applications | Stable |
| `desktop` | macOS, Linux, Windows | Stable |
| `ios` | iOS applications | Stable |
| `android` | Android applications | Stable |
| `ohos` | OpenHarmony (HarmonyOS) | Beta |
| `server` | Server-side rendering | Stable |
| `liveview` | Liveview server | Stable |

### OpenHarmony (OHOS) Support

OHOS support is currently in **Beta**. To build for OHOS:

1. Set up the environment variables:
```bash
export OHOS_SDK_HOME=/path/to/ohos/sdk
export OHOS_NDK_HOME=/path/to/ohos/ndk
```

2. Add the OHOS target:
```bash
rustup target add aarch64-unknown-linux-ohos
```

3. Build and run:
```bash
dx serve --platform ohos
```

For detailed OHOS setup instructions, see the [OHOS CLI Reference](./OHOS_CLI_REFERENCE.md).
