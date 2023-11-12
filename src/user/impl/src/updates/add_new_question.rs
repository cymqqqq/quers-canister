use crate::{mutate_state, replace_state, RuntimeState};
use crate::guards::caller_is_quers_user;
use ic_cdk_macros::update;
use user_canister::add_new_question::{Args, Response::*,*};

#[update]
fn add_new_question(args: Args) -> Response {
    mutate_state(|state| add_new_question_impl(&args, state))
}

fn add_new_question_impl(args: &Args, state: &mut RuntimeState) -> Response {
    state.data.add_question(
        &args.question_logo,
        &args.question_title,
        &args.question_description,
        &args.question_image,    
        &args.question_asker,
        &args.tags,
    );
    Response::Success
}