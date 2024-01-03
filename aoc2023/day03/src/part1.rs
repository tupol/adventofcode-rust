use std::num::ParseIntError;
use std::str::{FromStr, Lines};
use std::{error, fs};

use crate::puzzle::Number;


pub fn run() -> () {
    let sample = String::from("
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
");

    fn parse_digits(input: &str) -> Vec<Number> {
        fn extract_num(buf: &Vec<(usize, char)>) -> Number {
            let t: String = buf.iter().map(|(i, c)| c).collect();
            Number {
                value: u32::from_str(&t).unwrap(),
                from: buf.first().unwrap().0.clone(),
            }
        }
        let mut nums = Vec::<Number>::new();
        let mut buff = Vec::<(usize, char)>::new();
        let res = input.chars().enumerate()
            .filter(|(i, c)| c.is_digit(10))
            .fold((nums.as_mut(), buff.as_mut()),
                  |(acc, buf): (&mut Vec::<Number>, &mut Vec::<(usize, char)>), (i, c) | {
                if !buf.is_empty() {
                    if &buf.last().unwrap().0 + 1 == i {
                        buf.push((i, c));
                        (acc, buf)
                    } else {
                        acc.push( extract_num(buf));
                        buf.clear();
                        buf.push((i, c));
                        (acc, buf)
                    }
                } else {
                    buf.push((i, c));
                    (acc, buf)
                }
            });
        let nums = res.0;
        if res.1.len() > 0 {
            nums.push( extract_num(&res.1));
        }
        nums.to_vec()
    }

    parse_digits("467..123.114").iter().for_each(|n| println!("{:?}", n));


    // let sample_res: u32 = parse_games(sample.lines())
    //     .into_iter()
    //     .filter(|g| g.check_game(&sample_reference))
    //     .map(|g| g.id)
    //     .sum();
    // println!("Sample 1: {:?}", sample_res);
    //
    // let input = fs::read_to_string("day02/input.txt").unwrap();
    // let res: u32 = parse_games(input.lines())
    //     .into_iter()
    //     .filter(|g| g.check_game(&sample_reference))
    //     .map(|g| g.id)
    //     .sum();
    // println!("Result 1: {:?}", res);
}
