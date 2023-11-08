use crate::{read_state, RuntimeState};
use ic_cdk_macros::query;
use user_canister::get_all_comment_list::{Args, Response::*,*};
// #[query(guard="caller_is_quers_user")]
#[query]
fn get_all_comment_list(args: Args) -> Response {
    read_state(|state| get_all_comment_list_impl(&args, state))
}

fn get_all_comment_list_impl(args: &Args, state: &RuntimeState) -> Response {
    let comment_vec = state.data.get_all_comment_list(&args.answer_pid);
    Success(SuccessResult{
        comment_list: comment_vec,
    })
}