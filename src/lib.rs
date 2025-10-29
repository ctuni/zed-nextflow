use zed_extension_api::{
    self as zed,
    lsp::{Completion, CompletionKind},
    register_extension, CodeLabel, CodeLabelSpan, DownloadedFileType, Extension,
    GithubReleaseOptions, LanguageServerId, LanguageServerInstallationStatus, Worktree,
};

use std::env;
use std::fs;
use std::path::PathBuf;

const LS_JAR: &str = "language-server-all.jar";
const LS_DIR: &str = "server";

struct NextflowExtension {}

impl NextflowExtension {
    /// Constructs the full path for where the language server JAR should be stored.
    /// It uses the `ZED_EXTENSION_DIR` environment variable provided by Zed.
    fn get_jar_path() -> zed::Result<PathBuf> {
        let base_dir = env::var("ZED_EXTENSION_DIR")
            .map_err(|_| "ZED_EXTENSION_DIR environment variable not set".to_string())?;
        let dir = PathBuf::from(base_dir).join(LS_DIR);
        if !dir.exists() {
            fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        }
        Ok(dir.join(LS_JAR))
    }

    /// This function ensures the language server JAR is available, downloading it if necessary.
    /// The download is synchronous (blocking), which is required by this version of the API.
    fn ensure_server_jar_is_downloaded(
        &self,
        language_server_id: &LanguageServerId,
    ) -> zed::Result<PathBuf> {
        let jar_path = Self::get_jar_path()?;
        if jar_path.exists() {
            return Ok(jar_path);
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &LanguageServerInstallationStatus::Downloading,
        );

        // Get the latest release from GitHub to find the download URL.
        let release = zed::latest_github_release(
            "nextflow-io/language-server",
            GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let download_url = format!(
            "https://github.com/nextflow-io/language-server/releases/download/{}/language-server-all.jar",
            release.version
        );

        // The download path needs to be a string.
        let jar_path_str = jar_path
            .to_str()
            .ok_or_else(|| "Failed to convert JAR path to string".to_string())?;

        // Perform the synchronous download.
        zed::download_file(
            &download_url,
            jar_path_str,
            DownloadedFileType::Uncompressed,
        )
        .map_err(|e| format!("Failed to download language server JAR: {e}"))?;

        Ok(jar_path)
    }
}

impl Extension for NextflowExtension {
    fn new() -> Self {
        Self {}
    }

    /// This is the main entry point called by Zed to start the language server.
    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        _worktree: &Worktree,
    ) -> zed::Result<zed::Command> {
        zed::set_language_server_installation_status(
            language_server_id,
            &LanguageServerInstallationStatus::CheckingForUpdate,
        );

        // 1. Ensure the language server JAR is downloaded.
        let jar_path = self.ensure_server_jar_is_downloaded(language_server_id)?;

        // 2. Construct the command to run the language server.
        //    We rely on the user having `java` in their system's PATH.
        Ok(zed::Command {
            command: "java".to_string(),
            args: vec!["-jar".into(), jar_path.to_string_lossy().into()],
            env: Vec::new(),
        })
    }

    /// Provides custom labels for LSP completion items.
    fn label_for_completion(
        &self,
        _language_server_id: &LanguageServerId,
        completion: Completion,
    ) -> Option<CodeLabel> {
        match completion.kind? {
            CompletionKind::Class | CompletionKind::Enum | CompletionKind::Interface => {
                Some(CodeLabel {
                    code: format!("{} variable", completion.label),
                    spans: vec![
                        CodeLabelSpan::code_range(0..completion.label.len()),
                        CodeLabelSpan::literal(format!(" (import {})", completion.detail?), None),
                    ],
                    filter_range: (0..completion.label.len()).into(),
                })
            }
            CompletionKind::Method => {
                let code = format!("{}()", completion.label);
                Some(CodeLabel {
                    spans: vec![CodeLabelSpan::code_range(0..code.len())],
                    code,
                    filter_range: (0..completion.label.len()).into(),
                })
            }
            CompletionKind::Variable => {
                let def = "def ";
                let code = format!("{def}{}", completion.label);
                Some(CodeLabel {
                    spans: vec![CodeLabelSpan::code_range(def.len()..code.len())],
                    code,
                    filter_range: (0..completion.label.len()).into(),
                })
            }
            _ => None,
        }
    }
}

// Register the extension with Zed. This is crucial.
register_extension!(NextflowExtension);
