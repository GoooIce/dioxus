// Copyright 2024-2025 the Dioxus authors. All rights reserved.
// MIT License

//! Mobile platform support utilities
//!
//! This module provides common functionality for mobile platforms (Android, iOS, OpenHarmony)
//! using cargo-mobile2 as the underlying framework.

use anyhow::Result;

/// Base environment type from cargo-mobile2
///
/// On non-Windows platforms, this is `cargo_mobile2::env::Env`.
/// On Windows, it's `cargo_mobile2::os::Env` which has additional Windows-specific functionality.
#[cfg(not(windows))]
pub type BaseEnv = cargo_mobile2::env::Env;

#[cfg(windows)]
pub type BaseEnv = cargo_mobile2::os::Env;

/// Get the base environment for mobile platforms
///
/// This creates a new cargo-mobile2 environment which provides access to:
/// - Home directory detection
/// - Temporary directory management
/// - Platform-specific environment variables
/// - Path normalization
///
/// # Errors
///
/// Returns an error if the environment cannot be initialized (e.g., cannot determine home directory).
///
/// # Example
///
/// ```no_run
/// use dioxus_cli::mobile::base_env;
///
/// let env = base_env()?;
/// let home = env.home_dir();
/// println!("Home directory: {:?}", home);
/// ```
pub fn base_env() -> Result<BaseEnv> {
    BaseEnv::new().map_err(Into::into)
}
