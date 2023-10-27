mod profile;
use crate::profile::{Profile,
    register,
    update,
    get, 
    get_all,
    get_specific, 
    get_url, 
    set_description,
    set_url};
use ic_cdk::{api, Principal, storage};

use ic_cdk_macros::*;
use serde::*;
use candid::{CandidType, candid_method};

use ic_ledger_types::{AccountIdentifier};


thread_local! {
    static PROFILE_LIST: RefCell<Vec<Profile>> = RefCell::new(Vec::new());

}

#[derive(Deserialize, CandidType, Serialize, Clone)]
pub struct UpgradePayload {
    pub profile_list: Vec<Profile>,
    //pub friend_list: Vec<NewFriend>,
}

#[query]
#[candid_method(query)]
async fn get_all_emails() -> Vec<(String, String)> {
    profile::get_all_emails()
}

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

//To set the image url of the current user
#[update]
#[candid_method(update)]
async fn set_profile_url(url: String) {
    set_url(url);
}

//To obtain the profile information of the current user
#[query]
#[candid_method(query)]
async fn get_profile() -> Option<Profile> {
    get()
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

//To obtain the image url of the current user
#[query]
#[candid_method(query)]
async fn get_profile_url() -> Option<String> {
    get_url()
}

//To modify the profile information that includes nickname and description of 
// the current user.
#[update]
#[candid_method(update)]
async fn update_profile(nickname: String, description: String) {
    update(nickname, description);
}
/* 
//Add friend information from the profile contract into the friend list of the curent user
#[update]
#[candid_method(update)]
async fn add_friend(pid: Principal, name: String) {
    add(pid, name);
}

//To display the friend list of the current user
#[query]
#[candid_method(query)]
async fn display() -> Vec<NewFriend> {
    let pid = ic_cdk::api::caller();
    all(pid)
}

//To delete the specific friend information of the current user
#[update]
#[candid_method(update)]
async fn delete_friend(fid: Principal) {
    let pid = ic_cdk::api::caller();
    delete(pid, fid);
}
*/
//To obtain the nums of the ICP of the current user
#[update]
#[candid_method(update)]
async fn balance() -> u64 {
    let icp = get_icp().await;
    icp
}

pub fn get_account(mut account: Vec<AccountResult>) -> Vec<String> {
    let mut account_set = Vec::new();
    for item in &mut account {
        account_set.push(item.get_account_id().clone());
    }
    account_set
}

#[update]
#[candid_method(update)]
async fn get_yumi_nft_single(pid: String, name: String) -> Data{

    let ppid = Principal::from_text(pid.to_string()).unwrap();
    let sub_acc = ic_ledger_types::Subaccount([0u8; 32]);
    let aid = AccountIdentifier::new(&ppid, &sub_acc);
     
    let mut data = Data::new();
    //let name = String::from("dyijn-laaaa-aaaah-abkva-cai");

    
    let item_id = ic_cdk::export::Principal::from_text(name.clone()).unwrap();

    let item_info: Result<(NFTResult,), _>  = ic_cdk::api::call::call(item_id.clone(), "tokens_ext", (aid.to_string().clone(), )).await; 
    let item_res = &item_info.as_ref().unwrap().0;
    //let token_id = item_res.get_token_idx_set();
    let token_nft = item_res.decode_nft();
    data.insert_account_id(aid.to_string().clone());

    data.insert(vec![], token_nft.clone());
    //serde_json::to_string(&data.clone()).unwrap()
    data
    
}
/* 
//To obtain the canister list of the Yumi platform
#[query]
async fn list_cid() -> String {
    let mut list_id = ListId::new();
    let mut vec = Vec::new();
    let name = String::from("ajy76-hiaaa-aaaah-aa3mq-cai");
    let canister_id = ic_cdk::export::Principal::from_text(name.clone()).unwrap();
    let canister_info: Result<(Vec<String>,), _>  = ic_cdk::api::call::call(canister_id, "listCollections", ()).await;
    for item in canister_info.unwrap().0.iter() {
        let item_id = ic_cdk::export::Principal::from_text(item.clone()).unwrap();
        let account_id: Result<(Vec<AccountResult>,), _> = ic_cdk::api::call::call(item_id.clone(), "getRegistry", ()).await;  
        let account_set = get_account(account_id.unwrap().0.clone());
        list_id.insert_id(item.clone(), account_set.clone());
        vec.push(list_id.clone());
    }
    serde_json::to_string(&vec).unwrap()
}

//To obtain the canister, account id pair with specific acount id
#[update]
async fn get_cid(find_id: String) -> String {
    let mut hashmap: HashMap<String, Vec<String>> = HashMap::new();
    let mut _c_id = String::new();
    let mut _a_id = String::new();
    let name = String::from("ajy76-hiaaa-aaaah-aa3mq-cai");
    let canister_id = ic_cdk::export::Principal::from_text(name.clone()).unwrap();
    let canister_info: Result<(Vec<String>,), _>  = ic_cdk::api::call::call(canister_id, "listCollections", ()).await;
    
    for item in canister_info.unwrap().0.iter() {
        let item_id = ic_cdk::export::Principal::from_text(item.clone()).unwrap();
        let account_id: Result<(Vec<AccountResult>,), _> = ic_cdk::api::call::call(item_id.clone(), "getRegistry", ()).await;  
        let account_set = get_account(account_id.unwrap().0.clone());
        hashmap.insert(item.clone(), account_set.clone());
    }

    for (cid, aid) in &hashmap {
        for item in aid {
            if item.to_string() == find_id {
                _a_id = item.to_string().clone();
                _c_id = cid.to_string().clone();

            } 
        }

    }
    format!("{:?} {:?}", _a_id.clone(), _c_id.clone()) 
}


//To obtain all the nft information of the Yumi platform
#[update]
async fn listnft() -> String {
    let mut final_vec: Vec<(String, String)> = Vec::new();
    let mut data = Data::new();
    let mut _c_id = String::new();
    let mut _a_id = String::new();
    let mut hashmap: HashMap<String, Vec<String>> = HashMap::new();
    let name = String::from("ajy76-hiaaa-aaaah-aa3mq-cai");
    let canister_id = ic_cdk::export::Principal::from_text(name.clone()).unwrap();
    let canister_info: Result<(Vec<String>,), _>  = ic_cdk::api::call::call(canister_id, "listCollections", ()).await;
    

    for item in canister_info.unwrap().0.iter() {
        let item_id = ic_cdk::export::Principal::from_text(item.clone()).unwrap();
        let account_id: Result<(Vec<AccountResult>,), _> = ic_cdk::api::call::call(item_id.clone(), "getRegistry", ()).await;  
        let account_set = get_account(account_id.unwrap().0.clone());
        hashmap.insert(item.clone(), account_set.clone());
    }

    
    for (cid, aid) in &hashmap {
        for item in aid {
                _c_id = cid.to_string().clone();
                _a_id = item.to_string().clone();
                final_vec.push((_c_id.clone(), _a_id.clone()));
            
        }
    }

    for (cid, aid) in &final_vec {
        let single_canister_id = ic_cdk::export::Principal::from_text(cid.clone()).unwrap();

        let item_info: Result<(NFTResult,), _>  = ic_cdk::api::call::call(single_canister_id.clone(), "tokens_ext", (aid.clone(), )).await; 
        let item_res = item_info.unwrap().0;
        
        let token_id = item_res.get_token_idx_set();
        let token_nft = item_res.decode_nft();
        data.insert_account_id(_a_id.clone());

        data.insert(token_id.clone(), token_nft.clone());
    }
    let data_res = serde_json::to_string(&data).unwrap();
    
    data_res
}


#[update]
async fn get_yumi_nft_by_2d_img(pid: String) -> Data{
    let ppid = Principal::from_text(&pid.to_string()).unwrap();
    let name = String::from("ajy76-hiaaa-aaaah-aa3mq-cai");
    let mut data = Data::new();
    let canister_id = ic_cdk::export::Principal::from_text(name.clone()).unwrap();
    let canister_info: Result<(Vec<String>,), _>  = ic_cdk::api::call::call(canister_id, "listCollections", ()).await;
    
    let mut hashmap: HashMap<String, Vec<String>> = HashMap::new();
    let mut final_vec: Vec<(String, String)> = Vec::new();
    

    let sub_acc = ic_ledger_types::Subaccount([0u8; 32]);
    let final_id = AccountIdentifier::new(&ppid, &sub_acc);
    
    for item in canister_info.as_ref().unwrap().0.iter() {
        let item_id = ic_cdk::export::Principal::from_text(item).unwrap();
        let account_id: Result<(Vec<AccountResult>,), _> = ic_cdk::api::call::call(item_id.clone(), "getRegistry", ()).await;  
        let account_set = get_account(account_id.unwrap().0.clone());
        hashmap.insert(item.to_string(), account_set);
    }
    

    for (cid, aid) in &hashmap {
        for item in aid {
            if item.to_string() == final_id.to_string() {
                final_vec.push((cid.to_string(), item.to_string()));
            } 
        }
    }


    for (cid, aid) in final_vec {
        let single_canister_id = ic_cdk::export::Principal::from_text(cid).unwrap();

        let item_info: Result<(NFTResult,), _>  = ic_cdk::api::call::call(single_canister_id, "tokens_ext", (aid.to_string(), )).await; 
        let item_res = &item_info.as_ref().unwrap().0;
        let token_id = item_res.get_token_idx_set();
        let token_nft = item_res.decode_nft();
        data.insert_account_id(aid.to_string().clone());

        data.insert(token_id.clone(), token_nft.clone());
    }
    data
    
}





/* 
#[update]
async fn save_cid(pid: String, cid: String) -> String{
    CANISTER_LIST.with(|canisterlist| 
        canisterlist.borrow_mut()
                    .save(pid, cid)
    )
}
*/
#[update]
async fn list_ids(pid: String) -> Vec<String>{
    let ppid = Principal::from_text(&pid.to_string()).unwrap();
    let name = String::from("ajy76-hiaaa-aaaah-aa3mq-cai");
    let canister_id = ic_cdk::export::Principal::from_text(name.clone()).unwrap();
    let canister_info: Result<(Vec<String>,), _>  = ic_cdk::api::call::call(canister_id, "listCollections", ()).await;
    
    let mut hashmap: HashMap<String, Vec<String>> = HashMap::new();
    let mut final_vec: Vec<String> = Vec::new();
    

    let sub_acc = ic_ledger_types::Subaccount([0u8; 32]);
    let final_id = AccountIdentifier::new(&ppid, &sub_acc);
    
    for item in canister_info.as_ref().unwrap().0.iter() {
        let item_id = ic_cdk::export::Principal::from_text(item).unwrap();
        let account_id: Result<(Vec<AccountResult>,), _> = ic_cdk::api::call::call(item_id.clone(), "getRegistry", ()).await;  
        let account_set = get_account(account_id.unwrap().0.clone());
        hashmap.insert(item.to_string(), account_set);
    }
    

    for (cid, aid) in &hashmap {
        for item in aid {
            if item.to_string() == final_id.to_string() {
                final_vec.push(cid.to_string());   
            } 
        }
    } 
    final_vec
}
/* 
#[update]
async fn get_yumi_nft() -> Vec<JsonInfo> {
    NFTLIST.with(|nftlist| 
        nftlist.borrow()
                .get_nft()
    )
}

#[update]
async fn get_yumi_nft_by_pid(pid: String) -> Vec<JsonInfo> {
    NFTLIST.with(|nftlist| 
        nftlist.borrow()
                .get_nft_by_pid(pid.clone())
    )
}*/
*/


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

#[post_upgrade]
fn post_upgrade() {
    let (down,): (UpgradePayload,) = storage::stable_restore().unwrap();

    /*FRIEND_LIST.with(|friend| {
        *friend.borrow_mut() = down.friend_list.clone();
    });*/
    PROFILE_LIST.with(|profile| {
        *profile.borrow_mut() = down.profile_list.clone();
    });
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
