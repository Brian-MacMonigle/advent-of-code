use std::env;
use std::fs;

fn main() {
    println!("Hello Day 3!\n");

    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(&path).expect(format!("Can not read {}", &path).as_str());

    let banks = parse_banks(input);

    let total_joltage: u32 = banks.iter().map(to_joltage).sum();
    println!("Total output joltage: {}", total_joltage);
}

fn parse_banks(input: String) -> Vec<String> {
    input
        .split('\n')
        .map(|text| text.trim())
        .filter(|&text| !text.is_empty())
        .map(|text| text.to_owned())
        .collect()
}

fn to_joltage(bank: &String) -> u32 {
    let numbers = bank
        .chars()
        .map(|char| char.to_digit(10).expect("Not a digit"))
        .enumerate();

    let mut num_one = 0;
    let mut num_one_index = -1;
    let mut num_two = 0;

    for (index, battery) in numbers {
        if battery > num_one && index != bank.len() - 1 {
            num_one_index = index as i32;
            num_one = battery;

            // Reset num two with new num one
            num_two = 0;
        } else if battery > num_two && index > num_one_index as usize {
            num_two = battery;
        }
    }

    num_one * 10 + num_two
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
    assert_eq!(to_joltage(&"12345".to_string()), 45);
    assert_eq!(to_joltage(&"123450".to_string()), 50);
    assert_eq!(to_joltage(&"54123".to_string()), 54);
    assert_eq!(to_joltage(&"987654321111111".to_string()), 98);
    assert_eq!(to_joltage(&"811111111111119".to_string()), 89);
    assert_eq!(to_joltage(&"234234234234278".to_string()), 78);
    assert_eq!(to_joltage(&"818181911112111".to_string()), 92);
}
