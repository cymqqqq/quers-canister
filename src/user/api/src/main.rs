use candid_gen::generate_candid_method;

#[allow(deprecate)]
fn main() {
    generate_candid_method!(user, init_state, query);
    generate_candid_method!(user, get_user_profile, query);

    generate_candid_method!(user, update_user_followers, update);
    generate_candid_method!(user, update_user_holders, update);
    generate_candid_method!(user, update_user_holding, update);

    generate_candid_method!(user, update_user_tvl, update);
    generate_candid_method!(user, set_user_principal, update);
    generate_candid_method!(user, update_profile_description, update);

    candid::export_service!();
    std::print!("{}", __export_service());
}