#![no_main]

use casper_types::{U256, bytesrepr::Bytes};
use flashborrower;
use casper_contract::contract_api::{self, runtime};
use casper_erc20::Address;

#[no_mangle]
fn on_flash_loan () {
    let initiator:Address = runtime::get_named_arg("initiator");
    let token:Address = runtime::get_named_arg("token");
    let amount:U256 = runtime::get_named_arg("amount");
    let fee:U256 = runtime::get_named_arg("fee");
    let data:Bytes = runtime::get_named_arg("data");
    flashborrower::default().on_flash_loan(initiator,
        token,
        amount,
        fee,
        data,)
}
#[no_mangle]
fn call() {

}