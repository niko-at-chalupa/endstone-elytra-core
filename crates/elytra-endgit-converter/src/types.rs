use elytra_core::types::Plugin;
use nikos_endgit_wrapper::types::Plugin as EndGitPlugin;

#[derive(Debug)]
pub struct EndGitPluginAdapter {
    plugin: EndGitPlugin,
    name: Box<str>,
    repository_url: Box<str>,
}

impl EndGitPluginAdapter {
    pub fn new(plugin: EndGitPlugin) -> Self {
        Self {
            plugin: plugin.clone(),
            name: plugin.name.into_boxed_str(),
            repository_url: plugin.repo_url.into_boxed_str(),
        }
    }

    pub fn endgit_plugin(&self) -> &EndGitPlugin {
        &self.plugin
    }
}

impl Plugin for EndGitPluginAdapter {
    fn name(&self) -> &str {
        &self.name
    }

    fn repository_url(&self) -> &str {
        &self.repository_url
    }
}