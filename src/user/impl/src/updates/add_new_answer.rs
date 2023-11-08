use crate::{mutate_state, replace_state, RuntimeState};
use crate::guards::caller_is_quers_user;
use ic_cdk_macros::update;
use user_canister::add_new_answer::{Args, Response::*,*};

#[update]
fn add_new_answer(args: Args) -> Response {
    mutate_state(|state| add_new_answer_impl(&args, state))
}

fn add_new_answer_impl(args: &Args, state: &mut RuntimeState) -> Response {
    state.data.add_answer(
        &args.question_id,
        &args.answer_pid,
        &args.answer_content,  
    );
    Response::Success
}