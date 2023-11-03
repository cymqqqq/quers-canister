use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct Profile {
    pub owner: Principal,
    pub acount_id: String,
    pub tvl: u32,
    pub description: String,
    pub holders: u32,
    pub followers: u32,
    pub holding: u32,
    pub qa_mod: QuesAns,
}

#[derive(Debug, Clone, Deserialize, CandidType, Serialize)]
pub struct QuesAns {
    pub questions: Vec<String>,
    pub answers: Vec<String>,
}

impl Default for QuesAns {
    fn default() -> Self {
        Self {
            questions: Vec::new(),
            answers: Vec::new(),
        }
    }
}

impl QuesAns {
    // update method
    pub fn add_profile_question(&mut self, question: &str) {
        self.questions.push(question.into())
    }

    pub fn add_profile_answer(&mut self, answer: &str) {
        self.answers.push(answer.into())
    }

    // query method
    pub fn get_all_questions_list(self) -> Vec<String> {
        self.questions
    }

    pub fn get_all_answers_list(self) -> Vec<String> {
        self.answers
    }
}

impl Default for Profile {
    fn default() -> Self {
        Self { 
                 owner: Principal::anonymous(), 
                 acount_id: "".into(), 
                 tvl: 0u32, 
                 description: "".into(), 
                 holders: 0u32, 
                 followers: 0u32,
                 holding: 0u32,
                 qa_mod: QuesAns::default(),
            }
    }
}


impl Profile {
    // update method
    pub fn set_user_principal(&mut self, principal: &Principal) {
        self.owner = *principal;
    }
    pub fn update_profile_description(&mut self, desc: &String) {
        self.description = desc.into();
    }

    pub fn update_profile_tvl(&mut self, tvl: &u32) {
        self.tvl += tvl;
    }

    pub fn update_profile_holders(&mut self, holders: &u32) {
        self.holders += holders;
    }

    pub fn update_profile_followers(&mut self, followers: &u32) {
        self.followers += followers;
    }

    pub fn update_profile_holding(&mut self, holding: &u32) {
        self.holding += holding;
    }

    pub fn add_profile_question(&mut self, question: &str) {
        self.qa_mod.add_profile_question(question);
    }

    pub fn add_profile_answer(&mut self, answer: &str) {
        self.qa_mod.add_profile_answer(answer);
    }
    // query method


    pub fn get_profile_description(&self) -> String {
        self.description.clone()
    }

    pub fn get_profile_tvl(&self) -> u32 {
        self.tvl.into()
    }

    pub fn get_profile_holders(&self) -> u32 {
        self.holders.into()
    }

    pub fn get_profile_holding(&self) -> u32 {
        self.holding.into()
    }

    pub fn get_profile_followers(&self) -> u32 {
        self.followers.into()
    }
}