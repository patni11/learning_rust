use ethers::prelude::{k256::ecdsa::SigningKey, *};
use std::collections::HashMap;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::str::FromStr;
//use std::hash::{Hash, Hasher};
use std::cmp::{Eq, PartialEq};


const ONE_ETHER: U256 = U256([1_000_000_000_000_000_000, 0, 0, 0]);

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum PoolType {
    Synthetic = 0,
    SturdySilo = 1,
    Aave = 2,
    DaiSavings = 3,
    CompoundV3 = 4,
    Morpho = 5,
}

impl FromStr for PoolType {
    type Err = String;

    fn from_str(input: &str) -> Result<PoolType, Self::Err> {
        match input {
            "Synthetic" => Ok(PoolType::Synthetic),
            "SturdySilo" => Ok(PoolType::SturdySilo),
            "Aave" => Ok(PoolType::Aave),
            "DaiSavings" => Ok(PoolType::DaiSavings),
            "CompoundV3" => Ok(PoolType::CompoundV3),
            "Morpho" => Ok(PoolType::Morpho),
            _ => Err(format!("Invalid enum name: {}", input)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasePool{
    pub pool_model_disc:String,
    pub contract_address: String,
    pub pool_type:PoolType,
    pub base_rate: U256,
    pub base_slope: U256,
    pub kink_slope: U256,
    pub optimal_util_rate: U256,
    pub borrow_amount: U256,
    pub reserve_size: U256,
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


pub trait BasePoolModel {
    fn util_rate(&self) -> U256;
    fn borrow_rate(&self) -> U256;
    fn supply_rate(&self) -> U256;
}

pub trait ChainBasedPoolModel{
    fn validator_pool_type(&self) -> PoolType;
    async fn pool_init(middleware: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>, contract_address:Address) -> Self;
    fn sync(&self) -> bool;
    fn supply_rate(&self) -> U256;
}

impl BasePoolModel for BasePool{
    fn util_rate(&self) -> U256 {
        return self.borrow_amount / self.reserve_size
    }

    fn borrow_rate(&self) -> U256 {
        let util_rate = self.util_rate();
        if util_rate < self.optimal_util_rate {
            self.base_rate + ((util_rate/self.optimal_util_rate)*self.base_slope)
        }else{
            return self.base_rate + self.base_slope + (((util_rate-self.optimal_util_rate)/(ONE_ETHER-self.optimal_util_rate))*self.kink_slope);
        }        
    }

    fn supply_rate(&self) -> U256{
        self.util_rate()*self.borrow_rate()
    }

}