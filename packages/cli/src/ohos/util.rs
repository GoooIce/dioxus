// Copyright 2024-2025 the Dioxus authors. All rights reserved.
// MIT License

//! OpenHarmony utilities

use std::path::PathBuf;

/// Get the OpenHarmony ABI directory name from a target triple
///
/// This function converts a Rust target triple to the corresponding
/// OpenHarmony ABI directory name used in the project structure.
///
/// # Examples
///
/// ```no_run
/// use dioxus_cli::ohos::ohos_abi;
///
/// let triple: target_lexicon::Triple = "aarch64-unknown-linux-ohos".parse().unwrap();
/// let abi = ohos_abi(&triple);
/// assert_eq!(abi, "arm64-v8a");
/// ```
pub fn ohos_abi(triple: &target_lexicon::Triple) -> PathBuf {
    match triple.architecture {
        target_lexicon::Architecture::Aarch64(_) => PathBuf::from("arm64-v8a"),
        target_lexicon::Architecture::Arm(_) => PathBuf::from("armeabi-v7a"),
        target_lexicon::Architecture::X86_64 => PathBuf::from("x86_64"),
        target_lexicon::Architecture::X86_32(_) => PathBuf::from("x86"),
        _ => PathBuf::from(triple.architecture.to_string()),
    }
}
