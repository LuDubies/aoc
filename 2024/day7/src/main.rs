use std::io::{BufReader, BufRead};
use std::fs::File;
use std::env;
use std::fmt;
use regex::Regex;
use std::time::Instant;

fn main() {

    let args: Vec<String> = env::args().collect();
    let mut filename = "input.txt";
    if args.len() > 1 && args[1] == "testdata"{
        filename = "testinput.txt";
    }
    let file = File::open(filename).expect("No such file!");
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.expect("Could not read line!"));

    // parsing
    let equation_re = Regex::new(r"(?<result>\d+): (?<numbers>[ \d]+)").unwrap();
    let mut equations: Vec<Equation> = Vec::new();

    for line in lines {
        let cap = equation_re.captures(&line).unwrap();
        equations.push(Equation {
            result: cap.name("result").unwrap().as_str().parse::<usize>().unwrap(),
            numbers: cap.name("numbers").unwrap().as_str().split(" ").map(|n| n.parse::<usize>().unwrap()).collect()
        })
    }

    // Part 1
    let part1_starttime = Instant::now();
    let mut solvable_euqations_sum: usize = 0;

    for equ in &equations {
        if equ.find_solution(false) {
            solvable_euqations_sum += equ.result;
            println!("Solvable: {}", equ);
        }
    }

    println!("[Part 1] Total calibration result: {} [Solved in {:?}]", solvable_euqations_sum, part1_starttime.elapsed());

    // Part 2
    let part2_starttime = Instant::now();
    let mut solvable_euqations_sum_2: usize = 0;

    for equ in &equations {
        if equ.find_solution(true) {
            solvable_euqations_sum_2 += equ.result;
            println!("Solvable: {}", equ);
        }
    }

    println!("[Part 2] Total calibration result: {} [Solved in {:?}]", solvable_euqations_sum_2, part2_starttime.elapsed());
}

struct Equation {
    result: usize,
    numbers: Vec<usize>
}

impl fmt::Display for Equation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} :", self.result)?;

        for num in &self.numbers {
            write!(f, " {}", num)?;
        }
        Ok(())
    }
}

impl Equation {

    fn find_solution(&self, allow_concat: bool) -> bool {
        if self.numbers[0] > self.result {
            return false;
        }
        if self.numbers.len() == 2 {
            if self.result == Operator::ADDITION.apply(self.numbers[0], self.numbers[1]) {
                return true;
            }
            else if self.result == Operator::MULTIPLICATION.apply(self.numbers[0], self.numbers[1]) {
                return true;
            }
            else if allow_concat && self.result == Operator::CONCATENATION.apply(self.numbers[0], self.numbers[1]) {
                return true;
            }
            else {
                return false;
            }
        }
        else {

            let mut subeq = Equation {
                result: self.result,
                numbers: vec![Operator::MULTIPLICATION.apply(self.numbers[0], self.numbers[1]); self.numbers.len() - 1]
            };
            subeq.numbers[1..].clone_from_slice(&self.numbers[2..]);

            if subeq.find_solution(allow_concat) {
                return true;
            }

            subeq.numbers[0] = Operator::ADDITION.apply(self.numbers[0], self.numbers[1]);
            if subeq.find_solution(allow_concat) {
                return true;
            }

            if allow_concat {
                subeq.numbers[0] = Operator::CONCATENATION.apply(self.numbers[0], self.numbers[1]);
                return subeq.find_solution(true);
            }
            false
        }
    }
}


enum Operator {
    ADDITION,
    MULTIPLICATION,
    CONCATENATION
}

impl Operator {
    fn apply(&self, op1: usize, op2: usize) -> usize {
        match self {
            Operator::ADDITION => op1 + op2,
            Operator::MULTIPLICATION => op1 * op2,
            Operator::CONCATENATION => {
                op1 * (10usize.pow(op2.checked_ilog10().unwrap_or(0) + 1)) + op2            
            }
        }
    }
}

