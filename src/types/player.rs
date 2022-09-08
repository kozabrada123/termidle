use std::collections::HashMap;
use crate::types as types;

/*

A struct that holds all data related to the player, inluding but not limited to

- player's bit number
- player's upgrades
- etc etc

*/
pub struct Player {

    bits : u128, // the player's bits! Uses the optimized struct

    upgrades : HashMap<u64, u64>, // A hashmap telling us all of the player's upgrades.

    // The first u8 the global id of the upgrade, the second one is how upgraded it is.

}