use std::fs;

fn main() {
    println!("Hello Day 5!\n");

    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let almanac = Almanac::parse(&input);

    let lowest_location = min_seed_location(&almanac);
    println!("Lowest seed location: {lowest_location}");

    let lowest_seed_range_location = min_seed_range_loction(&almanac);
    println!("Lowest seed range location: {lowest_seed_range_location}");
}

#[derive(Debug)]
struct Map {
    dest: u64,
    source: u64,
    range: u64,
}

impl Map {
    fn transform(&self, value: u64) -> Option<u64> {
        if value >= self.source && value <= self.source + self.range {
            Some(value - self.source + self.dest)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    seed_soil: Vec<Map>,
    soil_fertilizer: Vec<Map>,
    fertilizer_water: Vec<Map>,
    water_light: Vec<Map>,
    light_temprature: Vec<Map>,
    temperature_humidity: Vec<Map>,
    humidity_location: Vec<Map>,
}

impl Almanac {
    fn parse(text: &str) -> Almanac {
        // Does not support /r/n line format
        let mut lines = text.trim().split("\n\n");

        // We assume the input file is in a specific order
        Almanac {
            seeds: Self::parse_seeds(lines.next().expect("seeds not found")),
            seed_soil: Self::parse_map(lines.next().expect("seed-to-soil not found")),
            soil_fertilizer: Self::parse_map(lines.next().expect("soil-to-fertilizer not found")),
            fertilizer_water: Self::parse_map(lines.next().expect("fertilizer-to-water not found")),
            water_light: Self::parse_map(lines.next().expect("water-to-light not found")),
            light_temprature: Self::parse_map(lines.next().expect("light-to-temprature not found")),
            temperature_humidity: Self::parse_map(
                lines.next().expect("temprature-to-humidity not found"),
            ),
            humidity_location: Self::parse_map(
                lines.next().expect("humidity-to-location not found"),
            ),
        }
    }

    fn parse_seeds(value: &str) -> Vec<u64> {
        value
            .split(':')
            .skip(1)
            .next()
            .expect("Failed to parse seeds")
            .split(" ")
            .filter_map(|digit| digit.parse::<u64>().ok())
            .collect()
    }

    fn parse_map(value: &str) -> Vec<Map> {
        value
            .lines()
            .skip(1)
            .map(|line| {
                let digits = line
                    .split(" ")
                    .filter_map(|digit| digit.parse::<u64>().ok())
                    .collect::<Vec<u64>>();
                Map {
                    dest: digits[0],
                    source: digits[1],
                    range: digits[2],
                }
            })
            .collect::<Vec<Map>>()
    }

    fn get_location(&self, seed: u64) -> u64 {
        vec![
            &self.seed_soil,
            &self.soil_fertilizer,
            &self.fertilizer_water,
            &self.water_light,
            &self.light_temprature,
            &self.temperature_humidity,
            &self.humidity_location,
        ]
        .iter()
        .fold(seed, |acc, maps| {
            maps.iter()
                .filter_map(|mapper| mapper.transform(acc))
                .next()
                .unwrap_or(acc)
        })
    }
}

fn min_seed_location(almanac: &Almanac) -> u64 {
    almanac
        .seeds
        .iter()
        .map(|seed| almanac.get_location(*seed))
        .min()
        .unwrap()
}

fn min_seed_range_loction(almanac: &Almanac) -> u64 {
    // This is way too computationally expensive, and takes multiple hours to run
    // TODO: Optimise this
    almanac
        .seeds
        .chunks(2)
        .flat_map(|window| window[0]..=(window[0] + window[1]))
        .map(|seed| (seed, almanac.get_location(seed)))
        .map(|(seed, location)| {
            // println!("{seed} : {location}");
            location
        })
        .min()
        .unwrap()
    // 0
}

#[cfg(test)]
mod tests {
    use super::*;

    static ALMANAC_TEXT: &str = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_part_one() {
        let almanac = Almanac::parse(ALMANAC_TEXT);
        assert_eq!(min_seed_location(&almanac), 35);
    }

    #[test]
    fn test_part_two() {
        let almanac = Almanac::parse(ALMANAC_TEXT);
        assert_eq!(min_seed_range_loction(&almanac), 46);
    }
}
