use crate::{mutate_state, RuntimeState};
// use crate::guards::caller_is_quers_user;
use ic_cdk_macros::update;
use user_canister::add_new_comment::*;

#[update]
fn add_new_comment(args: Args) -> Response {
    mutate_state(|state| add_new_comment_impl(&args, state))
}

fn add_new_comment_impl(args: &Args, state: &mut RuntimeState) -> Response {
    state.data.add_comment(
        &args.question_id,
        &args.answer_pid,
        &args.comment_pid,
        &args.comment_content,  
    );
    Response::Success
}