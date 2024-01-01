use std::collections::HashSet;
use std::fs;

#[derive(Debug)]
struct DigInfo {
    pub value: u32,
    pub label: String,
    pub first_char: char,
}

/// Example
/// ```
/// let result = calibrate("one2three4five");
/// assert_eq!(result, 15)
/// ```
fn calibrate(input: &str) -> u64 {

    let digit_labels: Vec<&str> = vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let maybe_digit: HashSet<char> = (&digit_labels)
        .into_iter()
        .map(|s| s.chars().next().unwrap())
        .collect();

    let digits_info: Vec<DigInfo> = digit_labels
        .into_iter()
        .enumerate()
        .map(|(i, label)| {
            let d = u32::try_from(i).unwrap();
            let first_char = label.chars().next().unwrap();
            DigInfo {
                value: d,
                label: label.to_string(),
                first_char: first_char,
            }
        }).collect();

    let mut digits = input
        .chars()
        .into_iter()
        .enumerate()
        .flat_map(|(i, c)| {
            if c.is_digit(10) {
                c.to_digit(10)
            } else {
                match maybe_digit.get(&c) {
                    None => None,
                    Some(_) => (&digits_info)
                        .into_iter()
                        .filter(|di| {
                            di.first_char == c
                                && (di.label.len() + i) <= input.len()
                                && input[i..(di.label.len() + i)] == di.label
                        })
                        .map(|di| di.value)
                        .next(),
                }
            }
        });
    let a = digits.next().unwrap_or(0);
    let b = digits.last().unwrap_or(a);
    (a * 10 + b).into()
}

pub fn run() -> () {

    let sample = String::from(
        "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
    );

    let sample_res: u64 = sample
        .lines()
        .map(|l| calibrate(l))
        .sum();
    println!("Sample 2: {:?}", sample_res);

    let input = fs::read_to_string("day01/input.txt").unwrap();
    let res: u64 = input
        .lines()
        .map(|l| calibrate(l))
        .sum();
    println!("Result 2: {:?}", res);
}
