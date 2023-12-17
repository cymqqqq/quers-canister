use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub owner: Principal,
    pub to_follow: Principal
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Response {
    Success,
    DownVoteInvalid,
}