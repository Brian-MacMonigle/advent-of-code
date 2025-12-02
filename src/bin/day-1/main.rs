use std::env;
use std::fs;

fn main() {
    println!("Hello Day 1!\n");

    let path = env::args().nth(1).unwrap_or("input.txt".to_string());

    let input = fs::read_to_string(&path).expect(format!("Can't read {}", &path).as_str());

    let lines: Vec<&str> = input
        .split_whitespace()
        .filter(|line| !line.is_empty())
        .collect();

    let password = count_zeros(&lines);

    println!("Password: {}", password)
}

fn count_zeros(lines: &[&str]) -> u32 {
    let mut location = 50;
    let mut zero_counter = 0;
    for line in lines {
        location = rotate(location, &line);
        if location == 0 {
            zero_counter += 1;
        }
    }
    zero_counter
}

fn rotate(location: u32, change: &str) -> u32 {
    // Parse the change text
    let dir: i32 = match change.chars().nth(0) {
        Some('L') => -1,
        Some('R') => 1,
        _ => panic!("Unknown rotate direction {}", change),
    };
    let amount_slice = &change[1..change.len()];
    let amount = amount_slice
        .parse::<i32>()
        .expect(format!("Not a number: {}", amount_slice).as_str());

    // Calculate the end location
    let end = dir * (amount as i32) + (location as i32);
    // Normalize to [0, 99] scale
    (if end < 0 {
        ((end % 100) + 100) % 100
    } else {
        end % 100
    }) as u32
}

#[test]
fn test_count_zeros() {
    let lines = vec![
        "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
    ];

    let password = count_zeros(&lines);
    assert_eq!(password, 3);
}

#[test]
fn test_rotate_left() {
    let mut location = 82;
    location = rotate(location, "L30");
    assert_eq!(location, 52);

    let mut location = 82;
    location = rotate(location, "L3");
    assert_eq!(location, 79);
}

#[test]
fn test_rotate_left_rollover() {
    let mut location = 50;
    location = rotate(location, "L68");
    assert_eq!(location, 82);

    let mut location = 0;
    location = rotate(location, "L115");
    assert_eq!(location, 85);

    let mut location = 0;
    location = rotate(location, "L300");
    assert_eq!(location, 0);
}

#[test]
fn test_rotate_right() {
    let mut location = 0;
    location = rotate(location, "R14");
    assert_eq!(location, 14);

    let mut location = 0;
    location = rotate(location, "R5");
    assert_eq!(location, 5);

    let mut location = 0;
    location = rotate(location, "R300");
    assert_eq!(location, 0);
}

#[test]
fn test_rotate_right_rollover() {
    let mut location = 95;
    location = rotate(location, "R60");
    assert_eq!(location, 55);

    let mut location = 95;
    location = rotate(location, "R963");
    assert_eq!(location, 58);
}
