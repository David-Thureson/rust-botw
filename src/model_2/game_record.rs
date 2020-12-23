// https://www.joshmcguigan.com/blog/build-your-own-shell-rust/

use serde::export::Formatter;
use serde::export::fmt::Error;
use std::fmt::Display;

use super::controller::GameClock;

#[derive(serde::Serialize)]
#[derive(Debug)]
pub struct Game_Record {
    pub name: String,
    pub events: Vec<GameEvent>,
}

#[derive(serde::Serialize)]
#[derive(Debug)]
pub struct GameEvent {
    name: String,
    typ: GameEventType,
    time: usize,
}

#[derive(serde::Serialize)]
#[derive(Debug)]
pub enum GameEventType {
    FindLocation,
    StartQuest,
    CompleteQuest,
    StartShrine,
    CompleteShrine,
    SetItemCount {
        count: usize,
    },
    SetArmorLevel {
        level: u8,
    },
    MeetCharacter,
    MeetCharacterFlashback,
    CharacterDeath,
}

impl Game_Record {

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
    pub fn new(name: &str, typ: GameEventType, time: usize) -> Self {
        Self {
            name: name.to_string(),
            typ,
            time,
        }
    }
}

impl Display for GameEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let type_details = match self.typ {
            GameEventType::FindLocation => format!("find location \"{}\"", self.name),
            GameEventType::StartQuest => format!("start quest \"{}\"", self.name),
            GameEventType::CompleteQuest => format!("complete quest \"{}\"", self.name),
            GameEventType::StartShrine => format!("start shrine \"{}\"", self.name),
            GameEventType::CompleteShrine => format!("complete shrine \"{}\"", self.name),
            GameEventType::SetItemCount { count } => format!("set count {} = {}", self.name, count),
            GameEventType::SetArmorLevel { level } => format!("set armor level {} = {}", self.name, level),
            GameEventType::MeetCharacter=> format!("meet character {}", self.name),
            _ => {
                dbg!(self);
                panic!("Unexpected GameEventType.");
            },
        };
        let s = format!("{:?}: {}", GameClock::format_time(self.time), type_details);
        write!(f, "{}", s)
    }
}

