use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{self, AtomicU16};
use std::collections::{HashMap, HashSet};
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
    pub profile_image_url: String,
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
                question: Question
    )
    {
        self.question_list.insert(question_id.to_string(), question);
    }
}

#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct Question {
    pub question_id: String,
    pub question_logo: String,
    pub question_title: String,
    pub question_description: String,
    pub question_date: String,
    pub question_image: String,
    pub question_asker: String,
    pub votes: u32,
    pub tags: Vec<String>,
    pub answers: HashMap<String, Answer>,
}


impl Default for Question {
    fn default() -> Self {
        Self {
            question_id: "".to_string(),
            question_logo: "".to_string(),
            question_title: "".to_string(),
            question_description: "".to_string(),
            question_date: now_nanos().to_string(),
            question_image: "".to_string(),
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
                question_logo: String,
                question_image: String,
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

    pub fn answer_question(&mut self, aq_pid: Principal, answer: Answer) {
        self.answers.insert(aq_pid.to_string(), answer);
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
    pub questions: HashSet<String>,
    pub answers: HashSet<String>,
    pub watch_list:HashSet<String>,
}

impl Default for QuesAns {
    fn default() -> Self {
        Self {
            questions: HashSet::new(),
            answers: HashSet::new(),
            watch_list: HashSet::new(),
        }
    }
}

impl QuesAns {
    // update method
    pub fn add_profile_question_id(&mut self, question_id: &String) {
        self.questions.insert(question_id.to_string());
    }

    pub fn add_profile_answer_id(&mut self, answer_question_id: &String) {
        self.answers.insert(answer_question_id.to_string());
    }

    pub fn add_watch_list(&mut self, question_id: &String) {
        self.watch_list.insert(question_id.into());
    }

    // query method
    pub fn get_all_profile_questions_id_list(self) -> Vec<String> {
        self.questions.into_iter().collect()
    }

    pub fn get_all_profile_answers_id_list(self) -> Vec<String> {
        self.answers.into_iter().collect()
    }

    pub fn get_all_profile_watch_list(self) -> Vec<String> {
        self.watch_list.into_iter().collect()
    }
}

impl Default for Profile {
    fn default() -> Self {
        Self { 
                 owner: Principal::anonymous().to_string(), 
                 tvl: 0u32, 
                 description: "".into(), 
                 profile_image_url: "".into(),
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
        profile_image_url: String,
    ) -> Self {
        Self {
            owner: owner.to_text(),
            tvl: 0u32,
            description: description,
            profile_image_url: profile_image_url,
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

    pub fn update_profile_image_url(&mut self, profile_image_url: &String) {
        self.profile_image_url = profile_image_url.into();
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

    pub fn update_profile_holders(&mut self, holders: &u32) {
        self.holders += holders;
    }

    pub fn update_profile_holding(&mut self, holding: &u32) {
        self.holding += holding;
    }

    pub fn add_profile_question_id(&mut self, question_id: &String) {
        self.qa_mod.add_profile_question_id(&question_id);
    }

    pub fn add_profile_answer_id(&mut self, question_answer_id: &String) {
        self.qa_mod.add_profile_answer_id(&question_answer_id);
    }

    pub fn add_watch_list(&mut self, question_id: &String) {
        self.qa_mod.add_watch_list(&question_id);
    }
    // query method

    pub fn get_profile_description(self) -> String {
        self.description
    }

    pub fn get_profile_tvl(self) -> u32 {
        self.tvl
    }

    pub fn get_profile_holders(self) -> u32 {
        self.holders
    }

    pub fn get_profile_holding(self) -> u32 {
        self.holding
    }

    pub fn get_profile_question_id_list(self) -> Vec<String> {
        self.qa_mod.get_all_profile_questions_id_list()
    }

    pub fn get_all_profile_answer_question_id_list(self) -> Vec<String> {
        self.qa_mod.get_all_profile_questions_id_list()
    }

    pub fn get_profile_watch_list(self) -> Vec<String> {
        self.qa_mod.get_all_profile_watch_list()
    }
}

#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct Follow {
    pub followers: HashMap<String, HashSet<String>>,
    pub followings: HashMap<String, HashSet<String>>,
}

impl Default for Follow {
    fn default() -> Self {
        Self {
            followers: HashMap::new(),
            followings: HashMap::new(),
        }
    }
}


impl Follow {

    fn get_followers_set(&self, owner: &Principal) -> HashSet<String> {
        match self.followers.get(&owner.to_string()) {
            Some(followers_set) => followers_set.clone(),
            None => HashSet::new(),
        }
    }

    fn get_followings_set(&self, owner: &Principal) -> HashSet<String> {
        match self.followings.get(&owner.to_string()) {
            Some(following_set) => following_set.clone(),
            None => HashSet::new(),
        }
    }

    fn add_following_operation(&mut self, owner: &Principal, to_follow: &Principal) {
        let mut following_map = self.get_followings_set(&owner);
        following_map.insert(to_follow.to_string());
        self.followings.insert(owner.to_string(), following_map);
    }

    fn add_follower_operation(&mut self, to_follow: &Principal, owner: &Principal) {
        let mut follower_map = self.get_followers_set(&to_follow);
        follower_map.insert(owner.to_string());
        self.followers.insert(to_follow.to_string(), follower_map);
    }

    fn remove_following_operation(&mut self, owner: &Principal, to_follow: &Principal) {
        let mut following_map = self.get_followings_set(&owner);
        following_map.remove(&to_follow.to_string());
        self.followings.insert(owner.to_string(), following_map);
    }

    fn remove_follower_operation(&mut self, to_follow: &Principal, owner: &Principal) {
        let mut follower_map = self.get_followers_set(&to_follow);
        follower_map.remove(&owner.to_string());
        self.followings.insert(to_follow.to_string(), follower_map);
    }

    pub fn follow(&mut self, owner: &Principal, to_follow: &Principal) {
        self.add_follower_operation(&to_follow, &owner);
        self.add_following_operation(&owner, &to_follow);
    }

    pub fn un_follow(&mut self, owner: &Principal, to_follow: &Principal) {
        self.remove_follower_operation(&to_follow, &owner);
        self.remove_following_operation(&owner, &to_follow);
    }

    pub fn get_profile_follower_counts(&self, owner: &Principal) -> usize {
        self.get_followers_set(&owner).len()
    }

    pub fn get_profile_following_counts(&self, owner: &Principal) -> usize {
        self.get_followings_set(&owner).len()
    }
}