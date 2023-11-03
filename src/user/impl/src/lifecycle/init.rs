use crate::lifecycle::{init_env, init_state};
use crate::{Data, RuntimeState};
use ic_cdk_macros::init;
use utils::env::Environment;

#[init]
fn init() {
    let env = init_env();
    let data = Data::new();
    init_state(env, data);
}