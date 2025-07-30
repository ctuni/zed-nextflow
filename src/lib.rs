use std::fs;
use zed_extension_api::{
    self as zed, download_file, latest_github_release,
    lsp::{Completion, CompletionKind},
    register_extension, set_language_server_installation_status, CodeLabel, CodeLabelSpan,
    DownloadedFileType, Extension, GithubReleaseOptions, LanguageServerId,
    LanguageServerInstallationStatus, Worktree,
};

struct NextflowExtension {
    cached_jar_path: Option<String>,
}

impl NextflowExtension {
    fn language_server_jar_path(
        &mut self,
        language_server_id: &LanguageServerId,
    ) -> zed::Result<String> {
        let jar_path = "language-server-all.jar".to_string();

        if let Some(path) = &self.cached_jar_path {
            if fs::metadata(path).map_or(false, |stat| stat.is_file()) {
                return Ok(path.clone());
            }
        }
        if fs::metadata(&jar_path).map_or(false, |stat| stat.is_file()) {
            self.cached_jar_path = Some(jar_path.clone());
            return Ok(jar_path);
        }

        set_language_server_installation_status(
            &language_server_id,
            &LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = latest_github_release(
            "nextflow-io/language-server",
            GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == "language-server-all.jar")
            .ok_or_else(|| "No language-server-all.jar asset found".to_string())?;

        set_language_server_installation_status(
            &language_server_id,
            &LanguageServerInstallationStatus::Downloading,
        );

        // ✅ Download into a temp dir instead of a file
        let tmp_dir = "jar-download";
        fs::create_dir_all(tmp_dir).ok();

        download_file(&asset.download_url, tmp_dir, DownloadedFileType::Zip)
            .map_err(|e| format!("failed to download jar: {e}"))?;

        // ✅ Find the downloaded file inside tmp_dir
        let downloaded = fs::read_dir(tmp_dir)
            .map_err(|e| format!("failed to list downloaded jar: {e}"))?
            .filter_map(|e| e.ok())
            .find(|e| e.file_type().map(|t| t.is_file()).unwrap_or(false))
            .ok_or_else(|| "downloaded jar not found".to_string())?
            .path();

        fs::rename(&downloaded, &jar_path).map_err(|e| format!("failed to move jar: {e}"))?;

        let _ = fs::remove_dir_all(tmp_dir);

        self.cached_jar_path = Some(jar_path.clone());
        Ok(jar_path)
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
        let jar_path = self.language_server_jar_path(language_server_id)?;
        Ok(zed::Command {
            command: "./bin/java".into(), // use bundled JRE inside the extension
            args: vec!["-jar".into(), jar_path],
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
