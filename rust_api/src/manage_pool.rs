use ethers::prelude::*;
use std::collections::HashMap;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
enum PoolType {
    Synthetic = 0,
    SturdySilo = 1,
    Aave = 2,
    DaiSavings = 3,
    CompoundV3 = 4,
    Morpho = 5,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasePool{
    pool_model_disc:String,
    contract_address: String,
    pool_type:PoolType,
    base_rate: U256,
    base_slope: U256,
    kink_slope: U256,
    optimal_util_rate: U256,
    borrow_amount: U256,
    reserve_size: U256,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct PoolRequest{
    pub id: String,
    pub validator: String,
    pub timestamp: DateTime<Utc>,
    pub total_assets: U256,
    pub pools: HashMap<String, BasePool>,
}   

pub type DB = Arc<Mutex<Vec<PoolRequest>>>;

pub fn pool_db() -> DB {
    Arc::new(Mutex::new(Vec::new()))
}