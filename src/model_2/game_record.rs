// https://www.joshmcguigan.com/blog/build-your-own-shell-rust/

use serde::export::Formatter;
use serde::export::fmt::Error;
use std::fmt::Display;

use super::model::*;
use super::runtime::GameClock;
use std::cell::RefCell;
use util_rust::format;
use serde::Serialize;

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
    MentionCharacter,
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

    pub fn add_event(&mut self, model: &mut Model, mut event: GameEvent) {
        let mut additional_events = event.apply(model);
        self.events.append(&mut additional_events);
        self.events.push(event);
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

    pub fn apply(&mut self, model: &mut Model) -> Vec<GameEvent> {
        let mut additional_events = vec![];
        let GameEvent { time, typ, name, number, ref mut previous_number } = self;
        match typ {
            GameEventType::CompleteQuest => {
                let (started_time, completed_time) = model.get_quest_started_completed(name);
                assert!(completed_time == NULL_TIME);
                if !started_time == NULL_TIME {
                    // The quest is not yet marked as having started so create a StartQuest event
                    // first.
                    Self::add_and_apply_event(model, &mut additional_events, GameEvent::new(*time, GameEventType::StartQuest, name, None));
                }
                let quest = model.get_quest(name);
                let mut quest = RefCell::borrow_mut(&quest);
                quest.completed_time = *time;
            },
            GameEventType::CompleteShrine => {
                // Isolate the borrow in its own scope so that in a few lines if we need to call
                // Self::add_and_apply_event we're no longer making an immutable borrow of model.
                let (started_time, completed_time) = model.get_shrine_started_completed(name);
                if started_time == NULL_TIME {
                    // Mark the shrine as started.
                    Self::add_and_apply_event(model, &mut additional_events, GameEvent::new(*time, GameEventType::StartShrine, name, None));
                }
                assert!(completed_time == NULL_TIME);
                let location = model.get_location(name);
                let mut location = RefCell::borrow_mut(&location);
                match location.typ {
                    LocationType::Shrine { challenge: _, quest: _, started_time: _, ref mut completed_time} => {
                        *completed_time = *time;
                    },
                    _ => panic!("Location \"{}\" is not a shrine.", name),
                }
                // let (_challenge, _quest, _started_time, completed_time) = model.borrow_shrine_mut(name);
                // *completed_time = *time;
            },
            GameEventType::DiscoverLocation => {
                println!("DiscoverLocation: \"{}\"", &name);
                let parent_location = model.get_parent_location(name);
                if let Some(parent_location) = parent_location {
                    let parent_location = RefCell::borrow(&parent_location);
                    if !parent_location.is_discovered() {
                        // Add an event for the discovery of the parent location.
                        Self::add_and_apply_event(model, &mut additional_events, GameEvent::new(*time, GameEventType::DiscoverLocation, &parent_location.name, None));
                    }
                }
                let location = model.get_location(name);
                dbg!(&location);
                let mut location = RefCell::borrow_mut(&location);
                assert!(!location.is_discovered());
                location.discovered_time = *time;
            },
            GameEventType::FindDogTreasure => {
                let location = model.get_location(name);
                let mut location = RefCell::borrow_mut(&location);
                assert!(location.has_dog_treasure());
                assert!(!location.is_dog_treasure_found());
                location.dog_treasure_found_time = *time;
            },
            GameEventType::LightFlame => {
                let location = model.get_location(name);
                let mut location = RefCell::borrow_mut(&location);
                match location.typ {
                    LocationType::TechLab { ref mut flame_lit_time} => {
                        assert!(*flame_lit_time == NULL_TIME);
                        *flame_lit_time = *time;
                    },
                    _ => panic!("GameEventType::LightFlame for non tech lab location \"{}\".", name),
                }
            },
            GameEventType::MeetCharacter => {
                let character = model.get_character(name);
                let mut character = RefCell::borrow_mut(&character);
                assert!(!character.is_met());
                character.met_time = *time;
            },
            GameEventType::MeetCharacterFlashback => {
                let character = model.get_character(name);
                let mut character = RefCell::borrow_mut(&character);
                assert!(!character.is_met_in_flashback());
                character.met_in_flashback_time = *time;
            },
            GameEventType::MentionCharacter => {
                let character = model.get_character(name);
                let mut character = RefCell::borrow_mut(&character);
                assert!(!character.is_mentioned());
                character.mentioned_time = *time;
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
                let quest = model.get_quest(name);
                let mut quest = RefCell::borrow_mut(&quest);
                assert!(!quest.is_started());
                assert!(!quest.is_completed());
                quest.started_time = *time;
            },
            GameEventType::StartShrine => {
                let (started_time, completed_time) = model.get_shrine_started_completed(name);
                assert!(started_time == NULL_TIME);
                assert!(completed_time == NULL_TIME);
                if let Some(quest) = model.get_shrine_quest(name) {
                    let quest = RefCell::borrow(&quest);
                    if !quest.is_completed() {
                        // Mark the related shrine quest as completed.
                        Self::add_and_apply_event(model, &mut additional_events, GameEvent::new(*time, GameEventType::CompleteQuest, &quest.name, None));
                    }
                }
                let location = model.get_location(name);
                let mut location = RefCell::borrow_mut(&location);
                match location.typ {
                    LocationType::Shrine { challenge: _, quest: _, ref mut started_time, ..} => {
                        *started_time = *time;
                    },
                    _ => panic!("Location \"{}\" is not a shrine.", name),
                }
            },
            _ => unimplemented!()
        }
        additional_events
    }

    fn add_and_apply_event(model: &mut Model, additional_events: &mut Vec<GameEvent>, mut event: GameEvent) {
        let mut more_additional_events = event.apply(model);
        additional_events.append(&mut more_additional_events);
        additional_events.push(event);
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
            GameEventType::MentionCharacter => format!("Mentioned {}.", self.name),
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
impl GameEventType {
    pub fn variant_to_string(&self) -> &str {
        match self {
            GameEventType::AddToCompendium => "AddToCompendium",
            GameEventType::CharacterDeath => "CharacterDeath",
            GameEventType::CompleteQuest => "CompleteQuest",
            GameEventType::CompleteShrine => "CompleteShrine",
            GameEventType::DiscoverLocation => "DiscoverLocation",
            GameEventType::FindDogTreasure => "FindDogTreasure",
            GameEventType::IdentifyItem => "IdentifyItem",
            GameEventType::LightFlame => "LightFlame",
            GameEventType::MeetCharacter => "MeetCharacter",
            GameEventType::MeetCharacterFlashback => "MeetCharacterFlashback",
            GameEventType::MentionCharacter => "MentionCharacter",
            GameEventType::SetArmorLevel => "SetArmorLevel",
            GameEventType::SetHearts => "SetHearts",
            GameEventType::SetItemCount => "SetItemCount",
            GameEventType::SetStamina => "SetStamina",
            GameEventType::StartQuest => "StartQuest",
            GameEventType::StartShrine => "StartShrine",
        }
    }

    pub fn string_to_variant(s: &str) -> Self {
        match s {
            "AddToCompendium" => GameEventType::AddToCompendium,
            "CharacterDeath" => GameEventType::CharacterDeath,
            "CompleteQuest" => GameEventType::CompleteQuest,
            "CompleteShrine" => GameEventType::CompleteShrine,
            "DiscoverLocation" => GameEventType::DiscoverLocation,
            "FindDogTreasure" => GameEventType::FindDogTreasure,
            "IdentifyItem" => GameEventType::IdentifyItem,
            "LightFlame" => GameEventType::LightFlame,
            "MeetCharacter" => GameEventType::MeetCharacter,
            "MeetCharacterFlashback" => GameEventType::MeetCharacterFlashback,
            "MentionCharacter" => GameEventType::MentionCharacter,
            "SetArmorLevel" => GameEventType::SetArmorLevel,
            "SetHearts" => GameEventType::SetHearts,
            "SetItemCount" => GameEventType::SetItemCount,
            "SetStamina" => GameEventType::SetStamina,
            "StartQuest" => GameEventType::StartQuest,
            "StartShrine" => GameEventType::StartShrine,
            _ => panic!(format!("Unexpected GameEventType variant name \"{}\".", s))
        }
    }
}

pub fn try_create_events() {
    let mut model = Model::new();
    let mut game_record = GameRecord::new("Test");
    let mut clock = GameClock::new_running(1_000);

    game_record.add_event(&mut model,GameEvent::new(clock.time(), GameEventType::DiscoverLocation, "Phalian Highlands", None));

    // clock.add_seconds(1234);

    game_record.print_events_serialized();

}