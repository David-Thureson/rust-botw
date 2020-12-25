#![feature(str_split_once)]

use std::fs;
use std::collections::BTreeMap;

extern crate strum;
extern crate strum_macros;

extern crate util_rust;
pub use util_rust::*;
// pub use util::format::*;
// pub use util::parse::*;
// use crate::model::PREFIX_COMMENT;

pub mod model_3;

const FILE_REPORT: &str = "Report.txt";

pub fn report_to_file(content: &str) {
    fs::write(FILE_REPORT, content).unwrap();
    println!("{}", content);
}

