use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{self, AtomicU16};
use std::collections::HashMap;
use crate::env::{
    TimestampNanos,
};
use crate::time::{now_nanos};

static QID: AtomicU16 = AtomicU16::new(0);
// generate new question id
fn new_qid() -> String {
    let cid = ic_cdk::api::id();

    let qid = QID.fetch_add(1, atomic::Ordering::SeqCst);
    format!("{}-{}", cid.to_text(), qid + 1)
}

static AID: AtomicU16 = AtomicU16::new(0);
// generate new answer id
fn new_aid() -> String {
    let cid = ic_cdk::api::id();

    let aid = QID.fetch_add(1, atomic::Ordering::SeqCst);
    format!("{}-{}", cid.to_text(), aid + 1)
}

static CID: AtomicU16 = AtomicU16::new(0);
// generate new comment id
fn new_cid() -> String {
    let cid = ic_cdk::api::id();

    let comment_id = CID.fetch_add(1, atomic::Ordering::SeqCst);
    format!("{}-{}", cid.to_text(), comment_id + 1)
}

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



#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct HomePage {
    pub question_list: HashMap<String, Question>,
}

impl Default for HomePage {
    fn default() -> Self {
        Self {
            question_list: HashMap::new(),
        }
    }
}

impl HomePage {
    pub fn ask_question(&mut self, 
        question_logo: Option<String>,
        question_title: String,
        question_description: String,
        question_image: Option<String>,
        question_asker: Principal,
        tags: Vec<String>,
    ) {
        let q_id = new_qid();
        let question_obj = Question::new(
            &q_id,
            question_title,
                question_description,
                question_logo,
                question_image,
                question_asker,
                tags,
        );
        self.question_list.insert(q_id, question_obj);

    }
    
    pub fn get_all_question_list(&self) -> Vec<Question> {
        self.question_list.values().cloned().collect()
    }
}

#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct Question {
    pub question_id: String,
    pub question_logo: Option<String>,
    pub question_title: String,
    pub question_description: String,
    pub question_date: u64,
    pub question_image: Option<String>,
    pub question_asker: Principal,
    pub up_thumb: u32,
    pub down_thumb: u32,
    pub tags: Vec<String>,
    pub answers: HashMap<Principal, Answer>,
}


impl Default for Question {
    fn default() -> Self {
        Self {
            question_id: "".to_string(),
            question_logo: Some("".to_string()),
            question_title: "".to_string(),
            question_description: "".to_string(),
            question_date: now_nanos(),
            question_image: Some("".to_string()),
            question_asker: Principal::anonymous(),
            up_thumb: 0u32,
            down_thumb: 0u32,
            tags: Vec::new(),
            answers: HashMap::new(),
        }
    }
}

impl Question {
    pub fn new(question_id: &String,
                question_title: String, 
                question_description: String,
                question_logo: Option<String>,
                question_image: Option<String>,
                question_asker: Principal,
                tags: Vec<String>,
    ) -> Self {
        Self {
            question_id: question_id.into(),
            question_logo: question_logo,
            question_title: question_title,
            question_image: question_image,
            question_description: question_description,
            question_date: now_nanos(),
            question_asker: question_asker,
            up_thumb: 0u32,
            down_thumb: 0u32,
            tags: tags,
            answers: HashMap::new(),
        }
    }

    pub fn answer_question(&mut self, aq_pid: Principal, answer: Answer) {
        self.answers.insert(aq_pid, answer);
    }

    pub fn get_all_answers_list(&self) -> Vec<Answer> {
        self.answers.values().cloned().collect()
    }

}

#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct Answer {
    pub answer_content: String,
    pub answer_id: String,
    pub answer_date: u64,
    pub answer_pid: Principal,
    pub up_thumb: u32,
    pub down_thumb: u32,
    pub comments: HashMap<Principal, Comment>,
}

impl Default for Answer {
    fn default() -> Self {
        Self {
            answer_content: "".to_string(),
            answer_id: "".to_string(),
            answer_date: now_nanos(),
            answer_pid: Principal::anonymous(),
            up_thumb: 0u32,
            down_thumb: 0u32,
            comments: HashMap::new(),
        }
    }
}


impl Answer {
    pub fn new(answer_content: String, 
                answer_pid: Principal,
                ) -> Self {
                    Self {
                        answer_content: answer_content,
                        answer_id: new_aid(),
                        answer_pid: answer_pid,
                        up_thumb: 0u32,
                        down_thumb: 0u32,
                        answer_date: now_nanos(),
                        comments: HashMap::new(),
        }
    }

    pub fn add_comment(&mut self, 
                    commenter: Principal, 
                    comment: Comment) {
        self.comments.insert(commenter, comment);
    }

    pub fn get_all_commet_list(&self) -> Vec<Comment> {
        self.comments.values().cloned().collect()
    }
}

#[derive(Debug, Clone ,CandidType, Serialize, Deserialize)]
pub struct Comment {
    pub comment_content: String,
    pub comment_id: String,
    pub comment_pid: Principal,
    pub comment_date: u64,
    pub up_comment: u32,
    pub down_comment: u32,
}

impl Default for Comment {
    fn default() -> Self {
        Self {
            comment_content: "".to_string(),
            comment_id: "".to_string(),
            comment_pid: Principal::anonymous(),
            comment_date: now_nanos(),
            up_comment: 0u32,
            down_comment: 0u32,
        }
    }
}

impl Comment {
    pub fn new(
        comment_pid: Principal,
        comment_content: String,
    ) -> Self {
        Self {
            comment_id: new_cid(),
            comment_pid: comment_pid,
            comment_content: comment_content,
            comment_date: now_nanos(),
            up_comment: 0u32,
            down_comment: 0u32,
        }
    }
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
    pub fn get_all_profile_questions_list(self) -> Vec<String> {
        self.questions
    }

    pub fn get_all_profile_answers_list(self) -> Vec<String> {
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