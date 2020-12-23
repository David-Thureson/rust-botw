#![allow(unused_imports)]
#![allow(dead_code)]

use std::rc::Rc;
use std::cell::{RefCell, Ref};

use crate::*;
use std::borrow::Borrow;

pub const PREFIX_LF_HEADER: &str = "\n#";
pub const PREFIX_LF_SUBHEAD: &str = "\n>";
pub const PREFIX_COMMENT: &str = "//";

const QUEST_DEFEAT_ONE_DIVINE_BEAST: &str = "Defeat One Divine Beast";
const QUEST_DEFEAT_TWO_DIVINE_BEASTS: &str = "Defeat Two Divine Beasts";
const QUEST_DEFEAT_THREE_DIVINE_BEASTS: &str = "Defeat Three Divine Beasts";
const QUEST_DEFEAT_FOUR_DIVINE_BEASTS: &str = "Defeat All Four Divine Beasts";
const QUEST_COMPLETE_ALL_SHRINES: &str = "Complete All Shrines";
const SHRINE_COUNT: u32 = 120;
// const LOCATION_COUNT: u32 = 120;

pub fn main() {
    println!("\nBotW::model start\n");
    try_load();
    println!("\nBotW::model done\n");
}

fn try_load() {
    let _model = Model::new();
}

pub struct Model<'a> {
    pub items: Vec<Rc<RefCell<Item<'a>>>>,
    pub report_prices: Vec<Price<'a>>,
}


#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum ShrineState {
    NotDiscovered,
    Discovered,
    Completed,
}

#[derive(PartialEq, Eq, PartialOrd)]
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

#[derive(PartialEq, Eq, PartialOrd)]
pub struct Quest {
    pub name: String,
    pub state: QuestState,
}

pub struct Location {

}

impl <'a> Model<'a> {

    pub fn new() -> Self {
        let mut model = Model {
            items: vec![],
            report_prices: vec![],
        };
        Item::load_items(&mut model);
        // model.describe();
        model.show_remaining_price_report();
        model
    }

    pub fn describe(&self) {
        let mut s = "".to_string();
        self.describe_deep(& mut s, 0, None);
        println!("{}", &s);
    }

    pub fn describe_deep(&self, s: &mut String, depth: usize, _max_depth: Option<usize>) {
        s.push_str(&format_indent_line(depth, "Botw Model"));
        for item in self.items.iter() {
            // let item: Ref<'_, Item<'a>> = item.borrow();
            RefCell::borrow(&item).describe_deep(s, depth + 1, _max_depth);
        }
    }

    fn item_name_to_index(&self, name: &str) -> usize {
        let name = name.to_string();
        for (index, item) in self.items.iter().enumerate() {
            if RefCell::borrow(&item).name == name {
                return index;
            }
        }
        panic!("Didn't find an item named \"{}\"", name);
    }

    pub fn item_by_name(&self, name: &str) -> &Rc<RefCell<Item<'a>>> {
        self.items.get(self.item_name_to_index(name)).unwrap()
    }

    /*
    fn show_remaining_price_report(&self) {
        let mut price: Price = Price::new();
        // for i in 0..self.items.len() {
            // let item: &Rc<RefCell<Item>> = self.items.get(0).unwrap();
            // let item: Ref<Item> = RefCell::borrow(item);
            // price.add_price(item.price.as_ref());
            price.add_price(RefCell::borrow(self.items.get(0).unwrap()).price.as_ref());
            //price.add_prices(item.upgrade_prices.as_ref());
        // }
        // price.print_report("Total Cost for All Purchases and Upgrades");
    }
    */


    // ::<&Rc<RefCell<Item<'a>>>>

    fn show_remaining_price_report(&self) {
        let mut price = Price::new();
        for item in self.items.iter() {
            for one_price in RefCell::borrow(item).prices.iter() {
                if let Some(one_price) = one_price {
                    for part in one_price.parts.iter() {
                        // price.parts.push(part.clone());
                        price_add_part(self, &mut price, Some(&part.clone()));
                    }
                }
            }
        }
        price.print_report("Total Cost for All Purchases and Upgrades");
    }

    /*
    fn show_remaining_price_report(&self) {
        let mut borrow = None;
        let mut price = Price::new();
        for item in self.items.iter() {
            borrow = Some(RefCell::borrow(item));
            let prices_cloned = borrow.unwrap().prices_cloned();
            for one_price in prices_cloned {
                for part in one_price.parts {
                    price.parts.push(part.clone());
                }
                // price.add_price(Some(&one_price.clone()));
            }
        }
        price.print_report("Total Cost for All Purchases and Upgrades");
    }
    */

    /*
    fn show_remaining_price_report(&self) {
        let mut price = Price::new();
        for item in self.items.iter() {
            let item: Ref<Item> = RefCell::borrow(item);
            if let Some(item_price) = &item.price {
                price.add_price(item_price);
            }
            if let Some(item_upgrade_prices) = &item.upgrade_prices {
                for upgrade_price in item_upgrade_prices.iter() {
                    price.add_price(upgrade_price);
                }
            }
        }
        price.print_report("Total Cost for All Purchases and Upgrades");
    }
    */

}


/*
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
*/


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

