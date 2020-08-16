use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, Storage};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};

pub static CONFIG_KEY: &[u8] = b"config";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub items: Vec<CanonicalAddr>,
    pub whitelist: Vec<CanonicalAddr>,
    pub contract_owner: CanonicalAddr,
    pub seed: Vec<u8>,
    pub entropy: Vec<u8>,
    pub start_time: u64,
    pub winner: CanonicalAddr,
    pub winner1: CanonicalAddr,
    pub winner2: CanonicalAddr,
    pub winner3: CanonicalAddr
}

pub fn config<S: Storage>(storage: &mut S) -> Singleton<S, State> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read<S: Storage>(storage: &S) -> ReadonlySingleton<S, State> {
    singleton_read(storage, CONFIG_KEY)
}
