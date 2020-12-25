// https://www.joshmcguigan.com/blog/build-your-own-shell-rust/

use serde::export::Formatter;
use serde::export::fmt::Error;
use std::fmt::Display;

use super::model::*;
use super::runtime::GameClock;
use std::borrow::Borrow;

pub const NULL_TIME: usize = usize::MAX;

#[derive(serde::Serialize)]
#[derive(Debug)]
pub struct GameRecord {
    pub name: String,
    pub events: Vec<GameEvent>,
}

#[derive(serde::Serialize)]
#[derive(Debug)]
pub struct GameEvent {
    time: usize,
    typ: GameEventType,
    name: String,
    number: Option<usize>,
    previous_number: Option<usize>,
}

#[derive(serde::Serialize)]
#[derive(Debug)]
pub enum GameEventType {
    AddToCompendium,
    CharacterDeath,
    CompleteQuest,
    CompleteShrine,
    DiscoverLocation,
    FindDogTreasure,
    IdentifyItem,
    LightFlame,
    MeetCharacter,
    MeetCharacterFlashback,
    SetArmorLevel,
    SetHearts,
    SetItemCount,
    SetStamina,
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

    pub fn apply(&mut self, model: &mut Model) -> Vec<GameEvent> {
        let mut additional_events = vec![];
        let GameEvent { time, typ, name, number, ref mut previous_number } = self;
        match typ {
            GameEventType::CompleteQuest => {
                let (is_started, is_completed) = {
                    let quest = model.borrow_quest(name);
                    (quest.is_started(), quest.is_completed())
                };
                assert!(!is_completed);
                if !is_started {
                    // The quest is not yet marked as having started so create a StartQuest event
                    // first.
                    let mut event = GameEvent::new(*time, GameEventType::StartQuest, name, None);
                    event.apply(model);
                    additional_events.push(event);
                }
                model.borrow_quest_mut(name).completed_time = *time;
            },
            GameEventType::DiscoverLocation => {
                let mut location = model.borrow_location_mut(name);
                assert!(!location.is_discovered());
                location.discovered_time = *time;
            },
            GameEventType::FindDogTreasure => {
                let mut location = model.borrow_location_mut(name);
                assert!(location.has_dog_treasure());
                assert!(!location.is_dog_treasure_found());
                location.dog_treasure_found_time = *time;
            },
            GameEventType::LightFlame => {
                let location = model.borrow_location_mut(name);
                match location.typ {
                    LocationType::TechLab { mut flame_lit_time} => {
                        assert!(flame_lit_time == NULL_TIME);
                        flame_lit_time = *time;
                    },
                    _ => panic!("GameEventType::LightFlame for non tech lab location \"{}\".", name),
                }
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
                let mut quest = model.borrow_quest_mut(name);
                assert!(!quest.is_started());
                assert!(!quest.is_completed());
                quest.started_time = *time;
            },
            _ => unimplemented!()
        }
        additional_events
    }
}

impl Display for GameEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let type_details = match self.typ {
            GameEventType::AddToCompendium => format!("Added {} to compendium.", self.name),
            GameEventType::CharacterDeath => format!("{} died.", self.name),
            GameEventType::CompleteQuest | GameEventType::CompleteShrine => format!("Completed {}.", self.name),
            GameEventType::DiscoverLocation => format!("Discovered {}.", self.name),
            GameEventType::FindDogTreasure => format!("Found dog treasure at {}.", self.name),
            GameEventType::IdentifyItem => format!("Identified {}.", self.name),
            GameEventType::LightFlame => format!("Lit flame at {}.", self.name),
            GameEventType::MeetCharacter => format!("Met {}.", self.name),
            GameEventType::MeetCharacterFlashback => format!("Met {} in a flashback.", self.name),
            GameEventType::SetArmorLevel => format!("Changed {} from {} to {}.", self.name, self.previous_number.unwrap(), self.number.unwrap()),
            GameEventType::SetHearts => format!("Changed hearts from {} to {}.", self.previous_number.unwrap(), self.number.unwrap()),
            GameEventType::SetItemCount => format!("Changed the count for {} from {} to {}.", self.name, self.previous_number.unwrap(), self.number.unwrap()),
            GameEventType::SetStamina => format!("Changed stamina from {} to {}.", self.previous_number.unwrap(), self.number.unwrap()),
            GameEventType::StartQuest | GameEventType::StartShrine => format!("Started {}.", self.name),
        };
        let s = format!("{:?}: {}", GameClock::format_time(self.time), type_details);
        write!(f, "{}", s)
    }
}
