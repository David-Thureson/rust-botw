#![allow(unused_imports)]
#![allow(dead_code)]

use crate::*;
use std::rc::Rc;
use std::cell::{RefCell, Ref};
use std::fmt;
use std::cmp;
use std::str::FromStr;
use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::model::{Model, ComponentReference, PREFIX_LF_HEADER, PREFIX_LF_SUBHEAD, PREFIX_COMMENT};
use crate::location::*;
use crate::shrine::*;
use crate::quest::*;

const FILE_ITEMS: &str = "Breath of the Wild Items.txt";
const FILE_INVENTORY: &str = "Breath of the Wild Inventory.txt";
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
pub struct Item {
    pub sequence: usize,
    pub name: String,
    pub item_type: ItemType,
    pub quantity: usize,
    pub needed: usize,
    pub effect: Option<Effect>,
    pub upgrade_level: usize,
    pub prices: Vec<Price>,
    pub is_monster_part: bool,
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
    RestoreStamina,
    TemporaryStamina,
    MovementSpeed,
    Fireproof,
    ColdResistance,
    HeatResistance,
    ShockResistance,
    Attack,
    Defense,

    Fire,
    Cold,
    Shock,
    NightMovementSpeed,
    SwimmingSpeed,
    ClimbingSpeed,
    Disguise,
    Stealth,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Price {
    pub components: Vec<CountedComponent>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct CountedComponent {
    pub component_reference: ComponentReference,
    pub quantity: usize,
}

impl Item {

    pub fn new(sequence: usize, name: String, item_type: ItemType) -> Self { Item {
            sequence,
            name,
            item_type,
            quantity: 0,
            needed: 0,
            effect: None,
            upgrade_level: 0,
            prices: vec![],
            is_monster_part: false,
            mon_sell_price: None,
        }
    }

    pub fn load_items(model: &mut Model) {
        let mut sections = read_file_into_sections(FILE_ITEMS, PREFIX_LF_HEADER);
        Self::load_items_in_order(model, sections.remove(SECTION_ITEMS_IN_ORDER).unwrap());
        Self::load_item_effects(model, sections.remove(SECTION_ITEM_EFFECTS).unwrap());
        Self::load_purchase_and_upgrade(model, sections.remove(SECTION_PURCHASE_AND_UPGRADE).unwrap());
        // model.items.sort_unstable();
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
            for item_name in section_to_lines(subsections.remove(*subsection_name).unwrap()) {
                assert!(!model.items.contains_key(&item_name), "Item \"{}\" is already in the model.", item_name);
                model.items.insert(item_name.clone(), Item::new(model.items.len(), item_name, *item_type));
            }
        }
    }

    fn load_item_effects(model: &mut Model, section: String) {
        for line in section_to_lines(section) {
            let (name, description) = split_2(&line, "\t");
            let effect = Effect::from_description(description);
            let is_monster_part = description.contains("Cook to with ingredients");
            if let Some(effect) = effect {
                model.items.get_mut(name).unwrap().effect = Some(effect);
            }
            if is_monster_part {
                model.items.get_mut(name).unwrap().is_monster_part = true;
            }
        }
    }

    fn load_purchase_and_upgrade(model: &mut Model, section: String) {
        let section = section.replace("\n\t", "\t");
        for line in section_to_lines(section) {
            let mut prices = vec![];
            let (first, rest) = split_1_or_2(&line, "\t");
            let (item_name, purchase_price) = split_1_or_2(first, ": ");
            prices.push(Price::parse(model,purchase_price));
            if let Some(rest) = rest {
                let tab_split = rest.split("\t");
                for item in tab_split {
                    let (_, price) = split_2(&item, ": ");
                    prices.push(Price::parse(model, Some(price)));
                }
            }
            assert!(prices.len() == 1 || prices.len() == 5);
            assert!(model.items.contains_key(item_name), "Item \"{}\" not found.", item_name);
            model.items.get_mut(item_name).unwrap().prices = prices;
        }
    }

    pub fn load_inventory(model: &mut Model) {
        let file = File::open(FILE_INVENTORY).unwrap();
        let reader = BufReader::new(file);
        for line in reader.lines()
                .map(|line| line.unwrap().trim().to_string())
                .filter(|line| !line.starts_with(PREFIX_COMMENT)) {
            let (item_name, quantity, upgrade_level) = if line.contains("[") {
                // Something like "Climbing Boots [2]" showing an upgrade level.
                let (item_name, rest) = split_2(&line, " [");
                let (upgrade_level, _) = split_2(rest, "]");
                let upgrade_level = usize::from_str(upgrade_level).expect(&format!("Could not parse the upgrade level \"{}\" in the line \"{}\"", &upgrade_level, &line));
                (item_name, 1, upgrade_level)
            } else {
                // Something like "Fire Keese Wing: 23" showing a quantity.
                let (quantity, item_name) = rsplit_2(&line, ": ");
                let quantity = usize::from_str(quantity).expect(&format!("Could not parse the quantity \"{}\" in the line \"{}\"", &quantity, &line));
                (item_name, quantity, 0)
            };
            let item = model.items.get_mut(item_name).unwrap();
            item.quantity = quantity;
            item.upgrade_level = upgrade_level;
        }
    }

    pub fn max_upgrade_level(&self) -> usize {
        if self.prices.len() > 1 { 4 } else { 0 }
    }

    pub fn has_purchase_price(&self) -> bool {
        self.prices.len() > 0 && self.prices.get(0).unwrap().components.len() > 0
    }

    pub fn is_jewelry(&self) -> bool {
        self.name.contains("Earring") || self.name.contains("Circlet")
    }

    pub fn description(&self) -> String {
        let type_name = self.item_type.to_string();
        let has_item_indicator = if self.quantity > 0 { "* " } else { "  " };
        let quantity_upgrade = if self.item_type == ItemType::Armor {
            format!("[{}]", self.upgrade_level)
        } else if self.needed > 0 {
            format!("{} / {}", format_count(self.quantity), format_count(self.needed))
        } else {
            format_count(self.quantity)
        };
        let effect = self.effect.as_ref().map_or("".to_string(), |effect| format!("; effect = {}", effect));
        // let price = self.price.as_ref().map_or("".to_string(), |price| format!("; price = {}", price.description()));
        let line = format!("{}{} ({}): {}{}", &has_item_indicator, &self.name, &type_name, quantity_upgrade, &effect);
        line
    }

    pub fn describe_deep(&self, s: &mut String, depth: usize, _max_depth: Option<usize>) {
        let line = self.description();
        s.push_str(&format_indent_line_space(depth, &line));
        for (index, price) in self.prices.iter().enumerate() {
            if price.components.len() > 0 {
                let price_line = format!("{}: {}", index, price.description());
                s.push_str(&format_indent_line_space(depth + 1, &price_line));
            }
        }
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.sequence.cmp(&other.sequence)
    }
}

impl fmt::Display for ItemType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Price {

    pub fn new() -> Price {
        Price {
            components: vec![],
        }
    }

    pub fn add_price(&mut self, other: &Price) {
        for other_part in other.components.iter() {
            self.add_part(other_part);
        }
    }

    pub fn add_part(&mut self, other_part: &CountedComponent) {
        match self.find_same_reference(other_part) {
            Some(found_index) => {
                self.components.get_mut(found_index).unwrap().add_part(other_part);
            },
            None => {
                self.components.push(other_part.clone());
                self.components.sort_unstable();
            }
        }
    }

    fn find_same_reference(&self, other_part: &CountedComponent) -> Option<usize> {
        for (index, part) in self.components.iter().enumerate() {
            if part.same_reference(other_part) {
                return Some(index);
            }
        }
        None
    }

    fn parse(model: &Model, s: Option<&str>) -> Self {
        let mut price = Price::new();
        if let Some(s) = s {
            for split in s.split(", ") {
                price.components.push(CountedComponent::parse(model, split));
            }
        }
        price
    }

    pub fn description(&self) -> String {
        self.components.iter().map(|part| part.description()).collect::<Vec<String>>().join(", ")
    }

    pub fn print_report(&self, label: &str) {
        let max_qty = self.components.iter()
            .map(|part| part.quantity)
            .max()
            .unwrap() as usize;
        let formatted_number_len = format_count(max_qty).len();
        let mut s = String::new();
        s.push_str(&format!("\n\n{}\n", label));
        for part in self.components.iter() {
            let quantity = part.quantity;
            let quantity = if quantity > 0 { format_count(quantity)} else { "".to_string() };
            s.push_str(&format!("\n{:>width$} {}", quantity, part.component_name(), width=formatted_number_len));
        }
        report_to_file(&s);
    }

}

impl fmt::Display for Price {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.description())
    }
}

impl CountedComponent {

    pub fn same_reference(&self, other: &CountedComponent) -> bool {
        self.component_reference == other.component_reference
    }

    pub fn reference_name(&self) -> &str {
        &self.component_reference.name()
    }

    pub fn add_part(&mut self, other: &CountedComponent) {
        assert!(self.same_reference(other));
        self.quantity += other.quantity;
    }

    fn parse(model: &Model, s: &str) -> Self {
        if s.chars().next().unwrap().is_numeric() {
            let (quantity, name) = split_2(s, " ");
            let quantity = usize::from_str(quantity).unwrap();
            let component_reference = ComponentReference::new_item(model, name);
            return Self {
                component_reference,
                quantity,
            }
        } else {
            if s.contains("Shrine") {
                return Self {
                    component_reference: ComponentReference::new_shrine(s),
                    quantity: 0,
                };
            } else {
                return Self {
                    component_reference: ComponentReference::new_quest(s),
                    quantity: 0,
                }
            }
        }
    }

    pub fn description(&self) -> String {
        let quantity = if self.quantity > 0 {
            format!("{} ", format_count(self.quantity ))
        } else {
            "".to_string()
        };
        format!("{}{}", quantity, self.component_name())
    }

    pub fn component_name(&self) -> &str {
        &self.component_reference.name()
    }

}

impl Effect {
    pub fn from_description(description: &str) -> Option<Self> {
        for (substring, effect) in [
            ("additional hearts", Effect::TemporaryHearts),
            ("hearts restored", Effect::RestoreHearts),
            ("attack", Effect::Attack),
            ("defense", Effect::Defense),
            ("temporary stamina", Effect::TemporaryStamina),
            ("stamina restor", Effect::RestoreStamina),
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
            Effect::ShockResistance => "Shock Resistance",
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
    }
}

