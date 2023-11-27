use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use utils::types::Question;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub page: usize,
    pub num_of_page:Option<usize>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Response {
    Success(SuccessResult),
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct SuccessResult {
    pub question_list: (i32, Vec<Question>),
}

