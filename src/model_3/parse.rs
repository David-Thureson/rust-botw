use std::io::{BufReader, BufRead};
use std::fs::File;
use std::str::FromStr;

use crate::*;
use super::model::*;
use util_rust::parse;

pub const PREFIX_HEADER: &str = "#";
pub const PREFIX_SUBHEADER: &str = ">";
//pub const PREFIX_LF_SUBHEAD: &str = "\n>";
pub const PREFIX_COMMENT: &str = "//";
pub const SUFFIX_TOWN: &str = " (town)";
pub const SUFFIX_SHRINE: &str = " Shrine";
pub const SUFFIX_STABLE: &str = " Stable";
pub const SUFFIX_TOWER: &str = " Tower";
pub const SUFFIX_TECH_LAB: &str = " Tech Lab";
#[allow(dead_code)]
const FILE_NAME_CHARACTERS: &str = "Breath of the Wild Characters.txt";
#[allow(dead_code)]
const FILE_NAME_ITEMS: &str = "Breath of the Wild Items.txt";
#[allow(dead_code)]
const FILE_NAME_INVENTORY: &str = "Breath of the Wild Inventory.txt";
#[allow(dead_code)]
const FILE_NAME_LOCATIONS: &str = "Breath of the Wild Locations.txt";
#[allow(dead_code)]
const FILE_NAME_DOG_TREASURES: &str = "Breath of the Wild Dog Treasures.txt";
#[allow(dead_code)]
const FILE_NAME_SHRINES: &str = "Breath of the Wild Shrines.txt";
#[allow(dead_code)]
const FILE_NAME_QUESTS: &str = "Breath of the Wild Quests.txt";
const AMIIBO: &str = "Amiibo";
const SUFFIX_DLC: &str = " (DLC)";
const SUFFIX_FREE_DLC: &str = " (Free DLC)";
const READ_AMIIBO: bool = false;

pub fn load_characters(model: &mut Model) {
    let file = File::open(FILE_NAME_CHARACTERS).unwrap();
    let reader = BufReader::new(file);
    let mut race = None;
    for line in reader.lines()
            .map(|line| line.unwrap().trim().to_string())
            .filter(|line| !line.is_empty())
            .filter(|line| !line.starts_with(PREFIX_COMMENT)) {
        //rintln!("{}", line);
        if line.starts_with(PREFIX_HEADER) {
            let race_name = line.replace(PREFIX_HEADER, "");
            race = Some(Race::from_str(&race_name.replace(" ", "")).unwrap());
        } else {
            let (first, second) = parse::split_1_or_2(&line, ":");
            let (main, champion, merchant, alive) = if let Some(tags) = second {
                (tags.contains("main"), tags.contains("champion"), tags.contains("merchant"), !tags.contains("dead"))
            } else {
                (false, false, false, true)
            };
            let name = first;
            model.add_character(Character::new(name, &(race.as_ref().unwrap().clone()), main, champion, merchant, alive));
        }
    }
}

/*
pub fn load_shrines(model: &mut Model) {
    let file = File::open(FILE_NAME_SHRINES).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines()
            .map(|line| line.unwrap().trim().to_string())
            .filter(|line| !line.is_empty())
            .filter(|line| !line.starts_with(PREFIX_COMMENT)) {
        let (name, challenge) = parse::split_2(&line, ":");
        let name = name.trim().to_string();
        let challenge = challenge.trim().to_string();
        model.add_location(Location::new(&name, &Region::ShrinePlaceholder, LocationType::new_shrine(&challenge)));
    }
}
*/

pub fn read_file_into_sections(file_name: &str, header_prefix: &str) -> BTreeMap<String, String> {
    let content = fs::read_to_string(file_name).unwrap();
    break_into_sections(content, header_prefix)
}

pub fn break_into_sections(content: String, header_prefix: &str) -> BTreeMap<String, String> {
    let mut map = BTreeMap::new();
    // dbg!(&content.split(&format!("\n{}", header_prefix)));
    // for s in content.split(&format!("\r\n{}", header_prefix)) {
    for split in content.split(header_prefix) {
        // println!("|{}|", &s);


        // let mut section_split = split.splitn(2, "\n");
        // let header = section_split.next().unwrap();
        //rintln!("|{}|", &header);
        // let section_content = section_split.next().unwrap();

        let (header, section_content) = parse::split_2(split, "\n");
        map.insert(header.to_string(), section_content.to_string());
    }
    map
}

pub fn section_to_lines(content: String) -> Vec<String> {
    content.split("\n")
        .map(|line| line.trim())
        .filter(|line| line.len() > 0
            && !line.starts_with(PREFIX_COMMENT)
            && (READ_AMIIBO || !line.contains(AMIIBO)))
        .map(|line| line.replace(SUFFIX_DLC, "").replace(SUFFIX_FREE_DLC, "").to_string())
        .collect()
}

pub fn load_locations(model: &mut Model) {
    let file = File::open(FILE_NAME_LOCATIONS).unwrap();
    let reader = BufReader::new(file);
    let mut region_name = None;
    let mut area_name = None;
    for line in reader.lines()
        .map(|line| line.unwrap().trim().to_string())
        .filter(|line| !line.is_empty())
        .filter(|line| !line.starts_with(PREFIX_COMMENT)) {
        //rintln!("{}", line);
        if line.starts_with(PREFIX_HEADER) {
            let name = line.replace(PREFIX_HEADER, "");
            region_name = Some(name.to_string());
            model.add_location(Location::new(&name, LocationType::Region, None));
        } else if line.starts_with(PREFIX_SUBHEADER) {
            let (name, location_type) = if line.ends_with(SUFFIX_TOWN) {
                (line.replace(SUFFIX_TOWN, ""), LocationType::Town)
            } else {
                (line, LocationType::Area)
            };
            assert!(region_name.is_some());
            let name = name.replace(PREFIX_SUBHEADER, "");
            area_name = Some(name.to_string());
            model.add_location(Location::new(&name, location_type, region_name.clone()));
        } else {
            let (name, location_type) = if line.ends_with(SUFFIX_TOWN) {
                (line.replace(SUFFIX_TOWN, ""), LocationType::Town)
            } else if line.ends_with(SUFFIX_SHRINE) {
                (line, LocationType::Shrine)
            } else if line.ends_with(SUFFIX_TOWER) {
                (line, LocationType::Tower)
            } else if line.ends_with(SUFFIX_TECH_LAB) {
                (line, LocationType::TechLab)
            } else if line.ends_with(SUFFIX_STABLE) {
                (line, LocationType::Stable)
            } else {
                (line, LocationType::Normal)
            };
            assert!(area_name.is_some());
            model.add_location(Location::new(&name, location_type, area_name.clone()));
        }
    }
    add_location_parent_references(model);
    load_dog_treasures(model);
    load_shrines(model);
}

fn add_location_parent_references(model: &mut Model) {
    let mut child_to_parent: BTreeMap<String, String> = BTreeMap::new();
    for parent_location in model.locations.values() {
        let parent_name = parent_location.name.clone();
        for child_name in parent_location.child_locations.iter() {
            assert_ne!(&parent_name, child_name);
            child_to_parent.insert(child_name.to_string(), parent_location.name.clone());
        }
    }
    for child_location in model.locations.values_mut() {
        if let Some(parent_name) = child_to_parent.get(&child_location.name) {
            child_location.parent_location = Some(parent_name.clone())
        }
    }
}

fn load_dog_treasures(model: &mut Model) {
    let file = File::open(FILE_NAME_DOG_TREASURES).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines()
        .map(|line| line.unwrap().trim().to_string())
        .filter(|line| !line.is_empty()) {
        //rintln!("{}", line);

        let (name, treasure) = parse::split_2(&line, ": ");
        model.get_location_mut(name).dog_treasure = Some(treasure.to_string());
    }
    /*
    // Show the locations with dog treasures.
    for location in model
            .locations
            .values()
            .filter(|location_rc| RefCell::borrow(location_rc).dog_treasure.is_some()) {
        dbg!(location);
    }
    */
}

fn load_shrines(model: &mut Model) {
    let file = File::open(FILE_NAME_SHRINES).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines()
        .map(|line| line.unwrap().trim().to_string())
        .filter(|line| !line.is_empty())
        .filter(|line| !line.starts_with(PREFIX_COMMENT)) {

        let (name, challenge) = parse::split_2(&line, ":");
        let name = name.trim();
        let challenge_value = challenge.trim().to_string();

        model.get_shrine_mut(name).challenge = Some(challenge_value);
    }
    for location in model
        .locations
        .values()
        .filter(|location| {
            match location.typ {
                LocationType::Shrine => location.challenge.is_none(),
                _ => false,
            }
        }) {

        dbg!(location);
    }
        //.count();
    //bg!(&missing_challenge_count);
}

pub fn load_quests(model: &mut Model) {
    let file = File::open(FILE_NAME_QUESTS).unwrap();
    let reader = BufReader::new(file);
    let mut quest_type_name = None;
    for line in reader.lines()
        .map(|line| line.unwrap().trim().to_string())
        .filter(|line| !line.is_empty())
        .filter(|line| !line.starts_with(PREFIX_COMMENT)) {

        if line.starts_with(PREFIX_HEADER) {
            quest_type_name = Some(line.replace(PREFIX_HEADER, ""));
        } else {
            match quest_type_name.clone().unwrap().as_ref() {
                "Main" => {
                    model.add_quest(Quest::new_main(&line));
                },
                "Side" => {
                    let (name, notes) = parse::extract_optional(&line, "(", ")");
                    model.add_quest(Quest::new_side(name.trim(), notes.clone()));
                },
                "Shrine" => {
                    let (name, shrine_name) = parse::split_2_trim(&line, ":");
                    //bg!(shrine_name);
                    model.get_shrine_mut(shrine_name).quest = Some(name.to_string());
                    model.add_quest(Quest::new_shrine(name, shrine_name));
                },
                _ => panic!()
            };
        }
    }
}

/*
pub fn load_locations(model: &mut Model) {
    add_location(model, "Akkala Tower", LocationType::Tower, Region::Akkala);
    add_location(model, "Central Tower", LocationType::Tower, Region::Central);
    add_location(model, "DuelingPeaks Tower", LocationType::Tower, Region::DuelingPeaks);
    add_location(model, "Eldin Tower", LocationType::Tower, Region::Eldin);
    add_location(model, "Faron Tower", LocationType::Tower, Region::Faron);
    add_location(model, "Gerudo Tower", LocationType::Tower, Region::Gerudo);
    add_location(model, "GreatPlateau Tower", LocationType::Tower, Region::GreatPlateau);
    add_location(model, "Hateno Tower", LocationType::Tower, Region::Hateno);
    add_location(model, "Hebra Tower", LocationType::Tower, Region::Hebra);
    add_location(model, "Lake Tower", LocationType::Tower, Region::Lake);
    add_location(model, "Lanayru Tower", LocationType::Tower, Region::Lanayru);
    add_location(model, "Ridgeland Tower", LocationType::Tower, Region::Ridgeland);
    add_location(model, "Tabantha Tower", LocationType::Tower, Region::Tabantha);
    add_location(model, "Woodland Tower", LocationType::Tower, Region::Woodland);
    add_location(model, "Wasteland Tower", LocationType::Tower, Region::Wasteland);

    add_location(model, "Flight Range", LocationType::Town, Region::Tabantha);
    add_location(model, "Gerudo Town", LocationType::Town, Region::Gerudo);
    add_location(model, "Goron City", LocationType::Town, Region::Eldin);
    add_location(model, "Hateno Ancient Tech Lab", LocationType::Town, Region::Hateno);
    add_location(model, "Hateno Village", LocationType::Town, Region::Hateno);
    add_location(model, "Kakariko Village", LocationType::Town, Region::DuelingPeaks);
    add_location(model, "Kara Kara Bazaar", LocationType::Town, Region::Gerudo);
    add_location(model, "Korok Forest", LocationType::Town, Region::Woodland);
    add_location(model, "Lurelin Village", LocationType::Town, Region::Faron);
    add_location(model, "Rito Village", LocationType::Town, Region::Tabantha);
    add_location(model, "Tarrey Town", LocationType::Town, Region::Akkala);
    add_location(model, "Yiga Clan Hideout", LocationType::Town, Region::Gerudo);
    add_location(model, "Zora's Domain", LocationType::Town, Region::Lanayru);

    add_location(model, "Dueling Peaks Stable", LocationType::Stable, Region::DuelingPeaks);
    add_location(model, "East Akkala Stable", LocationType::Stable, Region::Akkala);
    add_location(model, "Foothill Stable", LocationType::Stable, Region::Eldin);
    add_location(model, "Gerudo Canyon Stable", LocationType::Stable, Region::Gerudo);
    add_location(model, "Highland Stable", LocationType::Stable, Region::Lake);
    add_location(model, "Lakeside Stable", LocationType::Stable, Region::Faron);
    add_location(model, "Outskirt Stable", LocationType::Stable, Region::Central);
    add_location(model, "Rito Stable", LocationType::Stable, Region::Tabantha);
    add_location(model, "Riverside Stable", LocationType::Stable, Region::Central);
    add_location(model, "Serenne Stable", LocationType::Stable, Region::Woodland);
    add_location(model, "Snowfield Stable", LocationType::Stable, Region::Hebra);
    add_location(model, "South Akkala Stable", LocationType::Stable, Region::Akkala);
    add_location(model, "Tabantha Bridge Stable", LocationType::Stable, Region::Ridgeland);
    add_location(model, "Wetland Stable", LocationType::Stable, Region::Lanayru);
    add_location(model, "Woodland Stable", LocationType::Stable, Region::Woodland);
}

fn add_location(model: &mut Model, name: &str, typ: LocationType, region: Region) {
    model.add_location(Location::new(name, &region, typ));
}
*/