// use candid::{CandidType, Principal};
// use ic_ledger_types::{
//     AccountIdentifier, 
//     Memo, 
//     Subaccount,
//     Tokens,
//     DEFAULT_SUBACCOUNT
// };
// use serde::{Deserialize, Serialize};

// const ICP_FEE: u128 = 10_000;

// #[derive(CandidType, Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash)]
// pub enum CryptoToken {
//     ICP,
//     Other(String),
// }

// impl CryptoToken {
//     pub fn token_symbol(&self) -> &str {
//         match self {
//             CryptoToken::ICP => "ICP",
//             CryptoToken::Other(symbol) => symbol,
//         }
//     }

//     pub fn decimals(&self) -> Option<u8> {
//         match self {
//             CryptoToken::ICP => Some(8),
//             CryptoToken::Other(_) => None,
//         }
//     }

//     pub fn fee(&self) => Option<u128> {
//         match self {
//             CryptoToken::ICP => Some(ICP_FEE),
//             CryptoToken::Other(_) => None,
//         }
//     }

//     pub fn ledger_canister_id(&self) -> Option<Principal> {
//         match self {
//             CryptoToken::ICP => Some(Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap()),
//             CryptoToken::Other(_) => None,
//         }
//     }
// }

// pub type TransactionHash = [u8; 32];

// #[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
// pub enum CryptoTransaction {
//     Pending(PendingCryptoTransaction),
//     Completed(CompletedCryptoTransaction),
//     Failed(FailedCryptoTransaction),
// }

// #[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
// pub enum PendingCryptoTransaction {
//     ICRC1()
// }

// pub mod icrc1 {
//     use super::*;
//     use candid::Nat;
//     use serde_bytes::ByteBuf;

//     pub type Subaccount = [u8; 32];

//     #[derive(Serialize, CandidType, Deserialize, Clone, Debug, Copy)]
//     pub struct Account {
//         pub owner: Principal,
//         pub subaccount: Option<Subaccount>,
//     }

//     impl From<Principal> for Account {
//         fn from(value: Principal) -> Self {
//             Account {
//                 owner: value,
//                 subaccount: None,
//             }
//         }
//     }

//     #[derive(Serialize, Deserialize, CandidType, Clone, Debug, Default)]
//     #[serde(transparent)]
//     pub struct Memo(pub ByteBuf);

//     impl From<u64> for Memo {
//         fn from(num: u64) -> Self {
//             Self(ByteBuf::from(num.to_be_bytes().to_vec()))
//         }
//     }

//     impl From<ByteBuf> for Memo {
//         fn from(byte_buf: ByteBuf) -> Self {
//             Self(byte_buf)
//         }
//     }

//     impl From<Vec<u8>> for Memo {
//         fn from(vec: Vec<u8>) -> Self {
//             Self::from(ByteBuf::from(vec))
//         }
//     }

//     impl From<Memo> for ByteBuf {
//         fn from(memo: Memo) -> Self {
//             memo.0
//         }
//     }

//     #[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
//     pub enum CryptoAccount {
//         Mint, 
//         Account(Account),
//     }

//     pub type NumTokens = Nat;
//     pub type BlockIndex = Nat;

//     #[derive(CandidType, Deserialize, Clone, Debug)]
//     pub struct TransferArg {
//         #[serde(default)]
//         pub from_subaccount: Option<Subaccount>,
//         pub to: Account,
//         #[serde(default)]
//         pub fee: Option<NumTokens>,
//         #[serde(default)]
//         pub created_at_time: Option<u64>,
//         #[serde(default)]
//         pub memo: Option<Memo>,
//         pub amount: NumTokens,
//     }

//     #[derive(CandidType, Deserialize, Clone, Debug)]
//     pub enum TransferError {
//         BadFee { expected_fee: NumTokens },
//         BadBurn { min_burn_amount: NumTokens },
//         InsufficientFunds { balance: NumTokens },
//         TooOld,
//         CreatedInFuture { ledger_time: u64 },
//         TemporarilyUnavailable,
//         Duplicate { duplicate_of: BlockIndex },
//         GenericError { error_code: Nat, message: String },
//     }

//     #[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
//     pub struct PendingCryptoTransaction {
//         pub ledger: Principal,
//         pub token: CryptoToken,
//         pub amount: u128,
//         pub to: Account,
//         pub fee: u128,
//         pub memo: Option<Memo>,
//         // pub created_at_time: 
//     }

//     #[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
//     pub struct CompletedCryptoTransaction {
//         pub ledger: Principal,
//         pub token: CryptoToken,
//         pub amount: u128,
//         pub from: CryptoAccount,
//         pub to: CryptoAccount,
//         pub fee: u128,
//         pub memo: Option<Memo>,
//         // pub created:
//         pub block_index: u64,
//     }

//     #[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
//     pub struct FailedCryptoTransaction {
//         pub ledger: Principal,
//         pub token: CryptoToken,
//         pub amount: u128,
//         pub fee: u128,
//         pub from: CryptoAccount,
//         pub to: CryptoAccount,
//         pub memo: Option<Memo>,
//         pub error_message: String,
//     }

//     impl From<CompletedCryptoTransaction> for super::CompletedCryptoTransaction {
//         fn from(value: CompletedCryptoTransaction) -> Self {
//             super::CompletedCryptoTransaction::ICRC1(value)
//         }
//     }

//     impl From<FailedCryptoTransaction> for super::FailedCryptoTransaction {
//         fn from(value: FailedCryptoTransaction) -> Self {
//             super::FailedCryptoTransaction::ICRC1(value)
//         }
//     }
// }

// fn u64_from_bytes(bytes: &[u8]) -> u64 {
//     assert!(bytes.len() <= 8);
//     let mut u64_bytes = [0u8; 8];
//     u64_bytes[(8 - bytes.len())..].copy_from_slice(bytes);
//     u64::from_be_bytes(u64_bytes)
// }


