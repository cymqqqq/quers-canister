use candid_gen::generate_candid_method;

#[allow(deprecate)]
fn main() {
    generate_candid_method!(user, update_user_description, update);

    candid::export_service!();
    std::print!("{}", __export_service());
}