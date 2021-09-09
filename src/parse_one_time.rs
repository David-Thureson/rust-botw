use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn main() {
    parse_locations();
}

fn parse_locations() {
    // This uses the JS script from https://www.reddit.com/r/zelda/comments/60z2ho/botw_i_datamined_a_map_of_all_226_discoverable/
    // A typical line is:
    //   {"internal_name":"Location_DeathMountain_Entrance", "display_name":"Maw of Death Mountain", "x":2402.58, "y":-1320.01},
    let file_name = "Breath of the Wild Map Locations for Completion Raw.txt";
    let file = File::open(file_name).unwrap();
    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        if line.starts_with("{\"internal_name\"") {
            let location_name = util::parse::between(&line, "\"display_name\":\"", "\", \"x\"").trim();
            println!("{}", location_name);
        }
    }
}


