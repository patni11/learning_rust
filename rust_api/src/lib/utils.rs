use std::time::Duration;
use rand::Rng;
use tokio::time::sleep;
use ethers::prelude::k256::ecdsa::SigningKey;
use ethers::prelude::*;
use dotenv::dotenv;

pub async fn retry_with_backoff<T, F, Fut>(mut func: F) -> Result<T, String>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, String>>,
{
    let max_retries = 5;
    let base_delay = 0.1;
    let max_delay = 60.0;

    let mut retries = 0;

    while retries < max_retries {
        match func().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                if e.contains("Rate limited") {
                    let delay = (base_delay * 2.0_f64.powi(retries)) as f64;
                    let capped_delay = delay.min(max_delay);
                    let jitter = rand::thread_rng().gen_range(capped_delay / 2.0..capped_delay * 1.5);
                    sleep(Duration::from_secs_f64(jitter)).await;
                    retries += 1;
                } else {
                    return Err(e);
                }
            }
        }
    }

    Err(format!("Maximum retries exceeded."))
}

/// Converts &str to Address.
pub fn address(address: &str) -> Address {
    address.parse::<Address>().unwrap()
}

/// Converts normal input into 1e18.
pub fn to_1e18(input: u64) -> U256 {
    let ether: U256 = U256::exp10(18);
    let parsed: U256 = input.into();
    parsed * ether
}

/// Sets up middleware w/ our private key env var.
pub async fn setup_signer(
    provider: Provider<Http>,
) -> SignerMiddleware<Provider<Http>, Wallet<SigningKey>> {
    dotenv().ok();
    let chain_id = provider
        .get_chainid()
        .await
        .expect("Failed to get chain id.");

    let priv_key = std::env::var("PRIVATE_KEY").expect("missing PRIVATE_KEY");

    let wallet = priv_key
        .parse::<LocalWallet>()
        .expect("Failed to parse wallet")
        .with_chain_id(chain_id.as_u64());

    SignerMiddleware::new(provider, wallet)
}

/// Creates a binding for an ABI.
/// Example: bind("Example", "src/abi/example.json");
pub fn bind(name: &str, abi: &str) {
    let name: String = format!("b_{}", name);
    let bindings = Abigen::new(&name, abi).unwrap().generate().unwrap();
    let path: String = format!("src/bindings/{}.rs", name);
    match std::fs::File::create(path.clone()) {
        Ok(_) => {}
        Err(_) => {}
    }
    bindings.write_to_file(&path).unwrap();
}