// Only run this as a WASM if the export-abi feature is not set.
#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]
extern crate alloc;

// Modules and imports
mod erc20;
mod event;
use event::Event; // Trae la estructura Event al ámbito actual

use alloy_primitives::{Address, U256};
use stylus_sdk::{
    msg,
    prelude::*
};
use crate::erc20::{Erc20, Erc20Params, Erc20Error};
/// Immutable definitions
struct ArenatonParams;
impl Erc20Params for ArenatonParams {
    const NAME: &'static str = "Arenaton";
    const SYMBOL: &'static str = "ATON";
    const DECIMALS: u8 = 18;
}

// Define the entrypoint as a Solidity storage object. The sol_storage! macro
// will generate Rust-equivalent structs with all fields mapped to Solidity-equivalent
// storage slots and types.
sol_storage! {
    #[entrypoint]
    struct Arenaton {
        // Allows erc20 to access Arenaton's storage and make calls
        #[borrow]
        Erc20<ArenatonParams> erc20;
    }

}

#[public]
#[inherit(Erc20<ArenatonParams>)]
impl Arenaton {
    /// Mints tokens
    /// 
    pub fn addEvent(&mut self, _id: u8, _starttime: U256) -> Result<(), Erc20Error> {
        Ok(())
    }


    pub fn mint(&mut self, value: U256) -> Result<(), Erc20Error> {
        self.erc20.mint(msg::sender(), value)?;
        Ok(())
    }

    /// Mints tokens to another address
    pub fn mint_to(&mut self, to: Address, value: U256) -> Result<(), Erc20Error> {
        self.erc20.mint(to, value)?;
        Ok(())
    }

    /// Burns tokens
    pub fn burn(&mut self, value: U256) -> Result<(), Erc20Error> {
        self.erc20.burn(msg::sender(), value)?;
        Ok(())
    }

}