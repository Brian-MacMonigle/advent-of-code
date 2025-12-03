use std::env;
use std::fs;
use std::ops::Range;

fn main() {
    println!("Hello Day 2!\n");

    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(&path).expect(format!("Can not read {}", &path).as_str());

    let ranges = input
        .split(',')
        .filter(|&text| !text.is_empty())
        .map(|text| text.trim())
        .map(parse_range)
        .collect::<Vec<Range<u64>>>();

    let invalid_range_sum = sum_invalid_ids(&ranges, is_repeat);

    println!("Sum of invalid ids: {}", invalid_range_sum);

    let multi_repeat_invalid_range_sum = sum_invalid_ids(&ranges, is_multi_repeat);

    println!(
        "Sum of multi repeat invalid ids: {}",
        multi_repeat_invalid_range_sum
    );
}

fn parse_range(text: &str) -> Range<u64> {
    let mut parts = text.split('-');
    let first_str = parts.next().expect("Can't read first range param");
    let second_str = parts.next().expect("Can't read second range param");

    let first = first_str.parse::<u64>();
    let second = second_str.parse::<u64>();

    match (first, second) {
        (Ok(start), Ok(end)) => Range { start, end },
        (Err(err), Ok(_)) => panic!("Error in first range: {} - {}", &first_str, &err),
        (Ok(_), Err(err)) => panic!("Error in second range: {} - {}", &second_str, &err),
        (Err(err1), Err(err2)) => panic!(
            "Error in both ranges: {} - {} & {} - {}",
            &first_str, &err1, &second_str, &err2
        ),
    }
}

fn sum_invalid_ids(ranges: &[Range<u64>], matches: fn(u64) -> bool) -> u64 {
    let mut sum = 0;
    for range in ranges {
        for id in range.start..=range.end {
            if matches(id) {
                sum += id;
            }
        }
    }
    sum
}

fn is_repeat(id: u64) -> bool {
    let id_str = id.to_string();
    // only even length can be halved for comparisons
    if id_str.len() % 2 == 1 {
        return false;
    }
    let first = &id_str[0..id_str.len() / 2];
    let last = &id_str[id_str.len() / 2..id_str.len()];
    first == last
}

fn is_multi_repeat(id: u64) -> bool {
    let id_str = id.to_string();

    for size in (1..id_str.len()).filter(|size| id_str.len() % size == 0) {
        let parts_to_find = id_str.len() / size;
        let find = &id_str[0..size];
        let mut matched = true;
        for i in 1..parts_to_find {
            let found = &id_str[i * size..i * size + size];
            if find != found {
                matched = false;
                // break early if we find one false match for this size
                break;
            }
        }
        if matched {
            return true;
        }
    }
    return false;
}

#[test]
fn test_parse_range() {
    assert_eq!(parse_range("11-22"), Range { start: 11, end: 22 });
    assert_eq!(
        parse_range("1188511880-1188511890"),
        Range {
            start: 1188511880,
            end: 1188511890
        }
    );
}

#[test]
fn test_sum_invalid_ids() {
    assert_eq!(
        sum_invalid_ids(&vec![Range { start: 11, end: 22 }], is_repeat),
        11 + 22
    );

    assert_eq!(
        sum_invalid_ids(
            &vec![Range {
                start: 95,
                end: 115
            }],
            is_repeat
        ),
        99
    );

    assert_eq!(
        sum_invalid_ids(
            &vec![
                Range { start: 1, end: 2 },
                Range { start: 3, end: 4 },
                Range { start: 11, end: 11 },
                Range { start: 22, end: 22 },
            ],
            is_repeat
        ),
        11 + 22
    );
}

#[test]
fn test_is_repeat_true() {
    assert_eq!(is_repeat(11), true);
    assert_eq!(is_repeat(22), true);
    assert_eq!(is_repeat(99), true);
    assert_eq!(is_repeat(1010), true);
    assert_eq!(is_repeat(1188511885), true);
    assert_eq!(is_repeat(222222), true);
    assert_eq!(is_repeat(446446), true);
    assert_eq!(is_repeat(38593859), true);
}

#[test]
fn test_is_repeat_false() {
    assert_eq!(is_repeat(1698522), false);
    assert_eq!(is_repeat(1698528), false);
    assert_eq!(is_repeat(2121212118), false);
    assert_eq!(is_repeat(565653), false);
    assert_eq!(is_repeat(565659), false);
}

#[test]
fn test_is_multi_repeat_true() {
    assert_eq!(is_multi_repeat(11), true);
    assert_eq!(is_multi_repeat(1010), true);
    assert_eq!(is_multi_repeat(123123123), true);
}

#[test]
fn test_is_multi_repeat_false() {
    assert_eq!(is_multi_repeat(1698522), false);
    assert_eq!(is_multi_repeat(1698528), false);
    assert_eq!(is_multi_repeat(2121212118), false);
    assert_eq!(is_multi_repeat(565653), false);
    assert_eq!(is_multi_repeat(565659), false);
}
