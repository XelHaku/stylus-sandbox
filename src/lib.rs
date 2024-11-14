// Only run this as a WASM if the export-abi feature is not set.
#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]
extern crate alloc;
use alloc::string::String;
// Modules and imports
mod erc20;

use alloy_primitives::{Address, U256,Uint};

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
        // mapping(uint8 => Event) events;
        /// Maps users to a mapping of each spender's allowance
        /// 
        uint8 id;
       
        #[borrow]
        Erc20<ArenatonParams> erc20;
    }
           

    //         pub struct Event {
    //     bytes8 id; // can't erase a map
    //     uint256 starttime ;     // can't erase vector of maps

    // }

   

    //           pub struct Player {
    //     uint8 id; // can't erase a map
    //     uint256 starttime ;     // can't erase vector of maps
    // }


}

#[public]
#[inherit(Erc20<ArenatonParams>)]
impl Arenaton {
  pub fn set_id(&mut self, _id: u8) -> Result<(), Erc20Error> {
        // Verifica si el evento ya existe
        // Crea un nuevo evento y lo guarda en el mapeo `events`
        let id = Uint::<8, 1>::from(_id);

        
        self.id.set( id);
        Ok(())
    }





//   pub fn set_event(&mut self, id: [u8; 8], starttime: U256) -> Result<(), Erc20Error> {
//         // Verifica si el evento ya existe
// let __Id:StorageFixedBytes::FixedBytes8 = id.into();
//         // Crea un nuevo evento y lo guarda en el mapeo `events`
//         let new_event = Event { id, starttime };
//         self.events.set(id, new_event);
//         Ok(())
//     }

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