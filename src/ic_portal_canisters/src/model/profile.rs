use candid::{Principal, CandidType};

use anyhow::Result;
use serde::*;
use ic_ledger_types::{AccountIdentifier};
use std::collections::HashMap;
use std::cell::RefCell;
thread_local! {
    static PROFILE: RefCell<UserIndex>  = RefCell::new(UserIndex::new());
    // static PROFILE_LIST: RefCell<Vec<Profile>> = RefCell::new(Vec::new());

}

#[derive(Clone, CandidType, Deserialize)]
pub struct UserIndex {
    pub profile: HashMap<Principal, Profile>,
}

impl UserIndex {
    pub fn new() -> Self {
        Self { profile: HashMap::new(), }
    }
}
#[derive(PartialEq, Clone, Serialize, Deserialize, CandidType)]
pub struct Profile {
    pub principal: Principal,
    pub acount_id: String,
    pub tvl: u32,
    pub description: String,
    pub holders: u32,
    pub followers: u32,
    pub holding: u32,
    pub qa_mod: QuesAns,
}

#[derive(Default, Debug, Copy, Clone, Deserialize, CandidType)]
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
                 pid: Principal::anonymous(), 
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
    pub fn update_description(&mut self, desc: &String) {
        self.description = desc.into();
    }


    pub fn update_profile_info(&mut self, nickname: &String, desc: &String) {
        self.nick_name = nickname.into();
        self.description = desc.into();
    }

}

pub fn get_all() -> Vec<Profile> {
    PROFILE.with(|profile|
        profile.borrow()
        .values()
        .cloned()
        .collect::<Vec<Profile>>()
    )

}


pub fn get_specific(pid: Principal) -> Option<Profile> {
    PROFILE.with(|profile|
        profile.borrow().get(&pid).cloned()

    )
}

pub fn set_description(desc: String) -> Result<()> {
    let pid = ic_cdk::api::caller();

    PROFILE.with(|profile| 
        {profile.borrow_mut().get_mut(&pid).and_then(|pro| Some(pro.update_description(desc)))}
        
    );
    Ok(())
}

pub fn set_url(urlink: String) -> Result<()> {
    let pid = ic_cdk::api::caller();
    PROFILE.with(|profile|
        {
            profile.borrow_mut().get_mut(&pid).and_then(|pro| Some(pro.update_url(urlink)))
        }
    );
    Ok(())
}
    
pub fn update(nickname: String, desc: String) -> Result<()> {
    let pid = ic_cdk::api::caller();

    PROFILE.with(|profile|
        {
            profile.borrow_mut().get_mut(&pid).and_then(|pro| Some(pro.update_profile_info(nickname, desc)))
        }
    );
    Ok(())  
}

