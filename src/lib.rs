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
const LS_DIR: &str = "nextflow-language-server";

struct NextflowExtension {}

impl NextflowExtension {
    fn ensure_server_jar_is_downloaded(
        &self,
        language_server_id: &LanguageServerId,
    ) -> zed::Result<PathBuf> {
        // We use a simple relative path. Zed will place this inside a private,
        // writable directory specific to this extension.
        let jar_dir = PathBuf::from(LS_DIR);
        let jar_path = jar_dir.join(LS_JAR);

        if jar_path.exists() {
            return Ok(jar_path);
        }

        if !jar_dir.exists() {
            fs::create_dir_all(&jar_dir).map_err(|e| e.to_string())?;
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &LanguageServerInstallationStatus::Downloading,
        );

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

        let jar_path_str = jar_path
            .to_str()
            .ok_or_else(|| "Failed to convert JAR path to string".to_string())?;

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

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        _worktree: &Worktree,
    ) -> zed::Result<zed::Command> {
        zed::set_language_server_installation_status(
            language_server_id,
            &LanguageServerInstallationStatus::CheckingForUpdate,
        );

        // 1. Download the JAR to a relative path.
        let relative_jar_path = self.ensure_server_jar_is_downloaded(language_server_id)?;

        // 2. Get the absolute path to the current working directory.
        //    When Zed runs the extension, this is the private, writable directory.
        let work_dir = env::current_dir().map_err(|e| e.to_string())?;
        let absolute_jar_path = work_dir.join(relative_jar_path);

        // 3. Construct the shell command using the absolute path.
        let jar_path_str = absolute_jar_path.to_string_lossy();
        let command_string = format!("java -jar '{}'", jar_path_str);

        // 4. Execute via a shell to ensure `java` is found in the PATH.
        Ok(zed::Command {
            command: "/bin/sh".to_string(),
            args: vec!["-c".to_string(), command_string],
            env: Default::default(),
        })
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &LanguageServerId,
        _worktree: &Worktree,
    ) -> zed::Result<Option<zed::serde_json::Value>> {
        Ok(Some(zed::serde_json::json!({
            "nextflow": {
                "completion": {
                    "extended": false,
                    "maxItems": 100,
                },
                "debug": false,
                "errorReportingMode": "warnings",
                "files": {
                    "exclude": [".git", ".nf-test", "work"],
                },
                "formatting": {
                    "harshilAlignment": false,
                    "maheshForm": false,
                    "sortDeclarations": false,
                },
                "java": {
                    "home": "",
                },
                "languageVersion": "26.04",
                "telemetry": {
                    "enabled": false,
                },
            },
        })))
    }

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

register_extension!(NextflowExtension);
