use crate::{mutate_state, replace_state, RuntimeState};
use crate::guards::caller_is_quers_user;
use ic_cdk_macros::update;
use user_canister::update_profile_description::{Args, Response::*,*};

// #[update(guard="caller_is_quers_user")]
#[update]
fn update_profile_description(args: Args) -> Response {
    mutate_state(|state| update_profile_description_impl(&args, state))
}

fn update_profile_description_impl(args: &Args, state: &mut RuntimeState) -> Response {
    state.data.update_user_description(&args.owner, &args.description);
    Response::Success
}