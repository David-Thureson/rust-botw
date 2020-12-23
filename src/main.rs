#![allow(dead_code)]

const FILE_ITEMS: &str = "Breath of the Wild Items.txt";

use botw::*;

pub fn main() {
    println!("\nBotW start\n");

    model_2::model::main();

    //shell::run("Tracker");
    println!("\nBotW done\n");
}