use candid_gen::generate_candid_method;

#[allow(deprecate)]
fn main() {
    generate_candid_method!(user, init_state, query);
    generate_candid_method!(user, set_user_profile, update);
    generate_candid_method!(user, get_user_profile, query);

    generate_candid_method!(user, update_user_holders, update);
    generate_candid_method!(user, update_user_holding, update);
    generate_candid_method!(user, update_user_tickets, update);
    generate_candid_method!(user, update_username, update);
    generate_candid_method!(user, update_name, update);
    generate_candid_method!(user, add_profile_watch_list, update);

    generate_candid_method!(user, update_user_tvl, update);
    // generate_candid_method!(user, set_user_principal, update);
    generate_candid_method!(user, update_profile_description, update);

    generate_candid_method!(user, add_new_question, update);
    generate_candid_method!(user, up_vote, update);
    generate_candid_method!(user, down_vote, update);

    generate_candid_method!(user, view_by_page, query);
    generate_candid_method!(user, get_question_by_id, query);
    generate_candid_method!(user, get_all_question_id_list, query);
    generate_candid_method!(user, get_all_question_list, query);
    generate_candid_method!(user, get_question_votes_by_id, query);

    generate_candid_method!(user, add_new_answer, update);
    generate_candid_method!(user, get_all_answers_list_by_question_id, query);

    generate_candid_method!(user, add_new_comment, update);
    generate_candid_method!(user, get_all_comment_list, query);

    // follow operation state
    generate_candid_method!(user, follow_operation, update);
    generate_candid_method!(user, un_follow_operation, update);
    // get follow state operation
    generate_candid_method!(user, get_profile_followers_count, query);
    generate_candid_method!(user, get_profile_followings_count, query);
    // get profile question list
    generate_candid_method!(user, get_profile_question_list, query);
    generate_candid_method!(user, get_profile_answer_question_list, query);
    generate_candid_method!(user, get_profile_watch_list, query);
    // get profile follow state
    generate_candid_method!(user, get_profile_followers_set, query);
    generate_candid_method!(user, get_profile_followings_set, query);
    candid::export_service!();
    std::print!("{}", __export_service());
}