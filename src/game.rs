use std::{time::{Duration}, fmt};
use serde::{Deserialize, Serialize};
use crate::types::{nums::*, player::Player};

// Very main struct that has literally everything
// Mostly so that the struct holds everything we need
pub struct GameStruct {
    pub player : Player,
    pub upgrades : Vec<Vec<Upgrade>>,
    pub hardware : Vec<HardwareItem>,
    pub manufacturers : Vec<Manufacturer>,
}

impl GameStruct {

    // Calculate monee for a duration
    pub fn moneyProcess(&mut self, duration: Duration) -> u64 {

        // Our basis for calculations should be for one second.

        // Number of items we make per second.
        let mut output: f64 = 1.0;

        // Get all bonuses from upgrades
        let mut speed_multiplier: f64 = 1.0;

        let mut price_multiplier: f64 = 1.0;

        /*

        Hard coded list of upgrade functions

        0 - test

        1 - Increase Base Manufacturing rate
        2 - Increase Computer Value
        3 - Increase Overclock Rate
        4 - Increase Manufacturer Barter

        5 - Increase Max Overclock Rate

        */

        for upgrade in self.player.upgrades.iter() {
            let upgradedata = self.getUpgrade(*upgrade.0).unwrap();

            // Apply the bonus stacking

            // Manufacturing rate / speed upgrade
            if upgradedata.function == 1 {
                speed_multiplier = speed_multiplier * (1.0 + (upgradedata.multiplier * *upgrade.1 as f64)); // Multiply the curr speed bonus with the base multiplier * the times we've upgraded it
                println!("Player has upgrade {}, increasing speed by {}x to {}", upgradedata.name, upgradedata.multiplier, speed_multiplier);
            }

            // Value / price upgrade
            else if upgradedata.function == 2 {
                price_multiplier = price_multiplier * (1.0 + (upgradedata.multiplier * *upgrade.1 as f64)); // Multiply the curr price bonus with the base multiplier * the times we've upgraded it
                println!("Player has upgrade {}, increasing price by {}x to {}", upgradedata.name, upgradedata.multiplier, price_multiplier);
            }
        }

        // Apply all our speed bonuses
        output = output * speed_multiplier;

        println!("Player made {} products", output);

        // Get the price for one item
        let mut oneprice = self.getHardware(self.player.currhardware.try_into().unwrap()).unwrap().price as f64;

        println!("One item by default costs {}$", oneprice);

        // Apply the money bonus per item
        oneprice = oneprice * price_multiplier;

        println!("Bonuses bring that up to {}$", oneprice);

        // Calculate how much money we get for all those items.
        let money: f64 = output * oneprice;

        println!("Player then makes {}$", money);

        // Calculate how much we got for all the duration then and truncate that to a u64.
        return (money * duration.as_secs_f64()) as u64;

    }


    // Game data loaders
    pub fn loadHardwares() -> Vec<HardwareItem> {

        let mut data = "".to_string();

        // Try all the folders
        match std::fs::read_to_string("./gamedata/Hardwares/main.json") {
            Ok(fdata) => {data = fdata;},
            Err(_e) => println!("Couldn't read ./gamedata/ ..")
        }

        match std::fs::read_to_string("../../gamedata/Hardwares/main.json") {
            Ok(fdata) => {data = fdata;},
            Err(_e) => println!("Couldn't read ../../gamedata/ ..")
        }

        match std::fs::read_to_string("~/termidle/gamedata/Hardwares/main.json") {
            Ok(fdata) => {data = fdata;},
            Err(_e) => println!("Couldn't read ~/termidle/gamedata/ ..")
        }

        // If it's none of them, we'll get an error from serde


        let data: Vec<HardwareItem> = serde_json::from_str(&data).unwrap();

        data
    }

    pub fn loadManufacturers() -> Vec<Manufacturer> {

        let mut data = "".to_string();

        // Try all the folders
        match std::fs::read_to_string("./gamedaa/Manufacturers/main.json") {
            Ok(fdata) => {data = fdata;},
            Err(_e) => println!("Couldn't read ./gamedata/ ..")
        }

        match std::fs::read_to_string("../../gamedata/Manufacturers/main.json") {
            Ok(fdata) => {data = fdata;},
            Err(_e) => println!("Couldn't read ../../gamedata/ ..")
        }

        match std::fs::read_to_string("~/termidle/gamedata/Manufacturers/main.json") {
            Ok(fdata) => {data = fdata;},
            Err(_e) => println!("Couldn't read ~/termidle/gamedata/ ..")
        }

        // If it's none of them, we'll get an error from serde

        let data: Vec<Manufacturer> = serde_json::from_str(&data).unwrap();

        data
    }

    pub fn loadUpgrades() -> Vec<Vec<Upgrade>> { // Double vec because first one holds tiers, second one hold upgrades

        let mut data = "".to_string();

        // Try all the folders
        match std::fs::read_to_string("./gamedaa/Upgrades/main.json") {
            Ok(fdata) => {data = fdata;},
            Err(_e) => println!("Couldn't read ./gamedata/ ..")
        }

        match std::fs::read_to_string("../../gamedata/Upgrades/main.json") {
            Ok(fdata) => {data = fdata;},
            Err(_e) => println!("Couldn't read ../../gamedata/ ..")
        }

        match std::fs::read_to_string("~/termidle/gamedata/Upgrades/main.json") {
            Ok(fdata) => {data = fdata;},
            Err(_e) => println!("Couldn't read ~/termidle/gamedata/ ..")
        }

        // If it's none of them, we'll get an error from serde

        let data: Vec<Vec<Upgrade>> = serde_json::from_str(&data).unwrap();

        data
    }


    // Game data functions
    pub fn getHardware(&self, id: u64) -> Option<&HardwareItem> {

        // Just look through everything lol
        for item in &self.hardware {
            if item.id == id {return Some(item);}
        }

        return None;
        
    }

    pub fn getManufacturer(&self, id: u64) -> Option<&Manufacturer> {

        // Just look through everything lol
        for item in &self.manufacturers {
            if item.id == id {return Some(item);}
        }

        return None;
        
    }
    
    pub fn getUpgrade(&self, id: u64) -> Option<&Upgrade> {

        // Just look through everything lol
        for tier in &self.upgrades {
            for item in tier {
                if item.id == id {return Some(item);}
            }
        }

        return None;
        
    }
    
    pub fn getUpgradeTier(&self, id: u64) -> Option<&Vec<Upgrade>> {

        // Just look through everything lol
        for i in 0..self.upgrades.len() {
            if i == id as usize {return Some(&self.upgrades[i]);}
        }

        return None;
        
    }
    

    // Test funcs
    pub fn blank() -> GameStruct {
        GameStruct { player: Player::blank(), upgrades: GameStruct::loadUpgrades(), hardware: GameStruct::loadHardwares(), manufacturers: GameStruct::loadManufacturers() }
    }

    pub fn fromPlayer(player: Player) -> GameStruct {
        GameStruct { player: player, upgrades: GameStruct::loadUpgrades(), hardware: GameStruct::loadHardwares(), manufacturers: GameStruct::loadManufacturers() }
    }
}

// ---------------
// Structs
// ---------------

// A Hardware item
#[derive(Serialize, Deserialize)]
pub struct HardwareItem {
    id : u64,
    name : String,
    desc : String,
    price : u128,
    manufacturer : u128, // The manufacturer's id
}

// So that we can display
impl fmt::Debug for HardwareItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Hardware {{ {}, {}, {}, {}$ /i, (Made by {}) }}", self.id, self.name, self.desc, self.price, self.manufacturer)
    }
}


// A Manufacturer.
#[derive(Serialize, Deserialize)]
pub struct Manufacturer {
    id : u64,
    name : String,
    desc : String,
}

// So that we can display
impl fmt::Debug for Manufacturer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Manufacturer {{ {}, {}, {} }}", self.id, self.name, self.desc)
    }
}

// An upgrade.
#[derive(Serialize, Deserialize)]
pub struct Upgrade {
    id : u64,
    name: String,
    desc: String,
    function: u128, // Id for the upgrade's function
    multiplier: f64, // How much one does per upgrade
    base_cost: f64, // What the base cost is (Should triple each time you buy it)
    tiers: u8, // How many times you can buy the upgrade
}

// So that we can display
impl fmt::Debug for Upgrade {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Upgrade {{ {}, {}, {}, {}, +{}x, {}$ (x5 per level) }}", self.id, self.name, self.desc, self.function, self.multiplier, self.base_cost)
    }
}





// ---------------
// Tests!
// ---------------

// There are none
// :(