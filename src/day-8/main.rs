use std::collections::HashMap;
use std::fs;

static START_NODE: &str = "AAA";
static END_NODE: &str = "ZZZ";

fn main() {
    println!("Hello Day 7!\n");

    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let map = Map::parse(&input);

    let steps = map.count_steps();
    println!("There are {} steps to reach ZZZ", steps);

    let ghost_steps = map.count_ghost_steps();
    println!("There are {} ghost steps to reach **Z", ghost_steps);
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Map {
    directions: Vec<Direction>,
    nodes: HashMap<String, (String, String)>,
}

impl Direction {
    fn parse(dir: char) -> Direction {
        match dir {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Unknown Direction {}", dir),
        }
    }
}

impl Map {
    fn parse(text: &str) -> Map {
        let mut lines = text.lines().filter(|line| !line.is_empty());
        let directions = lines
            .next()
            .expect("Could not parse directions")
            .chars()
            .map(Direction::parse)
            .collect();

        let mut nodes = HashMap::new();

        for text in lines {
            let tokens = text
                .chars()
                .filter(|c| c.is_whitespace() || c.is_alphanumeric())
                .collect::<String>();
            let mut iter = tokens.split(" ").filter(|sub| !sub.is_empty());

            nodes.insert(
                iter.next().expect("Could not parse id").to_string(),
                (
                    iter.next()
                        .expect("Could not parse left branch")
                        .to_string(),
                    iter.next()
                        .expect("Could not parse right branch")
                        .to_string(),
                ),
            );
        }

        Map { directions, nodes }
    }

    fn next(&self, loc: &str, dir: &Direction) -> &str {
        let (left, right) = self.nodes.get(loc).expect("location not valid");
        match dir {
            Direction::Left => left,
            Direction::Right => right,
        }
    }

    fn count_steps(&self) -> usize {
        let mut location = START_NODE;
        let (count, _) = self
            .directions
            .iter()
            .cycle()
            .enumerate()
            .find(|(_, dir)| {
                location = self.next(location, dir);
                location == END_NODE
            })
            .expect("End node never found");
        count + 1
    }

    fn count_ghost_steps(&self) -> u64 {
        let locations: Vec<String> = self
            .nodes
            .keys()
            .filter(|key| key.ends_with("A"))
            .cloned()
            .collect();

        // Each ghost's path through the nodes will eventually reach a cycle
        // We find the cycle length by finding the steps between start and end
        // We then find the least common multiple of each cycle
        // Which is the minimum number of steps for all cycles to reach their end state
        let cycles: Vec<u64> = locations
            .into_iter()
            .map(|mut location| {
                let (count, _) = self
                    .directions
                    .iter()
                    .cycle()
                    .enumerate()
                    .find(|(_, dir)| {
                        location = self.next(&location, dir).to_string();
                        location.ends_with('Z')
                    })
                    .expect("End node never found");
                (count + 1) as u64
            })
            .collect();

        cycles
            .into_iter()
            .reduce(lcm)
            .expect("Could not factor Least Common Multiple")
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    if a == 0 {
        return b;
    }
    return gcd(b % a, a);
}

fn lcm(a: u64, b: u64) -> u64 {
    return (a * b) / gcd(a, b);
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEXT: &str = "\
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn test_part_one() {
        let map = Map::parse(TEXT);

        assert_eq!(map.count_steps(), 2);
    }

    static TEXT_TWO: &str = "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn test_part_one_two() {
        let map = Map::parse(TEXT_TWO);

        assert_eq!(map.count_steps(), 6);
    }

    static TEXT_THREE: &str = "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn test_part_two() {
        let map = Map::parse(TEXT_THREE);

        assert_eq!(map.count_ghost_steps(), 6);
    }
}
