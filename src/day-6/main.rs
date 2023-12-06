use std::fs;

fn main() {
    println!("Hello Day 6!\n");

    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let races = Race::parse(&input);

    let product_winning_presses_counts = margin_of_error(&races);
    println!("Product of number of ways each race can be won: {product_winning_presses_counts}");

    let race = vec![Race::parse_race(&input)];
    let winning_values = margin_of_error(&race);
    println!("Number of ways to win race: {winning_values}");
}

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn parse(text: &str) -> Vec<Race> {
        let mut lines = text.lines();
        let time = lines
            .next()
            .expect("Times not found")
            .split(" ")
            .filter_map(|digit| digit.parse::<u64>().ok());
        let distance = lines
            .next()
            .expect("Distance not found")
            .split(" ")
            .filter_map(|digit| digit.parse::<u64>().ok());
        time.zip(distance)
            .map(|(time, distance)| Race { time, distance })
            .collect()
    }

    fn parse_race(text: &str) -> Race {
        let mut lines = text.lines();
        let time = lines
            .next()
            .expect("Times not found")
            .split(" ")
            .skip(1)
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>()
            .join("")
            .parse::<u64>()
            .expect("Failed to parse Time digit");
        let distance = lines
            .next()
            .expect("Distance not found")
            .split(" ")
            .skip(1)
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>()
            .join("")
            .parse::<u64>()
            .expect("Failed to parse Distance digit");
        Race { time, distance }
    }

    fn distance(&self, pressed_time: u64) -> u64 {
        (self.time - pressed_time) * pressed_time
    }

    fn is_record(&self, pressed_time: u64) -> bool {
        self.distance(pressed_time) > self.distance
    }

    fn min_record_time(&self) -> u64 {
        (1..=self.time)
            .find(|pressed| self.is_record(*pressed))
            .expect("No record possible")
    }

    fn max_record_time(&self) -> u64 {
        (1..=self.time)
            .rev()
            .find(|pressed| self.is_record(*pressed))
            .expect("No record possible")
    }

    fn record_pressed_times_count(&self) -> u64 {
        self.max_record_time() - self.min_record_time() + 1
    }
}

fn margin_of_error(races: &Vec<Race>) -> u64 {
    races
        .iter()
        .map(|race| race.record_pressed_times_count())
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEXT: &str = "\
Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_part_one() {
        let races = Race::parse(TEXT);
        dbg!(&races);
        assert_eq!(margin_of_error(&races), 288);
    }

    #[test]
    fn test_part_two() {
        let races = vec![Race::parse_race(TEXT)];
        assert_eq!(margin_of_error(&races), 71503);
    }
}
