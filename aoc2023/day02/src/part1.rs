use std::collections::HashMap;
use std::error::Error;
use std::{error, fs};
use std::iter::Map;
use std::num::ParseIntError;
use std::str::{FromStr, Lines};

use crate::game::{ Game, GameTurn };

pub fn run() -> () {

    let sample = String::from("
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
    );

    fn parse_games(lines: Lines) -> Vec<Game> {
        lines
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .map(|l| Game::from_str(l).unwrap())
            .collect()
    };


    let sample_reference = GameTurn{ red: 12, green: 13, blue: 14 };
    let sample_res: u32 = parse_games(sample.lines()).into_iter()
        .filter(|g| g.check_game(&sample_reference))
        .map(|g| g.id).sum();
    println!("Sample 1: {:?}", sample_res);


    let input = fs::read_to_string("day02/input.txt").unwrap();
    let res: u32 = parse_games(input.lines()).into_iter()
        .filter(|g| g.check_game(&sample_reference))
        .map(|g| g.id).sum();
    println!("Result 1: {:?}", res);

}
