dfx start --background --clean
dfx deploy user

owner="kkwoi-3jebw-6qx6z-yeah7-pgtlm-gbqdm-kkvyt-eqgbl-x3vpw-wfu2w-rqe"
# dfx canister call user init_state "(record {owner=principal \"$owner\";})"
# dfx canister call user get_user_profile "(record {owner=principal \"$owner\";})"
# dfx canister call user set_user_principal "(record {owner=principal \"$owner\";})"
# dfx canister call user update_profile_description "(record {owner=principal \"$owner\"; description=\"hello world\";})"

# dfx canister call user get_user_profile "(record {owner=principal \"$owner\";})"

# dfx canister call user update_user_tvl "(record {owner=principal \"$owner\"; tvl=12;})"
# dfx canister call user get_user_profile "(record {owner=principal \"$owner\";})"

# dfx canister call user update_user_holders "(record {owner=principal \"$owner\"; holders=12;})"
# dfx canister call user get_user_profile "(record {owner=principal \"$owner\";})"

# dfx canister call user update_user_followers "(record {owner=principal \"$owner\"; followers=12;})"
# dfx canister call user get_user_profile "(record {owner=principal \"$owner\";})"


# dfx canister call user update_user_holding "(record {owner=principal \"$owner\"; holding=12;})"
# dfx canister call user get_user_profile "(record {owner=principal \"$owner\";})"

dfx canister call user add_new_question \
"record {question_title=\"hello\"; \
question_asker= principal \"$owner\"; \
question_logo=opt \"\"; \
question_description=\"hello\"; \
tags=vec {\"ETH\"}; \
question_image=opt \"\"; \
}"

# dfx canister call 

dfx canister call user get_all_question_list "record {}"

dfx stop