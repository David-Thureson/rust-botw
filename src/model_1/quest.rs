use std::{cmp, fmt};
use std::fmt::{Display, Formatter};
use crate::format_indent_line_space;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum QuestState {
    NotStarted,
    Started,
    Completed,
}

#[derive(PartialEq, Eq, PartialOrd)]
pub struct Quest {
    pub name: String,
    pub state: QuestState,
}

impl Display for QuestState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            QuestState::NotStarted => "Not Started",
            QuestState::Started => "Started",
            QuestState::Completed => "Completed",
        })
    }
}

impl Quest {
    pub fn describe_deep(&self, s: &mut String, depth: usize, _max_depth: Option<usize>) {
        let line = self.description();
        s.push_str(&format_indent_line_space(depth, &line));
    }

    pub fn description(&self) -> String {
        format!("{}: {}", self.name, self.state)
    }
}

impl Ord for Quest {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.name.cmp(&other.name)
    }
}
