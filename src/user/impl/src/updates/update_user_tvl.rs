use crate::{mutate_state, RuntimeState};
// use crate::guards::caller_is_quers_user;
use ic_cdk_macros::update;
use user_canister::update_user_tvl::*;

#[update]
fn update_user_tvl(args: Args) -> Response {
    mutate_state(|state| update_user_tvl_impl(&args, state))
}

fn update_user_tvl_impl(args: &Args, state: &mut RuntimeState) -> Response {
    state.data.update_user_tvl(&args.owner, &args.tvl);
    Response::Success
}