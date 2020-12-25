#![allow(dead_code)]

const FILE_ITEMS: &str = "Breath of the Wild Items.txt";

use botw::*;

pub fn main() {
    println!("\nBotW start\n");

    // model_3::model::main();
    model_3::game_record::try_create_events();

    //shell::run("Tracker");
    println!("\nBotW done\n");
}