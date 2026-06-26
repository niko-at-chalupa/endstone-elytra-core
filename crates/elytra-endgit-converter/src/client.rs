use nikos_endgit_wrapper::client::{Client, ApiError};
use crate::types::EndGitPluginAdapter;

pub struct EndGitConverterClient {
    client: Client,
}

impl EndGitConverterClient {
    pub fn new() -> Result<Self, ApiError> {
        let client = Client::new(None)?; 
        Ok(Self { client })
    }

    pub async fn get_plugin(&self, name: &str) -> Result<EndGitPluginAdapter, ApiError> {
        let endgit_plugin = self.client.get_plugin(name).await?;
        Ok(EndGitPluginAdapter::new(endgit_plugin))
    }

    pub async fn all_plugins(&self) -> Result<Vec<EndGitPluginAdapter>, ApiError> {
        let plugins = self.client.search_plugins("").await?;
        let mut adapted_plugins: Vec<EndGitPluginAdapter> = vec![];
        for plugin in plugins.data.plugins {
            adapted_plugins.push(EndGitPluginAdapter::new(plugin));
        }
        Ok(adapted_plugins)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[tracing_test::traced_test]
    async fn test_get_plugin() -> Result<(), ApiError> {
        let client = EndGitConverterClient::new()?;
        let plugin = client.get_plugin("endstone-tebex-integration").await?;

        tracing::info!("{:?}", plugin);

        Ok(())
    }

    #[tokio::test]
    #[tracing_test::traced_test]
    async fn test_all_plugins() -> Result<(), ApiError> {
        let client = EndGitConverterClient::new()?;
        let plugins = client.all_plugins().await?;

        tracing::info!("{:?}", plugins);

        // Very dumb assertion, nonetheless will work.
        assert!(plugins.len() > 3);

        Ok(())
    }
}