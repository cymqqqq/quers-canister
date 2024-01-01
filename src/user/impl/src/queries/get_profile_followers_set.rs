


use crate::{read_state, RuntimeState};
use ic_cdk_macros::query;
use user_canister::get_profile_followers_set::{Args, Response::*,*};

#[query]
fn get_profile_followers_set(args: Args) -> Response {
    read_state(|state| get_profile_followers_set_impl(&args, state))
}

fn get_profile_followers_set_impl(args: &Args, state: &RuntimeState) -> Response {
    let followers_set = state.data.get_profile_followers_set(&args.owner);
    Success(SuccessResult{
        followers_set: followers_set,
    })
}
