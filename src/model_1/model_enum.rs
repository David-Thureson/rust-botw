use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

pub struct Model {
    pub items: Vec<Item>,
}

pub struct Item {
    pub name: String,
    pub item_type: ItemType,
}

pub enum ItemType {
    Money,
    Material,
    Weapon,
    Shield,
    Bow,
    Arrow,
    Armor,
    Special,
    Shrine,
    Quest,
    Location,
}

pub struct Game {
    pub game_items: Vec<GameItem>,
}

pub enum QuestState {
    NotStarted,
    Started,
    Completed,
}

pub struct GameItem {
    pub item: Item,
    pub quantity: u32,
    pub quest_state: Option<QuestState>,
    pub upgrade_level: u32,
}

impl Model {
    pub fn load() -> Self {
        let mut model = Model {
            items: vec![],
        };
        model.load_items();
        model
    }

    fn load_items(&mut self) {

    }
}

pub fn read_file_into_sections(file_name: &str, header_prefix: &str) -> HashMap<String, Vec<String>> {
    let file = File::open(FILE_ITEMS).unwrap();
    let reader = BufReader::new(file);
    break_into_sections(reader
                            .lines()
                            .map(|line| line.unwrap().trim())
                            .filter(|line| line.len() > 0 && !line.starts_with(PREFIX_COMMENT))
                            .map(|line| line.to_string())
                            .collect(),
                        header_prefix)
}

pub fn break_into_sections(lines: Vec<String>, header_prefix: &str) -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();
    let mut current_header: Option<String> = None;
    for line in lines {
        if line.starts_with(header_prefix) {
            let header = line[header_prefix.len()..].trim().to_string();
            map.insert(header, vec![]);
            let
        } else {
            map.get_mut(current_header).
        }
    }
    map
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

