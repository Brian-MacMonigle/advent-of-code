use std::fs;

fn main() {
    let string = fs::read_to_string("input.txt")
        .expect("Couldn't read input.txt!");

    let depths: Vec<u32> = string.lines()
        .map(|num| num.parse().expect("Not a number!"))
        .collect();
    
    let mut last_value: u32 = 0;
    let mut increased_count = 0;

    for value in depths {
        if last_value == 0 { last_value = value; continue; }
        if last_value < value {
            println!("{} (increased)", value);
            increased_count += 1;
        }
        if last_value > value {
            println!("{} (decreased)", value);
        }
        last_value = value;
    }

    println!("increased {} times!", increased_count);
}