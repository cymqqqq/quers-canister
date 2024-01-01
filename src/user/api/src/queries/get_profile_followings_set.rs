use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use utils::types::Profile;
#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub owner: Principal,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Response {
    Success(SuccessResult),
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct SuccessResult {
    pub followings_set: Vec<Profile>,
}

