use std::sync::Arc;
use std::hash::{Hash, Hasher};
use std::cmp::PartialEq;
use crate::manage_pool::{PoolType, ChainBasedPoolModel};
use crate::lib::utils::retry_with_backoff;
use ethers::prelude::{k256::ecdsa::SigningKey, *};

abigen!(AToken, "src/abi/AToken.json");
abigen!(Pool, "src/abi/Pool.json");
abigen!(IERC20, "src/abi/IERC20.json");
abigen!(IReserveInterestRateStrategy, "src/abi/IReserveInterestRateStrategy.json");
abigen!(IVariableDebtToken, "src/abi/IVariableDebtToken.json");

pub struct AaveV3DefaultInterestRatePool {
    pool_model_disc:String,
    pool_type:PoolType,
    pub user_address:Address,
    pub contract_address:Address,
    _initted: bool ,
    _atoken_contract: AToken<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    _pool_contract: Pool<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    _underlying_asset_contract: IERC20<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    _underlying_asset_address: Address,
    // _reserve_data: Option<String>, // Replace with actual type
    // _strategy_contract: Option<IReserveInterestRateStrategy<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>>,
    // _next_total_stable_debt: Option<U256>,
    // _next_avg_stable_borrow_rate: Option<U256>,
    // _variable_debt_token_contract: Option<IVariableDebtToken<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>>,
    // _total_variable_debt: Option<U256>,
    // _reserve_factor: Option<U256>,
    // _collateral_amount: Option<U256>,
    _total_supplied: Option<U256>,
    // _decimals: Option<U256>,
}

impl ChainBasedPoolModel for AaveV3DefaultInterestRatePool{
    async fn pool_init(middleware: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>, contract_address:Address) -> Self{
        println!("Creating Aave Pool");

        let atoken_contract = AToken::new(contract_address, Arc::clone(&middleware));
        
        let decimals: u8 = retry_with_backoff(|| async {
            atoken_contract.decimals().call().await.map_err(|e| e.to_string())
        }).await.unwrap();

        println!("decimals {}", decimals);
        
        let pool_address: Address = retry_with_backoff(|| async {
            atoken_contract.pool().call().await.map_err(|e| e.to_string())
        }).await.unwrap();

        println!("Got Pool Address {}", pool_address);
        let pool_contract = Pool::new(pool_address, Arc::clone(&middleware));
        
        let underlying_asset_address = retry_with_backoff(|| async {
            atoken_contract.underlying_asset_address().call().await.map_err(|e| e.to_string())
        }).await.unwrap();

        println!("Got underlying_asset_address {}", underlying_asset_address);
        
        let underlying_asset_contract = IERC20::new(underlying_asset_address, Arc::clone(&middleware));
        
        let total_supplied = retry_with_backoff(|| async {
            atoken_contract.total_supply().call().await.map_err(|e| e.to_string())
        }).await.unwrap();
        
        
        Self {
            pool_model_disc:"CHAIN".to_string(),
            pool_type: PoolType::Aave,
            user_address:Address::zero(),
            contract_address:contract_address,
            _initted:true,
            _atoken_contract:atoken_contract,
            _pool_contract:pool_contract,
            _underlying_asset_contract:underlying_asset_contract,
            _underlying_asset_address:underlying_asset_address, 
            _total_supplied:Some(total_supplied)
        }
    }

    //TODO: to be implemented
    fn validator_pool_type(&self) -> PoolType{
        return PoolType::Aave;
    }

    //TODO: to be implemented    
    fn sync(&self) -> bool{
        return true;
    }

    //TODO: to be implemented
    fn supply_rate(&self) -> U256{
        return U256::from(1);
    }
}

impl AaveV3DefaultInterestRatePool {
    pub fn underlying_asset_address(&self) -> Address{
        return self._underlying_asset_address;
    }
}

impl Hash for AaveV3DefaultInterestRatePool {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.contract_address.as_ref().hash(state);
        self._underlying_asset_address.as_ref().hash(state);
    }
}

impl PartialEq for AaveV3DefaultInterestRatePool {
    fn eq(&self, other: &Self) -> bool {
        self.contract_address.as_ref() == other.contract_address.as_ref()
            && self._underlying_asset_address.as_ref() == other._underlying_asset_address.as_ref()
    }
}





