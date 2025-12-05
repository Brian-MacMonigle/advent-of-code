use std::cmp::{max, min};
use std::env;
use std::error::Error;
use std::fs;
use std::num::ParseIntError;

fn main() {
    println!("Hello Day 5!\n");

    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(&path).expect(format!("Can not read {}", &path).as_str());

    let lines = input.lines();

    let mut ranges = Ranges::new();
    let mut ingredients = Vec::new();

    for line in lines {
        if let Ok(range) = parse_range(line) {
            ranges.merge(range);
        } else if let Ok(ingredient) = parse_ingredient(line) {
            ingredients.push(ingredient);
        } else if !line.trim().is_empty() {
            panic!("Not range or ingredient")
        }
    }

    let mut fresh_count = 0;
    for ingredient in ingredients {
        if ranges.contains(ingredient) {
            fresh_count += 1
        }
    }

    println!("Fresh Ingredients: {}", fresh_count);
}

fn parse_range(line: &str) -> Result<Range, Box<dyn Error>> {
    let mut parts = line.split('-');
    let start = parts
        .next()
        .ok_or("Can't parse range start")?
        .parse::<u64>()?;
    let end = parts
        .next()
        .ok_or("Can't parse range end")?
        .parse::<u64>()?;
    Ok(Range { start, end })
}

fn parse_ingredient(line: &str) -> Result<u64, ParseIntError> {
    Ok(line.parse::<u64>()?)
}

#[derive(PartialEq, Debug, Clone, Copy)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn contains(&self, value: u64) -> bool {
        value >= self.start && value <= self.end
    }

    fn overlap(&self, other: &Range) -> bool {
        self.contains(other.start)
            || self.contains(other.end)
            || other.contains(self.start)
            || other.contains(self.end)
    }

    fn merge(&self, other: &Range) -> Result<Range, String> {
        if !self.overlap(other) {
            return Err("Ranges must overlap to merge".to_owned());
        }
        Ok(Range {
            start: min(self.start, other.start),
            end: max(self.end, other.end),
        })
    }
}

struct Ranges(Vec<Range>);

impl Ranges {
    fn new() -> Self {
        Ranges(Vec::new())
    }

    fn merge(&mut self, other: Range) {
        let mut merged_ranges = Vec::with_capacity(self.0.len() + 1);
        let mut merged = other;
        for range in self.0.iter() {
            if let Ok(result) = merged.merge(&range) {
                merged = result
            } else {
                merged_ranges.push(*range);
            }
        }
        merged_ranges.push(merged);
        self.0 = merged_ranges
    }

    fn contains(&self, value: u64) -> bool {
        self.0.iter().any(|range| range.contains(value))
    }
}

#[test]
fn test_parse_range() {
    assert_eq!(
        parse_range("123-456").expect("Parse range failure"),
        Range {
            start: 123,
            end: 456
        }
    );
}

#[test]
fn test_parse_ingredient() {
    assert_eq!(
        parse_ingredient("123").expect("Parse ingredient failure"),
        123
    );
}

#[test]
fn test_range_overlap_edge() {
    let one = Range { start: 10, end: 20 };
    let two = Range { start: 5, end: 10 };
    assert!(one.overlap(&two));
    assert!(two.overlap(&one));
}

#[test]
fn test_range_overlap_subset() {
    let one = Range { start: 10, end: 20 };
    let two = Range { start: 12, end: 15 };
    assert!(one.overlap(&two));
    assert!(two.overlap(&one));
}

#[test]
fn test_range_merge() {
    let one = Range { start: 10, end: 20 };
    let two = Range { start: 20, end: 30 };
    assert_eq!(one.merge(&two), Ok(Range { start: 10, end: 30 }));
    assert_eq!(two.merge(&one), Ok(Range { start: 10, end: 30 }));
}

#[test]
fn test_range_merge_subset() {
    let one = Range { start: 10, end: 20 };
    let two = Range { start: 12, end: 15 };
    assert_eq!(one.merge(&two), Ok(Range { start: 10, end: 20 }));
    assert_eq!(two.merge(&one), Ok(Range { start: 10, end: 20 }));
}

#[test]
fn test_range_merge_overlap() {
    let one = Range { start: 10, end: 20 };
    let two = Range { start: 15, end: 30 };
    assert_eq!(one.merge(&two), Ok(Range { start: 10, end: 30 }));
    assert_eq!(two.merge(&one), Ok(Range { start: 10, end: 30 }));
}

#[test]
fn test_range_merge_failure() {
    let one = Range { start: 10, end: 20 };
    let two = Range { start: 21, end: 30 };
    assert!(one.merge(&two).is_err());
}

#[test]
fn test_ranges_merge_join() {
    let one = Range { start: 10, end: 20 };
    let two = Range { start: 15, end: 30 };
    let mut ranges = Ranges::new();
    ranges.merge(one);
    assert_eq!(ranges.0[0], one);

    ranges.merge(two);
    let expected = one.merge(&two).expect("Merge failed");
    assert_eq!(ranges.0[0], expected);
    assert_eq!(ranges.0.len(), 1);
}

#[test]
fn test_ranges_merge_concat() {
    let one = Range { start: 10, end: 20 };
    let two = Range { start: 25, end: 30 };
    let mut ranges = Ranges::new();
    ranges.merge(one);
    ranges.merge(two);

    assert!(ranges.0.contains(&one));
    assert!(ranges.0.contains(&two));
}

#[test]
fn test_ranges_merge_multiple() {
    let one = Range { start: 10, end: 20 };
    let two = Range { start: 25, end: 30 };
    let three = Range { start: 20, end: 25 };
    let mut ranges = Ranges::new();
    ranges.merge(one);
    ranges.merge(two);
    ranges.merge(three);

    let expected = Range { start: 10, end: 30 };
    assert!(ranges.0.contains(&expected));
    assert_eq!(ranges.0.len(), 1);
}

#[test]
fn test_ranges_contains() {
    let one = Range { start: 10, end: 15 };
    let two = Range { start: 20, end: 30 };
    let three = Range { start: 50, end: 60 };
    let mut ranges = Ranges::new();
    ranges.merge(one);
    ranges.merge(two);
    ranges.merge(three);

    assert!(ranges.contains(10));
    assert!(ranges.contains(15));
    assert!(ranges.contains(25));
    assert!(ranges.contains(55));

    assert!(!ranges.contains(17));
    assert!(!ranges.contains(40));
}
