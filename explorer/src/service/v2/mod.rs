use serde::{Deserialize, Serialize};

pub mod asset;
pub mod block;
pub mod claim;
pub mod delegation;
pub mod error;
pub mod evm_to_native;
pub mod native_to_evm;
pub mod other;
pub mod transaction;
pub mod undelegation;

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub enum TransactionType {
    Native,
    Evm,
    HideAssetType,
    HideAssetAmount,
    HideAssetTypeAndAmount,
    AbarToBar,
    AbarToAbar,
    BarToAbar,
    NativeToEVM,
    EVMToNative,
    Staking,
    UnStaking,
    Claim,
    DefineAsset,
    IssueAsset,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryResult<T> {
    pub total: i64,
    pub page: i32,
    pub page_size: i32,
    pub data: T,
}
