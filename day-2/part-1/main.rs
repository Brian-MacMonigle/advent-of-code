use std::fs;

#[derive(Debug)]
enum Direction {
    Forward,
    Down,
    Up,
}

impl Direction {
    fn from(string: &str) -> Option<Direction> {
        match string {
            "forward" => Some(Direction::Forward),
            "down" => Some(Direction::Down),
            "up" => Some(Direction::Up),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Instruction {
    dir: Direction,
    mag: i32,
}

fn main() {
    let string = fs::read_to_string("input.txt")
        .expect("Couldn't read input.txt!");

    let instructions: Vec<Instruction> = string.lines()
        .map(|instr| {
            let mut instr_parts = instr.split(" ");
            return Instruction {
                dir: Direction::from(instr_parts.next()
                    .expect("Direction not in instruction!"))
                    .expect("Unable to parse Direction from instruction!"),
                mag: instr_parts.next()
                    .expect("Magnatude not in instruction!")
                    .parse()
                    .expect("Not a number!"),
            }
        })
        .collect();

    let mut forward = 0;
    let mut depth = 0;
    for Instruction { dir, mag } in instructions {
        match dir {
            Direction::Forward => forward += mag,
            Direction::Up => depth -= mag,
            Direction::Down => depth += mag,
        };

        println!("Move: {:?} {}", dir, mag);
        println!("Forward: {}; Depth: {}", forward, depth);
        println!("Product {}", forward * depth);
    };
}