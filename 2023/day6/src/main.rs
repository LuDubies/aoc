use std::io::{BufReader, BufRead};
use std::fs::File;

fn main() {
    let file = File::open("input.txt").expect("No such file!");
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines()
                    .map(|l| l.expect("Could not read line!"))
                    .collect();

    let times = lines.iter()
                            .nth(0)
                            .unwrap()
                            .split(":")
                            .nth(1)
                            .unwrap()
                            .trim()
                            .split(" ")
                            .filter(|s| !s.is_empty())
                            .map(|s| s.parse::<i64>())
                            .filter_map(|i| i.ok());

    let distances = lines.iter()
                            .nth(1)
                            .unwrap()
                            .split(":")
                            .nth(1)
                            .unwrap()
                            .trim()
                            .split(" ")
                            .filter(|s| !s.is_empty())
                            .map(|s| s.parse::<i64>())
                            .filter_map(|i| i.ok());
    
    let result = times.zip(distances).map(|(t, d)| winning_options(t, d)).fold(1_i64, |a, n| a*n);

    println!("Result for Part 1: {}", result);

    // PART 2
    let one_race_time = lines.iter()
                            .nth(0)
                            .unwrap()
                            .split(":")
                            .nth(1)
                            .unwrap()
                            .chars()
                            .filter(|c| !c.is_whitespace())
                            .collect()
                            .


}

fn winning_options(time: i64, distance: i64) -> i64 {
    let mut winnings: i64 = 0;
    for holding_time in 1..time {
        let travelled: i64 = (time -holding_time) * holding_time;
        if travelled > distance {
            winnings += 1;
        }
    }
    return winnings;
}
