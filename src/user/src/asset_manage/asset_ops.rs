use crate::asset_manage::asset_info::Asset;
use ic_ledger_types::{AccountIdentifier, AccountBalanceArgs, Tokens, DEFAULT_SUBACCOUNT, MAINNET_LEDGER_CANISTER_ID, account_balance};
use ic_cdk::api::{caller, call::call};


pub const ICP_SUBDIVID: u64 = 100_000_000;


pub async fn get_e8s() -> u64 {
        let tokens = account_balance(
            MAINNET_LEDGER_CANISTER_ID,
            AccountBalanceArgs {
                account: AccountIdentifier::new(&caller(), &DEFAULT_SUBACCOUNT),
            },
        )
        .await
        .expect("call to ledger failed");
        tokens
            .e8s()
            .clone()
    }

pub async fn get_icp() -> u64 {
        get_e8s().await / ICP_SUBDIVID
}

