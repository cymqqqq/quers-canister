use std::cell::RefCell;
use std::collections::HashMap;
use anyhow::Result;
use candid::{CandidType, Principal};
use serde::*;
thread_local! {
    static FRIENDS: RefCell<HashMap<Principal,Vec<NewFriend>>>  = RefCell::new(HashMap::new());
}
#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct  NewFriend {
    pub url: String,
    pub pid: Principal,
    pub nick_name: String,
}
/* 
pub fn add(pid: Principal, name: String) -> Result<()> {
    FRIENDS.with(|friends| {

        friends.borrow_mut().entry(pid).or_insert(vec![
            NewFriend {
                pid,
                nick_name: name,
                url: "".into()
            }
        ])
    });

    Ok(())
}

pub fn all(pid: Principal) -> Vec<NewFriend> {
    if let Some(&friends) = FRIENDS.with(|friends| friends.get_mut().get(&pid)) {
        friends
    } else {
        vec![]
    }
}

pub fn delete(pid: Principal, f_pid: Principal) -> Result<()> {
    FRIENDS.with(|friends| {
    if let Some(friends) = friends.get_mut().get(&pid) {
        if let Some(index) = friends.iter().position(|v| v.pid == f_pid) {
            Ok(friends.remove(index))
        } else {
            return Err("no friend is exits")
        }
    } else {
        Err("no found any friend")
    }
    });
    Ok(())
}
*/