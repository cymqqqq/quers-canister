use ic_cdk_macros::*;
use candid::CandidType;
use serde::*;

#[derive(Clone, Copy,CandidType)]
pub struct Asset {
    pub balance: u64,
}

impl Default for Asset {
    fn default() -> Self {
        Asset { balance: 0u64 }
    }
}

