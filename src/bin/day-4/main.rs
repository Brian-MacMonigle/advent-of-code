use std::env;
use std::fmt;
use std::fs;

fn main() {
    println!("Hello Day 4!\n");

    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(&path).expect(format!("Can not read {}", &path).as_str());

    let grid = Grid::<140, 140>::new(&input).expect("Grid parse failed!");
    let accessible_string = grid.to_accessible_string();
    println!("{}\n", accessible_string);
    let accessible_sum = grid.sum_accessible();

    println!("Sum of accessible papers: {}", accessible_sum);

    let removed = grid.remove_all_accessible();
    let removed_string = removed.to_accessible_string();
    println!("{}\n", removed_string);

    let removed_papers = grid.total_papers() - removed.total_papers();

    println!("Total papers removed: {}", removed_papers);
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum Square {
    Clear,
    Paper,
}

impl Square {
    fn new(input: char) -> Result<Square, String> {
        match input {
            '.' => Ok(Square::Clear),
            '@' => Ok(Square::Paper),
            'x' => Ok(Square::Clear),
            x => Err(format!("{} is not '.', '@', or 'x'", x)),
        }
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Square::Clear => write!(f, "."),
            Square::Paper => write!(f, "@"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Row<const W: usize>([Square; W]);

impl<const W: usize> Row<W> {
    fn new(input: &str) -> Result<Row<W>, String> {
        if input.len() != W {
            return Err(format!("Width {} is expected but found {}", W, input.len()));
        }
        let mut row: [Square; W] = [Square::Clear; W];
        for (index, char) in input.chars().enumerate() {
            row[index] = Square::new(char)?;
        }
        Ok(Row(row))
    }
}

#[derive(Debug, Copy, Clone)]
struct Grid<const W: usize, const H: usize>([Row<W>; H]);

impl<const W: usize, const H: usize> Grid<W, H> {
    fn new(input: &str) -> Result<Grid<W, H>, String> {
        let mut columns: [Row<W>; H] = [Row([Square::Clear; W]); H];

        let mut input_iter = input
            .split('\n')
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .enumerate()
            .peekable();
        while let Some((index, line)) = input_iter.next() {
            if index > H {
                return Err(format!(
                    "Expected {} lines but found at least {} lines",
                    H, index
                ));
            } else if index != H - 1 && input_iter.peek().is_none() {
                return Err(format!(
                    "Expected {} lines but found {} lines",
                    H,
                    index + 1
                ));
            }

            columns[index] = Row::new(line)?;
        }

        Ok(Grid(columns))
    }

    fn accessible(self, row: usize, column: usize) -> bool {
        // convert type for easier mathing
        let row = row as i32;
        let column = column as i32;

        let top_left = (row - 1, column - 1);
        let top = (row, column - 1);
        let top_right = (row + 1, column - 1);
        let left = (row - 1, column);
        let right = (row + 1, column);
        let bot_left = (row - 1, column + 1);
        let bot = (row, column + 1);
        let bot_right = (row + 1, column + 1);

        let mut paper_count = 0;
        for (x, y) in [
            top_left, top, top_right, left, right, bot_left, bot, bot_right,
        ] {
            // check for out of bounds
            // safe conversion because < 0 case already checked
            if x < 0 || x as usize >= W || y < 0 || y as usize >= H {
                continue;
            }
            if self.0[y as usize].0[x as usize] == Square::Paper {
                paper_count += 1;
            }
        }

        paper_count < 4
    }

    fn sum_accessible(self) -> usize {
        let mut accessible = 0;

        for (y, row) in self.0.iter().enumerate() {
            for (x, square) in row.0.iter().enumerate() {
                if *square == Square::Paper && self.accessible(x, y) {
                    accessible += 1;
                }
            }
        }

        accessible
    }

    fn to_accessible_string(self) -> String {
        // add padding for newlines
        let mut output = String::with_capacity((W + 2) * H + 2);

        for (y, row) in self.0.iter().enumerate() {
            for (x, square) in row.0.iter().enumerate() {
                if *square == Square::Paper && self.accessible(x, y) {
                    output.push('x');
                } else {
                    output.push_str(&square.to_string());
                }
            }
            if y != H - 1 {
                output.push('\n');
            }
        }

        output
    }

    fn remove_accessible(self) -> Grid<W, H> {
        let accessible_string = self.to_accessible_string();
        let removed = Grid::new(&accessible_string).expect("Failed parsing accessible string");
        removed
    }

    fn remove_all_accessible(self) -> Grid<W, H> {
        let mut accessible = self.sum_accessible();
        let mut removed = self.remove_accessible();
        while accessible != 0 {
            removed = removed.remove_accessible();
            accessible = removed.sum_accessible();
        }
        removed
    }

    fn total_papers(self) -> usize {
        let mut count = 0;
        for row in self.0.iter() {
            for square in row.0.iter() {
                if *square == Square::Paper {
                    count += 1;
                }
            }
        }
        count
    }
}

#[test]
fn test_square_new_clear() {
    assert_eq!(Square::new('.'), Ok(Square::Clear));
}

#[test]
fn test_square_new_paper() {
    assert_eq!(Square::new('@'), Ok(Square::Paper));
}

#[test]
fn test_square_new_removed_paper() {
    assert_eq!(Square::new('x'), Ok(Square::Clear));
}

#[test]
fn test_square_new_invalid() {
    assert!(Square::new('B').is_err());
}

#[test]
fn test_row_new() {
    let row = Row::<6>::new("x.@.@x").expect("Row parse failed!");

    assert_eq!(row.0[0], Square::Clear);
    assert_eq!(row.0[1], Square::Clear);
    assert_eq!(row.0[2], Square::Paper);
    assert_eq!(row.0[3], Square::Clear);
    assert_eq!(row.0[4], Square::Paper);
    assert_eq!(row.0[5], Square::Clear);
}

#[test]
fn test_row_new_invalid_length_short() {
    assert!(Row::<2>::new("...").is_err());
}

#[test]
fn test_row_new_invalid_length_long() {
    assert!(Row::<4>::new("...").is_err());
}

#[test]
fn test_row_new_invalid_contents() {
    assert!(Row::<4>::new("a.@x").is_err());
}

#[test]
fn test_grid_new() {
    let grid: Grid<4, 2> = Grid::new(".x@.\n.@.x").expect("Grid parse failed!");
    assert_eq!(grid.0[0].0[0], Square::Clear);
    assert_eq!(grid.0[0].0[1], Square::Clear);
    assert_eq!(grid.0[0].0[2], Square::Paper);
    assert_eq!(grid.0[0].0[3], Square::Clear);

    assert_eq!(grid.0[1].0[0], Square::Clear);
    assert_eq!(grid.0[1].0[1], Square::Paper);
    assert_eq!(grid.0[1].0[2], Square::Clear);
    assert_eq!(grid.0[1].0[3], Square::Clear);
}

#[test]
fn test_grid_width_short() {
    assert!(Grid::<5, 2>::new("xx@x\nx@xx").is_err());
}

#[test]
fn test_grid_width_long() {
    assert!(Grid::<2, 2>::new("xx@x\nx@xx").is_err());
}

#[test]
fn test_grid_hight_short() {
    assert!(Grid::<4, 3>::new("xx@x\nx@xx").is_err());
}

#[test]
fn test_grid_hight_long() {
    assert!(Grid::<4, 1>::new("xx@x\nx@xx").is_err());
}

#[test]
fn test_grid_count_papers() {
    assert_eq!(
        Grid::<4, 2>::new("..@.\n.@..")
            .expect("Grid parse failed!")
            .total_papers(),
        2
    );
}

#[test]
fn test_grid_accessible() {
    let input = "
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
    ";
    let grid = Grid::<10, 10>::new(input).expect("Grid parse failed!");
    let accessible = grid.to_accessible_string();
    println!("{}", accessible);
    let expected = "
..xx.xx@x.
x@@.@.@.@@
@@@@@.x.@@
@.@@@@..@.
x@.@@@@.@x
.@@@@@@@.@
.@.@.@.@@@
x.@@@.@@@@
.@@@@@@@@.
x.x.@@@.x.
"
    .trim();
    assert_eq!(accessible, expected);

    assert_eq!(grid.sum_accessible(), 13);
}

#[test]
fn test_grid_remove_accessible() {
    let input = "
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
    ";
    let grid = Grid::<10, 10>::new(input).expect("Grid parse failed!");
    let removed = grid.remove_accessible();
    let accessible = removed.to_accessible_string();
    println!("{}", accessible);
    // Note: we are using the 'now accessible' from the example
    let expected = "
.......x..
.@@.x.x.@x
x@@@@...@@
x.@@@@..x.
.@.@@@@.x.
.x@@@@@@.x
.x.@.@.@@@
..@@@.@@@@
.x@@@@@@@.
....@@@...
"
    .trim();
    assert_eq!(accessible, expected);

    assert_eq!(grid.sum_accessible(), 13);
}

#[test]
fn test_grid_remove_all_accessible() {
    let input = "
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
    ";
    let grid = Grid::<10, 10>::new(input).expect("Grid parse failed!");
    let removed = grid.remove_all_accessible();
    let accessible = removed.to_accessible_string();
    println!("{}", accessible);
    let expected = "
..........
..........
..........
....@@....
...@@@@...
...@@@@@..
...@.@.@@.
...@@.@@@.
...@@@@@..
....@@@...
"
    .trim();
    assert_eq!(accessible, expected);
    assert_eq!(removed.sum_accessible(), 0);
    assert_eq!(grid.total_papers() - removed.total_papers(), 43);
}
