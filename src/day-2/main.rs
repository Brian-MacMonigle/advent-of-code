use std::cmp::max;
use std::fs;

fn main() {
    println!("Hello Day 2!\n");

    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let possible = count_possible_games(&input, 12, 13, 14);
    println!("Sum of possible game id's with 12 red cubes, 13 green cubes, and 14 blue cubes: {possible}\n");

    let power_sum = min_needed(&input);
    println!("Sum of power of all games is: {power_sum}");
}

#[derive(Debug)]
struct Game {
    id: u8,
    max_round: Round,
}

#[derive(Debug)]
struct Round {
    red: u8,
    green: u8,
    blue: u8,
}

fn count_possible_games(text: &str, red: u8, green: u8, blue: u8) -> u16 {
    text.lines()
        .map(parse_game)
        .filter(|game| {
            game.max_round.red <= red
                && game.max_round.green <= green
                && game.max_round.blue <= blue
        })
        .fold(0, |acc, next: Game| acc + (next.id as u16))
}

fn min_needed(text: &str) -> u64 {
    text.lines().map(parse_game).fold(0, |acc, next: Game| {
        acc + (next.max_round.red as u64)
            * (next.max_round.blue as u64)
            * (next.max_round.green as u64)
    })
}

fn parse_game(text: &str) -> Game {
    let mut game_iter = text.trim().split(':');
    let id = game_iter.next().unwrap()[5..].parse::<u8>().unwrap();
    let max_round = game_iter
        .next()
        .unwrap()
        .split(";")
        .map(parse_round)
        .reduce(|acc, next| Round {
            red: max(acc.red, next.red),
            green: max(acc.green, next.green),
            blue: max(acc.blue, next.blue),
        })
        .unwrap();
    Game {
        id: id,
        max_round: max_round,
    }
}

fn parse_round(text: &str) -> Round {
    let mut red: u8 = 0;
    let mut green: u8 = 0;
    let mut blue: u8 = 0;
    for pick in text.split(',') {
        let mut pick_iter = pick.trim().split(" ");
        let value: u8 = pick_iter.next().unwrap().parse().unwrap();
        let color = pick_iter.next().unwrap();

        match color {
            "red" => red = value,
            "green" => green = value,
            "blue" => blue = value,
            _ => panic!(),
        }
    }
    Round {
        red: red,
        green: green,
        blue: blue,
    }
}

#[test]
fn test_count_possible_games() {
    assert_eq!(
        count_possible_games(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
            12, // Red
            13, // Blue
            14  // Green
        ),
        8
    );
}

#[test]
fn test_min_needed() {
    assert_eq!(
        min_needed(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ),
        2286
    );
}
