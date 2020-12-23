#![allow(unused_imports)]
#![allow(dead_code)]

use std::rc::Rc;
use std::cell::{RefCell, Ref};

use strum;
use strum_macros::EnumString;

use std::borrow::Borrow;
use std::collections::HashMap;
use std::cmp::Ordering;
use std::fs::File;
use std::io::Write;
use itertools::sorted;
use std::fmt::Display;
use serde::export::Formatter;
use serde::export::fmt::Error;
use std::{cmp, fmt};

use crate::*;
use super::parse;

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
    let start = std::time::Instant::now();
    let mut model = Model::new();
    util::format::print_elapsed_from_start(true, "new", "", start);

    model.try_load();
    dbg!(&model);
}

#[derive(Debug)]
pub struct Model {
    pub components: BTreeMap<String, Component>,
}

#[derive(Debug)]
pub struct Component {
    pub name: String,
    pub mentioned: bool,
    pub type_: ComponentType,
}

#[derive(Debug)]
pub enum ComponentType {
    Character {
        race: Race,
        main: bool,
        champion: bool,
        merchant: bool,
        alive: bool,
        met: bool,
        met_in_flashback: bool,
    },
    Location {
        region: Region,
        discovered: bool,
        type_: LocationType,
    },
    /*
    Item {
        type_: ItemType,
        quantity: usize,
        needed: usize,
        effect: Option<Effect>,
        upgrade_level: usize,
        prices: Vec<Price>,
        is_monster_part: bool,
        mon_sell_price: Option<usize>,
    },
    */
    Quest {
        type_: QuestType,
        started: bool,
        completed: bool,
    },
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

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
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

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum LocationType {
    Shrine {
        challenge: String,
        completed: bool,
    },
    Tower,
    Town,
    Stable,
}

#[derive(Clone, Debug, EnumString)]
pub enum QuestType {
    Main,
    Side {
        notes: Option<String>,
    },
    Shrine {
        shrine_name: String,
    },
}

#[derive(Clone, Debug, EnumString, Eq, Ord, PartialEq, PartialOrd)]
pub enum Race {
    Amiibo,
    GreatFairy,
    Deity,
    Demon,
    Gerudo,
    Goron,
    Horse,
    Hylian,
    Korok,
    Rito,
    SandSeal,
    Sheikah,
    SheikahMonk,
    Yiga,
    Zora,
}

#[derive(Debug)]
pub enum Region {
    Akkala,
    Central,
    DuelingPeaks,
    Eldin,
    Faron,
    Gerudo,
    GreatPlateau,
    Hateno,
    Hebra,
    Lake,
    Lanayru,
    Ridgeland,
    Tabantha,
    Woodland,
    Wasteland,
    ShrinePlaceholder,
}

impl Model {
    pub fn new() -> Self {
        let mut model = Model {
            components: BTreeMap::new(),
        };
        parse::load_characters(&mut model);
        parse::load_shrines(&mut model);
        parse::load_locations(&mut model);
        parse::load_quests(&mut model);
        // parse::load_items(&mut model);

        // Shrine::load_shrines(&mut model);
        model
    }

    pub fn try_load(&mut self) {
        let start = std::time::Instant::now();
        // Item::load_inventory(self);
        util::format::print_elapsed_from_start(true, "load inventory", "", start);

        // let include_jewelry = false;
        // self.set_needed_items(include_jewelry);
        // self.describe();
        // self.show_remaining_price_report(include_jewelry);
        // model.gen_inventory_file();
        // model.report_effect(Effect::TemporaryHearts);
        // model.report_effect(Effect::Stealth);
        // model.report_effect(Effect::MovementSpeed);
        // model.report_upgrade_and_acquire();
    }

    pub fn add_component(&mut self, component: Component) {
        let key = component.name.clone();
        assert!(key.trim().len() == key.len(), "component name \"{}\" is not trimmed.", &key);
        assert!(!self.components.contains_key(&key));
        self.components.insert(key, component);
    }
}

/*
pub fn describe(&self) {
    let mut s = "".to_string();
    self.describe_deep(& mut s, 0, None);
    report_to_file(&s);
}
*/

    /*
    pub fn items_in_order(&self) -> Vec<&Item> {
        let mut v: Vec<&Item> = self.items.values().collect();
        v.sort_by_key(|item| item.sequence);
        v
    }
    */

    /*
    pub fn describe_deep(&self, s: &mut String, depth: usize, _max_depth: Option<usize>) {
        s.push_str(&format_indent_line_space(depth, "Botw Model"));
        // let mut v: Vec<&Item> = self.items.values().collect();
        // v.sort_by_key(|item| item.sequence);
        for item in sorted(self.items.values()) {
            // let item: Ref<'_, Item<'a>> = item.borrow();
            item.describe_deep(s, depth + 1, _max_depth);
        }
        for shrine in sorted(self.shrines.values()) {
            shrine.describe_deep(s, depth + 1, _max_depth);
        }
    }

    fn all_prices(&self) -> Price {
        let mut price = Price::new();
        for item in self.items.values() {
            for one_price in item.prices.iter() {
                for part in one_price.components.iter() {
                    price.add_part(&part);
                }
            }
        }
        price
    }

    fn remaining_prices(&self, include_jewelry: bool) -> Price {
        let mut price = Price::new();
        for item in self.items.values().filter(|item| include_jewelry || !item.is_jewelry()) {
            let min_index = if item.quantity == 0 {
                0
            } else {
                item.upgrade_level + 1
            };
            for one_price in item.prices.iter().skip(min_index) {
                for part in one_price.components.iter() {
                    price.add_part(&part);
                }
            }
        }
        price
    }

    fn show_all_price_report(&self) {
        self.all_prices().print_report("Total Cost for All Purchases and Upgrades");
    }

    fn show_remaining_price_report(&self, include_jewelry: bool) {
        let note = if include_jewelry { "" } else { " (excluding jewelry)"};
        self.remaining_prices(include_jewelry).print_report(&format!("{}{}", "Remaining Cost for All Purchases and Upgrades", note));
    }

    fn set_needed_items(&mut self, include_jewelry: bool) {
        for comp in self.remaining_prices(include_jewelry)
            .components
            .iter()
            .filter(|comp| comp.component_reference.is_item()) {
            self.items.get_mut(comp.component_reference.name()).unwrap().needed = comp.quantity;
        }
    }

    fn gen_inventory_file(&self) {
        // We want a list of all armor, other items that can be purchased, and items that are needed for purchases and
        // upgrades, along with special items.
        let items_in_prices: Vec<String> = self.all_prices()
            .components
            .iter()
            .filter(|comp| comp.quantity > 0)
            .map(|comp| comp.component_name().to_string())
            .collect();
        let mut v: Vec<&Item> = self.items.values().collect();
        v.sort_by_key(|item| item.sequence);
        for item in v.iter()
            .filter(|item|
                item.item_type == ItemType::Armor
                    || item.item_type == ItemType::KeyItem
                    || item.prices.len() > 0
                    || items_in_prices.contains(&item.name)) {
            let line = if item.item_type == ItemType::Armor {
                format!("// {} [0]", item.name)
            } else {
                format!("{}: 0", item.name)
            };
            println!("{}", &line);
        }

    }

    pub fn report_effect(&self, effect: Effect) {
        let mut s = String::new();
        s.push_str(&format!("\n{}\n\nMaterials:\n", effect));
        for item in self.items_in_order().iter()
            .filter(|item| item.item_type == ItemType::Material && item.effect == Some(effect)) {
            s.push_str(&format!("\n\t{}", item.description()));
        }
        s.push_str(&format!("\n\nArmor:\n"));
        for item in self.items_in_order().iter()
            .filter(|item| item.item_type == ItemType::Armor && item.effect == Some(effect) ) {
            s.push_str(&format!("\n\t{}", item.description()));
        }
        s.push_str(&format!("\n\nMonster Part Needs:\n"));
        for item in self.items_in_order().iter()
            .filter(|item| item.is_monster_part && item.quantity > 0 ) {
            s.push_str(&format!("\n\t{}", item.description()));
        }
        report_to_file(&s);
    }

    pub fn report_upgrade_and_acquire(&self) {
        let mut s = String::new();
        s.push_str(&format!("\n\nArmor to Upgrade:\n"));
        for item in self.items_in_order().iter()
            .filter(|item| item.item_type == ItemType::Armor && item.quantity > 0 && item.max_upgrade_level() > item.upgrade_level) {
            s.push_str(&format!("\n\t{} [{}]:", item.name, item.upgrade_level));
            let price = item.prices.get(item.upgrade_level + 1).unwrap();
            self.add_price_to_report(&mut s, price, "\t\t");
        }
        s.push_str(&format!("\n\nItems to Acquire:\n"));
        for item in self.items_in_order().iter()
            .filter(|item| item.has_purchase_price() && item.quantity == 0) {
            s.push_str(&format!("\n\t{}", item.name));
            let price = item.prices.get(0).unwrap();
            self.add_price_to_report(&mut s, price, "\t\t");
        }
        report_to_file(&s);
    }

    fn add_price_to_report(&self, s: &mut String, price: &Price, prefix: &str) {
        for comp in price.components.iter() {
            if comp.component_reference.is_item() {
                let current_count = self.items.get(comp.component_name()).unwrap().quantity;
                let needed_quantity = comp.quantity;
                let star = if current_count >= needed_quantity { "* " } else { "  " };
                let current_count = format_count(current_count);
                let needed_quantity = format_count(needed_quantity);
                let item_name = comp.component_name();
                if item_name == "Mon" || item_name == "Rupee" {
                    s.push_str(&format!("\n{}{}{} {}", prefix, star, needed_quantity, item_name));
                } else {
                    s.push_str(&format!("\n{}{}{}/{} {}", prefix, star, current_count, needed_quantity, item_name));
                }
            } else {
                s.push_str(&format!("\n{}  {}", prefix, comp.component_name()));
            }
        }
    }

    pub fn partial_match_references(&self, partial: &str, include_items: bool, include_shrines: bool, include_quests: bool, include_locations: bool) -> Vec<ComponentReference> {
        let mut v = vec![];
        let partial = partial.trim().to_lowercase();
        // Sort by the referenced component. For shrines, quests, and locations this will mean
        // sorting by name but for items this will be by sequence, the order in which items appear
        // in the Hyrule Compendium. Note that the closing paren of sorted() is before the map() so
        // we're sorting by the ComponentReference rather than necessarily its name.
        if include_items {
            for name in sorted(self.items
                .values()
                .filter(|x| x.name.to_lowercase().contains(&partial)))
                .map(|x| &x.name) {
                v.push(ComponentReference::new_item(self, name))
            }
        }
        if include_shrines {
            for name in sorted(self.shrines
                .values()
                .filter(|x| x.name.to_lowercase().contains(&partial)))
                .map(|x| &x.name) {
                v.push(ComponentReference::new_shrine(name))
            }
        }
        if include_quests {
            for name in sorted(self.quests
                .values()
                .filter(|x| x.name.to_lowercase().contains(&partial)))
                .map(|x| &x.name) {
                v.push(ComponentReference::new_quest(name))
            }
        }
        if include_locations {
            for name in sorted(self.locations
                .values()
                .filter(|x| x.name.to_lowercase().contains(&partial)))
                .map(|x| &x.name) {
                v.push(ComponentReference::new_location(name))
            }
        }
        v
    }
    */


/*
impl ComponentReference {

    pub fn new_item(model: &Model, name: &str) -> Self {
        let sequence = model.items.get(name).map_or(0, |item| item.sequence);
        ComponentReference::Item {
            name: name.to_string(),
            sequence,
        }
    }

    pub fn new_shrine(name: &str) -> Self {
        ComponentReference::Shrine {
            name: name.to_string()
        }
    }

    pub fn new_quest(name: &str) -> Self {
        ComponentReference::Quest {
            name: name.to_string()
        }
    }

    pub fn new_location(name: &str) -> Self {
        ComponentReference::Location {
            name: name.to_string()
        }
    }

    pub fn is_item(&self) -> bool {
        match self {
            ComponentReference::Item { name: _, sequence: _ } => true,
            _ => false,
        }
    }

    pub fn is_shrine(&self) -> bool {
        match self {
            ComponentReference::Shrine { name: _ } => true,
            _ => false,
        }
    }

    pub fn is_quest(&self) -> bool {
        match self {
            ComponentReference::Quest { name: _ } => true,
            _ => false,
        }
    }

    pub fn is_location(&self) -> bool {
        match self {
            ComponentReference::Location { name: _ } => true,
            _ => false,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            ComponentReference::Item { name, sequence: _ } => &name,
            ComponentReference::Shrine { name} => &name,
            ComponentReference::Quest { name} => &name,
            ComponentReference::Location { name} => &name,
        }
    }
}

impl Ord for ComponentReference {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (ComponentReference::Item { name: _, sequence: self_sequence }, ComponentReference::Item { name: _, sequence: other_sequence })  => self_sequence.cmp(&other_sequence),
            _ => self.name().cmp(&other.name()),
        }
    }
}

impl PartialOrd for ComponentReference {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ComponentReference {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}

impl Eq for ComponentReference {}

impl Display for ComponentReference {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            ComponentReference::Item { name, sequence } => write!(f, "Item: {} [{}]", name, sequence),
            ComponentReference::Shrine { name} => write!(f, "Shrine: {}", name),
            ComponentReference::Quest { name} => write!(f, "Quest: {}", name),
            ComponentReference::Location { name} => write!(f, "Location: {}", name),
        }
    }
}

*/

/*
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
*/

