use std::io::{BufReader, BufRead};
use std::fs::File;
use std::str::FromStr;

use crate::*;
use super::*;
use crate::model_2::model::*;

pub const PREFIX_HEADER: &str = "#";
// pub const PREFIX_LF_SUBHEAD: &str = "\n>";
pub const PREFIX_COMMENT: &str = "//";
const FILE_NAME_CHARACTERS: &str = "Breath of the Wild Characters.txt";
const FILE_NAME_ITEMS: &str = "Breath of the Wild Items.txt";
const FILE_NAME_INVENTORY: &str = "Breath of the Wild Inventory.txt";
const FILE_NAME_SHRINES: &str = "Breath of the Wild Shrines.txt";
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
            let (first, second) = util::parse::split_1_or_2(&line, ":");
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

pub fn load_shrines(model: &mut Model) {
    let file = File::open(FILE_NAME_SHRINES).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines()
            .map(|line| line.unwrap().trim().to_string())
            .filter(|line| !line.is_empty())
            .filter(|line| !line.starts_with(PREFIX_COMMENT)) {
        let (name, challenge) = util::parse::split_2(&line, ":");
        let name = name.trim().to_string();
        let challenge = challenge.trim().to_string();
        model.add_location(Location::new(&name, &Region::ShrinePlaceholder, LocationType::new_shrine(&challenge)));
    }
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
            let quest = match quest_type_name.clone().unwrap().as_ref() {
                "Main" => Quest::new(&line, QuestType::new_main()),
                "Side" => {
                    let (name, notes) = util::parse::extract_optional(&line, "(", ")");
                    Quest::new(name.trim(), QuestType::new_side(notes))
                },
                "Shrine" => {
                    let (name, shrine_name) = util::parse::split_2(&line, ":");
                    let shrine = model.locations.get(name).unwrap().clone();
                    Quest::new(name, QuestType::new_shrine(shrine))
                },
                _ => panic!()
            };
            model.add_quest(quest);
        }
    }
}

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

        let (header, section_content) = util::parse::split_2(split, "\n");
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

/*
pub fn split_1_or_2<'a>(line: &'a str, pat: &str) -> (&'a str, Option<&'a str>) {
    let mut split = line.splitn(2, pat);
    (
        split.next().expect(&format!("No first split item found for line = \"{}\"", line)),
        split.next()
    )
}

pub fn split_2<'a>(line: &'a str, pat: &str) -> (&'a str, &'a str) {
    let mut split = line.splitn(2, pat);
    (
        split.next().expect(&format!("No first split item found for line = \"{}\"", line)),
        split.next().expect(&format!("No second split item found for line = \"{}\"", line))
    )
}

pub fn rsplit_2<'a>(line: &'a str, pat: &str) -> (&'a str, &'a str) {
    let mut split = line.rsplitn(2, pat);
    (
        split.next().expect(&format!("No first split item found for line = \"{}\"", line)),
        split.next().expect(&format!("No second split item found for line = \"{}\"", line))
    )
}
*/

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
