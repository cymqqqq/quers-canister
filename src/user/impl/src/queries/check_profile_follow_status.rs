use crate::{mutate_state, RuntimeState};
// use crate::guards::caller_is_quers_user;
use ic_cdk_macros::query;
use user_canister::check_profile_follow_status::{Args, Response::*,*};

#[query]
fn check_profile_follow_status(args: Args) -> Response {
    mutate_state(|state| check_profile_follow_status_impl(&args, state))
}

fn check_profile_follow_status_impl(args: &Args, state: &mut RuntimeState) -> Response {
    let status = state.data.check_profile_follow_status(&args.owner, &args.to);
    Success(SuccessResult{
        follow_status: status,
    })
}