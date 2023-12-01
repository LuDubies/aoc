use std::cmp::Ordering;
use std::io::{BufReader, BufRead, Result};
use std::fs::File;

fn main() -> Result<()>{

    let file = File::open("input.txt").expect("No such file!");
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines()
                    .map(|l| l.expect("Could not read line!"))
                    .collect();

    let mut total_sum: usize = 0;

    let digit_strings = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
                            "0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

    for line in lines {

        let occurrences = digit_strings.map(|ds| first_and_last_occurence(&line, ds));

        let first_occs = occurrences.map(|(a, _)| a);
        let last_occs = occurrences.map(|(_, b)| b);

        let first_digit = first_occs.iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| min_options(**a, **b))
        .map(|(index, _)| index).unwrap() % 10;

        let last_digit = last_occs.iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
        .map(|(index, _)| index).unwrap() % 10;

        println!("{} {}", first_digit, last_digit);

        total_sum += 10 * first_digit + last_digit;

    }

    println!("Total sum is: {}", total_sum);
    return Ok(());
}

fn first_and_last_occurence(searchstring: &str, pattern: &str) -> (Option<usize>, Option<usize>) {
    let first_occ = searchstring.find(&pattern);
    let last_occ = searchstring.rfind(&pattern);

    return (first_occ, last_occ);
}

fn min_options(a: Option<usize>, b: Option<usize>) -> Ordering {
    match a {
        None => return std::cmp::Ordering::Greater,
        Some(xa) => match b {
            None => return std::cmp::Ordering::Less,
            Some(xb) => return xa.partial_cmp(&xb).unwrap(),

        },
    }
}