use crate::{mutate_state, RuntimeState};
use crate::guards::caller_is_quers_user;
use ic_cdk_macros::update;
use user_canister::set_user_principal::{Args, Response::*,*};

// #[update(guard="caller_is_quers_user")]
#[update]
fn set_user_principal(args: Args) -> Response {
    mutate_state(|state| set_user_principal_impl(&args, state))
}

fn set_user_principal_impl(args: &Args, state: &mut RuntimeState) -> Response {
    // let caller = state.env.caller();
    state.data.users.set_user_principal(&args.owner);
    // profile.update_profile_description(&args.description);
    Response::Success
}