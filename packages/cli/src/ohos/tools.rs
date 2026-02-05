// Copyright 2024-2025 the Dioxus authors. All rights reserved.
// MIT License

//! OpenHarmony tooling detection and configuration

use crate::error::Result;
use anyhow::Context;
use cargo_mobile2::open_harmony::env::Env;
use std::path::PathBuf;

/// OpenHarmony tooling configuration
///
/// Provides paths to the HarmonyOS SDK and NDK tools.
#[derive(Debug, Clone)]
pub struct OhosTooling {
    /// Path to the HarmonyOS SDK (OHOS_HOME)
    pub sdk_path: PathBuf,

    /// Path to the HarmonyOS NDK (OHOS_NDK_HOME)
    pub ndk_path: PathBuf,

    /// Version of the detected HarmonyOS SDK
    pub sdk_version: Option<String>,
}

impl OhosTooling {
    /// Create a new OHOS tooling configuration
    pub fn new(sdk_path: PathBuf, ndk_path: PathBuf) -> Self {
        Self {
            sdk_path,
            ndk_path,
            sdk_version: None,
        }
    }

    /// Get the path to the OHOS C compiler (clang)
    pub fn ohos_cc(&self) -> PathBuf {
        self.llvm_bin_dir().join(format!("clang{}", Self::exe_suffix()))
    }

    /// Get the path to the OHOS C++ compiler (clang++)
    pub fn ohos_cxx(&self) -> PathBuf {
        self.llvm_bin_dir().join(format!("clang++{}", Self::exe_suffix()))
    }

    /// Get the path to the LLVM ar tool
    pub fn ar(&self) -> PathBuf {
        self.llvm_bin_dir().join(format!("llvm-ar{}", Self::exe_suffix()))
    }

    /// Get linker flags for the given target triple
    pub fn linker_flags(&self, _triple: &target_lexicon::Triple) -> Vec<String> {
        vec![
            format!("-L{}", self.ndk_path.join("llvm/lib").display()),
            "-C".to_string(),
            "link-arg=-unwindlib=c++_shared".to_string(),
            "-C".to_string(),
            "link-arg=-l:libc++_shared.so".to_string(),
        ]
    }
    pub fn ranlib(&self) -> PathBuf {
        self.llvm_bin_dir().join(format!("llvm-ranlib{}", Self::exe_suffix()))
    }

    /// Get the path to the LLVM bin directory
    fn llvm_bin_dir(&self) -> PathBuf {
        self.ndk_path
            .join("llvm")
            .join("bin")
    }

    /// Get the executable suffix for the current platform
    fn exe_suffix() -> &'static str {
        #[cfg(windows)]
        {
            ".exe"
        }
        #[cfg(not(windows))]
        {
            ""
        }
    }
}

/// Check for OpenHarmony tools
///
/// This function detects the HarmonyOS SDK and NDK by checking:
/// 1. Environment variables (OHOS_SDK_HOME, OHOS_NDK_HOME)
/// 2. Default DevEco Studio installation locations
///
/// Returns an error if the tools are not found.
pub fn check_tools() -> Result<OhosTooling> {
    use crate::mobile::base_env;

    // Get base environment from cargo-mobile2
    let base_env = base_env().context("Failed to initialize base environment")?;

    // Detect OHOS environment
    let env = Env::from_env(base_env).context(
        "Failed to detect OpenHarmony environment. Please ensure HarmonyOS SDK is installed.",
    )?;

    // Use the OHOS_HOME path from the environment
    let ohos_home = PathBuf::from(env.path());
    let ndk_path = ohos_home.join("llvm");

    Ok(OhosTooling::new(ohos_home, ndk_path))
}
