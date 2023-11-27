use crate::{read_state, RuntimeState};
// use crate::guards::caller_is_quers_user;
use ic_cdk_macros::query;
use user_canister::view_by_page::{Args, Response::*,*};
use utils::types::*;
// #[query(guard="caller_is_quers_user")]
#[query]
fn view_by_page(args: Args) -> Response {
    read_state(|state| view_by_page_impl(&args, state))
}

fn view_by_page_impl(args: &Args, state: &RuntimeState) -> Response {
    let question_list = state.data.view_by_page(args.page, args.num_of_page);
    Success(SuccessResult{
        question_list: question_list,
    })
}