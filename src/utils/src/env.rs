use candid::Principal;
use rand::rngs::StdRng;

pub type TimestampMillis = u64;
pub type TimestampNanos = u64;
pub type Milliseconds = u64;
pub type CanisterId = Principal;
pub type Cycles = u128;
pub trait Environment {
    fn now_nanos(&self) -> TimestampNanos;
    fn caller(&self) -> Principal;
    fn canister_id(&self) -> CanisterId;
    fn cycles_balance(&self) -> Cycles;
    fn rng(&mut self) -> &mut StdRng;

    fn now(&self) -> TimestampMillis {
        self.now_nanos() / 1_000_000
    }
}