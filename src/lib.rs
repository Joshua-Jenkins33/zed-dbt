use zed_extension_api::{self as zed, settings::LspSettings, LanguageServerId, Result};

struct DbtExtension;

impl zed::Extension for DbtExtension {
    fn new() -> Self {
        DbtExtension
    }

    /// Returns the command Zed uses to start sqlfmt-lsp.
    ///
    /// sqlfmt does not speak the Language Server Protocol natively.
    /// `sqlfmt-lsp` (in ./sqlfmt-lsp) is a thin wrapper that bridges LSP
    /// formatting requests to sqlfmt's stdin/stdout interface.
    ///
    /// # Toggle sqlfmt on save
    ///
    /// In your Zed settings, set:
    ///   { "languages": { "dbt": { "format_on_save": "on" } } }
    ///
    /// To disable:
    ///   { "languages": { "dbt": { "format_on_save": "off" } } }
    ///
    /// # Override the binary path
    ///
    /// If sqlfmt-lsp is not on your PATH, set the path explicitly:
    ///   {
    ///     "lsp": {
    ///       "sqlfmt": {
    ///         "binary": { "path": "/path/to/sqlfmt-lsp" }
    ///       }
    ///     }
    ///   }
    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree).ok();

        let binary = settings
            .as_ref()
            .and_then(|s| s.binary.as_ref())
            .and_then(|b| b.path.as_deref())
            .unwrap_or("sqlfmt-lsp");

        Ok(zed::Command {
            command: binary.to_string(),
            args: vec![],
            env: Default::default(),
        })
    }
}

zed::register_extension!(DbtExtension);
