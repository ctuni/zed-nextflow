use std::env;
use std::fs;
use std::path::PathBuf;
use zed_extension_api::{
    self as zed,
    lsp::{Completion, CompletionKind},
    register_extension, CodeLabel, CodeLabelSpan, DownloadedFileType, Extension,
    GithubReleaseOptions, LanguageServerId, LanguageServerInstallationStatus, Worktree,
};

struct NextflowExtension {
    cached_jar_path: Option<String>,
}

impl NextflowExtension {
    fn jar_absolute_path() -> PathBuf {
        // ✅ Use Zed's extension dir, always writable
        let base = env::var("ZED_EXTENSION_DIR").unwrap_or_else(|_| ".".to_string());
        let mut path = PathBuf::from(base);
        path.push("language-server-all.jar");
        path
    }

    fn ensure_language_server(
        &mut self,
        language_server_id: &LanguageServerId,
    ) -> zed::Result<String> {
        let jar_path = Self::jar_absolute_path();
        println!("Using absolute JAR path: {:?}", jar_path);

        if jar_path.is_file() {
            println!("Found existing JAR at {:?}", jar_path);
            self.cached_jar_path = Some(jar_path.to_string_lossy().to_string());
            return Ok(jar_path.to_string_lossy().to_string());
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &LanguageServerInstallationStatus::CheckingForUpdate,
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

        println!("Downloading JAR from {}", download_url);

        zed::set_language_server_installation_status(
            language_server_id,
            &LanguageServerInstallationStatus::Downloading,
        );

        zed::download_file(
            &download_url,
            &jar_path.to_string_lossy(),
            DownloadedFileType::Uncompressed,
        )
        .map_err(|e| format!("failed to download jar: {e}"))?;

        println!("Downloaded JAR to {:?}", jar_path);

        self.cached_jar_path = Some(jar_path.to_string_lossy().to_string());
        Ok(jar_path.to_string_lossy().to_string())
    }
}

impl Extension for NextflowExtension {
    fn new() -> Self {
        Self {
            cached_jar_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        _worktree: &Worktree,
    ) -> zed::Result<zed::Command> {
        let jar_path = self.ensure_language_server(language_server_id)?;
        let abs_path = std::fs::canonicalize(&jar_path)
            .unwrap_or_else(|_| PathBuf::from(&jar_path))
            .to_string_lossy()
            .to_string();

        println!(
            "Launching language server with absolute jar at {:?}",
            abs_path
        );

        Ok(zed::Command {
            command: "/usr/bin/java".into(),
            args: vec!["-jar".into(), abs_path],
            env: Vec::new(),
        })
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
