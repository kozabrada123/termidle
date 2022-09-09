mod types;
mod game;
use crate::types::nums::BeegNum;

fn main() {    
    let mut testgame = game::GameStruct::blank();

    testgame.player.upgrades.insert(0, 5);
    testgame.player.upgrades.insert(1, 5);

    println!("{}$", testgame.moneyProcess(std::time::Duration::from_secs(1)));
}

