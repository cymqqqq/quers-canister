
use ic_cdk::{api, Principal, storage};

use ic_cdk_macros::*;
use serde::*;
use candid::{CandidType, candid_method};

use ic_ledger_types::{AccountIdentifier};

#[update]
#[candid_method(update)]
async fn register_profile(nick_name: String) {
    register(nick_name);
}

//To set description of the current user
#[update]
#[candid_method(update)]
async fn set_profile_description(description: String) {
    set_description(description);
}


#[query]
#[candid_method(query)]
async fn get_profile_specific(pid: Principal) -> Option<Profile> {
    get_specific(pid)
}

//To obtain all the profile information
#[query]
#[candid_method(query)]
async fn get_all_profiles() -> Vec<Profile> {
    get_all()
}


//To modify the profile information that includes nickname and description of 
// the current user.
#[update]
#[candid_method(update)]
async fn update_profile(nickname: String, description: String) {
    update(nickname, description);
}


#[pre_upgrade]
fn pre_upgrade() {
    //let flist = FRIEND_LIST.with(|friend| friend.borrow().clone());
    let plist = profile::get_all();
    let up = UpgradePayload {
        profile_list: plist,
        //friend_list: flist,
    };
    storage::stable_save((up,)).unwrap();
}


#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    ic_cdk::export::candid::export_service!();
    __export_service()
}
#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
    std::print!("{}", export_candid());
}
#[cfg(any(target_arch = "wasm32", test))]
fn main() {}
