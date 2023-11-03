use crate::{read_state, RuntimeState};
// use crate::guards::caller_is_quers_user;
use ic_cdk_macros::query;
use user_canister::get_user_profile::{Args, Response::*,*};
// use crate::model::profile::Profile;
use utils::types::*;
// #[query(guard="caller_is_quers_user")]
#[query]
fn get_user_profile(args: Args) -> Response {
    read_state(|state| get_user_profile_impl(&args, state))
}

fn get_user_profile_impl(args: &Args, state: &RuntimeState) -> Response {
    let profile = state.data.users.get_user_profile(&args.owner);
    Success(SuccessResult{
        profile: profile,
    })
}