use std::fs::File;
use std::io;
use std::io::BufRead;
use util::tab::*;
use super::*;

const PATH_DATA: &str = r"T:\Docs\Games\Breath of the Wild";

pub fn main() {
    let game = parse_game();
}

fn parse_game() -> Game {

    /*
    let val_trim = "1".replace("\"", "").replace(",", "").trim().to_string();
    dbg!(&val_trim);

    let a = val_trim.parse::<usize>().unwrap();
    dbg!(a);
    */

    let mut game = Game::new();
    parse_sessions(&mut game);




    game.report_sessions();
    game
}

fn parse_sessions(game: &mut Game) {
    let file = File::open(format!("{}/{}", PATH_DATA, "Sess.txt")).unwrap();
    for (line_index, raw_line_result) in io::BufReader::new(file).lines().enumerate() {
        if line_index > 0 {
            let line = raw_line_result.unwrap();
            let cells = line.split("\t").collect::<Vec<_>>();
            let session_number = cell_as_usize(cells[0]);
            if session_number == 0 {
                break;
            }
            let seconds = cell_as_usize(cells[4]);
            game.sessions.insert(session_number, seconds);
        }
    }
}

