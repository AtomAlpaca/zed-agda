use std::fs;
use zed::LanguageServerId;
use zed_extension_api::{self as zed, Result};

struct AgdaExtension;
impl AgdaExtension {
    fn language_server_binary_path(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result <String> {
        if let Some(path) = worktree.which("agda-language-server") {
            return Ok(path);
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = zed::latest_github_release(
            "agda/agda-language-server",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let (platform, _) = zed::current_platform();
        let asset_name = format!(
            "als-{os}.zip",
            os = match platform {
                zed::Os::Mac => "macos",
                zed::Os::Linux => "ubuntu", // why
                zed::Os::Windows => "windows",
            },
        );

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("no asset found matching {:?}", asset_name))?;

        let version_dir = format!("agda-language-server-{}", release.version);
        let binary_path = format!(
            "{version_dir}/als{extension}",
            extension = match platform {
                zed::Os::Mac | zed::Os::Linux => "",
                zed::Os::Windows => ".exe",
            },
        );
        if !fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(
                &asset.download_url,
                &version_dir,
                zed::DownloadedFileType::Zip,
            )
            .map_err(|e| format!("failed to download file: {e}"))?;
        }
        Ok(binary_path)
    }
}

impl zed::Extension for AgdaExtension {
    fn new() -> Self {
        Self { }
    }
    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        Ok(zed::Command {
            command: self.language_server_binary_path(language_server_id, worktree)?,
            args: Default::default(),
            env: Default::default(),
        })
    }
}

zed::register_extension!(AgdaExtension);
