use crate::{
    config::Config, lib::utils::address, manage_pool::{ChainBasedPoolModel, PoolRequest, PoolType}, pools::aave::{self, AaveV3DefaultInterestRatePool}, response::{GenericResponse, PoolRequestListResponse}, WebResult, DB
};
use chrono::prelude::*;
use warp::{http::StatusCode, reply::json, reply::with_status, Reply};
//use std::collections::HashMap;

pub async fn hello() -> WebResult<impl Reply> {
    const MESSAGE: &str = "Hello, World!"; // the message to return
    let response = &GenericResponse{
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };
    Ok(json(response)) // return a JSON response
}

pub async fn pool_request_list(db:DB) -> WebResult<impl Reply> {
    let pools = db.lock().await;
    //let pools = pools.clone().into_iter();
    let json_response = PoolRequestListResponse {
        status: "success".to_string(),
        results: pools.len(),
        pools: pools.to_vec()
    };
 
    Ok(json(&json_response))
}

// fn get_pool_allocation(poolData: &PoolRequest) -> PoolAllocation {
//     let pools = poolData.pools;
//     let total_assets = poolData.total_assets;
//     let mut allocations = HashMap::new();

// }

pub async fn receive_pool_handler(mut body:PoolRequest, db:DB) -> WebResult<impl Reply> {
    let mut vec = db.lock().await; 

    for pool_req in vec.iter(){
        if pool_req.id == body.id{
            let error_message = &GenericResponse{
                status: "error".to_string(),
                message:  format!("pool_req already exists: {}", pool_req.id)  
            };
            return Ok(with_status(json(&error_message), StatusCode::CONFLICT))            
        }
    }

    body.timestamp = Utc::now(); 
    let pool = body.to_owned();

    for (pool_id, pool) in body.pools.iter() {
        if pool.pool_type == PoolType::Aave {
            // Logic for Aave pools
            println!("Found an Aave pool with ID: {}", pool_id);
            let config = Config::new().await;            
        
            // Call the pool_init method to initialize the pool asynchronously            
            let aavePool = AaveV3DefaultInterestRatePool::pool_init(config.http, address(&pool.contract_address)).await;
            println!("Created the Aave Pool Successfully for address {}", aavePool.underlying_asset_address())
        }
    }
    
    vec.push(body);
    let json_response = &GenericResponse {
        status: "success".to_string(),
        message: format!("Pool saved: {}", pool.id),
    };

    Ok(with_status(json(&json_response), StatusCode::CREATED))
}