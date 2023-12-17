use crate::{mutate_state, RuntimeState};
// use crate::guards::caller_is_quers_user;
use ic_cdk_macros::update;
use user_canister::add_profile_watch_list::*;

#[update]
fn add_profile_watch_list(args: Args) -> Response {
    mutate_state(|state| add_profile_watch_list_impl(&args, state))
}

fn add_profile_watch_list_impl(args: &Args, state: &mut RuntimeState) -> Response {
    // let caller = state.env.caller();
    state.data.add_profile_watch_list(&args.user, &args.question_id);
    // profile.update_profile_description(&args.description);
    Response::Success
}