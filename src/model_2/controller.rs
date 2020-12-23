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
#![allow(unused_variables)]

use std::io::{stdout, stdin, Write};

use crate::model::{ComponentReference, Model};
use crate::game_record::{GameRecord, GameEvent};
use std::fmt::Display;
use serde::export::Formatter;
use serde::export::fmt::Error;

pub fn run(game_record_name: &str) {
    let mut paused = true;
    let mut command_options = vec![];
    let mut model = Model::new();
    let mut game_record = GameRecord::new(game_record_name);
    loop {
        // use the `>` character as the prompt
        // need to explicitly flush this to ensure it prints before read_line
        let prompt = if !command_options.is_empty() { "# " } else if paused { "> " } else { ">>> " };
        print!("{}", prompt);
        // #[allow(unused_must_use)]
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        if !command_options.is_empty() {
            let option_index = input.trim().parse::<usize>().unwrap();
            assert!(option_index < command_options.len());
            if option_index > 0 {
                execute_command(&mut model, &mut game_record, command_options.remove(option_index));
            }
            command_options.clear();
        } else {
            let is_query = input.starts_with(" ");
            let command_line = input.trim();
            if command_line == "" {
                if paused {
                    game_record.start();
                    paused = false;
                } else {
                    game_record.stop();
                    paused = true;
                }
            } else {
                let command_parts = command_line.split(" ").collect::<Vec<_>>();
                let first_word = command_parts[0];
                match first_word {
                    "q" | "quit" => {
                        break;
                    },
                    "r" | "review" => {
                        let event_count = if command_parts.len() == 1 {
                            10
                        } else {
                            command_parts[1].parse::<usize>().unwrap()
                        };
                        game_record.review(event_count);
                    },
                    _ => {
                        // println!("Unknown command.");
                        if is_query {
                            command_options = create_query_options(&model, first_word);
                        } else {
                            if command_parts.len() > 1 {
                                let number = command_parts[1].parse::<usize>().unwrap();
                                command_options = create_number_options(&model, first_word, number);
                            } else {
                                command_options = create_non_number_options(&model, first_word);
                            }
                        }
                        for (index, option) in command_options.iter().enumerate() {
                            println!("{}: {}", index, option);
                        }
                    },
                }
            }
        }
        //}
        //rintln!("Command was \"{}\".", command);

        // let mut child = Command::new(command)
        //     .spawn()
        //    .unwrap();

        // don't accept another command until this one completes
        // child.wait();
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

impl Display for CommandOption {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            CommandOption::Cancel => write!(f, "Cancel"),
            CommandOption::Query { component_reference} => write!(f, "? {}", component_reference),
            CommandOption::GameEvent { label, game_event: _ } => write!(f, "{}", label),
        }
    }
}


 */