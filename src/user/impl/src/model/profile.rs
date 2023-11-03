use candid::{Principal, CandidType};
use anyhow::Result;
use serde::*;
use ic_ledger_types::{AccountIdentifier};
use std::collections::HashMap;
use std::cell::RefCell;
use utils::types::*;


#[derive(Debug, Clone, CandidType, Deserialize, Serialize)]
pub struct UserIndex {
    pub profile: HashMap<Principal, Profile>,
}

impl UserIndex {
    pub fn new() -> Self {
        Self { profile: HashMap::new(), }
    }

    pub fn get_user_profile(&self, principal: &Principal) -> Profile {
        match self.profile.get(principal) {
            Some(profile) => profile.clone(),
            None => Profile::default(),
        }
    }

    pub fn set_user_principal(&mut self, principal: &Principal) {
        let mut profile_map = match self.profile.get_mut(principal) {
            Some(profile) => profile.clone(),
            None => Profile::default(),
        };
        profile_map.set_user_principal(principal);
        self.profile.insert(*principal, profile_map);

    }

    pub fn update_user_description(&mut self, owner: &Principal, description: &String) {
        let mut profile_map = match self.profile.get_mut(owner) {
            Some(profile) => profile.clone(),
            None => Profile::default(),
        };
        profile_map.update_profile_description(description);
        self.profile.insert(*owner, profile_map);
    }

    pub fn update_user_tvl(&mut self, owner: &Principal, tvl: &u32) {
        let mut profile_map = match self.profile.get_mut(owner) {
            Some(profile) => profile.clone(),
            None => Profile::default(),
        };
        profile_map.update_profile_tvl(tvl);
        self.profile.insert(*owner, profile_map);
    }

    pub fn update_user_holders(&mut self, owner: &Principal, holders: &u32) {
        let mut profile_map = match self.profile.get_mut(owner) {
            Some(profile) => profile.clone(),
            None => Profile::default(),
        };
        profile_map.update_profile_holders(holders);
        self.profile.insert(*owner, profile_map);
    }

    pub fn update_user_followers(&mut self, owner: &Principal, followers: &u32) {
        let mut profile_map = match self.profile.get_mut(owner) {
            Some(profile) => profile.clone(),
            None => Profile::default(),
        };
        profile_map.update_profile_followers(followers);
        self.profile.insert(*owner, profile_map);
    }

    pub fn update_user_holding(&mut self, owner: &Principal, holding: &u32) {
        let mut profile_map = match self.profile.get_mut(owner) {
            Some(profile) => profile.clone(),
            None => Profile::default(),
        };
        profile_map.update_profile_holding(holding);
        self.profile.insert(*owner, profile_map);
    }
    
}

