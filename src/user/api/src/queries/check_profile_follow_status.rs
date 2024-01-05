use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use utils::types::{FollowStatus};


#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub owner: Principal,
    pub to: Principal
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Response {
    Success(SuccessResult),
}
#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct SuccessResult {
    pub follow_status: FollowStatus,
}
