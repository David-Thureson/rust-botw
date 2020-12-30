use super::command::*;
use super::model::*;
use std::time::Instant;

pub fn main() {
    test_many_actions(30);
}

fn test_many_actions(max_seconds: usize) {
    let start_time = Instant::now();
    let model = Model::new();
    /*
    loop {
        let command_set = CommandSet::generate(&model, None, None).print_numbered(&model);
        if (Instant::now - start_time).as_secs() >= max_seconds {
            break;
        }
    }

     */

}
