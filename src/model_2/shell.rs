#![allow(unused_variables)]

use std::io::{stdout, stdin, Write};

use crate::model_2::model::*;
use crate::model_2::game_record::{GameRecord, GameEvent};
use std::fmt::Display;
use serde::export::Formatter;
use serde::export::fmt::Error;

/*
pub fn run(game_record_name: &str) {
    let mut paused = true;
    let mut command_options = vec![];
    let mut model = Model::new();
    let mut game_record = GameRecord::new(game_record_name);
    loop {
        let prompt = if !command_options.is_empty() { "# " } else if paused { "> " } else { ">>>>> " };
        print!("{}", prompt);
        // #[allow(unused_must_use)]
        // Need to flush this explicitly to make sure that it prints before the read_line().
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
*/

