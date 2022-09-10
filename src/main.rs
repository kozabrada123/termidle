mod types;
mod game;
use std::sync::Arc;

use crate::types::nums::BeegNum;


fn main() {

    let mut game = game::GameStruct::blank();

    let mut start = std::time::Instant::now();

    let mut prevbalance = game.player.balance.value.clone();

    loop {

        // If it's been a second, process a tick
        if start.elapsed().as_secs_f64() >= 1.0 {


            game.player.balance += BeegNum::new(game.money_process(start.elapsed()) as u128, 0);

            // If we can buy the first upgrade, buy it
            let canbuy = game.can_buy_upgrade(game.get_upgrade(0).unwrap());
            if canbuy.0 {
                
                game.buy_upgrade(game.get_upgrade(0).unwrap());

                let upgrade = game.get_upgrade(0).unwrap();

                println!("Bought Upgrade {} {} for {}$, now gives {}x bonus", upgrade.name, upgrade.desc, canbuy.1, upgrade.multiplier * canbuy.2 as f64);
            } 


            // If we can buy the second upgrade, buy it
            let canbuy = game.can_buy_upgrade(game.get_upgrade(1).unwrap());
            if canbuy.0 {
                
                game.buy_upgrade(game.get_upgrade(1).unwrap());

                let upgrade = game.get_upgrade(1).unwrap();

                println!("Bought Upgrade {} {} for {}$, now gives {}x bonus", upgrade.name, upgrade.desc, canbuy.1, upgrade.multiplier * canbuy.2 as f64);
            }

            // Restart the tick timer
            start = std::time::Instant::now();


        }

        if game.player.balance.value != prevbalance {
            println!("{}$", game.player.balance.value);
        }

        prevbalance = game.player.balance.value;
        
    }
}