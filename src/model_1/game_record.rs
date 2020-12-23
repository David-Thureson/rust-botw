// https://www.joshmcguigan.com/blog/build-your-own-shell-rust/

use std::time::SystemTime;
use std::fmt::Display;
use serde::export::Formatter;
use serde::export::fmt::Error;

#[derive(serde::Serialize)]
#[derive(Debug)]
pub struct GameRecord {
    name: String,
    pub events: Vec<GameEvent>,
}

#[derive(serde::Serialize)]
#[derive(Debug)]
pub struct GameEvent {
    time: SystemTime,
    name: String,
    event_type: GameEventType,
}

#[derive(serde::Serialize)]
#[derive(Debug)]
pub enum GameEventType {
    Start,
    Stop,
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

    pub fn start(&mut self) {
        self.events.push(GameEvent::new(GameEventType::Start));
    }

    pub fn stop(&mut self) {
        self.events.push(GameEvent::new(GameEventType::Stop));
    }
}

impl GameEvent {
    pub fn new(event_type: GameEventType) -> Self {
        Self {
            time: SystemTime::now(),
            event_type,
        }
    }
}

impl Display for GameEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let s = format!("{:?}: {}", self.time, self.event_type);
        write!(f, "{}", s)
    }
}

impl Display for GameEventType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let s = match self {
            GameEventType::Start => "start".to_string(),
            GameEventType::Stop => "stop/pause".to_string(),
            GameEventType::FindLocation { name } => format!("find location \"{}\"", name),
            GameEventType::StartQuest { name } => format!("start quest \"{}\"", name),
            GameEventType::CompleteQuest { name } => format!("complete quest \"{}\"", name),
            GameEventType::StartShrine { name } => format!("start shrine \"{}\"", name),
            GameEventType::CompleteShrine { name } => format!("complete shrine \"{}\"", name),
            GameEventType::SetItemCount { name, count } => format!("set count {} = {}", name, count),
            GameEventType::SetArmorLevel { name, level } => format!("set armor level {} = {}", name, level),
            GameEventType::MeetCharacter { name } => format!("meet character {}", name),
        };
        write!(f, "{}", s)
    }
}


