use crate::*;
use std::fmt::{Display, Formatter};
use std::{cmp, fmt, io};
use crate::model::Model;
use std::fs::File;
use std::io::BufRead;

const FILE_NAME_SHRINES: &str = "Breath of the Wild Shrines.txt";

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum ShrineState {
    NotDiscovered,
    Discovered,
    Completed,
}

#[derive(PartialEq, Eq, PartialOrd)]
pub struct Shrine {
    pub name: String,
    pub state: ShrineState,
}

impl Display for ShrineState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            ShrineState::NotDiscovered => "Not Discovered",
            ShrineState::Discovered => "Discovered",
            ShrineState::Completed => "Completed",
        })
    }
}

impl Shrine {

    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            state: ShrineState::NotDiscovered,
        }
    }

    pub fn load_shrines(model: &mut Model) {
        let file = File::open(FILE_NAME_SHRINES).unwrap();
        for raw_line in io::BufReader::new(file).lines() {
            let line = raw_line.unwrap().trim().to_string();
            model.shrines.insert(line.clone(), Shrine::new(&line));
        }
    }

    pub fn describe_deep(&self, s: &mut String, depth: usize, _max_depth: Option<usize>) {
        let line = self.description();
        s.push_str(&format_indent_line_space(depth, &line));
    }

    pub fn description(&self) -> String {
        format!("{}: {}", self.name, self.state)
    }
}

impl Ord for Shrine {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.name.cmp(&other.name)
    }
}
