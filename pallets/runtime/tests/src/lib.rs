#![allow(dead_code)]
#![cfg(test)]

pub mod storage;
pub use storage::{
    account_from, add_secondary_key, fast_forward_blocks, fast_forward_to_block, get_identity_id,
    make_account, make_account_with_balance, make_account_without_cdd, next_block,
    register_keyring_account_with_balance, TestStorage,
};

pub mod ext_builder;
pub use ext_builder::ExtBuilder;

mod asset_pallet;
#[macro_use]
mod asset_test;
mod asset_metadata_test;
mod balances_test;
mod committee_test;
mod compliance_manager_test;
mod contracts_test;
mod corporate_actions_test;
#[macro_use]
mod external_agents_test;
mod fee_details;
mod group_test;
mod identity_test;
mod multisig;
mod nft;
mod pips_test;
mod portfolio;
mod protocol_fee;
mod relayer_test;
mod settlement_pallet;
mod settlement_test;
mod signed_extra;
mod staking;
mod staking_extra_tests;
mod sto_test;
mod transaction_payment_test;
mod transfer_compliance_test;
mod treasury_test;
mod utility_test;
