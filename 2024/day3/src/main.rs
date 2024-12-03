use std::io::{BufReader, BufRead};
use std::fs::File;
use regex::Regex;

fn main() {

    let file = File::open("input.txt").expect("No such file!");
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.expect("Could not read line!"));

    let mut part1_result: u64 = 0;
    let mut part2_result: u64 = 0;
    let mut enabled: bool = true;

    let mul_re =  Regex::new(r"mul\((?<first>\d{1,3}),(?<second>\d{1,3})\)|(?<enable>do\(\))|(?<disable>don't\(\))").unwrap();

    for line in lines
    {
        for cap in mul_re.captures_iter(&line) //.map(|caps| parse_multiplication(caps))
        {
            /* Handle do() and dont't() matches. */
            if cap.name("enable").is_some()
            {
                enabled = true;
            }
            else if cap.name("disable").is_some()
            {
                enabled = false;
            }

            /* Handle multiplications. */
            if cap.name("first").is_some()
            {
                let mul_res = parse_multiplication(cap);
                part1_result += mul_res;
                if enabled
                {
                    part2_result += mul_res;
                }
            }
        }
    }

    println!("[Part 1] Total sum of multiplications: {}", part1_result);
    println!("[Part 2] Controlled sum of multiplications: {}", part2_result);
}

fn parse_multiplication(caps: regex::Captures) -> u64
{
    let first = caps["first"].parse::<u64>().unwrap();
    let second = caps["second"].parse::<u64>().unwrap();

    return first * second;
}
