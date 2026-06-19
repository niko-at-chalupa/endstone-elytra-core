use elytra_core::types::Plugin;
use elytra_core::id::kebabify;
use nikos_endgit_wrapper::types::Plugin as EndGitPlugin;

#[derive(Debug)]
pub struct EndGitPluginAdapter {
    plugin: EndGitPlugin,
    name: Box<str>,
    repository_url: Box<str>,
    kebabbed_name: Box<str>,
}

impl EndGitPluginAdapter {
    pub fn new(plugin: EndGitPlugin) -> Self {
        let kebabbed_name = kebabify(&plugin.name);
        Self {
            plugin: plugin.clone(),
            name: plugin.name.into_boxed_str(),
            repository_url: plugin.repo_url.into_boxed_str(),
            kebabbed_name: kebabbed_name,
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

    fn kebabbed_name(&self) -> &str {
        &self.kebabbed_name
    }
}