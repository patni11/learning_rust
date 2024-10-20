use ethers::prelude::*;
use std::collections::HashMap;
use serde::Serialize;
use crate::manage_pool::PoolRequest;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct PoolAllocation {
    pub validator: String,
    pub id: String,
    pub allocations: HashMap<String, U256>,
}

#[derive(Serialize, Debug)]
pub struct PoolRequestListResponse {
    pub status: String,
    pub results: usize,
    pub pools: Vec<PoolRequest>,
}