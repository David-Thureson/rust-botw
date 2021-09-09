#![allow(dead_code)]

const FILE_ITEMS: &str = "Breath of the Wild Items.txt";

use botw::*;

pub fn main() {
    println!("\nBotW start\n");

    // model_3::model::main();
    // model_3::game_record::try_create_events();
    // model_3::command::try_suggest_commands();
    // model_3::sim::test_many_actions();
    parse_one_time::main();

    //shell::run("Tracker");
    println!("\nBotW done\n");
}