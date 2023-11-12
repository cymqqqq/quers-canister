use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use utils::types::Comment;
#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub question_id: String,
    pub answer_pid: Principal,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Response {
    Success(SuccessResult),
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct SuccessResult {
    pub comment_list: Vec<Comment>,
}

