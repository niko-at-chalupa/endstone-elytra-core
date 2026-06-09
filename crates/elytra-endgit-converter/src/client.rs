use nikos_endgit_wrapper::client::{Client, ApiError};
use elytra_core::id::{kebabify, compute_plugin_id};
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
        Ok(EndGitPluginAdapter::new(endgit_plugin, compute_plugin_id(&kebabify(name))))
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
}