use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::ops::{Add, AddAssign};

const QUEST_DEFEAT_ONE_DIVINE_BEAST: &str = "Defeat One Divine Beast";
const QUEST_DEFEAT_TWO_DIVINE_BEASTS: &str = "Defeat Two Divine Beasts";
const QUEST_DEFEAT_THREE_DIVINE_BEASTS: &str = "Defeat Three Divine Beasts";
const QUEST_DEFEAT_FOUR_DIVINE_BEASTS: &str = "Defeat All Four Divine Beasts";
const QUEST_COMPLETE_ALL_SHRINES: &str = "Complete All Shrines";
const SHRINE_COUNT: u32 = 120;
// const LOCATION_COUNT: u32 = 120;

pub struct Model<'a> {
    pub items: Vec<Item<'a>>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Item<'a> {
    pub name: String,
    pub item_type: ItemType,
    pub quantity: u32,
    pub price: Option<Price<'a>>,
    pub upgrade_prices: Option<[Price<'a>; 4]>,
    pub mon_sell_price: Option<u32>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum ItemType {
    Money,
    Material {
        effect: Option<Effect>,
    },
    Weapon,
    Shield,
    Bow,
    Arrow,
    Armor {
        effect: Option<Effect>,
        upgrade_level: u8,
    },
    Special,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Effect {
    RestoreHearts,
    TemporaryHearts,
    Attack,
    Defense,
    RestoreStamina,
    HeatResistance,
    ColdResistance,
    ShockResistance,
    MovementSpeed,
    NightMovementSpeed,
    SwimmingSpeed,
    ClimbingSpeed,
    Disguise,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Price<'a> {
    pub parts: Vec<PricePart<'a>>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum PricePart<'a> {
    Item {
        item: &'a Item<'a>,
        quantity: usize,
    },
    Shrine {
        shrine: &'a Shrine,
    },
    Quest {
        quest: &'a Quest
    },
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum ShrineState {
    NotDiscovered,
    Discovered,
    Completed,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Shrine {
    pub name: String,
    pub state: ShrineState,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum QuestState {
    NotStarted,
    Started,
    Completed,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Quest {
    pub name: String,
    pub state: QuestState,
}

pub struct Location {

}

impl Price<'_> {

    fn add_price<'b>(&mut self, other: &'b Self) {
        for other_part in other.parts.iter() {
            self.add_part(other_part);
        }
    }

    fn add_part<'b>(&'b mut self, other_part: &'b PricePart) {
        match self.find_same_reference(other_part) {
            Some(found_index) => {
                self.parts.get_mut(found_index).unwrap().add_part(other_part);
            },
            None => {
                self.parts.push(other_part.clone());
            }
        }
    }

    fn find_same_reference(&self, other_part: &PricePart) -> Option<usize> {
        for (index, part) in self.parts.iter().enumerate() {
            if part.same_reference(other_part) {
                return Some(index);
            }
        }
        None
    }

}

impl PricePart<'_> {
    pub fn same_reference(&self, other: &PricePart) -> bool {
        match (self, other) {
            (PricePart::Item { item: self_item, quantity: _ }, PricePart::Item { item: other_item, quantity: _ }) => {
                self_item == other_item
            },
            (PricePart::Shrine { shrine: self_shrine }, PricePart::Shrine { shrine: other_shrine }) => {
                self_shrine == other_shrine
            },
            (PricePart::Quest { quest: self_quest }, PricePart::Quest { quest: other_quest }) => {
                self_quest == other_quest
            },
            _ => false
        }
    }

    pub fn add_part(&mut self, other: &PricePart) {
        assert!(self.same_reference(other));
        // If this is not an Item we have nothing to do since we already have the shrine or quest reference.
        match (self, other) {
            (PricePart::Item { item: _, ref mut quantity}, PricePart::Item { item: _, quantity: other_quantity }) => {
                *quantity += other_quantity;
            },
            _ => {}
        }

        /*
        match (self, other) {
            (PricePart::Item { item: _, quantity: self_quantity }, PricePart::Item { item: other_item, quantity: _ }) => {
                self_item == other_item
            },

        if let PricePart::Item { item: _, quantity: other_quantity } = other {
            self.quantity += other_quantity;
        }
        */
    }

}

/*
impl Clone for PricePart<'_> {
    fn clone(&self) -> Self {
        *self
    }
}
*/
/*
impl Ord for PricePart<'_> {
    fn cmp(&self, other: &Self) -> Ordering {

    }
}


impl Ord for PricePart<'_> {

}
*/
/*
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

*/