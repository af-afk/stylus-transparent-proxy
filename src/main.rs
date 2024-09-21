#![cfg_attr(target_arch = "wasm32", no_main, no_std)]

use stylus_sdk::{
    alloy_primitives::*,
    call::RawCall,
    contract, msg,
    storage::{GlobalStorage, StorageCache},
};

use alloy_sol_macro::sol;

use alloy_sol_types::SolCall;

use hex_literal::hex;

extern crate alloc;

//bytes32(uint256(keccak256('eip1967.proxy.implementation')) - 1)
const STORAGE_LOGIC: U256 = U256::from_be_bytes(hex!(
    "360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc"
));

//bytes32(uint256(keccak256('eip1967.proxy.admin')) - 1)
const STORAGE_ADMIN: U256 = U256::from_be_bytes(hex!(
    "b53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d6103"
));

sol! {
    function upgradeToAndSet(address,address) external;
}

#[no_mangle]
pub unsafe fn mark_used() {
    stylus_sdk::evm::pay_for_memory_grow(0);
    panic!();
}

#[no_mangle]
pub extern "C" fn user_entrypoint(len: usize) -> usize {
    let input = stylus_sdk::contract::args(len);
    // Get the sender, if they match the address of the storage for the admin, then we call
    // upgrade_to_and_set.
    let owner = unsafe { StorageCache::get::<20>(STORAGE_ADMIN, 12) };
    let owner: Address = owner.into();
    // If the owner is either not set, or we're the owner, we allow anyone to call this function.
    if owner.is_zero() || msg::sender() == owner {
        let upgradeToAndSetCall {
            _0: admin,
            _1: logic,
        } = upgradeToAndSetCall::abi_decode(&input, true).unwrap();
        unsafe {
            StorageCache::set::<20>(STORAGE_LOGIC, 12, *logic);
            StorageCache::set::<20>(STORAGE_ADMIN, 12, *admin);
        }
        StorageCache::flush();
        0
    } else {
        // We're not making writes here, so we don't need to flush the cache.
        let logic: Address = unsafe { StorageCache::get::<20>(STORAGE_LOGIC, 12).into() };
        let (s, d) = match RawCall::new_delegate().call(logic, &input) {
            Ok(d) => (0, d),
            Err(d) => (1, d),
        };
        contract::output(&d);
        s
    }
}

#[test]
fn print_storage() {
    dbg!(STORAGE_LOGIC, STORAGE_ADMIN);
}
