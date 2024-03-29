use crate::{mutate_state, Data, RuntimeState};
use std::time::Duration;
use utils::env::Environment;
use utils::canister::CanisterEnv;
use utils::raw_rand::get_random_seed;


mod init;
mod pre_upgrade;
mod post_upgrade;

const UPGRADE_BUFFER_SIZE: usize = 1024 * 1024; // 1MB



fn init_env() -> Box<CanisterEnv> {
    ic_cdk_timers::set_timer(Duration::ZERO, reseed_rng);
    Box::default()
}

fn init_state(env: Box<dyn Environment>, data: Data) {
    // let now = env.now();
    let state = RuntimeState::new(env, data);
    crate::init_state(state);
}

fn reseed_rng() {
    ic_cdk::spawn(reseed_rng_inner());

    async fn reseed_rng_inner() {
        let seed = get_random_seed().await;
        mutate_state(|state| state.env = Box::new(CanisterEnv::new(seed)));
    }
}