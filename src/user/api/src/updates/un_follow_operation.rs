use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub to_follow: Principal,
    pub owner: Principal
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Response {
    Success,
    DownVoteInvalid,
}