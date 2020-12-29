// https://www.joshmcguigan.com/blog/build-your-own-shell-rust/

use serde::export::Formatter;
use serde::export::fmt::Error;
use std::fmt::Display;
use std::{thread, time};

use super::model::*;
use super::runtime::GameClock;
use util_rust::format;

pub const NULL_TIME: usize = usize::MAX;

#[derive(Debug)]
pub struct GameRecord {
    pub name: String,
    pub events: Vec<GameEvent>,
}

#[derive(Clone, Debug)]
pub struct GameEvent {
    time: usize,
    typ: GameEventType,
    name: String,
    number: Option<usize>,
    previous_number: Option<usize>,
}

#[derive(serde::Serialize)]
#[derive(Clone, Debug)]
pub enum GameEventType {
    AddToCompendium,
    BloodMoon,
    CharacterDeath,
    CompleteQuest,
    CompleteShrine,
    DiscoverLocation,
    FindDogTreasure,
    IdentifyItem,
    KorokSeed,
    LightFlame,
    LinkDeath,
    MeetCharacter,
    MeetCharacterFlashback,
    MentionCharacter,
    OpenChest,
    SetArmorLevel,
    SetBowSlots,
    SetHearts,
    SetItemCount,
    SetShieldSlots,
    SetStamina,
    SetWeaponSlots,
    StartQuest,
    StartShrine,
}

impl GameRecord {

    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            events: vec![],
        }
    }

    pub fn add_event(&mut self, model: &mut Model, event: GameEvent) {
        dbg!(&event);
        let mut events: Vec<GameEvent> = vec![];
        events.push(event.clone());
        let mut current_event = event;
        // let mut previous_event_string = event.to_simple_text();
        loop {
            let current_event_opt = current_event.gen_predecessor(model);
            dbg!(&current_event);
            match current_event_opt {
                Some(this_event) => {
                    //assert_ne!(event.to_simple_text(), previous_event_string);
                    //previous_event_string = event.to_simple_text();
                    events.push(this_event.clone());
                    current_event = this_event;
                },
                None => {
                    break;
                }
            }
        }
        events.reverse();
        for one_event in events.iter_mut() {
            one_event.apply(model);
        }
        self.events.append(&mut events);
    }

    pub fn review(&mut self, event_count: usize) {
        let first_index = if event_count >= self.events.len() {
            0
        } else {
            self.events.len() - (event_count - 1)
        };
        for event in self.events[first_index..].iter() {
            println!("{}", event);
        }
    }

    pub fn print_events_serialized(&self) {
        format::println_indent_tab(0, &self.name);
        for event in self.events.iter() {
            format::println_indent_tab(1, &event.to_simple_text());
        }
    }
}

impl GameEvent {
    pub fn new(time: usize, typ: GameEventType, name: &str, number: Option<usize>) -> Self {
        Self {
            time,
            typ,
            name: name.to_string(),
            number,
            previous_number: None,
        }
    }

    pub fn to_simple_text(&self) -> String {
        let number = self.number.map_or("None".to_string(), |x| x.to_string());
        let previous_number = self.previous_number.map_or("None".to_string(), |x| x.to_string());
        format!("{}\t{}\t{}\t{}\t{}", self.time, self.typ.variant_to_string(), self.name, number, previous_number)
    }

    pub fn gen_predecessor(&self, model: &Model) -> Option<GameEvent> {
        dbg!(&self);
        let GameEvent { time, typ, name, .. } = self;
        match typ {
            GameEventType::CompleteQuest => {
                if !model.get_quest(name).is_started() {
                    Some(GameEvent::new(*time, GameEventType::StartQuest, name, None))
                } else {
                    None
                }
            },
            GameEventType::CompleteShrine => {
                dbg!(model.get_shrine(name));
                dbg!(model.get_shrine(name).is_started());
                if !model.get_shrine(name).is_started() {
                    Some(GameEvent::new(*time, GameEventType::StartShrine, name, None))
                } else {
                    None
                }
            },
            GameEventType::DiscoverLocation => {
                let parent_location = model.get_parent_location(name);
                //bg!(&parent_location);
                match parent_location {
                    Some(parent_location) => {
                        if !parent_location.is_discovered() {
                            Some(GameEvent::new(*time, GameEventType::DiscoverLocation, &parent_location.name, None))
                        } else {
                            None
                        }
                    },
                    None => None
                }
            },
            GameEventType::FindDogTreasure | GameEventType::LightFlame => {
                if !model.get_location(name).is_discovered() {
                    Some(GameEvent::new(*time, GameEventType::DiscoverLocation, name, None))
                } else {
                    None
                }
            },
            GameEventType::StartShrine => {
                let quest_name = &model.get_location(name).quest;
                match quest_name {
                    Some(quest_name) => {
                        if !model.get_quest(&quest_name).is_completed() {
                            Some(GameEvent::new(*time, GameEventType::CompleteQuest, &quest_name, None))
                        } else {
                            None
                        }
                    },
                    None => None
                }
            },
            _ => None
        }
    }

    pub fn apply(&mut self, model: &mut Model) {
        let GameEvent { time, typ, name, number, ref mut previous_number } = self;
        match typ {
            GameEventType::BloodMoon => {
                model.blood_moons += 1;
            },
            GameEventType::CompleteQuest => {
                model.get_quest_mut(name).completed_time = *time;
            },
            GameEventType::CompleteShrine => {
                model.get_shrine_mut(name).completed_time = *time;
            },
            GameEventType::DiscoverLocation => {
                model.get_location_mut(name).discovered_time = *time;
            },
            GameEventType::FindDogTreasure => {
                model.get_location_mut(name).dog_treasure_found_time = *time;
            },
            GameEventType::LightFlame => {
                model.get_location_mut(name).flame_lit_time = *time;
            },
            GameEventType::MeetCharacter => {
                model.get_character_mut(name).met_time = *time;
            },
            GameEventType::MeetCharacterFlashback => {
                model.get_character_mut(name).met_in_flashback_time = *time;
            },
            GameEventType::MentionCharacter => {
                model.get_character_mut(name).mentioned_time = *time;
            },
            GameEventType::SetHearts => {
                *previous_number = Some(model.hearts);
                model.hearts = number.unwrap();
            },
            GameEventType::SetStamina => {
                *previous_number = Some(model.stamina);
                model.stamina = number.unwrap();
            },
            GameEventType::StartQuest => {
                model.get_quest_mut(name).started_time = *time;
            },
            GameEventType::StartShrine => {
                model.get_shrine_mut(name).started_time = *time;
            },
            _ => unimplemented!()
        }
    }

}

impl Display for GameEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let type_details = match self.typ {
            GameEventType::AddToCompendium => format!("Added {} to compendium.", self.name),
            GameEventType::BloodMoon => "Blood moon.".to_string(),
            GameEventType::CharacterDeath => format!("{} died.", self.name),
            GameEventType::CompleteQuest | GameEventType::CompleteShrine => format!("Completed {}.", self.name),
            GameEventType::DiscoverLocation => format!("Discovered {}.", self.name),
            GameEventType::FindDogTreasure => format!("Found dog treasure at {}.", self.name),
            GameEventType::IdentifyItem => format!("Identified {}.", self.name),
            GameEventType::KorokSeed => format!("Korok seeds to {}.", self.number.unwrap()),
            GameEventType::LightFlame => format!("Lit flame at {}.", self.name),
            GameEventType::LinkDeath => format!("Link deaths to {}.", self.number.unwrap()),
            GameEventType::MeetCharacter => format!("Met {}.", self.name),
            GameEventType::MeetCharacterFlashback => format!("Met {} in a flashback.", self.name),
            GameEventType::MentionCharacter => format!("Mentioned {}.", self.name),
            GameEventType::OpenChest => format!("Opened chests to {}.", self.number.unwrap()),
            GameEventType::SetArmorLevel => format!("Changed {} from {} to {}.", self.name, self.previous_number.unwrap(), self.number.unwrap()),
            GameEventType::SetBowSlots => format!("Bow slots to {}.", self.number.unwrap()),
            GameEventType::SetHearts => format!("Changed hearts from {} to {}.", self.previous_number.unwrap(), self.number.unwrap()),
            GameEventType::SetItemCount => format!("Changed the count for {} from {} to {}.", self.name, self.previous_number.unwrap(), self.number.unwrap()),
            GameEventType::SetShieldSlots => format!("Shield slots to {}.", self.number.unwrap()),
            GameEventType::SetStamina => format!("Changed stamina from {} to {}.", self.previous_number.unwrap(), self.number.unwrap()),
            GameEventType::SetWeaponSlots => format!("Weapon slots to {}.", self.number.unwrap()),
            GameEventType::StartQuest | GameEventType::StartShrine => format!("Started {}.", self.name),
        };
        let s = format!("{:?}: {}", GameClock::format_time(self.time), type_details);
        write!(f, "{}", s)
    }
}

impl GameEventType {
    pub fn variant_to_string(&self) -> &str {
        match self {
            GameEventType::AddToCompendium => "AddToCompendium",
            GameEventType::BloodMoon => "BloodMoon",
            GameEventType::CharacterDeath => "CharacterDeath",
            GameEventType::CompleteQuest => "CompleteQuest",
            GameEventType::CompleteShrine => "CompleteShrine",
            GameEventType::DiscoverLocation => "DiscoverLocation",
            GameEventType::FindDogTreasure => "FindDogTreasure",
            GameEventType::IdentifyItem => "IdentifyItem",
            GameEventType::KorokSeed => "KorokSeed",
            GameEventType::LightFlame => "LightFlame",
            GameEventType::LinkDeath => "LinkDeath",
            GameEventType::MeetCharacter => "MeetCharacter",
            GameEventType::MeetCharacterFlashback => "MeetCharacterFlashback",
            GameEventType::MentionCharacter => "MentionCharacter",
            GameEventType::OpenChest => "OpenChest",
            GameEventType::SetArmorLevel => "SetArmorLevel",
            GameEventType::SetBowSlots => "SetBowSlots",
            GameEventType::SetHearts => "SetHearts",
            GameEventType::SetItemCount => "SetItemCount",
            GameEventType::SetShieldSlots => "SetShieldSlots",
            GameEventType::SetStamina => "SetStamina",
            GameEventType::SetWeaponSlots => "SetWeaponSlots",
            GameEventType::StartQuest => "StartQuest",
            GameEventType::StartShrine => "StartShrine",
        }
    }

    pub fn string_to_variant(s: &str) -> Self {
        match s {
            "AddToCompendium" => GameEventType::AddToCompendium,
            "BloodMoon" => GameEventType::BloodMoon,
            "CharacterDeath" => GameEventType::CharacterDeath,
            "CompleteQuest" => GameEventType::CompleteQuest,
            "CompleteShrine" => GameEventType::CompleteShrine,
            "DiscoverLocation" => GameEventType::DiscoverLocation,
            "FindDogTreasure" => GameEventType::FindDogTreasure,
            "IdentifyItem" => GameEventType::IdentifyItem,
            "KorokSeed" => GameEventType::KorokSeed,
            "LightFlame" => GameEventType::LightFlame,
            "LinkDeath" => GameEventType::LinkDeath,
            "MeetCharacter" => GameEventType::MeetCharacter,
            "MeetCharacterFlashback" => GameEventType::MeetCharacterFlashback,
            "MentionCharacter" => GameEventType::MentionCharacter,
            "OpenChest" => GameEventType::OpenChest,
            "SetArmorLevel" => GameEventType::SetArmorLevel,
            "SetBowSlots" => GameEventType::SetBowSlots,
            "SetHearts" => GameEventType::SetHearts,
            "SetItemCount" => GameEventType::SetItemCount,
            "SetShieldSlots" => GameEventType::SetShieldSlots,
            "SetStamina" => GameEventType::SetStamina,
            "SetWeaponSlots" => GameEventType::SetWeaponSlots,
            "StartQuest" => GameEventType::StartQuest,
            "StartShrine" => GameEventType::StartShrine,
            _ => panic!(format!("Unexpected GameEventType variant name \"{}\".", s))
        }
    }
}

pub fn try_create_events() {
    let mut model = Model::new();
    let mut game_record = GameRecord::new("Test");
    let clock = GameClock::new_running(1_000);

    game_record.add_event(&mut model,GameEvent::new(clock.time(), GameEventType::DiscoverLocation, "Phalian Highlands", None));

    // clock.add_seconds(1234);

    //bg!(&game_record);

    thread::sleep(time::Duration::from_secs(3));

    // Complete a shrine that has not been started and which has a quest that has not been started.
    // This should result in four events: start quest, complete quest, start shrine, complete shrine.
    game_record.add_event(&mut model, GameEvent::new(clock.time(), GameEventType::CompleteShrine, "Mezza Lo Shrine", None));

    // TO DO: Consider handling the case where there are multiple predecessor events. For instance,
    // starting a shrine means the location must be discovered but also if there's a quest it must
    // be completed. Maybe the predecessor function should return an array.

    game_record.print_events_serialized();

}
