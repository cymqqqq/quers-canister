ICP="rsgzv-imomc-br63p-uyvii-uaw5w-b33wg-u62uc-btu6p-s3l6c-gnrl4-xae"
OWN="wgcif-hsynd-rl4ww-aj6ok-d7a4a-c3wyz-4obd3-zcqk3-ul4ww-s3f73-qae"
question="Were recorded talks from RustConf '23 never released?"
desc="nothing"
dfx canister --network ic call user add_new_question \
"record {question_title=\"$question\"; \
question_asker= principal \"$OWN\"; \
question_logo=opt \"\"; \
question_description=\"$desc\"; \
tags=vec {\"Rust\"}; \
question_image=opt \"\"; \
}"