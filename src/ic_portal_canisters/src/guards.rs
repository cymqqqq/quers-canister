use crate::read_state;

pub fn caller_is_quers_user() -> Result<(), String> {
    if read_state(|state| state.is_caller_quers_user()) {
        Ok(())
    } else {
        Err("Caller is not an Quers user".to_string())
    }
}