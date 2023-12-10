use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{self, AtomicU16};
use std::collections::HashMap;
use crate::env::{
    TimestampNanos,
};
use crate::time::{now_nanos};
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Default)]
pub struct Empty {}

static QID: AtomicU16 = AtomicU16::new(0);
// generate new question id
pub fn new_qid() -> String {
    let cid = ic_cdk::api::id();

    let qid = QID.fetch_add(1, atomic::Ordering::SeqCst);
    format!("{}-{}", cid.to_text(), qid + 1)
}


#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct Profile {
    pub owner: String,
    pub tvl: u32,
    pub description: String,
    pub holders: u32,
    pub holding: u32,
    pub qa_mod: QuesAns,
    pub tickets: u32,
    pub name: String,
    pub username: String,
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
        question_id: &String,
        question: Question,
    ) {
        self.question_list.insert(question_id.into(), question);

    }
    
    pub fn get_all_question_list(&self) -> Vec<Question> {
        self.question_list.values().cloned().collect()
    }

    pub fn get_all_question_id_list(&self) -> Vec<String> {
        self.question_list.keys().cloned().collect()
    }

    pub fn get_question_by_id(&self, question_id: &String) -> Option<&Question> {
        self.question_list.get(question_id)
    }

    pub fn update_question_by_id(&mut self, 
                question_id: &String,
                question: &Question
    )
    {
        self.question_list.insert(question_id.to_string(), question.clone());
    }
}

#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct Question {
    pub question_id: String,
    pub question_logo: Option<String>,
    pub question_title: String,
    pub question_description: String,
    pub question_date: String,
    pub question_image: Option<String>,
    pub question_asker: String,
    pub votes: u32,
    pub tags: Vec<String>,
    pub answers: HashMap<String, Answer>,
}


impl Default for Question {
    fn default() -> Self {
        Self {
            question_id: "".to_string(),
            question_logo: Some("".to_string()),
            question_title: "".to_string(),
            question_description: "".to_string(),
            question_date: now_nanos().to_string(),
            question_image: Some("".to_string()),
            question_asker: Principal::anonymous().to_string(),
            votes: 0u32,
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
            question_date: now_nanos().to_string(),
            question_asker: question_asker.to_string(),
            votes: 0u32,
            tags: tags,
            answers: HashMap::new(),
        }
    }

    pub fn answer_question(&mut self, aq_pid: Principal, answer: &Answer) {
        self.answers.insert(aq_pid.to_string(), answer.clone());
    }

    pub fn get_answer_by_principal(&self, principal: &Principal) -> Option<&Answer> {
        let new_pid_to_string = principal.to_text();
        self.answers.get(&new_pid_to_string)
    }

    pub fn get_all_answers_list(&self) -> Vec<Answer> {
        self.answers.values().cloned().collect()
    }

    pub fn get_question_votes(&self) -> u32 {
        self.votes
    }

    pub fn up_vote(&mut self) {
        self.votes += 1;
    }

    pub fn down_vote(&mut self) {
        self.votes -= 1;
    }
}

#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct Answer {
    pub answer_content: String,
    // pub answer_id: String,
    pub answer_date: String,
    pub answer_pid: String,
    pub up_thumb: u32,
    pub comments: HashMap<String, Comment>,
}

impl Default for Answer {
    fn default() -> Self {
        Self {
            answer_content: "".to_string(),
            // answer_id: "".to_string(),
            answer_date: now_nanos().to_string(),
            answer_pid: Principal::anonymous().to_string(),
            up_thumb: 0u32,
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
                        // answer_id: new_aid(),
                        answer_pid: answer_pid.to_string(),
                        up_thumb: 0u32,
                        answer_date: now_nanos().to_string(),
                        comments: HashMap::new(),
        }
    }

    pub fn add_comment(&mut self, 
                    commenter: Principal, 
                    comment: Comment) {
        self.comments.insert(commenter.to_string(), comment);
    }

    pub fn get_all_comment_list(&self) -> Vec<Comment> {
        self.comments.values().cloned().collect()
    }
}

#[derive(Debug, Clone ,CandidType, Serialize, Deserialize)]
pub struct Comment {
    pub comment_content: String,
    // pub comment_id: String,
    pub comment_pid: String,
    pub comment_date: String,
    pub up_comment: u32,
}

impl Default for Comment {
    fn default() -> Self {
        Self {
            comment_content: "".to_string(),
            // comment_id: "".to_string(),
            comment_pid: Principal::anonymous().to_string(),
            comment_date: now_nanos().to_string(),
            up_comment: 0u32,
        }
    }
}

impl Comment {
    pub fn new(
        comment_pid: Principal,
        comment_content: String,
    ) -> Self {
        Self {
            // comment_id: new_cid(),
            comment_pid: comment_pid.to_string(),
            comment_content: comment_content,
            comment_date: now_nanos().to_string(),
            up_comment: 0u32,
        }
    }
}
#[derive(Debug, Clone, Deserialize, CandidType, Serialize)]
pub struct QuesAns {
    pub questions: Vec<Question>,
    pub answers: Vec<Answer>,
    pub watch_list:Vec<String>,
}

impl Default for QuesAns {
    fn default() -> Self {
        Self {
            questions: Vec::new(),
            answers: Vec::new(),
            watch_list: Vec::new(),
        }
    }
}

impl QuesAns {
    // update method
    pub fn add_profile_question(&mut self, question: &Question) {
        self.questions.push(question.clone());
    }

    pub fn add_profile_answer(&mut self, answer: &Answer) {
        self.answers.push(answer.clone());
    }

    pub fn add_watch_list(&mut self, question_id: &String) {
        self.watch_list.push(question_id.into());
    }

    // query method
    pub fn get_all_profile_questions_list(self) -> Vec<Question> {
        self.questions
    }

    pub fn get_all_profile_answers_list(self) -> Vec<Answer> {
        self.answers
    }

    pub fn get_all_profile_watch_list(self) -> Vec<String> {
        self.watch_list
    }
}

impl Default for Profile {
    fn default() -> Self {
        Self { 
                 owner: Principal::anonymous().to_string(), 
                 tvl: 0u32, 
                 description: "".into(), 
                 holders: 0u32, 
                 holding: 0u32,
                 qa_mod: QuesAns::default(),
                 tickets: 0u32,
                 name: "".into(),
                 username: "".into(),
            }
    }
}


impl Profile {
    // update method
    pub fn new(
        owner: Principal, 
        description: String, 
        name: String,
        username: String,

    ) -> Self {
        Self {
            owner: owner.to_text(),
            tvl: 0u32,
            description: description,
            holders: 0u32,
            holding: 0u32,
            tickets: 0u32,
            name: name,
            username: username,
            qa_mod: QuesAns::default(),
        }
    }

    pub fn set_user_principal(&mut self, principal: &Principal) {
        self.owner = principal.to_string();
    }
    pub fn update_profile_description(&mut self, desc: &String) {
        self.description = desc.into();
    }

    pub fn update_profile_tvl(&mut self, tvl: &u32) {
        self.tvl += tvl;
    }

    pub fn update_profile_tickets(&mut self, tickets: &u32) {
        self.tickets += tickets;
    }

    pub fn update_profile_username(&mut self, username: &String) {
        self.username = username.into();
    }

    pub fn update_profile_name(&mut self, name: &String) {
        self.name = name.into();
    }

    pub fn update_profile_holding(&mut self, holding: &u32) {
        self.holding += holding;
    }

    pub fn add_profile_question(&mut self, question: &Question) {
        self.qa_mod.add_profile_question(&question);
    }

    pub fn add_profile_answer(&mut self, answer: &Answer) {
        self.qa_mod.add_profile_answer(&answer);
    }

    pub fn add_watch_list(&mut self, question_id: &String) {
        self.qa_mod.add_watch_list(&question_id);
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

    pub fn get_profile_watch_list(self) -> Vec<String> {
        self.qa_mod.get_all_profile_watch_list()
    }
}
