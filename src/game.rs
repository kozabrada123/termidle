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
    pub vehicles : Vec<Vehicle>,
}


impl GameStruct {

    // Calculate monee for a duration
    pub fn money_process(&self, duration: Duration) -> u64 {

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
            let upgradedata = self.get_upgrade(*upgrade.0).unwrap();

            // Apply the bonus stacking

            // Manufacturing rate / speed upgrade
            if upgradedata.function == 1 {
                speed_multiplier = speed_multiplier * (1.0 + (upgradedata.multiplier * *upgrade.1 as f64)); // Multiply the curr speed bonus with the base multiplier * the times we've upgraded it
                //println!("Player has upgrade {}, increasing speed by {}x to {}x", upgradedata.name, upgradedata.multiplier * *upgrade.1 as f64, speed_multiplier);
            }

            // Value / price upgrade
            else if upgradedata.function == 2 {
                price_multiplier = price_multiplier * (1.0 + (upgradedata.multiplier * *upgrade.1 as f64)); // Multiply the curr price bonus with the base multiplier * the times we've upgraded it
                //println!("Player has upgrade {}, increasing price by {}x to {}x", upgradedata.name, upgradedata.multiplier * *upgrade.1 as f64, price_multiplier);
            }
        }

        // Apply all our speed bonuses
        output = output * speed_multiplier;

        // Here we should also check if shipping can handle it, but whatever

        //println!("Player made {} products", output);

        // Get the price for one item
        let mut oneprice = self.get_hardware(self.player.currhardware.try_into().unwrap()).unwrap().price as f64;

        //println!("One item by default costs {}$", oneprice);

        // Apply the money bonus per item
        oneprice = oneprice * price_multiplier;

        //println!("Bonuses bring that up to {}$", oneprice);

        // Calculate how much money we get for all those items.
        let money: f64 = output * oneprice;

        //println!("Player then makes {}$", money);

        // Calculate how much we got for all the duration then and truncate that to a u64.
        return (money * duration.as_secs_f64()) as u64;

    }

    // Dumb repetitive shit that I don't know how to do a different way
    // FIXME

    // Checks whether or not we can buy the upgrade.
    // Return (can_buy, upgrade_price, upgrade_tier)
    pub fn can_buy_upgrade(&self, tobuy: Upgrade) -> (bool, u128, u8) {

        // Check if the upgrade is buyable

        let mut upgradetier: u8 = 0;

        // Get our current upgrade tier
        for upgrade in self.player.upgrades.iter() {
            if *upgrade.0 == tobuy.id {
                upgradetier = *upgrade.1;
            }
        }

        // If we have the max ammount of tiers we can't buy it
        if upgradetier >= tobuy.tiers {
            return (false, 0, 0);
        }


        // Calculate the upgrade's price
        let nextprice = tobuy.calculatePrice(upgradetier + 1);

        // Check if we have enough money
        if self.player.balance.value > nextprice {

            // We can buy it
            return (true, nextprice, upgradetier + 1);
        }

        // We don't, we can't buy it
        return (false, 0, 0);
    }

    // Buys an upgrade.
    // Returns whether or not we bought it
    pub fn buy_upgrade(&mut self, tobuy: Upgrade) -> bool {

        // Check if we can buy it

        let canbuy = self.can_buy_upgrade(tobuy.clone());

        if canbuy.0 {
            // We can

            // Buy it
            self.player.balance -= BeegNum{value: canbuy.1, shift: 0};

            // Add it to the player's upgrade list
            self.player.upgrades.insert(tobuy.id, canbuy.2);

            return true;
        }

        return false;
    }


    // Game data loaders
    pub fn load_hardwares() -> Vec<HardwareItem> {

        let mut data = "".to_string();

        // Try all the folders
        match std::fs::read_to_string("./gamedata/Hardwares/main.json") {
            Ok(fdata) => {data = fdata;},
            Err(_e) => {} //println!("Couldn't read ./gamedata/ ..")
        }

        match std::fs::read_to_string("../../gamedata/Hardwares/main.json") {
            Ok(fdata) => {data = fdata;},
            Err(_e) => {} //println!("Couldn't read ../../gamedata/ ..")
        }

        match std::fs::read_to_string("~/termidle/gamedata/Hardwares/main.json") {
            Ok(fdata) => {data = fdata;},
            Err(_e) => {} //println!("Couldn't read ~/termidle/gamedata/ ..")
        }

        // If it's none of them, we'll get an error from serde


        let data: Vec<HardwareItem> = serde_json::from_str(&data).unwrap();

        data
    }

    pub fn load_manufacturers() -> Vec<Manufacturer> {

        let mut data = "".to_string();

        // Try all the folders
        match std::fs::read_to_string("./gamedata/Manufacturers/main.json") {
            Ok(fdata) => {data = fdata;},
            Err(_e) => {} //println!("Couldn't read ./gamedata/ ..")
        }

        match std::fs::read_to_string("../../gamedata/Manufacturers/main.json") {
            Ok(fdata) => {data = fdata;},
            Err(_e) => {} //println!("Couldn't read ../../gamedata/ ..")
        }

        match std::fs::read_to_string("~/termidle/gamedata/Manufacturers/main.json") {
            Ok(fdata) => {data = fdata;},
            Err(_e) => {} //println!("Couldn't read ~/termidle/gamedata/ ..")
        }

        // If it's none of them, we'll get an error from serde

        let data: Vec<Manufacturer> = serde_json::from_str(&data).unwrap();

        data
    }

    pub fn load_shipping() -> Vec<Vehicle> {

        let mut data = "".to_string();

        // Try all the folders
        match std::fs::read_to_string("./gamedata/Shipping/main.json") {
            Ok(fdata) => {data = fdata;},
            Err(_e) => {} //println!("Couldn't read ./gamedata/ ..")
        }

        match std::fs::read_to_string("../../gamedata/Shipping/main.json") {
            Ok(fdata) => {data = fdata;},
            Err(_e) => {} //println!("Couldn't read ../../gamedata/ ..")
        }

        match std::fs::read_to_string("~/termidle/gamedata/Shipping/main.json") {
            Ok(fdata) => {data = fdata;},
            Err(_e) => {} //println!("Couldn't read ~/termidle/gamedata/ ..")
        }

        // If it's none of them, we'll get an error from serde

        let data: Vec<Vehicle> = serde_json::from_str(&data).unwrap();

        data
    }


    pub fn load_upgrades() -> Vec<Vec<Upgrade>> { // Double vec because first one holds tiers, second one hold upgrades

        let mut data = "".to_string();

        // Try all the folders
        match std::fs::read_to_string("./gamedata/Upgrades/main.json") {
            Ok(fdata) => {data = fdata;},
            Err(_e) => {} //println!("Couldn't read ./gamedata/ ..")
        }

        match std::fs::read_to_string("../../gamedata/Upgrades/main.json") {
            Ok(fdata) => {data = fdata;},
            Err(_e) => {} //println!("Couldn't read ../../gamedata/ ..")
        }

        match std::fs::read_to_string("~/termidle/gamedata/Upgrades/main.json") {
            Ok(fdata) => {data = fdata;},
            Err(_e) => {} //println!("Couldn't read ~/termidle/gamedata/ ..")
        }

        // If it's none of them, we'll get an error from serde

        let data: Vec<Vec<Upgrade>> = serde_json::from_str(&data).unwrap();

        data
    }


    // Game data functions
    pub fn get_hardware(&self, id: u64) -> Option<HardwareItem> {

        // Just look through everything lol
        for item in self.hardware.clone() {
            if item.id == id {return Some(item);}
        }

        return None;
        
    }

    pub fn get_manufacturer(&self, id: u64) -> Option<Manufacturer> {

        // Just look through everything lol
        for item in self.manufacturers.clone() {
            if item.id == id {return Some(item);}
        }

        return None;
        
    }
    
    pub fn get_vehicle(&self, id: u64) -> Option<Vehicle> {

        // Just look through everything lol
        for item in self.vehicles.clone() {
            if item.id == id {return Some(item);}
        }

        return None;
        
    }
    

    pub fn get_upgrade(&self, id: u64) -> Option<Upgrade> {

        // Just look through everything lol
        for tier in self.upgrades.clone() {
            for item in tier {
                if item.id == id {return Some(item);}
            }
        }

        return None;
        
    }
    
    /*pub fn get_upgrade_tier(&self, id: u64) -> Option<Vec<Upgrade>> {

        // Just look through everything lol
        for i in 0..self.upgrades.len() {
            if i == id as usize {return Some(self.upgrades[i]);}
        }

        return None;
        
    }*/
    

    // Simple function that gets all the upgrades in vec so we can work with them easier
    pub fn get_all_upgrades() -> Vec<Upgrade> {


        let game = GameStruct::blank();

        let mut all = Vec::<Upgrade>::new();

        // Just iterate through all the upgrade tier
        for tier in game.upgrades.clone() {
            for upgrade in tier {
                all.push(upgrade.clone());
            }
        }

        return all;
    }

    // Test funcs
    pub fn blank() -> GameStruct {
        GameStruct { player: Player::blank(), upgrades: GameStruct::load_upgrades(), hardware: GameStruct::load_hardwares(), manufacturers: GameStruct::load_manufacturers(), vehicles: GameStruct::load_shipping() }
    }

    pub fn from_player(player: Player) -> GameStruct {
        let mut game = GameStruct::blank();
        game.player = player;

        game
    }
}

// ---------------
// Structs
// ---------------

// A Hardware item
#[derive(Serialize, Deserialize, Clone)]
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
#[derive(Serialize, Deserialize, Clone)]
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
#[derive(Serialize, Deserialize, Clone)]
pub struct Upgrade {
    pub id : u64,
    pub name: String,
    pub desc: String,
    pub function: u128, // Id for the upgrade's function
    pub multiplier: f64, // How much one does per upgrade
    pub base_cost: u128, // What the base cost is (Should triple each time you buy it)
    pub tiers: u8, // How many times you can buy the upgrade
}

// So that we can display
impl fmt::Debug for Upgrade {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Upgrade {{ {}, {}, {}, {}, +{}x, {}$ (x2 per level) }}", self.id, self.name, self.desc, self.function, self.multiplier, self.base_cost)
    }
}

impl Upgrade {

    // Calculates the cost base of how many times you bought it
    pub fn calculatePrice(&self, n: u8) -> u128 {
        let mut cost = self.base_cost.clone();

        for i in 0..n {
            cost = cost * 2;
        }

        return cost;
    }
}

// An shipping vehicle.
#[derive(Serialize, Deserialize, Clone)]
pub struct Vehicle {
    id : u64,
    name: String,
    price: u128, // Vehicles price
    capacity: u128, // How many things it can carry
}

// So that we can display
impl fmt::Debug for Vehicle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vehicle {{ {}, {}, {}$, holds {} pcs }}", self.id, self.name, self.price, self.capacity)
    }
}



// ---------------
// Tests!
// ---------------

// There are none
// :(