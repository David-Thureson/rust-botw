#![allow(unused_imports)]
#![allow(dead_code)]

use strum;
use strum_macros::EnumString;
use strum_macros::ToString;
use std::string::ToString;

use std::borrow::Borrow;
use std::collections::HashMap;
use std::cmp::Ordering;
use std::fs::File;
use std::io::Write;
use itertools::{sorted, Itertools};
use std::fmt::Display;
use serde::export::Formatter;
use serde::export::fmt::Error;
use std::{cmp, fmt};

use crate::*;
use super::parse;
use super::game_record::GameEvent;
use super::game_record::NULL_TIME;

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
    let model = Model::new();
    util::date_time::print_elapsed_from_start(true, "new", "", start);

    model.report_characters();
    model.report_location_types();
    model.report_shrines();
    model.report_quest_types();

    // model.try_load();
    //bg!(&model);
}

#[derive(Debug)]
pub struct Model {
    pub hearts: usize,
    pub stamina: usize,
    pub weapon_slots: usize,
    pub bow_slots: usize,
    pub shield_slots: usize,
    pub korok_seeds: usize,
    pub chests: usize,
    pub deaths: usize,
    pub blood_moons: usize,
    pub characters: BTreeMap<String, Character>,
    pub locations: BTreeMap<String, Location>,
    pub quests: BTreeMap<String, Quest>,
}

#[derive(Debug)]
pub struct Character {
    pub name: String,
    pub race: Race,
    pub main: bool,
    pub champion: bool,
    pub merchant: bool,
    pub alive: bool,
    pub mentioned_time: usize,
    pub met_time: usize,
    pub met_in_flashback_time: usize,
}

#[derive(Debug)]
pub struct Location {
    pub name: String,
    pub typ: LocationType,
    pub parent_location: Option<String>,
    pub dog_treasure: Option<String>,
    pub discovered_time: usize,
    pub dog_treasure_found_time: usize,
    pub challenge: Option<String>,
    pub quest: Option<String>,
    pub started_time: usize,
    pub completed_time: usize,
    pub flame_lit_time: usize,
    pub child_locations: Vec<String>,
}

// #[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
#[derive(Debug)]
pub enum LocationType {
    Region,
    Area,
    Shrine,
    Tower,
    TechLab,
    Town,
    Stable,
    Normal,
}

/*
pub struct Item {
    pub name: String,
    pub typ: ItemType,
    pub quantity: usize,
    pub needed: usize,
    pub effect: Option<Effect>,
    pub upgrade_level: usize,
    pub prices: Vec<Price>,
    pub is_monster_part: bool,
    pub mon_sell_price: Option<usize>,
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

*/

#[derive(Debug)]
pub struct Quest {
    pub name: String,
    pub typ: QuestType,
    pub started_time: usize,
    pub completed_time: usize,
    pub notes: Option<String>,
    pub shrine: Option<String>,
}

#[derive(Clone, Debug)]
// #[derive(EnumString)]
pub enum QuestType {
    Main,
    Side,
    Shrine,
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

#[derive(Clone, Debug, EnumString, Eq, Ord, PartialEq, PartialOrd, ToString)]
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

impl Model {
    pub fn new() -> Self {
        let mut model = Self {
            hearts: 3,
            stamina: 5,
            weapon_slots: 8,
            bow_slots: 5,
            shield_slots: 4,
            korok_seeds: 0,
            chests: 0,
            deaths: 0,
            blood_moons: 0,
            characters: Default::default(),
            locations: Default::default(),
            quests: Default::default(),
        };
        parse::load_characters(&mut model);
        parse::load_locations(&mut model);
        parse::load_quests(&mut model);
        // parse::load_items(&mut model);

        // Shrine::load_shrines(&mut model);

        //bg!(&model);

        model
    }

    pub fn add_character(&mut self, character: Character) {
        let key = character.name.to_lowercase().to_string();
        assert!(key.trim().len() == key.len(), "character name \"{}\" is not trimmed.", &key);
        assert!(!self.characters.contains_key(&key));
        self.characters.insert(key, character);
    }

    pub fn add_location(&mut self, location: Location) {
        let key = location.name.to_lowercase().to_string();
        assert!(key.trim().len() == key.len(), "location name \"{}\" is not trimmed.", &key);
        assert!(!self.locations.contains_key(&key), "Location {} already exists.", key);
        self.locations.insert(key, location);
    }

    pub fn add_quest(&mut self, quest: Quest) {
        let key = quest.name.to_lowercase().to_string();
        assert!(key.trim().len() == key.len(), "quest name \"{}\" is not trimmed.", &key);
        assert!(!self.quests.contains_key(&key));
        self.quests.insert(key, quest);
    }

    pub fn get_character<'a>(&'a self, name: &str) -> &'a Character {
        let get = self.characters.get(&name.to_lowercase());
        match get {
            Some(character) => character,
            None => panic!("Unknown character = \"{}\"", name),
        }
    }

    pub fn get_character_mut<'a>(&'a mut self, name: &str) -> &'a mut Character {
        let get = self.characters.get_mut(&name.to_lowercase());
        match get {
            Some(character) => character,
            None => panic!("Unknown character = \"{}\"", name),
        }
    }

    pub fn get_location<'a>(&'a self, name: &str) -> &'a Location {
        let get = self.locations.get(&name.to_lowercase());
        match get {
            Some(location) => location,
            None => panic!("Unknown location = \"{}\"", name),
        }
    }

    pub fn get_location_mut<'a>(&'a mut self, name: &str) -> &'a mut Location {
        let get = self.locations.get_mut(&name.to_lowercase());
        match get {
            Some(location) => location,
            None => panic!("Unknown location = \"{}\"", name),
        }
    }

    pub fn get_parent_location<'a>(&'a self, name: &str) -> Option<&'a Location> {
        self.get_location(name).parent_location.as_ref().map(|parent_name| self.get_location(&parent_name))
    }

    pub fn get_shrine<'a>(&'a self, name: &str) -> &'a Location {
        let location = self.get_location(name);
        match location.typ {
            LocationType::Shrine => location,
            _ => panic!("Location \"{}\" is not a shrine.", name),
        }
    }

    pub fn get_shrine_mut<'a>(&'a mut self, name: &str) -> &'a mut Location {
        let location = self.get_location_mut(name);
        match location.typ {
            LocationType::Shrine => location,
            _ => panic!("Location \"{}\" is not a shrine.", name),
        }
    }

    pub fn get_shrine_quest<'a>(&'a self, name: &str) -> Option<&'a Quest> {
        self.get_shrine(name).quest.as_ref().map(|quest_name| self.get_quest(&quest_name))
    }

    pub fn get_quest<'a>(&'a self, name: &str) -> &'a Quest {
        let get = self.quests.get(&name.to_lowercase());
        match get {
            Some(quest) => quest,
            None => panic!("Unknown quest = \"{}\"", name),
        }
    }

    pub fn get_quest_mut<'a>(&'a mut self, name: &str) -> &'a mut Quest {
        let get = self.quests.get_mut(&name.to_lowercase());
        match get {
            Some(quest) => quest,
            None => panic!("Unknown quest = \"{}\"", name),
        }
    }

    pub fn try_load(&mut self) {
        let start = std::time::Instant::now();
        // Item::load_inventory(self);
        util::date_time::print_elapsed_from_start(true, "load inventory", "", start);

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

    pub fn report_characters(&self) {
        let mut grouper_race = util::group::Grouper::new("Character Races");
        let mut grouper_flag = util::group::Grouper::new("Character Flags");
        for character in self.characters.values() {
            grouper_race.record_entry(&character.race.to_string());
            let flags = format::list_flags_with_not(
                &["main", "champion", "merchant", "alive", "mentioned", "met", "met_in_flashback"],
                &[character.main, character.champion, character.merchant, character.alive,
                    character.is_mentioned(), character.is_met(), character.is_met_in_flashback()]);
            grouper_flag.record_entry(&flags);
        }
        grouper_race.print_by_count(0, None);
        grouper_flag.print_by_count(0, None);
    }

    pub fn report_location_types(&self) {
        let mut grouper = util::group::Grouper::new("Location Types");
        for type_name in self
            .locations
            .values()
            .map(|location| location.typ.variant_to_string()) {

            grouper.record_entry(&type_name);
        }
        grouper.print_by_count(0, None);
    }

    pub fn report_shrines(&self) {
        let mut grouper_challenge = util::group::Grouper::new("Shrine Challenges");
        let mut grouper_has_quest = util::group::Grouper::new("Shrine Has Quest");
        for (challenge, has_quest) in self
            .locations
            .values()
            .map(|location| {
                let Location { typ, parent_location: _, dog_treasure: _, discovered_time: _, dog_treasure_found_time: _, challenge, quest, .. } = location;
                match typ {
                    LocationType::Shrine => Some((challenge.as_ref().unwrap(), quest.is_some())),
                    _ => None,
                }
            })
            .filter(|entry| entry.is_some())
            .map(|entry| entry.unwrap()) {

            grouper_challenge.record_entry(&challenge);
            grouper_has_quest.record_entry(&has_quest);
        }
        grouper_challenge.print_by_count(0, None);
        grouper_has_quest.print_by_count(0, None);
    }

    pub fn report_quest_types(&self) {
        let mut grouper = util::group::Grouper::new("Quest Types");
        for type_name in self
            .quests
            .values()
            .map(|quest| quest.typ.variant_to_string()) {

            grouper.record_entry(&type_name);
        }
        grouper.print_by_count(0, None);
    }
    /*
    pub fn apply_event(event: &mut GameEvent) {


    }

    pub fn undo_event(event: &mut GameEvent) {

    }
    */
}

impl Character {
    pub fn new(name: &str, race: &Race, main: bool, champion: bool, merchant: bool, alive: bool) -> Self {
        Self {
            name: name.to_string(),
            race: race.clone(),
            main,
            champion,
            merchant,
            alive,
            mentioned_time: NULL_TIME,
            met_time: NULL_TIME,
            met_in_flashback_time: NULL_TIME,
        }
    }

    pub fn is_mentioned(&self) -> bool {
        self.mentioned_time != NULL_TIME
    }

    pub fn is_met(&self) -> bool {
        self.met_time != NULL_TIME
    }

    pub fn is_met_in_flashback(&self) -> bool {
        self.met_in_flashback_time != NULL_TIME
    }

    pub fn status_description(&self, _model: &Model) -> String {
        match (self.alive, self.is_mentioned(), self.is_met_in_flashback(), self.is_met()) {
            (false, false, false, _) => "dead, not mentioned or met",
            (false, false, true, _) => "dead, met in flashback",
            (false, true, false, _) => "dead, mentioned but not met",
            (false, true, true, _) => "dead, mentioned and met in flashback",
            (true, false, false, false) => "not mentioned or met",
            (true, false, false, true) => "met",
            (true, false, true, false) => "met in flashback",
            (true, false, true, true) => "met",
            (true, true, false, false) => "mentioned, not met",
            (true, true, false, true) => "mentioned and met",
            (true, true, true, false) => "mentioned and met in flashback",
            (true, true, true, true) => "mentioned and met",
            // (_, _, _, _) => &format!("[[Unexpected combination: {} {} {} {}]]", character.alive, character.is_mentioned(), character.is_met_in_flashback(), character.is_met()),
        }.to_string()
    }

}

impl Location {
    pub fn new(name: &str, typ: LocationType, parent_location: Option<String>) -> Self {
        Self {
            name: name.to_string(),
            discovered_time: NULL_TIME,
            typ,
            parent_location,
            dog_treasure: None,
            dog_treasure_found_time: NULL_TIME,
            challenge: None,
            quest: None,
            started_time: NULL_TIME,
            completed_time: NULL_TIME,
            flame_lit_time: NULL_TIME,
            child_locations: vec![]
        }
    }

    pub fn add_child_location(&mut self, name: &str) {
        let name = name.to_string();
        assert!(name.trim().len() == name.len(), "location name \"{}\" is not trimmed.", &name);
        assert!(!self.child_locations.contains(&name), "Location {} already exists.", name);
        self.child_locations.push(name);
    }

    pub fn is_discovered(&self) -> bool {
        self.discovered_time != NULL_TIME
    }

    pub fn discovery_label(&self) -> &str {
        if self.is_discovered() {
            "discovered"
        } else {
            "not discovered"
        }
    }

    pub fn has_dog_treasure(&self) -> bool {
        self.dog_treasure.is_some()
    }

    pub fn is_dog_treasure_found(&self) -> bool {
        self.dog_treasure_found_time != NULL_TIME
    }

    pub fn is_started(&self) -> bool {
        self.started_time != NULL_TIME
    }

    pub fn is_completed(&self) -> bool {
        self.completed_time != NULL_TIME
    }

    pub fn completion_label(&self) -> &str {
        if self.is_completed() {
            "completed"
        } else if self.is_started() {
            "started"
        } else {
            "not started"
        }
    }

    pub fn is_flame_lit(&self) -> bool {
        self.flame_lit_time != NULL_TIME
    }

    pub fn name_with_shrine_challenge(&self) -> String {
        let challenge = self.challenge.as_ref().map_or("".to_string(), |challenge| format!(" ({})", challenge));
        format!("{}{}", self.name, challenge)
    }

    pub fn status_description(&self, model: &Model) -> String {
        let discovered_note = self.discovery_label();

        let dog_treasure_note = if self.has_dog_treasure() && !self.is_dog_treasure_found() {
            "; has dog treasure".to_string()
        } else {
            "".to_string()
        };

        let shrine_note = match self.typ {
            LocationType::Shrine => format!("; shrine is {}", self.completion_label()),
            _ => "".to_string(),
        };

        let quest_note = match &self.quest {
            Some(quest_name) => format!("; quest {} is {}", quest_name, model.get_quest(&quest_name).completion_label()),
            None => "".to_string(),
        };

        let flame_note = match self.typ {
            LocationType::TechLab => {
                if self.is_flame_lit() {
                    "; flame is lit".to_string()
                } else {
                    "; flame is not yet lit".to_string()
                }
            },
            _ => "".to_string(),
        };

        format!("{}{}{}{}{}", discovered_note, dog_treasure_note, shrine_note, quest_note, flame_note)
    }

}

impl LocationType {
    pub fn variant_to_string<'a>(&self) -> &'a str {
        match self {
            LocationType::Region => "Region",
            LocationType::Area => "Area",
            LocationType::Shrine => "Shrine",
            LocationType::Tower => "Tower",
            LocationType::TechLab => "TechLab",
            LocationType::Town => "Town",
            LocationType::Stable => "Stable",
            LocationType::Normal => "Normal",
        }
    }
}

impl Quest {
    fn new(name: &str, typ: QuestType, notes: Option<String>, shrine: Option<String>) -> Self {
        Self {
            name: name.to_string(),
            typ,
            started_time: NULL_TIME,
            completed_time: NULL_TIME,
            notes,
            shrine,
        }
    }

    pub fn new_main(name: &str) -> Self {
        Self::new(name, QuestType::Main, None, None)
    }

    pub fn new_side(name: &str, notes: Option<String>) -> Self {
        Self::new(name, QuestType::Side, notes, None)
    }

    pub fn new_shrine(name: &str, shrine: &str) -> Self {
        Self::new(name, QuestType::Shrine, None, Some(shrine.to_string()))
    }

    pub fn is_started(&self) -> bool {
        self.started_time != NULL_TIME
    }

    pub fn is_completed(&self) -> bool {
        self.completed_time != NULL_TIME
    }

    pub fn completion_label(&self) -> &str {
        if self.is_completed() {
            "completed"
        } else if self.is_started() {
            "started"
        } else {
            "not started"
        }
    }

    pub fn status_description(&self, model: &Model) -> String {

        let completion_note = self.completion_label();

        let type_note = match self.typ {
            QuestType::Main => "main quest".to_string(),
            QuestType::Side => "side quest".to_string(),
            QuestType::Shrine => {
                let shrine_name = self.shrine.as_ref().unwrap();
                let shrine_completion = model.get_shrine(&shrine_name).completion_label();
                format!("shrine quest for {} ({})", shrine_name, shrine_completion)
            },
        };

        format!("{}: {}", completion_note, type_note)
    }

}

impl QuestType {
    pub fn variant_to_string<'a>(&self) -> &'a str {
        match self {
            QuestType::Main => "Main",
            QuestType::Side => "Side",
            QuestType::Shrine => "Shrine",
        }
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

