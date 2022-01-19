//! Implementation of balances.
use alloc::string::String;

use casper_contract::{contract_api::storage, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{bytesrepr::ToBytes, URef, U256};

use crate::{
    constants::{LOAN_FEE_KEY_NAME, SUPPORT_TOKENS_KEY_NAME},
    detail, Address,
};

/// Creates a dictionary item key for a dictionary item.
#[inline]
fn make_dictionary_item_key(owner: Address) -> String {
    let preimage = owner.to_bytes().unwrap_or_revert();
    // NOTE: As for now dictionary item keys are limited to 64 characters only. Instead of using
    // hashing (which will effectively hash a hash) we'll use base64. Preimage is about 33 bytes for
    // both Address variants, and approximated base64-encoded length will be 4 * (33 / 3) ~ 44
    // characters.
    // Even if the preimage increased in size we still have extra space but even in case of much
    // larger preimage we can switch to base85 which has ratio of 4:5.
    base64::encode(&preimage)
}

pub(crate) fn get_loanfee_uref() -> URef {
    detail::get_uref(LOAN_FEE_KEY_NAME)
}

pub(crate) fn get_tokens_uref() -> URef {
    detail::get_uref(SUPPORT_TOKENS_KEY_NAME)
}

/// Reads a total supply from a specified [`URef`].
pub(crate) fn read_supported_tokens_from(uref: URef) -> Vec<Address> {
    storage::read(uref).unwrap_or_revert().unwrap_or_revert()
}

/// Writes token balance of a specified account into a dictionary.
pub(crate) fn write_loan_fee_to(loan_fee_uref: URef, address: Address, loan_fee: U256) {
    let dictionary_item_key = make_dictionary_item_key(address);
    storage::dictionary_put(loan_fee_uref, &dictionary_item_key, loan_fee);
}

/// If a given account does not have balances in the system, then a 0 is returned.
pub(crate) fn read_loan_fee_from(loan_fee_uref: URef, address: Address) -> U256 {
    let dictionary_item_key = make_dictionary_item_key(address);

    storage::dictionary_get(loan_fee_uref, &dictionary_item_key)
        .unwrap_or_revert()
        .unwrap_or_default()
}
