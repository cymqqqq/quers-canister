use crate::{read_state, RuntimeState};
use ic_cdk_macros::query;
use user_canister::get_profile_followers_count::{Args, Response::*,*};

#[query]
fn get_profile_followers_count(args: Args) -> Response {
    read_state(|state| get_profile_followers_count_impl(&args, state))
}

fn get_profile_followers_count_impl(args: &Args, state: &RuntimeState) -> Response {
    let followers = state.data.get_profile_followers_count(&args.owner);
    Success(SuccessResult{
        followers: followers,
    })
}