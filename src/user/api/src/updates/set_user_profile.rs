use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub owner: Principal, 
    pub description: String, 
    pub name: String,
    pub username: String,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Response {
    Success,
    PrincipalInvalid,
}