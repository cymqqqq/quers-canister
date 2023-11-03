owner="kkwoi-3jebw-6qx6z-yeah7-pgtlm-gbqdm-kkvyt-eqgbl-x3vpw-wfu2w-rqe"
dfx canister call user init_state "(record {owner=principal \"$owner\";})"
dfx canister call user get_user_profile "(record {owner=principal \"$owner\";})"
dfx canister call user set_user_principal "(record {owner=principal \"$owner\";})"
dfx canister call user update_profile_description "(record {owner=principal \"$owner\"; description=\"hello world\";})"

dfx canister call user get_user_profile "(record {owner=principal \"$owner\";})"

dfx canister call user update_user_tvl "(record {owner=principal \"$owner\"; tvl=12;})"
dfx canister call user get_user_profile "(record {owner=principal \"$owner\";})"
