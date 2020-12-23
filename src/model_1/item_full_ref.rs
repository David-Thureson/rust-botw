#![allow(unused_imports)]
#![allow(dead_code)]

use crate::*;
use std::rc::Rc;
use std::cell::{RefCell, Ref};
use std::fmt;
use std::cmp;
use std::str::FromStr;

const FILE_ITEMS: &str = "Breath of the Wild Items.txt";
const SECTION_ITEMS_IN_ORDER: &str = "Items in Order";
const SUBSECTION_MONEY: &str = "Money";
const SUBSECTION_MATERIALS: &str = "Materials";
const SUBSECTION_WEAPONS: &str = "Weapons";
const SUBSECTION_BOWS: &str = "Bows";
const SUBSECTION_ARROWS: &str = "Arrows";
const SUBSECTION_SHIELDS: &str = "Shields";
const SUBSECTION_ARMOR: &str = "Armor";
const SUBSECTION_DISHES: &str = "Dishes";
const SUBSECTION_ELIXERS: &str = "Elixers";
const SUBSECTION_ROASTED_FOODS: &str = "Roasted Foods";
const SUBSECTION_FROZEN_FOODS: &str = "Frozen Foods";
const SUBSECTION_KEY_ITEMS: &str = "Key Items";
const SECTION_ITEM_EFFECTS: &str = "Item Effects";
const SECTION_PURCHASE_AND_UPGRADE: &str = "Purchase and Upgrade";
const BOW: &str = "Bow";
/*
const EFFECT_DESC_RESTORE_HEARTS: &str = "";
const EFFECT_DESC_TEMPORARY_HEARTS: &str = "";
const EFFECT_DESC_ATTACK: &str = "";
const EFFECT_DESC_DEFENSE: &str = "";
const EFFECT_DESC_RESTORE_STAMINA: &str = "";
const EFFECT_DESC_TEMPORARY_STAMINA: &str = "";
const EFFECT_DESC_HEAT_RESISTANCE: &str = "";
const EFFECT_DESC_COLD_RESISTANCE: &str = "";
const EFFECT_DESC_SHOCK_RESISTANCE: &str = "";
const EFFECT_DESC_MOVEMENT_SPEED: &str = "";
*/


/*
|Items in Order|
|Food_Properties|
|Plants|
|Critters|
|Minerals|
|Monster Parts|
|Purchase and Upgrade|
|Mon Sell Prices|
*/


#[derive(PartialEq, Eq, PartialOrd)]
pub struct Item<'a> {
    pub sequence: usize,
    pub name: String,
    pub item_type: ItemType,
    pub quantity: usize,
    pub effect: Option<Effect>,
    pub upgrade_level: u8,
    pub prices: Vec<Option<Price<'a>>>,
    pub mon_sell_price: Option<usize>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub enum ItemType {
    Money,
    Weapon,
    Bow,
    Arrow,
    Shield,
    Armor,
    Material,
    Food,
    KeyItem,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub enum Effect {
    RestoreHearts,
    TemporaryHearts,
    Attack,
    Defense,
    RestoreStamina,
    TemporaryStamina,
    HeatResistance,
    ColdResistance,
    ShockResistance,
    Fireproof,
    Fire,
    Cold,
    Shock,
    MovementSpeed,
    NightMovementSpeed,
    SwimmingSpeed,
    ClimbingSpeed,
    Disguise,
    Stealth,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Price<'a> {
    pub parts: Vec<PricePart<'a>>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum PricePart<'a> {
    Item {
        item: Rc<RefCell<Item<'a>>>,
        quantity: usize,
    },
    Shrine {
        shrine: Rc<RefCell<Shrine>>,
    },
    Quest {
        quest: Rc<RefCell<Quest>>,
    },
}

impl <'a> Item<'a> {

    pub fn new(sequence: usize, name: String, item_type: ItemType) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Item {
            sequence,
            name,
            item_type,
            quantity: 0,
            effect: None,
            upgrade_level: 0,
            prices: vec![],
            mon_sell_price: None,
        }))
    }

    pub fn load_items(model: &mut Model) {
        let mut sections = read_file_into_sections(FILE_ITEMS, PREFIX_LF_HEADER);
        Self::load_items_in_order(model, sections.remove(SECTION_ITEMS_IN_ORDER).unwrap());
        Self::load_food_properties(model, sections.remove(SECTION_ITEM_EFFECTS).unwrap());
        Self::load_purchase_and_upgrade(model, sections.remove(SECTION_PURCHASE_AND_UPGRADE).unwrap());
        model.items.sort_unstable();
    }

    fn load_items_in_order(model: &mut Model, section: String) {
        let mut subsections = break_into_sections(section, PREFIX_LF_SUBHEAD);
        for (subsection_name, item_type) in [
            (SUBSECTION_MONEY, ItemType::Money),
            (SUBSECTION_WEAPONS, ItemType::Weapon),
            (SUBSECTION_BOWS, ItemType::Bow),
            (SUBSECTION_ARROWS, ItemType::Arrow),
            (SUBSECTION_SHIELDS, ItemType::Shield),
            (SUBSECTION_ARMOR, ItemType::Armor),
            (SUBSECTION_MATERIALS, ItemType::Material),
            (SUBSECTION_KEY_ITEMS, ItemType::KeyItem)].iter() {
            for line in section_to_lines(subsections.remove(*subsection_name).unwrap()) {
                model.items.push(Item::new(model.items.len(), line, *item_type));
            }
        }
    }

    fn load_food_properties(model: &mut Model, section: String) {
        for line in section_to_lines(section) {
            let (name, description) = split_2(&line, "\t");
            let effect = Effect::from_description(description);
            if let Some(effect) = effect {
                model.item_by_name(name).borrow_mut().effect = Some(effect);
            }
        }
    }

    fn load_purchase_and_upgrade(model: &mut Model, section: String) {
        let section = section.replace("\n\t", "\t");
        for line in section_to_lines(section) {
            let mut prices = vec![];
            let (first, rest) = split_1_or_2(&line, "\t");
            let (name, purchase_price) = split_1_or_2(first, ": ");
            let item = model.item_by_name(name);
            let purchase_price = purchase_price.and_then(|x| Price::parse(model,x) );
            prices.push(purchase_price);
            if let Some(rest) = rest {
                let tab_split = rest.split("\t");
                for item in tab_split {
                    let (_, price) = split_2(&item, ": ");
                    let price = Price::parse(model, price).unwrap();
                    prices.push(Some(price));
                }
            }
            assert!(prices.len() == 1 || prices.len() == 5);
            item.borrow_mut().prices = prices;
        }
    }

    pub fn prices_cloned(&self) -> Vec<Price<'a>> {
        self.prices.iter()
            .filter(|price| price.is_some())
            .map(|price| price.as_ref().unwrap().clone())
            .collect()
    }
    /*
    fn load_purchase_and_upgrade(model: &mut Model, section: String) {
        let mut item: Option<Rc<RefCell<Item>>> = None;
        let mut upgrade_prices: Option<Vec<Price>> = None;
        for line in section_to_lines(section) {
            if line.starts_with("\t1:") {
                item.unwrap().borrow_mut().upgrade_prices = Some(vec![]);
            }
            if line.starts_with("\t") {
                let (_, price) = split_2(&line, ": ");
                let price = Price::parse(model, price).unwrap();
                upgrade_prices.push(price);
            } else {
                if upgrade_prices.is_some() {

                }
                let (name, price) = split_1_or_2(&line, ": ");

            }
        }
    }
    */

    pub fn description(&self) -> String {
        let type_name = self.item_type.to_string();
        let effect = self.effect.as_ref().map_or("".to_string(), |effect| format!("; effect = {}", effect));
        // let price = self.price.as_ref().map_or("".to_string(), |price| format!("; price = {}", price.description()));
        let line = format!("{} ({}){}", &self.name, &type_name, &effect);
        line
    }

    pub fn describe_deep(&self, s: &mut String, depth: usize, _max_depth: Option<usize>) {
        let line = self.description();
        s.push_str(&format_indent_line(depth, &line));
        for (index, price) in self.prices.iter().enumerate() {
            if let Some(price) = price {
                let price_line = format!("{}: {}", index, price.description());
                s.push_str(&format_indent_line(depth + 1, &price_line));
            }
        }
    }

    // pub fn show_remaining_price_report(model: &Model) {
    //let mut _price: Price<'c> = Price::new();
    // for i in 0..self.items.len() {
    //let item: &Rc<RefCell<Item<'a>>> = model.items.get(0).unwrap();
    //let _item: Ref<Item<'a>> = RefCell::borrow(item);
    // price.add_price(item.price.as_ref());
    // price.add_price(RefCell::borrow(model.items.get(0).unwrap()).price.as_ref());
    // price.add_price(RefCell::borrow(model.items.get(0).unwrap()).price.clone().as_ref());
    //price.add_prices(item.upgrade_prices.as_ref());
    // }
    // price.print_report("Total Cost for All Purchases and Upgrades");
    // model.report_prices.push(price);
    // }

    /*
    pub fn show_remaining_price_report<'c, 'a: 'c>(model: &'a mut Model<'a>) {
        let mut price: Price<'c> = Price::new();
        // for i in 0..self.items.len() {
        let item: &Rc<RefCell<Item<'a>>> = model.items.get(0).unwrap();
        let item: Ref<Item<'a>> = RefCell::borrow(item);
        price.add_price(item.price.as_ref());
        // price.add_price(RefCell::borrow(model.items.get(0).unwrap()).price.as_ref());
        // price.add_price(RefCell::borrow(model.items.get(0).unwrap()).price.clone().as_ref());
        //price.add_prices(item.upgrade_prices.as_ref());
        // }
        // price.print_report("Total Cost for All Purchases and Upgrades");
        // model.report_prices.push(price);
    }


    fn load_items_in_order(model: &mut Model, section: String) {
        let mut subsections = break_into_sections(section, PREFIX_SUBHEAD);
        for line in section_to_lines(subsections.remove(SUBSECTION_MONEY).unwrap()) {
            //rintln!("|{}|", line);
            model.items.push(Item::new(line, ItemType::Money));
        }
        for line in section_to_lines(subsections.remove(SUBSECTION_MATERIALS).unwrap()) {
            model.items.push(Item::new(line, ItemType::Material));
        }
        for line in section_to_lines(subsections.remove(SUBSECTION_WEAPONS).unwrap()) {
            model.items.push(Item::new(line, ItemType::Weapon));
        }
        for line in section_to_lines(subsections.remove(SUBSECTION_BOWS_AND_ARROWS).unwrap()) {
            if line.contains(BOW) {
                model.items.push(Item::new(line, ItemType::Bow));
            } else {
                model.items.push(Item::new(line, ItemType::Arrow));
            }
        }
        for line in section_to_lines(subsections.remove(SUBSECTION_SHIELDS).unwrap()) {
            model.items.push(Item::new(line, ItemType::Shield));
        }
        for line in section_to_lines(subsections.remove(SUBSECTION_ARMOR).unwrap()) {
            model.items.push(Item::new(line, ItemType::Armor));
        }


    }
    */

}

impl Ord for Item<'_> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.sequence.cmp(&other.sequence)
    }
}

/*
impl PartialOrd for Item<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}
*/

impl fmt::Display for ItemType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl <'a> Price<'a> {

    pub fn new() -> Price<'a> {
        Price {
            parts: vec![],
        }
    }

    // pub fn add_prices<'b: 'a>(&mut self, other: Option<&'b Vec<Self>>) {
    pub fn add_prices(&mut self, other: Option<&'a Vec<Price<'a>>>) {
        if let Some(other) = other {
            for other_price in other.iter() {
                self.add_price(Some(other_price));
            }
        }
    }

    // pub fn add_price<'b: 'a>(&mut self, other: Option<&'b Self>) {
    pub fn add_price<'b: 'a>(&mut self, other: Option<&'a Self>) {
        if let Some(other) = other {
            for other_part in other.parts.iter() {
                self.add_part(Some(other_part));
            }
        }
    }

    /*
    pub fn add_price(&mut self, other: Option<&'a Self>) {
        if let Some(other) = other {
            for other_part in other.parts.iter() {
                self.add_part(Some(other_part));
            }
        }
    }
    */

    pub fn add_part(&mut self, other_part: Option<&'a PricePart<'a>>) {
        if let Some(other_part) = other_part {
            match self.find_same_reference(other_part) {
                Some(found_index) => {
                    self.parts.get_mut(found_index).unwrap().add_part(Some(other_part));
                },
                None => {
                    self.parts.push(other_part.clone());
                    self.parts.sort_unstable();
                }
            }
        }
    }

    fn find_same_reference(&self, other_part: &'a PricePart<'a>) -> Option<usize> {
        for (index, part) in self.parts.iter().enumerate() {
            if part.same_reference(other_part) {
                return Some(index);
            }
        }
        None
    }

    fn parse(model: &Model<'a>, s: &str) -> Option<Self> {
        let mut parts = vec![];
        for split in s.split(", ") {
            if let Some(part) = PricePart::parse(model, split) {
                parts.push(part);
            }
        }
        if parts.len() > 0 {
            Some(Price {
                parts: parts,
            })
        } else {
            None
        }
    }

    fn description(&self) -> String {
        self.parts.iter().map(|part| part.description()).collect::<Vec<String>>().join(", ")
    }

    pub fn print_report(&self, label: &str) {
        let max_qty = self.parts.iter()
            .map(|part| match part {
                PricePart::Item { item: _, quantity } => *quantity,
                _ => 0
            }).max()
            .unwrap() as usize;
        let formatted_number_len = format_count(max_qty).len();
        println!("{}", label);
        for part in self.parts.iter() {
            println!("{}", match part {
                PricePart::Item { ref item, quantity } => format!("{:>width$} {}", format_count(*quantity), item.borrow().name, width=formatted_number_len),
                PricePart::Shrine { ref shrine } => format!("{:>width$} {}", "", shrine.borrow().name, width=formatted_number_len),
                PricePart::Quest { ref quest } => format!("{:>width$} {}", "", quest.borrow().name, width=formatted_number_len),
            });
        }
    }

}

impl fmt::Display for Price<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.description())
    }
}

impl <'a> Clone for Price<'a> {
    fn clone(&self) -> Self {
        Price {
            parts: self.parts.iter().map(|part| part.clone()).collect(),
        }
    }
}

impl <'a> PricePart<'a> {
    pub fn same_reference(&self, other: &'a PricePart<'a>) -> bool {
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

    pub fn reference_name(&self) -> String {
        match self {
            PricePart::Item { item, quantity: _ } => RefCell::borrow(item).name.clone(),
            PricePart::Shrine { shrine} => RefCell::borrow(shrine).name.clone(),
            PricePart::Quest { quest} => RefCell::borrow(quest).name.clone(),
        }
    }

    pub fn add_part(&mut self, other: Option<&'a PricePart<'a>>) {
        if let Some(other) = other {
            assert!(self.same_reference(other));
            // If this is not an Item we have nothing to do since we already have the shrine or quest reference.
            match (self, other) {
                (PricePart::Item { item: _, ref mut quantity }, PricePart::Item { item: _, quantity: other_quantity }) => {
                    *quantity += other_quantity;
                },
                _ => {}
            }
        }
    }

    fn parse(model: &Model<'a>, s: &str) -> Option<Self> {
        if s.chars().next().unwrap().is_numeric() {
            let (quantity, name) = split_2(s, " ");
            let quantity = usize::from_str(quantity).unwrap();
            let item = Rc::clone(model.item_by_name(name));
            Some(PricePart::Item {
                item,
                quantity,
            })
        } else {
            // For now ignore shrines and quests.
            None
        }
    }

    fn description(&self) -> String {
        match self {
            PricePart::Item { item, quantity } => {
                format!("{} {}", format_count(*quantity), item.borrow().name)
            },
            PricePart::Shrine { shrine } => {
                shrine.borrow().name.clone()
            }
            PricePart::Quest { quest } => {
                quest.borrow().name.clone()
            }
        }
    }

    /*
    fn clone(&self) -> Self {
        match self {
            PricePart::Item { item, quantity } => {
                PricePart::Item {
                    item: Rc::clone(item),
                    quantity: *quantity,
                }
            },
            PricePart::Shrine { shrine } => {
                PricePart::Shrine {
                    shrine: Rc::clone(shrine),
                }
            },
            PricePart::Quest { quest } => {
                PricePart::Quest {
                    quest: Rc::clone(quest),
                }
            },
        }
    }
    */
}

impl <'a> Clone for PricePart<'a> {
    fn clone(&self) -> Self {
        match self {
            PricePart::Item { item, quantity } => {
                PricePart::Item {
                    item: Rc::clone(item),
                    quantity: *quantity,
                }
            },
            PricePart::Shrine { shrine } => {
                PricePart::Shrine {
                    shrine: Rc::clone(shrine),
                }
            },
            PricePart::Quest { quest } => {
                PricePart::Quest {
                    quest: Rc::clone(quest),
                }
            },
        }
    }
}

impl Ord for Shrine {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

impl Ord for Quest {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

impl Effect {
    pub fn from_description(description: &str) -> Option<Self> {
        for (substring, effect) in [
            ("hearts restored", Effect::RestoreHearts),
            ("attack", Effect::Attack),
            ("defense", Effect::Defense),
            ("additional hearts", Effect::TemporaryHearts),
            ("stamina restor", Effect::RestoreStamina),
            ("temporary stamina", Effect::TemporaryStamina),
            ("heat", Effect::HeatResistance),
            ("cold", Effect::ColdResistance),
            ("electric", Effect::ShockResistance),
            ("movement", Effect::MovementSpeed),
            ("stealth", Effect::Stealth)
        ].iter() {
            if description.contains(substring) {
                return Some(*effect);
            }
        }
        None
    }
}

impl fmt::Display for Effect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Effect::RestoreHearts => "Restore Hearts",
            Effect::TemporaryHearts => "Temporary Hearts",
            Effect::Attack => "Attack Boost",
            Effect::Defense => "Defense Boost",
            Effect::RestoreStamina => "Restore Stamina",
            Effect::TemporaryStamina => "Temporary Stamina",
            Effect::HeatResistance => "Heat Resistance",
            Effect::ColdResistance => "Cold Resistance",
            Effect::ShockResistance => "ShockResistance",
            Effect::Fireproof => "Fireproof",
            Effect::Fire => "Fire",
            Effect::Cold => "Frost",
            Effect::Shock => "Shock",
            Effect::MovementSpeed => "Movement Speed",
            Effect::NightMovementSpeed => "Night Movement Speed",
            Effect::SwimmingSpeed => "Swimming Speed",
            Effect::ClimbingSpeed => "Climbing Speed",
            Effect::Disguise => "Disguise",
            Effect::Stealth => "Stealth",
        })
        // write!(f, "{:?}", self)
    }
}

/*
pub fn price_add_part<'c, 'd: 'c> (price: &'d mut Price<'d>, other_part: Option<&'c PricePart<'c>>) {
    if let Some(other_part) = other_part {
        match price_find_same_reference(price, other_part) {
            Some(found_index) => {
                // part_add_part_test(price.parts.get_mut(found_index).unwrap(), Some(other_part));
            },
            None => {
                // price.parts.push(other_part.clone());
                // price.parts.sort_unstable();
            }
        }
    }
}

fn price_find_same_reference<'c, 'd: 'c>(price: &'d Price, other_part: &'c PricePart<'c>) -> Option<usize> {
    for (index, part) in price.parts.iter().enumerate() {
        if part.same_reference(other_part) {
            return Some(index);
        }
    }
    None
}
*/

/*
fn price_part_clone<'c, 'd: 'c>(part: &'c PricePart<'c>) -> PricePart<'d> {
    match part {
        PricePart::Item { item, quantity } => {
            PricePart::Item {
                item: Rc::clone(item),
                quantity: *quantity,
            }
        },
        PricePart::Shrine { shrine } => {
            PricePart::Shrine {
                shrine: Rc::clone(shrine),
            }
        },
        PricePart::Quest { quest } => {
            PricePart::Quest {
                quest: Rc::clone(quest),
            }
        },
    }
}
*/

pub fn price_add_part<'a, 'c> (model: &'a Model<'a>, price: &'a mut Price<'a>, other_part: Option<&'c PricePart<'c>>) {
    if let Some(other_part) = other_part {
        match price_find_same_reference(price, other_part) {
            Some(found_index) => {
                part_add_part(price.parts.get_mut(found_index).unwrap(), Some(other_part));
            },
            None => {
                // price.parts.push(price_part_clone(model, other_part));

                let part_clone =
                    match other_part {
                        PricePart::Item { item, quantity } => {
                            let item_name = &RefCell::borrow(&item).name;
                            let item = Rc::clone(model.item_by_name(item_name));
                            PricePart::Item {
                                // item: Rc::clone(item),
                                item,
                                quantity: *quantity,
                            }
                        },
                        PricePart::Shrine { shrine } => {
                            PricePart::Shrine {
                                shrine: Rc::clone(shrine),
                            }
                        },
                        PricePart::Quest { quest } => {
                            PricePart::Quest {
                                quest: Rc::clone(quest),
                            }
                        },
                    };
                price.parts.push(part_clone);

                price.parts.sort_unstable();
            }
        }
    }
}

pub fn part_add_part<'a>(part: &'a mut PricePart<'a>, other_part: Option<&PricePart>) {
    if let Some(other_part) = other_part {
        assert!(part.reference_name() == other_part.reference_name());
        // If this is not an Item we have nothing to do since we already have the shrine or quest reference.
        match (part, other_part) {
            (PricePart::Item { item: _, ref mut quantity }, PricePart::Item { item: _, quantity: other_quantity }) => {
                *quantity += other_quantity;
            },
            _ => {}
        }
    }
}

fn price_find_same_reference(price: &Price, other_part: &PricePart) -> Option<usize> {
    let other_part_ref_name = other_part.reference_name();
    for (index, part) in price.parts.iter().enumerate() {
        if part.reference_name() == other_part_ref_name {
            return Some(index);
        }
    }
    None
}

fn price_part_clone<'c, 'a: 'c>(model: &'a Model<'a>, part: &'c PricePart<'c>) -> PricePart<'a> {
    match part {
        PricePart::Item { item, quantity } => {
            let item_name = &RefCell::borrow(&item).name;
            let item = Rc::clone(model.item_by_name(item_name));
            PricePart::Item {
                // item: Rc::clone(item),
                item,
                quantity: *quantity,
            }
        },
        PricePart::Shrine { shrine } => {
            PricePart::Shrine {
                shrine: Rc::clone(shrine),
            }
        },
        PricePart::Quest { quest } => {
            PricePart::Quest {
                quest: Rc::clone(quest),
            }
        },
    }
}

