use crate::{read_state, RuntimeState};
// use crate::guards::caller_is_quers_user;
use ic_cdk_macros::query;
use user_canister::get_question_votes_by_id::{Args, Response::*,*};
// use utils::types::*;

#[query]
fn get_question_votes_by_id(args: Args) -> Response {
    read_state(|state| get_question_votes_by_id_impl(&args, state))
}

fn get_question_votes_by_id_impl(args: &Args, state: &RuntimeState) -> Response {
    let votes_result = state.data.get_question_votes_by_id(&args.question_id);
    Success(SuccessResult{
        votes: votes_result,
    })
}