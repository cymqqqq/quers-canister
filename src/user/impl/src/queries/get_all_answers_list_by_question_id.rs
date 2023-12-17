use crate::{read_state, RuntimeState};
// use crate::guards::caller_is_quers_user;
use ic_cdk_macros::query;
use user_canister::get_all_answers_list_by_question_id::{Args, Response::*,*};
// use utils::types::*;

#[query]
fn get_all_answers_list_by_question_id(args: Args) -> Response {
    read_state(|state| get_all_answers_list_by_question_id_impl(&args, state))
}

fn get_all_answers_list_by_question_id_impl(args: &Args, state: &RuntimeState) -> Response {
    let answer_result = state.data.get_all_answers_list_by_question_id(&args.question_id);
    Success(SuccessResult{
        answer_list: answer_result,
    })
}