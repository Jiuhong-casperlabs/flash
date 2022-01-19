use crate::Error;
use casper_contract::contract_api::runtime;
use casper_erc20::Address;
use casper_types::{bytesrepr::Bytes, runtime_args, system::CallStackElement, RuntimeArgs, U256};

fn call_stack_element_to_address(call_stack_element: CallStackElement) -> Address {
    match call_stack_element {
        CallStackElement::Session { account_hash } => Address::from(account_hash),
        CallStackElement::StoredSession { account_hash, .. } => {
            // Stored session code acts in account's context, so if stored session wants to interact
            // with an ERC20 token caller's address will be used.
            Address::from(account_hash)
        }
        CallStackElement::StoredContract {
            contract_package_hash,
            ..
        } => Address::from(contract_package_hash),
    }
}

pub fn on_flash_loan(
    lender: Address,
    initiator: Address,
    _token: Address,
    _amount: U256,
    _fee: U256,
    _data: Bytes,
) -> Result<[u8; 32], Error> {
    // require(
    //     initiator == address(this),
    //     "FlashBorrower: Untrusted loan initiator"
    // );

    let mut stacks = runtime::get_call_stack();
    // currect stack
    let thisstack = stacks.pop().unwrap();

    //current address
    let thisaddress = call_stack_element_to_address(thisstack);

    if thisaddress != initiator {
        return Err(Error::FlashBorrowerUntrustedLoanInitiator);
    }

    // require(
    //     msg.sender == address(lender),
    //     "FlashBorrower: Untrusted lender"
    // );
    // lender contract / previous stack
    let previousstack = stacks.pop().unwrap();

    let previousaddress = call_stack_element_to_address(previousstack);

    if previousaddress != lender {
        return Err(Error::FlashBorrowerUntrustedFender);
    }

    // Action about data -> plan to add history
    let string = "ERC3156FlashBorrower.onFlashLoan";
    let bytes = string.as_bytes();

    let callback_success: [u8; 32] = runtime::blake2b(bytes);

    Ok(callback_success)
}

pub fn flash_borrow(lender: Address, token: Address, amount: U256) -> Result<(), Error> {
    // allowance start
    let erc20_package_hash = match token {
        Address::Contract(contractpkhash) => contractpkhash,
        _ => panic!("incorrect token"),
    };

    // current stack:
    let thisstack = runtime::get_call_stack().pop().unwrap();
    let thisaddress = call_stack_element_to_address(thisstack);

    let _allowance: U256 = runtime::call_versioned_contract(
        erc20_package_hash,
        None,
        "allowance",
        runtime_args! {
            "owner" => thisaddress,
            "spender" => lender
        },
    );
    // allowance end

    // invoke entrypoint flashfee of lender contract start
    let lender_package_hash = match lender {
        Address::Contract(contractpkhash) => contractpkhash,
        _ => panic!("error"),
    };

    let _fee: U256 = runtime::call_versioned_contract(
        lender_package_hash,
        None,
        "flash_fee",
        runtime_args! {
            "token"=> token,
            "amount"=> amount,
        },
    );

    // flash fee end

    // repayment
    let _repayment: U256 = amount + _fee;

    // IERC20(token).approve(address(lender), _allowance + _repayment);

    let approve_result: Result<(), Error> = runtime::call_versioned_contract(
        erc20_package_hash,
        None,
        "approve",
        runtime_args! {
            "spender" => lender,
            "amount" => _allowance + _repayment,
        },
    );

    if approve_result.is_err() {
        return Err(Error::ERC20ApproveFailed);
    }

    // lender.flashLoan(this, token, amount, data);
    let result: bool = runtime::call_versioned_contract(
        lender_package_hash,
        None,
        "flash_loan",
        runtime_args! {
            "receiver"=> thisaddress,
            "token" => token,
            "amount"=> amount,
            "data" => "data",
        },
    );

    if !result {
        return Err(Error::FlashBorrowerInitialFailed);
    }
    Ok(())
}
