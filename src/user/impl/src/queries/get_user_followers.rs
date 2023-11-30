use crate::{read_state, RuntimeState};
// use crate::guards::caller_is_quers_user;
use ic_cdk_macros::query;
use user_canister::get_user_followers::{Args, Response::*,*};
use utils::types::*;
#[query]
fn get_user_followers(args: Args) -> Response {
    read_state(|state| get_user_followers_impl(&args, state))
}

fn get_user_followers_impl(args: &Args, state: &RuntimeState) -> Response {
    let followers = state.data.get_profile_followers(&args.owner);
    Success(SuccessResult{
        followers: followers,
    })
}