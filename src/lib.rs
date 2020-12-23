use std::fs;
use std::collections::BTreeMap;

extern crate strum;
extern crate strum_macros;

extern crate util;
pub use util::*;
// pub use util::format::*;
// pub use util::parse::*;
// use crate::model::PREFIX_COMMENT;

//pub mod model_1;
pub mod model_2;

const FILE_REPORT: &str = "Report.txt";

pub fn report_to_file(content: &str) {
    fs::write(FILE_REPORT, content).unwrap();
    println!("{}", content);
}
