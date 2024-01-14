use canister_library::canister_state;
use crate::model::profile::UserIndex;
use candid::Principal;
use candid::{CandidType, Deserialize};
use serde::{ Serialize};
// use std::cell::RefCell;
// use std::collections::{HashMap, HashSet};
use utils::env::Environment;
use utils::types::*;
use utils::types::{new_qid};
// use utils::time::DAY_IN_MS;
pub mod lifecycle;
pub mod guards;
pub mod model;
pub mod queries;
pub mod updates;
pub mod memory;

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

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
struct Data {
    pub users: UserIndex,
    pub homepage: HomePage,
    pub question: Question,
    pub answer: Answer,
    pub follow_state: Follow,
}


impl Data {
    pub fn new() -> Self {
        Self {
            users: UserIndex::new(),
            homepage: HomePage::default(),
            question: Question::default(),
            answer: Answer::default(),
            follow_state: Follow::default(),
        }
    }

    // pub fn set_user_principal(&mut self, owner: &Principal) {
    //     self.users.set_user_principal(&owner);
    // }

    pub fn update_user_description(&mut self, owner: &Principal, description: &String) {
        self.users.update_user_description(&owner, &description);
    }

    pub fn update_user_tvl(&mut self, owner: &Principal, tvl: &u32) {
        self.users.update_user_tvl(&owner, &tvl);
    }
    pub fn update_user_holders(&mut self, owner: &Principal, holders: &u32) {
        self.users.update_user_holders(&owner, &holders);
    }


    pub fn update_user_holding(&mut self, owner: &Principal, holding: &u32) {
        self.users.update_user_holding(&owner, &holding);
    }

    pub fn update_username(&mut self, owner: &Principal, username: &String) {
        self.users.update_username(&owner, &username);
    }

    pub fn update_name(&mut self, owner: &Principal, name: &String) {
        self.users.update_name(&owner, &name);
    }

    pub fn update_user_tickets(&mut self, owner: &Principal, tickets: &u32) {
        self.users.update_user_ticket(&owner, &tickets);
    }

    pub fn set_user_profile(&mut self, 
        owner: &Principal,
        description: &String,
        name: &String,
        username: &String,
        profile_image_url: &String,
        ) {
        let profile = Profile::new(
            owner.to_string(),
            description.to_string(),
            name.to_string(),
            username.to_string(),
            profile_image_url.to_string(),
        );
        self.users.set_user_profile(*owner, profile);
    }

    pub fn get_all_question_list(&self) -> Vec<Question> {
        self.homepage.get_all_question_list()
    }

    pub fn get_all_question_id_list(&self) -> Vec<String> {
        self.homepage.get_all_question_id_list()
    }
    
    pub fn add_question(&mut self,
        question_title: &String,
        question_description: &Option<String>,
        question_image: &Option<String>,
        question_asker: &Principal,
        reference_link: &Option<String>,
        reference_title: &Option<String>,
        tags: &Option<Vec<String>>,
        lang: &String,
    ) {
        let q_id = new_qid();
        let q_desc = match question_description {
            Some(description) => description.to_string(),
            None => "No description".to_string(),
        };

        let q_tags = match tags {
            Some(tags) => tags.to_vec(),
            None => vec![],
        };
        let q_r_link = match reference_link {
            Some(link) => link.to_string(),
            None => "".to_string(),
        };

        let q_r_title = match reference_title {
            Some(title) => title.to_string(),
            None => "".to_string(),
        };

        let q_image = match question_image {
            Some(image) => image.to_string(),
            None => "".to_string(),
        };

        let question_obj = Question::new(
            &q_id,
            question_title.to_string(),
                q_desc,
                q_image,
                *question_asker,
                q_r_link,
                q_r_title,
                q_tags,
        );

        self.users.update_user_question_id_list(&question_asker, &q_id);
        self.homepage.ask_question(&q_id, question_obj, lang.to_string());
    }

    pub fn add_profile_watch_list(&mut self, user: &Principal, question_id: &String) {
        self.users.update_user_watch_list(&user, &question_id);
    }

    pub fn get_question_by_id(&self, question_id: &String) -> Question {
        let question = match self.homepage.get_question_by_id(question_id) {
            Some(question_internal) => question_internal.clone(),
            None => Question::default(),
        };
        question
    }


    pub fn up_vote(&mut self, question_id: &String) {
        let mut question = self.get_question_by_id(&question_id);
        question.up_vote();
        self.homepage.update_question_by_id(&question_id, question);

    }

    pub fn down_vote(&mut self, question_id: &String) {
        let mut question = self.get_question_by_id(&question_id);
        question.down_vote();
        self.homepage.update_question_by_id(&question_id, question);
    }
    pub fn view_by_page(&self, page: usize , num_of_page: Option<usize>) -> (i32, Vec<Question>) {
        assert!(page > 0);
    
        let num_of_page = num_of_page.unwrap_or(10usize);
        let question_list = self.homepage.get_all_question_list();
       
            let start = (page - 1) * num_of_page;
            let end = std::cmp::min(question_list.len(), page * num_of_page);
            assert!(start < end);
    
            let mut v_rev = question_list[start..end].to_vec();
            v_rev.reverse();
            (question_list[start..end].len() as i32, v_rev)
    }

    pub fn get_question_votes_by_id(&self, question_id: &String) -> u32 {
        self.get_question_by_id(&question_id).get_question_votes()
    }
    

    pub fn add_answer(&mut self,
        question_id: &String,
        answer_pid: &Principal,
        answer_content: &String,
    ) {
        let mut question = self.get_question_by_id(&question_id);

        let answer = Answer::new(
            answer_content.to_string(),
            *answer_pid,
        );
        self.users.update_user_answer_id_list(&answer_pid, &question_id);
        question.answer_question(*answer_pid, answer);
        self.homepage.update_question_by_id(&question_id, question);
    }

    pub fn get_all_answers_list_by_question_id(&self, question_id: &String) -> Vec<Answer> {
        self.get_question_by_id(&question_id).get_all_answers_list()
    }
    
    pub fn add_comment(&mut self,
            question_id: &String,
            answer_pid: &Principal,
            comment_pid: &Principal,
            comment_content: &String
    ) {
        let mut question = self.get_question_by_id(&question_id);

        let mut answer = match question.get_answer_by_principal(&answer_pid) {
            Some(answer_internal) => answer_internal.clone(),
            None => Answer::default(),
        };

        let new_comment = Comment::new(*comment_pid, comment_content.to_string());
        answer.add_comment(*comment_pid, new_comment);
        question.answer_question(*answer_pid, answer);
        self.homepage.update_question_by_id(&question_id, question);

    }

    pub fn get_all_comment_list(&self, 
        question_id: &String,
        answer_pid: &Principal) -> Vec<Comment> {
        let question = self.get_question_by_id(&question_id);

        let answer= match question.get_answer_by_principal(&answer_pid)  
        {
            Some(answer_internal) => answer_internal.clone(),
            None => Answer::default(),
        };
        answer.get_all_comment_list()
    }

    pub fn follow_operation(&mut self, owner: &Principal, to_follow: &Principal) {
        self.follow_state.follow(&owner, &to_follow);
    }

    pub fn un_follow_operation(&mut self, to_follow: &Principal, owner: &Principal) {
        self.follow_state.un_follow(&to_follow, &owner);
    }

    pub fn get_profile_followers_count(&self, owner: &Principal) -> usize {
        self.follow_state.get_profile_follower_counts(&owner)
    }

    pub fn get_profile_followings_count(&self, owner: &Principal) -> usize {
        self.follow_state.get_profile_following_counts(&owner)
    }
    // get profile question list
    pub fn get_profile_question_list(&self, owner: &Principal) -> Vec<Question> {
        self.users
        .get_all_profile_questions_id_list(&owner)
        .iter()
        .map(|question_id| self.get_question_by_id(question_id))
        .collect()

    }
    // get profile answer question list
    pub fn get_profile_answer_question_list(&self, owner: &Principal) -> Vec<Question> {
        self.users
        .get_all_profile_answers_id_list(&owner)
        .iter()
        .map(|question_answer_id| self.get_question_by_id(question_answer_id))
        .collect()
    }

    // get profile watch list 
    pub fn get_profile_watch_list(&self, owner: &Principal) -> Vec<Question> {
        self.users
        .get_all_profile_watch_list(&owner)
        .iter()
        .map(|watch_list_id| self.get_question_by_id(watch_list_id))
        .collect()
    }

    // get user profile
    pub fn get_user_profile(&self, owner: &Principal) -> Profile {
        self.users.get_user_profile(owner)
    } 

    // get user followers set
    pub fn get_profile_followers_set(&self, owner: &Principal) -> Vec<Profile> {
        let followers_set = self.follow_state.get_followers_set(owner);
        followers_set
        .iter()
        .map(|user_pid| self.get_user_profile(&Principal::from_text(user_pid).unwrap()))
        .collect()

    }
    
    // get user followings set
    pub fn get_profile_followings_set(&self, owner: &Principal) -> Vec<Profile> {
        let followings_set = self.follow_state.get_followings_set(owner);
        followings_set
        .iter()
        .map(|user_pid| self.get_user_profile(&Principal::from_text(user_pid).unwrap()))
        .collect()

    }
    
    pub fn check_profile_follow_status(&self, owner: &Principal, to: &Principal) -> FollowStatus {
        self.follow_state.check_profile_follow_status(&owner, &to)
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

