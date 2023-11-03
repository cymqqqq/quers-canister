use crate::{mutate_state, RuntimeState};
use ic_cdk_macros::query;
use user_canister::init_state::{Response::*, *};
use crate::model::profile::Profile;
#[query]
fn init_state(args: Args) -> Response {
    mutate_state(|state| 
        init_state_impl(&args, state)
    )
}

fn init_state_impl(args: &Args, state: &mut RuntimeState) -> Response {
    let now = state.env.now();
    let profile = Profile::default();
    state.data.users.profile.insert(args.owner, profile);
    Success(SuccessResult {
        principal: args.owner,
    })
}