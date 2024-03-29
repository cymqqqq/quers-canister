// use utils::serializer;
use ic_cdk_macros::pre_upgrade;
// use ic_stable_structures::writer::{BufferedWriter, Writer};
// use crate::memory::get_upgrade_memory;
// use crate::lifecycle::UPGRADE_BUFFER_SIZE;
use tracing::info;
use crate::{take_state};
use crate::{Data, RuntimeState};

#[pre_upgrade]
fn pre_upgrade() {
    info!("Pre-upgrade starting");
    let state = take_state();
        if let Err(err) = ic_cdk::storage::stable_save::<(
            &Data,
        )>((
            &state.data,
        )) {
            // trap(&format!(
            //     "An error occurred when saving to stable memory (pre_upgrade): {:?}",
            //     err
            // ));
            format!(
                "An error occurred when saving to stable memory (pre_upgrade): {:?}",
                err
            );
        };
}
// #[pre_upgrade]
// fn pre_upgrade() {

//     let state = take_state();
//     let mut memory = get_upgrade_memory();
//     let writer = BufferedWriter::new(UPGRADE_BUFFER_SIZE, Writer::new(&mut memory, 0));
//     serializer::serialize(state.data, writer).unwrap();
// }