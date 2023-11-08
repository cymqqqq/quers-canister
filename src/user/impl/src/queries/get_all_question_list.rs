use crate::{read_state, RuntimeState};
use ic_cdk_macros::query;
use user_canister::get_all_question_list::{Response::*,*};
// #[query(guard="caller_is_quers_user")]
#[query]
fn get_all_question_list(_args: Args) -> Response {
    read_state(|state| get_all_question_list_impl(state))
}

fn get_all_question_list_impl(state: &RuntimeState) -> Response {
    let question_vec = state.data.get_all_question_list();
    Success(SuccessResult{
        question_list: question_vec,
    })
}