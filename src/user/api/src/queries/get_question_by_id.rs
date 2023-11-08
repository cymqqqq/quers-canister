use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use utils::types::Question;
#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub question_id: String,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Response {
    Success(SuccessResult),
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct SuccessResult {
    pub question: Question,
}

