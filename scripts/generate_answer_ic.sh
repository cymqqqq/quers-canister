
ICP="rsgzv-imomc-br63p-uyvii-uaw5w-b33wg-u62uc-btu6p-s3l6c-gnrl4-xae"
OWN="wgcif-hsynd-rl4ww-aj6ok-d7a4a-c3wyz-4obd3-zcqk3-ul4ww-s3f73-qae"
question="Hi community, I compiled the rgb-lib with new UDA feature, but the errors come out for some bugs, Does anyone have the same question?"
desc="about BTC, RGB"
question="g5fwl-3aaaa-aaaah-adtlq-cai-2"
answer="I have some advices"
dfx canister --network ic call user add_new_answer \
"record { \
answer_content=\"$answer\";\
answer_pid= principal \"$OWN\"; \
question_id=\"$question\";\
}"

