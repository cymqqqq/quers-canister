use crate::{read_state, RuntimeState};
use ic_cdk_macros::query;
use user_canister::get_profile_followings_set::{Args, Response::*,*};

#[query]
fn get_profile_followings_set(args: Args) -> Response {
    read_state(|state| get_profile_followings_set_impl(&args, state))
}

fn get_profile_followings_set_impl(args: &Args, state: &RuntimeState) -> Response {
    let followings_set = state.data.get_profile_followings_set(&args.owner);
    Success(SuccessResult{
        followings_set: followings_set,
    })
}