
use std::io::{BufReader, BufRead};
use std::fs::File;


fn main() {
    let file = File::open("input.txt").expect("No such file!");
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.expect("Could not read line!"));

    let results: (i64, i64) = lines
    .map(|l| parse_line(l))
    .map(|s| (calc_next_value(s.clone()), calc_prev_value(s)))
    .fold((0, 0), |(a0, a1), (n0, n1)| (a0 + n0, a1 + n1));
    
    println!("Results are {:?}", results);
}


fn parse_line(line: String) -> Vec<i64> {
    return line
    .split(" ")
    .map(|num| num.parse::<i64>().unwrap())
    .collect();
}


fn calc_next_value(series: Vec<i64>) -> i64 {
    if 0 == series.len() {
        println!("Hit a 0 len array, investigate!");
        return 0;
    }
    if series.iter().all(|e| 0 == *e) {
        return 0;
    }

    let mut nextvec: Vec<i64> = vec![0; series.len() - 1];

    for indx in 0..nextvec.len() {
        nextvec[indx] = series[indx + 1] - series[indx];
    }

    let diff_to_next_val = calc_next_value(nextvec);

    return series.last().unwrap() + diff_to_next_val;
}

fn calc_prev_value(series: Vec<i64>) -> i64 {
    if 0 == series.len() {
        println!("Hit a 0 len array, investigate!");
        return 0;
    }
    if series.iter().all(|e| 0 == *e) {
        return 0;
    }

    let mut nextvec: Vec<i64> = vec![0; series.len() - 1];

    for indx in 0..nextvec.len() {
        nextvec[indx] = series[indx + 1] - series[indx];
    }

    let diff_to_prev_val = calc_prev_value(nextvec);

    return series.first().unwrap() - diff_to_prev_val;
}