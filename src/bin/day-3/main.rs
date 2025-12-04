use std::collections::VecDeque;
use std::env;
use std::fs;

fn main() {
    println!("Hello Day 3!\n");

    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(&path).expect(format!("Can not read {}", &path).as_str());

    let banks = parse_banks(input);

    let total_joltage: u64 = banks.iter().map(|bank| to_joltage(bank, 2)).sum();
    println!("Total output joltage: {}", total_joltage);

    let total_12_joltage: u64 = banks.iter().map(|bank| to_joltage(bank, 12)).sum();
    println!("Total output 12 joltage: {}", total_12_joltage);
}

fn parse_banks(input: String) -> Vec<String> {
    input
        .split('\n')
        .map(|text| text.trim())
        .filter(|&text| !text.is_empty())
        .map(|text| text.to_owned())
        .collect()
}

fn to_joltage(bank: &String, amount: usize) -> u64 {
    // Go through numbers in reverse order so we can constant time check if we need to insert
    let rev_numbers = bank
        .chars()
        .map(|char| char.to_digit(10).expect("Not a digit") as u64)
        .rev();

    // Utilizes ring buffer for push/pop
    let mut joltage = VecDeque::with_capacity(amount);

    for battery in rev_numbers {
        // Fill up amount buffer
        if joltage.len() < amount {
            joltage.push_front(battery);
        // Check if we should insert this new value
        } else if &battery >= joltage.get(0).expect("Vec empty") {
            // We need to find the first number that's smaller than it's neighbor
            // Ex [3, 4], where 40 > 30, so we should remove 3
            let mut to_remove = 0;
            let mut iter = joltage.iter().enumerate().peekable();
            while let Some((idx, value)) = iter.next() {
                if let Some((_, next)) = iter.peek() {
                    if value < next {
                        to_remove = idx;
                        break;
                    }
                } else {
                    // We are an ordered list, so just pop the last element
                    to_remove = idx;
                }
            }
            joltage.remove(to_remove);
            joltage.push_front(battery);
        }
    }

    // Use 10^(2*index) to base 10 sum the numbers in buffer
    joltage
        .into_iter()
        .rev()
        .enumerate()
        .map(|(index, num)| num * 10_u64.pow(index as u32))
        .sum()
}

#[test]
fn test_parse_banks() {
    assert_eq!(
        parse_banks(
            "123
        456
        "
            .to_string()
        ),
        vec!["123", "456"]
    );
    assert_eq!(
        parse_banks(
            "3233434223352253322244323562413222322422522622312422332123223123235422212196323222332232332242222211".to_string()
        ),
        vec!["3233434223352253322244323562413222322422522622312422332123223123235422212196323222332232332242222211"]
    );
}

#[test]
fn test_to_joltage() {
    assert_eq!(to_joltage(&"12345".to_string(), 2), 45);
    assert_eq!(to_joltage(&"123450".to_string(), 2), 50);
    assert_eq!(to_joltage(&"4123".to_string(), 2), 43);
    assert_eq!(to_joltage(&"54123".to_string(), 2), 54);
    assert_eq!(to_joltage(&"987654321111111".to_string(), 2), 98);
    assert_eq!(to_joltage(&"811111111111119".to_string(), 2), 89);
    assert_eq!(to_joltage(&"234234234234278".to_string(), 2), 78);
    assert_eq!(to_joltage(&"818181911112111".to_string(), 2), 92);

    assert_eq!(to_joltage(&"43215".to_string(), 4), 4325);
    assert_eq!(to_joltage(&"43152".to_string(), 4), 4352);
    assert_eq!(to_joltage(&"43352".to_string(), 4), 4352);
    assert_eq!(to_joltage(&"435352".to_string(), 4), 5352);
    assert_eq!(to_joltage(&"435352".to_string(), 5), 45352);

    assert_eq!(to_joltage(&"987654321111111".to_string(), 12), 987654321111);
    assert_eq!(to_joltage(&"811111111111119".to_string(), 12), 811111111119);
    assert_eq!(to_joltage(&"234234234234278".to_string(), 12), 434234234278);
    assert_eq!(to_joltage(&"818181911112111".to_string(), 12), 888911112111);
}
