use std::fs;

fn main() {
    println!("Hello Day 9!\n");

    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let lines: Vec<ReportLine> = input.lines().map(ReportLine::parse).collect();

    let prediction_sum: i64 = lines.iter().map(|report_line| report_line.prediction).sum();
    println!("Sum of all predictions: {prediction_sum}");

    let history_sum: i64 = lines.iter().map(|report_line| report_line.history).sum();
    println!("Sum of all histories: {history_sum}");
}

struct ReportLine {
    #[allow(dead_code)]
    input: Vec<i64>,
    prediction: i64,
    history: i64,
}

impl ReportLine {
    fn parse(text: &str) -> ReportLine {
        let input: Vec<i64> = text
            .split(" ")
            .filter_map(|digit| digit.parse().ok())
            .collect();

        let mut accelerations: Vec<Vec<i64>> = vec![input.to_owned()];

        while accelerations.last().unwrap().iter().any(|num| *num != 0) {
            accelerations.push(Self::acceleration(accelerations.last().unwrap()));
        }

        let prediction = accelerations
            .iter()
            .rev()
            .map(|vec| vec.last().unwrap())
            .cloned()
            .reduce(|change, value| value + change)
            .unwrap();

        let history = accelerations
            .iter()
            .rev()
            .map(|vec| vec.first().unwrap())
            .cloned()
            .reduce(|change, value| value - change)
            .unwrap();

        ReportLine {
            input,
            prediction,
            history,
        }
    }

    fn acceleration(vec: &Vec<i64>) -> Vec<i64> {
        vec.windows(2).map(|slice| slice[1] - slice[0]).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEXT: &str = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_part_one() {
        let lines: Vec<ReportLine> = TEXT.lines().map(ReportLine::parse).collect();

        assert_eq!(lines[0].prediction, 18);
        assert_eq!(lines[1].prediction, 28);
        assert_eq!(lines[2].prediction, 68);
    }

    #[test]
    fn test_part_two() {
        let lines: Vec<ReportLine> = TEXT.lines().map(ReportLine::parse).collect();

        assert_eq!(lines[2].history, 5);
    }
}
