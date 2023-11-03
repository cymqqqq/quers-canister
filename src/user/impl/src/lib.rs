use canister_library::canister_state;
use crate::model::profile::UserIndex;
// use candid::Principal;
use serde::{Deserialize, Serialize};
// use std::cell::RefCell;
// use std::collections::{HashMap, HashSet};
use utils::env::Environment;
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
}


impl Data {
    pub fn new() -> Self {
        Self {
            users: UserIndex::new(),
        }
    }
}