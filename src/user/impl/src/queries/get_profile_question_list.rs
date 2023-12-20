use crate::{read_state, RuntimeState};
use ic_cdk_macros::query;
use user_canister::get_profile_question_list::{Args, Response::*,*};

#[query]
fn get_profile_question_list(args: Args) -> Response {
    read_state(|state| get_profile_question_list_impl(&args, state))
}

fn get_profile_question_list_impl(args: &Args, state: &RuntimeState) -> Response {
    let question_result = state.data.get_profile_question_list(&args.owner);
    Success(SuccessResult{
        question_list: question_result,
    })
}