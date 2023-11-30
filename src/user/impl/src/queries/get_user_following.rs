use crate::{read_state, RuntimeState};
// use crate::guards::caller_is_quers_user;
use ic_cdk_macros::query;
use user_canister::get_user_following::{Args, Response::*,*};
use utils::types::*;
#[query]
fn get_user_following(args: Args) -> Response {
    read_state(|state| get_user_following_impl(&args, state))
}

fn get_user_following_impl(args: &Args, state: &RuntimeState) -> Response {
    let following = state.data.get_profile_following(&args.owner);
    Success(SuccessResult{
        following: following,
    })
}