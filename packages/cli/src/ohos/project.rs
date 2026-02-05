// Copyright 2024-2025 the Dioxus authors. All rights reserved.
// MIT License

//! OpenHarmony project metadata and generation

use crate::error::Result;
use anyhow::Context;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// OpenHarmony project metadata
///
/// Contains all necessary information to generate an OpenHarmony project.
#[derive(Debug, Clone)]
pub struct OhosProjectMetadata {
    /// Application name (e.g., "My App")
    pub app_name: String,

    /// Bundle name (e.g., "com.example.myapp")
    pub bundle_name: String,

    /// Package name (e.g., "myapp")
    pub package_name: String,

    /// Library name (without lib prefix and .so suffix)
    pub lib_name: String,

    /// Publisher name (e.g., "Dioxus")
    pub publisher: String,

    /// Version string (e.g., "1.0.0")
    pub version: String,

    /// Project root directory
    pub project_root: PathBuf,

    /// Output directory for generated OHOS project
    pub output_dir: PathBuf,
}

/// Check if an OpenHarmony project already exists
///
/// Returns true if the `gen-ohos` directory exists and contains
/// the necessary OpenHarmony project files.
pub fn ohos_project_exists(workspace_root: &PathBuf) -> bool {
    let ohos_dir = workspace_root.join("gen-ohos");

    // Check if the directory exists
    if !ohos_dir.exists() {
        return false;
    }

    // Check for key OHOS project files
    let entry_hvigorfile = ohos_dir.join("entry").join("hvigorfile.ts");
    let app_scope = ohos_dir.join("AppScope").join("app.json5");

    entry_hvigorfile.exists() && app_scope.exists()
}

/// Generate an OpenHarmony project
///
/// Creates the OpenHarmony project structure by copying and
/// processing template files from the CLI's assets directory.
pub fn generate_ohos_project(metadata: &OhosProjectMetadata) -> Result<()> {
    tracing::info!("🔨 Generating OpenHarmony project structure...");

    // Get the path to the OHOS templates
    let template_dir = std::env::current_exe()?
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Failed to get executable directory"))?
        .join("../assets/openharmony");

    // If template directory doesn't exist (e.g., in development),
    // use the source directory
    let template_dir = if !template_dir.exists() {
        let manifest_dir =
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets/openharmony");
        if manifest_dir.exists() {
            manifest_dir
        } else {
            return Err(anyhow::anyhow!("OHOS templates not found. Please ensure Dioxus CLI is properly installed.").into());
        }
    } else {
        template_dir
    };

    // Create output directory
    fs::create_dir_all(&metadata.output_dir)
        .context("Failed to create output directory")?;

    // Create template data for Handlebars
    let mut app_data = build_app_data(metadata);
    app_data.insert("lib_name".to_string(), metadata.lib_name.clone());

    // Copy and process template files
    copy_template_dir(&template_dir, &metadata.output_dir, &app_data)?;

    tracing::info!("✅ OpenHarmony project generated successfully");

    Ok(())
}

/// Build app data for template rendering
fn build_app_data(metadata: &OhosProjectMetadata) -> HashMap<String, String> {
    let mut app_data = HashMap::new();
    app_data.insert("name".to_string(), metadata.app_name.clone());
    app_data.insert("identifier".to_string(), metadata.bundle_name.clone());
    app_data.insert("publisher".to_string(), metadata.publisher.clone());
    app_data.insert("version".to_string(), metadata.version.clone());
    app_data
}

/// Copy template directory recursively
fn copy_template_dir(src: &PathBuf, dst: &PathBuf, data: &HashMap<String, String>) -> Result<()> {
    fs::create_dir_all(dst).context("Failed to create destination directory")?;

    for entry in fs::read_dir(src).context("Failed to read source directory")? {
        let entry = entry?;
        let src_path = entry.path();
        let file_name = src_path
            .file_name()
            .ok_or_else(|| anyhow::anyhow!("Invalid filename"))?
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("Invalid UTF-8 in filename"))?
            .to_string();

        let dst_path = dst.join(&file_name);

        if entry.file_type()?.is_dir() {
            copy_template_dir(&src_path, &dst_path, data)?;
        } else {
            // Skip certain files
            if file_name == "CLAUDE.md" {
                continue;
            }

            // Copy file as-is for now (template processing can be added later)
            fs::copy(&src_path, &dst_path).context("Failed to copy file")?;
        }
    }

    Ok(())
}
