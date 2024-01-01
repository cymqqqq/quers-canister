use crate::lifecycle::{init_env, init_state, UPGRADE_BUFFER_SIZE};
use crate::memory::{get_upgrades_memory};
use crate::{mutate_state, Data, RuntimeState, take_state};
use ic_cdk_macros::post_upgrade;
use ic_stable_structures::reader::{BufferedReader, Reader};
use tracing::info;
use utils::{serializer, stable_memory};
use ic_cdk;

// #[post_upgrade]
// fn post_upgrade() {
//     let env = init_env();
//     let memory = get_upgrades_memory();

//     let reader = stable_memory::get_reader(&memory);
//     // let reader = BufferedReader::new(UPGRADE_BUFFER_SIZE, Reader::new(&memory, 0));
//     let data: Data= serializer::deserialize(reader).unwrap();
//     init_state(env, data);

//     // reset_memory_manager();
//     info!("Post-upgrade complete");

// }



#[post_upgrade]
fn post_upgrade() {
    let env = init_env();
    let (data_store, ) = ic_cdk::storage::stable_restore::<(
        Data,
    )>().unwrap();
    // let state = take_state();
    init_state(env, data_store.clone());

    mutate_state(|state| {
        
        
        // state.data = data_store;
        

        match ic_cdk::storage::stable_restore::<(
            Data,
        )>() {
            Ok((data_store, )) => {
                state.data = data_store.clone();
            }    

            Err(err) => {
               
                format!(
                    "An error occurred when loading from stable memory (post_upgrade): {:?}",
                    err
                );
            }
        }
    });

}
