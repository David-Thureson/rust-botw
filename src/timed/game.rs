use crate::*;

pub struct Game {
    pub sessions: BTreeMap<usize, usize>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            sessions: BTreeMap::new(),
        }
    }

    pub fn report_sessions(&self) {
        println!("Sessions:");
        for (session_number, seconds) in self.sessions.iter() {
            println!("{}: {}", session_number, util::format::format_count(*seconds));
        }
    }
}