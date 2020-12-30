use rand::Rng;
use super::command::*;
use super::model::*;
use std::time::Instant;
use crate::model_3::game_record::GameRecord;

pub fn test_many_actions() {
    let max_seconds = 10u64;
    let partial_name_substring_length = 2;

    let start_time = Instant::now();
    let mut model = Model::new();
    let mut game_record = GameRecord::new("Sim");

    dbg!(Instant::now() - start_time);

    let start_time = Instant::now();

    let mut special_commands = vec!["bl", "bo", "c", "die", "he", "k", "sh", "st", "we"]
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>();
    special_commands.sort();
    //rintln!("{}", special_commands.iter().map(|x| format!("\"{}\"", x)).join(", "));

    let mut partial_names = model.characters.keys()
        .chain(model.locations.keys())
        .chain(model.quests.keys())
        //.map(|x| format!("{:>width}", x, partial_name_length))
        .map(|x| x[..partial_name_substring_length].to_string())
        .collect::<Vec<_>>();
    partial_names.append(&mut special_commands.clone());
    partial_names.sort();
    partial_names.dedup();
    dbg!(Instant::now() - start_time);
    dbg!(partial_names.len());
    //bg!(&partial_names);
    // Intersection between special_commands and partial_names.
    //bg!(special_commands.iter().filter(|x| partial_names.contains(x)).collect::<Vec<_>>());

    let start_time = Instant::now();
    let mut rng = rand::thread_rng();
    let mut game_time = 0;
    loop {
        let partial_name_index = rng.gen_range(0..partial_names.len());
        let partial_name = &partial_names[partial_name_index];
        //rintln!("{}", partial_name);
        let mut command_set = CommandSet::new(&partial_name, None);
        command_set.include_empty_targets = false;
        command_set.add_to_special_commands = true;
        command_set.generate(&model);
        if command_set.command_count() == 0 {
            println!("Removing \"{}\", leaving {}.", partial_name, partial_names.len() - 1);
            partial_names.remove(partial_name_index);
            continue;
        }
        if command_set.number_targets {
            // target_number is 1-based.
            let target_number = rng.gen_range(1..=command_set.targets.len());
            command_set = command_set.regen_with_chosen_target(&model, target_number);
            assert!(!command_set.number_targets);
        }
        let command_count = command_set.command_count();
        assert!(command_count > 0);
        // command_number is 1-based.
        let command_number = rng.gen_range(1..=command_count);
        command_set.apply_command(&mut model, &mut game_record, game_time, command_number);

        game_time += rng.gen_range(0..60);

        if (Instant::now() - start_time).as_secs() >= max_seconds {
            break;
        }
        if partial_names.len() == special_commands.len() {
            break;
        }
    }
    dbg!(&partial_names);
    //game_record.print_events_serialized();
    dbg!(game_record.events.len());

    let start_time = Instant::now();
    let json = serde_json::to_string(&game_record).unwrap();
    dbg!(Instant::now() - start_time);
    //bg!(&json);
    dbg!(json.len());

    let start_time = Instant::now();
    let new_game_record: GameRecord = serde_json::from_str(&json).unwrap();
    dbg!(Instant::now() - start_time);
    dbg!(new_game_record.events.len());
}

/*
fn random_letter() -> String {
    let mut rng = rand::thread_rng();
    let letter: char = rng.gen_range(b'a'..=b'z') as char;
    format!("{}", letter)
}
*/
