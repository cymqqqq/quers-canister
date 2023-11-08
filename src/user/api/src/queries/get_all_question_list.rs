use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use utils::types::{Question, Empty};

pub type Args = Empty;


#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Response {
    Success(SuccessResult),
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct SuccessResult {
    pub question_list: Vec<Question>,
}

