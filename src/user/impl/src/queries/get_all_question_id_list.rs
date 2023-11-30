use crate::{read_state, RuntimeState};
use ic_cdk_macros::query;
use user_canister::get_all_question_id_list::{Response::*,*};
#[query]
fn get_all_question_id_list(_args: Args) -> Response {
    read_state(|state| get_all_question_id_list_impl(state))
}

fn get_all_question_id_list_impl(state: &RuntimeState) -> Response {
    let question_id_vec = state.data.get_all_question_id_list();
    Success(SuccessResult{
        question_id_list: question_id_vec,
    })
}