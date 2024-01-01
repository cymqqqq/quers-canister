use crate::{mutate_state, RuntimeState};
// use crate::guards::caller_is_quers_user;
use ic_cdk_macros::update;
use user_canister::set_user_profile::*;

#[update]
fn set_user_profile(args: Args) -> Response {
    mutate_state(|state| set_user_profile_impl(&args, state))
}

fn set_user_profile_impl(args: &Args, state: &mut RuntimeState) -> Response {
    state.data.set_user_profile(
        args.owner,
        &args.description, 
        &args.name,
        &args.username,
        &args.profile_image_url,
    );
    Response::Success
}