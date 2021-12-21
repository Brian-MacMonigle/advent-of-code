use std::fs;

fn main() {
    let string = fs::read_to_string("input.txt")
        .expect("Couldn't read input.txt!");

    let depths: Vec<u32> = string.lines()
        .map(|num| num.parse().expect("Not a number!"))
        .collect();

    let mut rolling_depths: Vec<u32> = vec![0; depths.len()];
    for (i, depth) in depths.into_iter().enumerate() {
        rolling_depths[i] += depth;
        if i + 1 < rolling_depths.len() { rolling_depths[i + 1] += depth };
        if i + 2 < rolling_depths.len() { rolling_depths[i + 2] += depth };
    }
    let rolling_depths: Vec<u32> = rolling_depths[2..]
        .to_vec();

    let mut last_value: u32 = 0;
    let mut increased_count = 0;

    for value in rolling_depths {
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