use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub question_title: String,
    pub question_description: String,
    pub question_image: Option<String>,
    pub question_asker: Principal,
    pub reference_link: Option<String>,
    pub reference_title: Option<String>,
    pub tags: Vec<String>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Response {
    Success,
    QuestionInvalid,
}