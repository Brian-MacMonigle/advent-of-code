use std::collections::HashMap;
use std::fs;

static RADIX: u32 = 10;

fn main() {
    println!("Hello Day 3!\n");

    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let schematic = Schematic::parse(&input);

    let engine_part_sum = sum_engine_parts(&schematic);

    println!("Sum of engine part numbers: {engine_part_sum}");

    let gear_ratio_sum = sum_gear_ratios(&schematic);

    println!("Sum of gear part numbers: {gear_ratio_sum}");
}

#[derive(Debug)]
struct Schematic {
    rows: Vec<Row>,
    neighbors: Vec<Neighbor>,
}

#[derive(Debug)]
struct Row {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}

#[derive(Clone, Debug)]
struct Neighbor {
    number: Number,
    symbol: Symbol,
}

#[derive(Clone, Copy, Debug)]
struct Number {
    value: u32,
    size: u32,
    row: u32,
    column: u32,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
struct Symbol {
    value: char,
    row: u32,
    column: u32,
}

impl Schematic {
    fn parse(text: &str) -> Schematic {
        let mut schematic = Schematic {
            rows: Vec::new(),
            neighbors: Vec::new(),
        };

        // Parse Numbers & Symbols from text
        for (row, line) in text.lines().enumerate() {
            schematic.rows.push(Row {
                numbers: Vec::new(),
                symbols: Vec::new(),
            });

            let mut char_iter = line.chars().enumerate().peekable();
            while let Some((column, value)) = char_iter.next() {
                if value.is_digit(RADIX) {
                    // Found a number
                    let mut number = Number {
                        value: value.to_digit(RADIX).unwrap(),
                        size: 1,
                        row: row as u32,
                        column: column as u32,
                    };
                    // Consume all next numbers
                    // We peek next value, without consuming, to know we have built the whole number
                    while let Some(true) = char_iter.peek().map(|(_, value)| value.is_digit(RADIX))
                    {
                        let (_, value) = char_iter.next().unwrap();
                        number.value = number.value * 10 + value.to_digit(RADIX).unwrap();
                        number.size = number.size + 1;
                    }
                    schematic.rows[row].numbers.push(number);
                } else if value != '.' {
                    // Found a symbol
                    schematic.rows[row].symbols.push(Symbol {
                        value: value,
                        row: row as u32,
                        column: column as u32,
                    });
                }
            }
        }

        // Parse Neighbors from Numbers & Symbols
        for (i, row) in schematic.rows.iter().enumerate() {
            for number in row.numbers.iter().cloned() {
                // There has to be a cleaner way to do this
                // For current row, previous row, and next row
                // We compare all symbols to number for is_neighbor
                // Then extend Schematic's neighbor list with results
                let mut symbols = row
                    .symbols
                    .iter()
                    .filter(|symbol| number.is_neighbor(symbol))
                    .cloned()
                    .collect::<Vec<Symbol>>();
                if i != 0 {
                    symbols.extend(
                        schematic.rows[i - 1]
                            .symbols
                            .iter()
                            .filter(|symbol| number.is_neighbor(symbol))
                            .cloned()
                            .collect::<Vec<Symbol>>(),
                    )
                }
                if i + 1 < schematic.rows.len() - 1 {
                    symbols.extend(
                        schematic.rows[i + 1]
                            .symbols
                            .iter()
                            .filter(|symbol| number.is_neighbor(symbol))
                            .cloned()
                            .collect::<Vec<Symbol>>(),
                    )
                }

                schematic.neighbors.extend(
                    symbols
                        .into_iter()
                        .map(|symbol| Neighbor { number, symbol }),
                );
            }
        }

        schematic
    }
}

impl Number {
    fn is_neighbor(&self, symbol: &Symbol) -> bool {
        self.row.abs_diff(symbol.row) <= 1
            && self.column <= symbol.column + 1
            && self.column + self.size >= symbol.column
    }
}

fn sum_engine_parts(schematic: &Schematic) -> u32 {
    // An engine part is any number which neighbors a symbol
    schematic
        .neighbors
        .iter()
        .map(|neighbor| neighbor.number.value)
        .sum()
}

fn sum_gear_ratios(schematic: &Schematic) -> u32 {
    // A gear pair are any two numbers which border a '*' symbol
    // A gear ratio is the product of a gear pair
    schematic
        .neighbors
        .iter()
        .filter(|neighbor| neighbor.symbol.value == '*')
        .map(|neighbor| (neighbor.symbol, neighbor.number))
        // Group numbers by neighbor symbol
        .fold(
            HashMap::<Symbol, Vec<Number>>::new(),
            |mut acc, (symbol, number)| {
                if let Some(numbers) = acc.get_mut(&symbol) {
                    numbers.push(number);
                } else {
                    acc.insert(symbol, vec![number]);
                }
                acc
            },
        )
        .into_values()
        .filter(|numbers| numbers.len() >= 2)
        // Calculate gear ratio from gears pairs found
        .map(|numbers| {
            numbers
                .into_iter()
                .map(|number| number.value)
                .product::<u32>()
        })
        .sum()
}

#[test]
fn test_sum_engine_parts() {
    assert_eq!(
        sum_engine_parts(&Schematic::parse(
            "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
        )),
        4361
    )
}

#[test]
fn test_sum_gear_ratios() {
    assert_eq!(
        sum_gear_ratios(&Schematic::parse(
            "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
        )),
        467835
    )
}
