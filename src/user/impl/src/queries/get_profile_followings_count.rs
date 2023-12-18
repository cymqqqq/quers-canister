use crate::{read_state, RuntimeState};
use ic_cdk_macros::query;
use user_canister::get_profile_followings_count::{Args, Response::*,*};

#[query]
fn get_profile_followings_count(args: Args) -> Response {
    read_state(|state| get_profile_followings_count_impl(&args, state))
}

fn get_profile_followings_count_impl(args: &Args, state: &RuntimeState) -> Response {
    let followings = state.data.get_profile_followings_count(&args.owner);
    Success(SuccessResult{
        followings: followings,
    })
}