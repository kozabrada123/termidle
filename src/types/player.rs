use std::collections::HashMap;
use crate::types::nums::*;

/*

A struct that holds all data related to the player, inluding but not limited to

- player's bit number
- player's upgrades
- etc etc

*/
pub struct Player {

    pub balance : BeegNum, // the player's balance! Uses the optimized struct

    pub upgrades : HashMap<u64, u8>, // A hashmap telling us all of the player's upgrades.

    // The first u64 the global id of the upgrade, the u8 is how upgraded it is.

    pub currhardware : u128 // The current hardware item id that we are on

}


impl Player {

    // Creates a blank player
    pub fn blank() -> Player {
        Player { balance: BeegNum { value: 0, shift: 0 }, upgrades: HashMap::new(), currhardware: 0 }
    }
}