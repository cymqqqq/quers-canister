use candid::{Principal, CandidType};
// use anyhow::Result;
use serde::*;
// use ic_ledger_types::{AccountIdentifier};
use std::collections::HashMap;
use utils::types::*;


#[derive(Debug, Clone, CandidType, Deserialize, Serialize)]
pub struct UserIndex {
    pub profile: HashMap<Principal, Profile>,
}

impl UserIndex {
    pub fn new() -> Self {
        Self { profile: HashMap::new(), }
    }

    pub fn set_user_profile(&mut self, profile: Profile) {
        let principal = Principal::from_text(&profile.owner).unwrap();
        self.profile.insert(principal, profile);

    }
    pub fn get_user_profile(&self, principal: &Principal) -> Profile {
        match self.profile.get(principal) {
            Some(profile) => profile.clone(),
            None => Profile::default(),
        }
    }

    pub fn set_user_principal(&mut self, principal: &Principal) {
        let mut profile_map = self.get_user_profile(&principal);
        profile_map.set_user_principal(principal);
        self.profile.insert(*principal, profile_map);

    }

    pub fn update_user_description(&mut self, owner: &Principal, description: &String) {
        let mut profile_map = self.get_user_profile(&owner);
        profile_map.update_profile_description(description);
        self.profile.insert(*owner, profile_map);
    }

    pub fn update_user_tvl(&mut self, owner: &Principal, tvl: &u32) {
        let mut profile_map = self.get_user_profile(owner);
        profile_map.update_profile_tvl(tvl);
        self.profile.insert(*owner, profile_map);
    }

    pub fn update_user_holders(&mut self, owner: &Principal, holders: &u32) {
        let mut profile_map = self.get_user_profile(owner);
        profile_map.update_profile_holders(holders);
        self.profile.insert(*owner, profile_map);
    }

    pub fn update_user_holding(&mut self, owner: &Principal, holding: &u32) {
        let mut profile_map = self.get_user_profile(owner);
        profile_map.update_profile_holding(holding);
        self.profile.insert(*owner, profile_map);
    }

    pub fn update_username(&mut self, owner: &Principal, username: &String) {
        let mut profile_map = self.get_user_profile(owner);
        profile_map.update_profile_username(&username);
        self.profile.insert(*owner, profile_map);
    }
    
    pub fn update_name(&mut self, owner: &Principal, name: &String) {
        let mut profile_map = self.get_user_profile(owner);
        profile_map.update_profile_name(&name);
        self.profile.insert(*owner, profile_map);
    }

    pub fn update_user_ticket(&mut self, owner: &Principal, tickets: &u32) {
        let mut profile_map = self.get_user_profile(owner);
        profile_map.update_profile_tickets(&tickets);
        self.profile.insert(*owner, profile_map);
    }

    pub fn update_user_question_id_list(&mut self, owner: &Principal, question_id: &String) {
        let mut profile_map = self.get_user_profile(&owner);
        profile_map.add_profile_question_id(&question_id);
        self.profile.insert(*owner, profile_map);
    }

    pub fn update_user_answer_id_list(&mut self, owner: &Principal, question_answer_id: &String) {
        let mut profile_map = self.get_user_profile(&owner);
        profile_map.add_profile_answer_id(&question_answer_id);
        self.profile.insert(*owner, profile_map);
    }

    pub fn update_user_watch_list(&mut self, owner: &Principal, question_id: &String) {
        let mut profile_map = self.get_user_profile(&owner);
        profile_map.add_watch_list(&question_id);
        self.profile.insert(*owner, profile_map);
    }

    pub fn get_all_profile_questions_id_list(&self, owner: &Principal) -> Vec<String> {
        self.get_user_profile(&owner)
        .get_profile_question_id_list()
    }

    pub fn get_all_profile_answers_id_list(&self, owner: &Principal) -> Vec<String> {
        self.get_user_profile(&owner)
        .get_all_profile_answer_question_id_list()
    }

    pub fn get_all_profile_watch_list(&self, owner: &Principal) -> Vec<String> {
        self.get_user_profile(&owner)
        .get_profile_watch_list()
    }

}

