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

    pub async fn get_plugin(&self, name: &str, id: u64) -> Result<EndGitPluginAdapter, ApiError> {
        let endgit_plugin = self.client.get_plugin(name).await?;
        Ok(EndGitPluginAdapter::new(endgit_plugin, id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[tracing_test::traced_test]
    async fn test_get_plugin() -> Result<(), ApiError> {
        let client = EndGitConverterClient::new()?;
        let plugin = client.get_plugin("endstone-tebex-integration", 1).await?;

        tracing::info!("{:?}", plugin);

        Ok(())
    }
}