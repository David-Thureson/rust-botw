use std::{cmp, fmt};
use std::fmt::{Display, Formatter};
use crate::format_indent_line_space;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Race {
    Amiibo,
    GreatFairy,
    Deity,
    Demon,
    Gerudo,
    Goron,
    Horse,
    Hylian,
    Korok,
    Rito,
    SandSeal,
    Sheikah,
    SheikahMonk,
    Yiga,
    Zora,
}

pub enum CharacterState {

}

#[derive(PartialEq, Eq, PartialOrd)]
pub struct Character {
    pub name: String,
    pub race: Race,
    pub main: bool,
    pub events: Vec<CharacterEvent>,
}

impl Display for LocationState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            LocationState::NotDiscovered => "Not Discovered",
            LocationState::Discoverd => "Discovered",
        })
    }
}

impl Location {
    pub fn describe_deep(&self, s: &mut String, depth: usize, _max_depth: Option<usize>) {
        let line = self.description();
        s.push_str(&format_indent_line_space(depth, &line));
    }

    pub fn description(&self) -> String {
        format!("{}: {}", self.name, self.state)
    }
}

impl Ord for Location {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.name.cmp(&other.name)
    }
}
