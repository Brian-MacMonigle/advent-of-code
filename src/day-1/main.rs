use std::fs;

// Part 1
static DIGITS_P1: [&str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
// Part 2
static DIGITS_P2: [&str; 20] = [
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", //
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    println!("Hello Day 1!\n");

    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let coordinates_p1 = parse_calibration(&input, &DIGITS_P1);

    println!("What is the sum of all of the calibration values?\n{coordinates_p1}\n");

    let coordinates_p2 = parse_calibration(&input, &DIGITS_P2);
    println!("What is the sum of all of the corrected calibration values?\n{coordinates_p2}");
}

fn parse_calibration(document: &str, digits: &[&str]) -> u32 {
    return document
        .lines()
        .map(|line| parse_calibration_line(line, digits))
        .sum();
}

#[derive(Debug)]
struct Match<'a> {
    pattern: &'a str,
    index: Option<usize>,
}

fn parse_calibration_line(text: &str, digits: &[&str]) -> u32 {
    let first = digits
        .iter()
        .map(|pattern| Match {
            pattern: pattern,
            index: text.find(pattern),
        })
        .filter(|matched| matched.index.is_some())
        .reduce(|cur, next| if next.index < cur.index { next } else { cur })
        .unwrap();
    let last = digits
        .iter()
        .map(|pattern| Match {
            pattern: pattern,
            index: text.rfind(pattern),
        })
        .filter(|matched| matched.index.is_some())
        .reduce(|cur, next| if next.index > cur.index { next } else { cur })
        .unwrap();

    return 10 * to_digit(first.pattern) + to_digit(last.pattern);
}

fn to_digit(num: &str) -> u32 {
    match num {
        "0" | "zero" => 0,
        "1" | "one" => 1,
        "2" | "two" => 2,
        "3" | "three" => 3,
        "4" | "four" => 4,
        "5" | "five" => 5,
        "6" | "six" => 6,
        "7" | "seven" => 7,
        "8" | "eight" => 8,
        "9" | "nine" => 9,
        _ => panic!("Oops!"),
    }
}

#[test]
fn test_parse_calibration_line() {
    assert_eq!(parse_calibration_line("1abc2", &DIGITS_P1), 12);
}

#[test]
fn test_parse_calibration() {
    assert_eq!(
        parse_calibration(
            "1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet",
            &DIGITS_P1
        ),
        142
    );
}

// Part 2
#[test]
fn test_find_first_digit_letter() {
    assert_eq!(parse_calibration_line("two1nine", &DIGITS_P2), 29);
}

#[test]
fn test_parse_calibration_letter() {
    assert_eq!(
        parse_calibration(
            "two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen",
            &DIGITS_P2
        ),
        281
    );
}
