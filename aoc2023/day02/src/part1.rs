use std::error::Error;
use std::fs;
use std::str::FromStr;

pub fn run() -> () {

    let sample = String::from("
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
    );

    #[derive(Debug, PartialEq)]
    struct GameTurn{ red: u32, green: u32, blue: u32 }

    impl FromStr for GameTurn {
        type Err = Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            s.trim().split(',')
                .map(|x| x.trim().split(' '));
            todo!()
        }
    }

    // impl GameTurn {
    //     fn from(input: &str) -> () {
    //         input.trim().split(',').map(|x| x.trim());
    //         ()
    //     }
    // }

}
