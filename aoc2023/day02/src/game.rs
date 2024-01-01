use std::cmp::max;
use std::collections::HashMap;
use std::error::Error;
use std::iter::Map;
use std::num::ParseIntError;
use std::str::{FromStr, Lines};
use std::{error, fs};

#[derive(Debug, PartialEq)]
pub struct GameTurn {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl GameTurn {
    pub fn max_turn(t1: GameTurn, t2: GameTurn) -> GameTurn {
        GameTurn {
            red: max(t1.red, t2.red),
            green: max(t1.green, t2.green),
            blue: max(t1.blue, t2.blue),
        }
    }
    pub fn check_turn(&self, other: &GameTurn) -> bool {
        self.green <= other.green && self.red <= other.red && self.blue <= other.blue
    }
}

impl FromStr for GameTurn {
    type Err = Box<dyn error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map: HashMap<String, Result<u32, ParseIntError>> = s
            .trim()
            .split(',')
            .flat_map(|x| {
                x.trim()
                    .split_once(' ')
                    .map(|(v, c)| (c.to_string(), u32::from_str(v)))
            })
            .collect();
        let red = (*map.get("red").unwrap_or(&Ok(0 as u32))).clone()?;
        let blue = (*map.get("blue").unwrap_or(&Ok(0 as u32))).clone()?;
        let green = (*map.get("green").unwrap_or(&Ok(0 as u32))).clone()?;
        Ok(GameTurn {
            red: red,
            green: green,
            blue: blue,
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct Game {
    pub id: u32,
    pub turns: Vec<GameTurn>,
}

impl Game {
    pub fn max_turn(self) -> GameTurn {
        self.turns
            .into_iter()
            .reduce(|a, b| GameTurn::max_turn(a, b))
            .unwrap()
    }
    pub fn max_game(g1: Game, g2: Game) -> GameTurn {
        GameTurn::max_turn(g1.max_turn(), g2.max_turn())
    }
    pub fn check_game(&self, reference: &GameTurn) -> bool {
        (&self.turns)
            .into_iter()
            .map(|t| t.check_turn(reference))
            .reduce(|a, b| a && b)
            .unwrap_or(true)
    }
}

impl FromStr for Game {
    type Err = Box<dyn error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = s
            .trim()
            .split_once(':')
            .map(|(gid, tx)| {
                let id = u32::from_str(&gid.to_lowercase().replace("game ", ""));
                let turns: Vec<GameTurn> =
                    (tx.split(";").map(|t| GameTurn::from_str(t).unwrap())).collect();
                id.map(|i| Game {
                    id: i,
                    turns: turns,
                })
            })
            .unwrap();
        Ok(res?)
    }
}

pub fn parse_games(lines: Lines) -> Vec<Game> {
    lines
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| Game::from_str(l).unwrap())
        .collect()
}
