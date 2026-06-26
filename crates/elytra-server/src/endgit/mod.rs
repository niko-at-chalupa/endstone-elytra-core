use elytra_core::types::Plugin;
use elytra_endgit_converter as converter;
use nikos_endgit_wrapper::client::ApiError;
use std::collections::HashMap;

pub struct Endgit {
    client: converter::client::EndGitConverterClient,
    plugins: HashMap<String, converter::types::EndGitPluginAdapter>,
}

impl Endgit {
    pub async fn fill(&mut self) -> Result<(), nikos_endgit_wrapper::client::ApiError> {
        let all_plugins = self.client.all_plugins().await?;
        let mut plugin_map: HashMap<String, converter::types::EndGitPluginAdapter> = HashMap::new();

        for plugin in all_plugins {
            plugin_map.insert(plugin.kebabbed_name().to_owned(), plugin);
        }

        self.plugins = plugin_map;

        Ok(())
    }

    pub fn new() -> Result<Self, ApiError> {
        Ok(Self {
            client: converter::client::EndGitConverterClient::new()?,
            plugins: HashMap::new()
        })
    }

    pub fn plugins(&self) -> &HashMap<String, converter::types::EndGitPluginAdapter> {
        &self.plugins
    }
}

#[cfg(test)]
mod tests {
    use nikos_endgit_wrapper::client::ApiError;
    use super::*;

    #[tokio::test]
    #[tracing_test::traced_test]
    async fn test_fill() -> Result<(), ApiError> {
        let mut endgit = Endgit::new()?;
        endgit.fill().await?;

        tracing::info!("{:?}", &endgit.plugins);
        assert!(endgit.plugins.len() > 5);

        Ok(())
    }
}