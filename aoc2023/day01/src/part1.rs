use std::fs;

/// # Example
/// ```
/// let result = calibrate("one2three4five6");
/// assert_eq!(result, 26)
/// ```
pub fn calibrate(input: &str) -> u64 {
    let mut digits = input.chars().into_iter().flat_map(|d| d.to_digit(10));
    let a = digits.next().unwrap_or(0);
    let b = digits.last().unwrap_or(a);
    (a * 10 + b).into()
}

pub fn run() -> () {
    let sample = String::from(
        "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
    );

    let sample_res: u64 = sample.lines().map(|l| calibrate(l)).sum();
    println!("Sample 1: {:?}", sample_res);

    let input = fs::read_to_string("day01/input.txt").unwrap();
    let res: u64 = input.lines().map(|l| calibrate(l)).sum();
    println!("Result 1: {:?}", res);
}
