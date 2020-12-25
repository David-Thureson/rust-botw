//use super::model::*;
//use super::game_record::{GameRecord, GameEvent};
//use std::fmt::Display;
//use serde::export::Formatter;
//use serde::export::fmt::Error;
use std::time::SystemTime;

#[derive(Debug)]
pub enum GameClock {
    Stopped {
        stop_time: usize,
    },
    Running {
        start_time: usize,
        start_system_time: SystemTime,
    },
}

/*
enum CommandOption {
    Cancel,
    Query {
        component_reference: ComponentReference,
    },
    GameEvent {
        label: String,
        game_event: GameEvent,
    },
}
*/

impl GameClock {
    pub fn new_stopped(stop_time: usize) -> Self {
        Self::Stopped {
            stop_time,
        }
    }

    pub fn new_running(start_time: usize) -> Self {
        Self::Running {
            start_time,
            start_system_time: SystemTime::now(),
        }
    }

    pub fn start(&self) -> Option<GameClock> {
        match self {
            GameClock::Stopped { stop_time } => Some(GameClock::new_running(*stop_time)),
            GameClock::Running { .. } => None,
        }
    }

    pub fn stop(&self) -> Option<GameClock> {
        let time = self.time();
        match self {
            GameClock::Stopped { .. } => None,
            GameClock::Running { .. } => Some(GameClock::new_stopped(time)),
        }
    }

    pub fn time(&self) -> usize {
        match self {
            GameClock::Stopped { stop_time } => *stop_time,
            GameClock::Running { start_time, start_system_time} => {
                let seconds_since_start = SystemTime::now().duration_since(*start_system_time).unwrap().as_secs();
                start_time + seconds_since_start as usize
            },
        }
    }

    /*
    pub fn add_seconds(&mut self, seconds: usize) {
        self.time += seconds;
    }
    */

    pub fn time_formatted(&self) -> String {
        Self::format_time(self.time())
    }

    pub fn format_time(time: usize) -> String {
        let hours = time / (3_600);
        let time = time - (hours * 3_600);
        let minutes = time / 60;
        let time = time - (minutes * 60);
        let seconds = time;
        format!("{:>3}:{:0>2}:{:0>2}", hours, minutes, seconds)
    }
}

/*
impl Display for CommandOption {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            CommandOption::Cancel => write!(f, "Cancel"),
            CommandOption::Query { component_reference} => write!(f, "? {}", component_reference),
            CommandOption::GameEvent { label, game_event: _ } => write!(f, "{}", label),
        }
    }
}

fn create_query_options(model: &Model, partial: &str) -> Vec<CommandOption> {
    let mut v = vec![CommandOption::Cancel];
    for component_reference in model.partial_match_references(partial, true, true, true, true) {
        v.push(CommandOption::Query { component_reference });
    }
    v
}

fn create_number_options(model: &Model, partial: &str, number: usize) -> Vec<CommandOption> {
    vec![]
}

fn create_non_number_options(model: &Model, partial: &str) -> Vec<CommandOption> {
    vec![]
}

fn execute_command(model: &mut Model, game_record: &mut GameRecord, command_option: CommandOption) {

}

*/