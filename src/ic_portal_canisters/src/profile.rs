use candid::{Principal, CandidType};

use anyhow::Result;
use serde::*;
use ic_ledger_types::*;
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
    pub url: String,
    pub principal: Principal,
    pub tvl: u32,
    pub nick_name: String,
    pub description: String,
    pub holders: u32,
    pub followers: u32,
    pub holding: u32,
}


impl Default for Profile {
    fn default() -> Self {
        Self { 
                url: "".into(),
                 pid: Principal::anonymous(), 
                 aid: "".into(), 
                 nick_name: "".into(), 
                 description: "".into(), 
                 level: "".into(), 
                 state: "".into(),
                 email: "".into(),
            }
    }
}

impl Profile {
    pub fn update_description(&mut self, desc: &String) {
        self.description = desc.into();
    }

    pub fn update_url(&mut self, urlink: &String) {
        self.url = urlink.into();
    }

    pub fn update_profile_info(&mut self, nickname: &String, desc: &String) {
        self.nick_name = nickname.into();
        self.description = desc.into();
    }

    pub fn get_url(&self) -> String {
        self.url
    }
}

pub fn register(
        nick_name: &String,
    ) -> Result<()>{

        let pid = ic_cdk::api::caller();
        let sub_acc = ic_ledger_types::Subaccount([0u8; 32]);
        let aid = AccountIdentifier::new(&pid, &sub_acc);
        
        let profile_ = Profile {
            url: "".into(),
            pid: pid,
            aid: aid.to_string(),
            nick_name: nick_name.into(),
            description: "".into(),
            level: "".into(),
            state: "".into(),
            email: "".into(),
        };
        PROFILE.with(|profile| {

            profile.borrow_mut().insert(pid, profile_)
        });
    Ok(())      
}

pub fn get_all() -> Vec<Profile> {
    PROFILE.with(|profile|
        profile.borrow()
        .values()
        .cloned()
        .collect::<Vec<Profile>>()
    )

}


pub fn get_all_emails() -> Vec<(String, String)> {
    let mut profile_vec = Vec::new();
    PROFILE.with(|profile| {
        profile.borrow()
        .iter()
        .for_each(|(_, i)| profile_vec.push((i.nick_name.clone(), i.email.clone())));
        profile_vec

    })
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

pub fn get() -> Option<Profile> {
    let pid = ic_cdk::api::caller();
    PROFILE.with(|profile|
        profile.borrow().get(&pid).cloned()
    )
}

pub fn get_url() -> Option<String> {
    let pid = ic_cdk::api::caller();
    PROFILE.with(|profile|
        profile.borrow().get(&pid).and_then(|pro| Some(pro.get_url()))
    )
}
