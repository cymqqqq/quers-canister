use crate::{mutate_state, RuntimeState};
use crate::guards::caller_is_quers_user;
use ic_cdk_macros::update;
use user_canister::update_profile_description::{Args, Response::*,*};

#[update(guard="caller_is_quers_user")]
fn update_profile_description(args: Args) -> Response {
    mutate_state(|state| update_profile_description_impl(&args, state))
}

fn update_profile_description_impl(args: &Args, state: &mut RuntimeState) -> Response {
    let caller = state.env.caller();
    let mut profile = state.data.users.get_user_profile(&caller);
    profile.update_profile_description(&args.description);
    Response::Success
}