

owner="wgcif-hsynd-rl4ww-aj6ok-d7a4a-c3wyz-4obd3-zcqk3-ul4ww-s3f73-qae"
question="GameFi, SocialFi, DeFi, Layer2, and new blockchains – which  sector do you think is more likely to emerge as the ultimate winner in  the next bull market?"
desc="about GameFi, SocialFi, DeFi, Layer2"
# dfx canister --network ic call user init_state "(record {owner=principal \"$owner\";})"
dfx canister --network ic call user get_user_profile "(record {owner=principal \"$owner\";})"
# dfx canister --network ic call user set_user_principal "(record {owner=principal \"$owner\";})"
# dfx canister --network ic call user update_profile_description "(record {owner=principal \"$owner\"; description=\"hello world\";})"

# dfx canister --network ic call user get_user_profile "(record {owner=principal \"$owner\";})"

# dfx canister --network ic call user update_user_tvl "(record {owner=principal \"$owner\"; tvl=12;})"
# dfx canister --network ic call user get_user_profile "(record {owner=principal \"$owner\";})"

# dfx canister --network ic call user update_user_holders "(record {owner=principal \"$owner\"; holders=12;})"
# dfx canister --network ic call user get_user_profile "(record {owner=principal \"$owner\";})"

# dfx canister --network ic call user update_user_followers "(record {owner=principal \"$owner\"; followers=12;})"
# dfx canister --network ic call user get_user_profile "(record {owner=principal \"$owner\";})"


# dfx canister --network ic call user update_user_holding "(record {owner=principal \"$owner\"; holding=12;})"
# dfx canister --network ic call user get_user_profile "(record {owner=principal \"$owner\";})"

# dfx canister --network ic call user add_new_question \
# "record {question_title=\"$question\"; \
# question_asker= principal \"$owner\"; \
# question_logo=opt \"\"; \
# question_description=\"$desc\"; \
# tags=vec {\"GameFi, SocialFi, DeFi, Layer2\"}; \
# question_image=opt \"\"; \
# }"

# 
dfx canister --network ic call user get_all_question_list "record {}"


# http://bitask.club:3001/homepage/viewByPage?sort=1&page=72&limit=10 404 (Not Found)
