use crate::lifecycle::{init_env, init_state, UPGRADE_BUFFER_SIZE};
use crate::memory::{get_upgrade_memory, reset_memory_manager};
use crate::{mutate_state, Data};
use ic_cdk_macros::post_upgrade;
use ic_stable_structures::reader::{BufferedReader, Reader};
use tracing::info;
use utils::serializer;


#[post_upgrade]
fn post_upgrade() {
    let env = init_env();
    let memory = get_upgrade_memory();
    let reader = BufferedReader::new(UPGRADE_BUFFER_SIZE, Reader::new(&memory, 0));
    let data: Data= serializer::deserialize(reader).unwrap();
    init_state(env, data);
    reset_memory_manager();
    info!("Post-upgrade complete");

}