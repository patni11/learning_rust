use std::sync::Arc;
use ethers::prelude::k256::ecdsa::SigningKey;
use ethers::prelude::*;
use crate::lib::utils::setup_signer;
use dotenv::dotenv;

pub struct Config {
    #[allow(dead_code)]
    pub http: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
}

impl Config {
    pub async fn new() -> Self {
        dotenv().ok();
        let network = std::env::var("NETWORK_RPC").expect("missing NETWORK_RPC");
        let provider: Provider<Http> = Provider::<Http>::try_from(network).unwrap();
        let middleware = Arc::new(setup_signer(provider.clone()).await);

        Self {
            http: middleware
        }
    }
}