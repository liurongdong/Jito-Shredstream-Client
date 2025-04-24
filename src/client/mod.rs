use tonic::transport::Channel;
use jito_protos::shredstream::{
    shredstream_proxy_client::ShredstreamProxyClient,
    SubscribeEntriesRequest,
};
use tokio::time::sleep;
use std::time::Duration;

pub struct ShredstreamClient {
    server_url: String,
}

impl ShredstreamClient {
    pub fn new(server_url: String) -> Self {
        Self { server_url }
    }

    pub async fn connect(&self) -> Result<ShredstreamProxyClient<Channel>, tonic::transport::Error> {
        ShredstreamProxyClient::connect(self.server_url.clone()).await
    }

    pub async fn subscribe_entries(
        &self,
        client: &mut ShredstreamProxyClient<Channel>
    ) -> Result<tonic::Streaming<jito_protos::shredstream::Entry>, Box<dyn std::error::Error>> {
        loop {
            let request = tonic::Request::new(SubscribeEntriesRequest {});
            match client.subscribe_entries(request).await {
                Ok(response) => return Ok(response.into_inner()),
                Err(_) => sleep(Duration::from_secs(5)).await,
            }
        }
    }
} 