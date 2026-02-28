use super::*;
use cargo_generate::{GenerateArgs, TemplatePath, Vcs};
use std::env;

#[derive(Clone, Debug, Default, Deserialize, Parser)]
#[clap(name = "init")]
pub struct Init {
    /// Create a new Dioxus project at PATH
    #[arg(default_value = ".")]
    pub path: PathBuf,

    /// Project name. Defaults to directory name
    #[arg(short, long)]
    pub name: Option<String>,

    /// Template path
    #[clap(short, long)]
    pub template: Option<String>,

    /// Initialize an OpenHarmony (OHOS) project
    #[arg(long, conflicts_with_all(["template", "branch", "revision", "tag", "subtemplate"]))]
    pub ohos: bool,

    /// Branch to select when using `template` from a git repository.
    /// Mutually exclusive with: `--revision`, `--tag`.
    #[clap(long, conflicts_with_all(["revision", "tag"]))]
    pub branch: Option<String>,

    /// A commit hash to select when using `template` from a git repository.
    /// Mutually exclusive with: `--branch`, `--tag`.
    #[clap(long, conflicts_with_all(["branch", "tag"]))]
    pub revision: Option<String>,

    /// Tag to select when using `template` from a git repository.
    /// Mutually exclusive with: `--branch`, `--revision`.
    #[clap(long, conflicts_with_all(["branch", "revision"]))]
    pub tag: Option<String>,

    /// Specify a sub-template within the template repository to be used as the actual template
    #[clap(long)]
    pub subtemplate: Option<String>,

    /// Pass `<option>=<value>` for the used template (e.g., `foo=bar`)
    #[clap(short, long)]
    pub option: Vec<String>,

    /// Skip user interaction by using the default values for the used template.
    /// Default values can be overridden with `--option`
    #[clap(short, long)]
    pub yes: bool,

    /// Specify the VCS used to initialize the generated template.
    /// Options: `git`, `none`.
    #[arg(long, value_parser)]
    pub vcs: Option<Vcs>,
}

impl Init {
    pub async fn init(mut self) -> Result<StructuredOutput> {
        // Project name defaults to directory name.
        if self.name.is_none() {
            self.name = Some(create::name_from_path(&self.path)?);
        }

        // If OHOS is requested, use the local OHOS template
        if self.ohos {
            return self.init_ohos().await;
        }

        // Perform a connectivity check so we just don't it around doing nothing if there's a network error
        if self.template.is_none() {
            create::check_connectivity().await?;
        }

        // If no template is specified, use the default one and set the branch to the latest release.
        create::resolve_template_and_branch(&mut self.template, &mut self.branch);

        // cargo-generate requires the path to be created first.
        std::fs::create_dir_all(&self.path)?;

        let args = GenerateArgs {
            define: self.option,
            destination: Some(self.path),
            init: true,
            name: self.name,
            silent: self.yes,
            vcs: self.vcs,
            template_path: TemplatePath {
                auto_path: self.template,
                branch: self.branch,
                revision: self.revision,
                subfolder: self.subtemplate,
                tag: self.tag,
                ..Default::default()
            },
            ..Default::default()
        };

        let path = cargo_generate::generate(args)?;
        _ = create::post_create(&path, &self.vcs.unwrap_or(Vcs::Git));
        Ok(StructuredOutput::Success)
    }

    /// Initialize an OpenHarmony (OHOS) project using the local template
    async fn init_ohos(self) -> Result<StructuredOutput> {
        use crate::TraceSrc;

        let project_name = self.name.as_ref().unwrap();
        let crate_name = project_name.to_lowercase().replace('-', "_");
        let bundle_name = format!("com.example.{}", project_name.to_lowercase().replace('-', ""));

        // Build template variables
        let mut define = vec![
            format!("project_name={}", project_name),
            format!("crate_name={}", crate_name),
            format!("bundle_name={}", bundle_name),
            format!("vendor=Example"),
            format!("version=0.1.0"),
            format!("dioxus_git=https://github.com/DioxusLabs/dioxus"),
        ];

        // Add user-provided options which can override defaults
        define.extend(self.option);

        // Get the OHOS template path from CLI assets
        let manifest_dir = env::var("CARGO_MANIFEST_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("."));
        let template_path = manifest_dir.join("assets").join("ohos");

        if !template_path.exists() {
            tracing::error!(dx_src = ?TraceSrc::Dev, "OHOS template not found at: {}", template_path.display());
            anyhow::bail!("OHOS template not found. Please ensure dioxus-cli is properly installed.");
        }

        tracing::info!(dx_src = ?TraceSrc::Dev, "Using OHOS template from: {}", template_path.display());

        // Create destination directory
        std::fs::create_dir_all(&self.path)?;

        let args = GenerateArgs {
            define,
            destination: Some(self.path.clone()),
            init: true,
            name: self.name.clone(),
            silent: self.yes,
            vcs: self.vcs.clone(),
            template_path: TemplatePath {
                auto_path: Some(template_path.to_string_lossy().to_string()),
                ..Default::default()
            },
            ..Default::default()
        };

        let generated_path = cargo_generate::generate(args)?;

        // Run post-create steps (formatting, git init, etc.)
        // Note: OHOS projects don't have Cargo.toml in the root, so we skip some steps
        if let Some(vcs) = &self.vcs {
            vcs.initialize(&self.path, Some("main"), true)?;
        }

        tracing::info!(dx_src = ?TraceSrc::Dev, "Generated OHOS project at {}\n\nAn OHOS project has been created with:\n  - Rust library in rust/\n  - OHOS application configuration\n  - HarmonyOS native integration\n\nTo build your OHOS app:\n  1. Install the HarmonyOS SDK\n  2. Configure DevEco Studio\n  3. Run `dx build --platform ohos`\n\nMore information is available in the generated README.md.\n\nBuild cool things! ✌️", self.path.display());

        Ok(StructuredOutput::Success)
    }
}

// todo: re-enable these tests with better parallelization
//
// #[cfg(test)]
// mod tests {
//     use std::{fs::create_dir_all, process::Command};
//     use tempfile::tempdir;

//     use super::create::tests::*;

//     // Note: tests below (at least 6 of them) were written to mainly test
//     // correctness of project's directory and its name, because previously it
//     // was broken and tests bring a peace of mind. And also so that I don't have
//     // to run my local hand-made tests every time.

//     fn subcommand_init() -> Command {
//         subcommand("init")
//     }

//     #[test]
//     fn test_subcommand_init_with_default_path() -> Result<()> {
//         let project_dir = "dir";
//         let project_name = project_dir;

//         let temp_dir = tempdir()?;
//         // Make current dir's name deterministic.
//         let current_dir = temp_dir.path().join(project_dir);
//         create_dir_all(&current_dir)?;
//         let project_path = &current_dir;
//         assert!(project_path.exists());

//         assert!(subcommand_init().current_dir(&current_dir).status().is_ok());

//         let cargo_toml_path = get_cargo_toml_path(project_path);
//         assert!(cargo_toml_path.exists());
//         assert_eq!(get_project_name(&cargo_toml_path)?, project_name);
//         Ok(())
//     }

//     #[test]
//     fn test_subcommand_init_with_1_dir_path() -> Result<()> {
//         let project_dir = "dir";
//         let project_name = project_dir;

//         let current_dir = tempdir()?;

//         assert!(subcommand_init()
//             .arg(project_dir)
//             .current_dir(&current_dir)
//             .status()
//             .is_ok());

//         let project_path = current_dir.path().join(project_dir);
//         let cargo_toml_path = get_cargo_toml_path(&project_path);
//         assert!(project_path.exists());
//         assert!(cargo_toml_path.exists());
//         assert_eq!(get_project_name(&cargo_toml_path)?, project_name);
//         Ok(())
//     }

//     #[test]
//     fn test_subcommand_init_with_2_dir_path() -> Result<()> {
//         let project_dir = "a/b";
//         let project_name = "b";

//         let current_dir = tempdir()?;

//         assert!(subcommand_init()
//             .arg(project_dir)
//             .current_dir(&current_dir)
//             .status()
//             .is_ok());

//         let project_path = current_dir.path().join(project_dir);
//         let cargo_toml_path = get_cargo_toml_path(&project_path);
//         assert!(project_path.exists());
//         assert!(cargo_toml_path.exists());
//         assert_eq!(get_project_name(&cargo_toml_path)?, project_name);
//         Ok(())
//     }

//     #[test]
//     fn test_subcommand_init_with_default_path_and_custom_name() -> Result<()> {
//         let project_dir = "dir";
//         let project_name = "project";

//         let temp_dir = tempdir()?;
//         // Make current dir's name deterministic.
//         let current_dir = temp_dir.path().join(project_dir);
//         create_dir_all(&current_dir)?;
//         let project_path = &current_dir;
//         assert!(project_path.exists());

//         assert!(subcommand_init()
//             .arg("--name")
//             .arg(project_name)
//             .current_dir(&current_dir)
//             .status()
//             .is_ok());

//         let cargo_toml_path = get_cargo_toml_path(project_path);
//         assert!(cargo_toml_path.exists());
//         assert_eq!(get_project_name(&cargo_toml_path)?, project_name);
//         Ok(())
//     }

//     #[test]
//     fn test_subcommand_init_with_1_dir_path_and_custom_name() -> Result<()> {
//         let project_dir = "dir";
//         let project_name = "project";

//         let current_dir = tempdir()?;

//         assert!(subcommand_init()
//             .arg(project_dir)
//             .arg("--name")
//             .arg(project_name)
//             .current_dir(&current_dir)
//             .status()
//             .is_ok());

//         let project_path = current_dir.path().join(project_dir);
//         let cargo_toml_path = get_cargo_toml_path(&project_path);
//         assert!(project_path.exists());
//         assert!(cargo_toml_path.exists());
//         assert_eq!(get_project_name(&cargo_toml_path)?, project_name);
//         Ok(())
//     }

//     #[test]
//     fn test_subcommand_init_with_2_dir_path_and_custom_name() -> Result<()> {
//         let project_dir = "a/b";
//         let project_name = "project";

//         let current_dir = tempdir()?;

//         assert!(subcommand_init()
//             .arg(project_dir)
//             .arg("--name")
//             .arg(project_name)
//             .current_dir(&current_dir)
//             .status()
//             .is_ok());

//         let project_path = current_dir.path().join(project_dir);
//         let cargo_toml_path = get_cargo_toml_path(&project_path);
//         assert!(project_path.exists());
//         assert!(cargo_toml_path.exists());
//         assert_eq!(get_project_name(&cargo_toml_path)?, project_name);
//         Ok(())
//     }
// }
