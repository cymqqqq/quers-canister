use canister_library::canister_state;
use crate::model::profile::UserIndex;
use candid::Principal;
use serde::{Deserialize, Serialize};
// use std::cell::RefCell;
// use std::collections::{HashMap, HashSet};
use utils::env::Environment;
use utils::types::*;
// use utils::time::DAY_IN_MS;
pub mod lifecycle;
pub mod guards;
pub mod model;
pub mod queries;
pub mod updates;

canister_state!(RuntimeState);

struct RuntimeState {
    pub env: Box<dyn Environment>,
    pub data: Data,
}

impl RuntimeState {
    pub fn new(env: Box<dyn Environment>, data: Data) -> RuntimeState {
        RuntimeState { env, data }
    }

    pub fn is_caller_quers_user(&self) -> bool {
        let caller = self.env.caller();
        self.data.users.profile.get(&caller).is_some()
    }
}

#[derive(Serialize, Deserialize)]
struct Data {
    pub users: UserIndex,
    pub homepage: HomePage,
    pub question: Question,
    pub answer: Answer,
}


impl Data {
    pub fn new() -> Self {
        Self {
            users: UserIndex::new(),
            homepage: HomePage::default(),
            question: Question::default(),
            answer: Answer::default(),
        }
    }

    pub fn set_user_principal(&mut self, owner: &Principal) {
        self.users.set_user_principal(&owner);
    }

    pub fn update_user_description(&mut self, owner: &Principal, description: &String) {
        self.users.update_user_description(&owner, &description);
    }

    pub fn update_user_tvl(&mut self, owner: &Principal, tvl: &u32) {
        self.users.update_user_tvl(&owner, &tvl);
    }
    pub fn update_user_holders(&mut self, owner: &Principal, holders: &u32) {
        self.users.update_user_holders(&owner, &holders);
    }

    pub fn update_user_followers(&mut self, owner: &Principal, followers: &u32) {
        self.users.update_user_followers(&owner, &followers);
    }

    pub fn update_user_holding(&mut self, owner: &Principal, holding: &u32) {
        self.users.update_user_holding(&owner, &holding);
    }

    pub fn get_all_question_list(&self) -> Vec<Question> {
        self.homepage.get_all_question_list()
    }

    pub fn add_question(&mut self,
        question_logo: Option<String>,
        question_title: String,
        question_description: String,
        question_image: Option<String>,
        question_asker: Principal,
        tags: Vec<String>,
    ) {
        self.homepage.ask_question(question_logo,
            question_title,
            question_description,
            question_image,
            question_asker,
            tags
        );
    }

    pub fn add_answer(&mut self,
        answer_pid: &Principal,
        answer_content: String,
    ) {
        let answer = Answer::new(
            answer_content,
            *answer_pid,
        );
        self.question.answer_question(*answer_pid, answer);
    }

    pub fn get_all_answers_list(&self) -> Vec<Answer> {
        self.question.get_all_answers_list()
    }
    
    pub fn add_comment(&mut self,
            comment_pid: &Principal,
            coment_content: String
    ) {
        let new_comment = Comment::new(*comment_pid, coment_content);
        self.answer.add_comment(*comment_pid, new_comment);
    }

    pub fn get_all_commet_list(&self) -> Vec<Comment> {
        self.answer.get_all_commet_list()
    }
}



// pub fn view_event() -> Option<(i32, Vec<Events>)> {
//     Some(EVENTS.with(|events|
//         (events.borrow().len() as i32, events.clone().borrow().to_vec())
//     ))
// }

// pub fn view_level_one() -> Vec<Events> {
//     LEVEL_ONE_EVENTS.with(|events|
//         {
//             let v = events.clone().into_inner();
//             let i = std::cmp::min(v.len(), NUM_OF_TOP);
//             let mut v_rev = v[..i].to_vec();
//             v_rev.reverse();
//             v_rev
            
//         }
//     )
// }

// pub fn view_by_id(id: String) -> Option<Events> {
//     EVENTS.with(|events|
//         {
//             events.clone()
//                     .into_inner()
//                     .iter()
//                     .find(|&e| e.event_id == id)
//                     .cloned()
//         }
//     )
// }

// pub fn view_yesterday(page: usize , num_of_page: Option<usize>) -> (i32, Vec<Events>) {
//     assert!(page > 0);

//     let num_of_page = num_of_page.unwrap_or(NUM_OF_PAGE);
//     EVENTS.with(|events| {
//         let v = events.clone().into_inner();
//         let start = (page - 1) * num_of_page;
//         let end = std::cmp::min(v.len(), page * num_of_page);
//         assert!(start < end);
//         let mut v_rev = v[start..end].to_vec();
//         v_rev.reverse();
//         (v[start..end].len() as i32, v_rev)
//     })
// }

// pub fn view_today(page: usize , num_of_page: Option<usize>) -> (i32, Vec<Events>) {
//     assert!(page > 0);

//     let num_of_page = num_of_page.unwrap_or(NUM_OF_PAGE);
//     EVENTS.with(|events| {
//         let v = events.clone().into_inner();
//         let start = (page - 1) * num_of_page;
//         let end = std::cmp::min(v.len(), page * num_of_page);
//         assert!(start < end);

//         let mut v_rev = v[start..end].to_vec();
//         v_rev.reverse();
//         (v[start..end].len() as i32, v_rev)
//     })
// }