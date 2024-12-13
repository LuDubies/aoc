use std::io::{BufReader, BufRead};
use std::fs::File;
use std::env;
use regex::Regex;

fn main() {

    let args: Vec<String> = env::args().collect();
    let mut filename = "input.txt";
    if args.len() > 1 && args[1] == "testdata"{
        filename = "testinput.txt";
    }
    let file = File::open(filename).expect("No such file!");
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.expect("Could not read line!"));

    let a_re = Regex::new(r"Button A: X\+(?<a_x>\d+), Y\+(?<a_y>\d+)").unwrap();
    let b_re = Regex::new(r"Button B: X\+(?<b_x>\d+), Y\+(?<b_y>\d+)").unwrap();
    let price_re = Regex::new(r"Prize: X=(?<price_x>\d+), Y=(?<price_y>\d+)").unwrap();

    let mut a_match:Option<(usize, usize)> = None;
    let mut b_match: Option<(usize, usize)> = None;
    let mut price_match: Option<(usize, usize)> = None;

    let mut total_cost: usize = 0;
    let mut total_cost_large: usize = 0;

    for line in lines {
        if let Some(a_cap) = a_re.captures(&line) {
            let a_x = a_cap.name("a_x").unwrap().as_str().parse::<usize>().unwrap();
            let a_y = a_cap.name("a_y").unwrap().as_str().parse::<usize>().unwrap();
            a_match = Some((a_x, a_y));
        }
        if let Some(b_cap) = b_re.captures(&line) {
            let b_x = b_cap.name("b_x").unwrap().as_str().parse::<usize>().unwrap();
            let b_y = b_cap.name("b_y").unwrap().as_str().parse::<usize>().unwrap();
            b_match = Some((b_x, b_y));
        }
        if let Some(price_cap) = price_re.captures(&line) {
            let price_x = price_cap.name("price_x").unwrap().as_str().parse::<usize>().unwrap();
            let price_y = price_cap.name("price_y").unwrap().as_str().parse::<usize>().unwrap();
            price_match = Some((price_x, price_y));
        }

        if a_match.is_some() && b_match.is_some() && price_match.is_some() {
            let clawy = ClawMachine {
                a_x: a_match.unwrap().0,
                a_y: a_match.unwrap().1,
                a_price: 3,
                b_x: b_match.unwrap().0,
                b_y: b_match.unwrap().1,
                b_price: 1,
                price_x: price_match.unwrap().0,
                price_y: price_match.unwrap().1,
            };

            let large_clawy = ClawMachine {
                a_x: a_match.unwrap().0,
                a_y: a_match.unwrap().1,
                a_price: 3,
                b_x: b_match.unwrap().0,
                b_y: b_match.unwrap().1,
                b_price: 1,
                price_x: price_match.unwrap().0 + 10000000000000,
                price_y: price_match.unwrap().1 + 10000000000000,
            };
            a_match = None;
            b_match = None;
            price_match = None;
            

            if let Some((res_a, res_b)) = clawy.solve_buttons() {
                total_cost += clawy.get_price(res_a, res_b);
            }

            if let Some((large_a, large_b)) = large_clawy.solve_buttons() {
                total_cost_large += large_clawy.get_price(large_a, large_b);
            }
        }
    }

    println!("[Part 1] Minimal cost for possible prices: {}", total_cost);
    println!("[Part 2] Minimal cost for possible prices: {}", total_cost_large);
}


struct ClawMachine {
    a_x: usize,
    a_y: usize,
    a_price: usize,
    b_x: usize,
    b_y: usize,
    b_price: usize,
    price_x: usize,
    price_y: usize
}


impl ClawMachine {
    fn solve_buttons(&self) -> Option<(usize, usize)> {
        if self.a_x * self.price_y > self.a_y * self.price_x {
            // solve for B
            let upper = self.a_x * self.price_y - self.a_y * self.price_x;
            let lower = self.a_x * self.b_y - self.a_y * self.b_x;
            if upper % lower == 0 {
                let b = upper / lower;
                if let Some(a) = self.a_from_b(b) {
                    return Some((a, b));
                }
            }
            return None;
        } else {
            // solve for A instead of B
            let upper = self.b_x * self.price_y - self.b_y * self.price_x;
            let lower = self.b_x * self.a_y - self.b_y * self.a_x;
            if upper % lower == 0 {
                let a = upper / lower;
                if let Some(b) = self.b_from_a(a) {
                    return Some((a, b));
                }
            }
            return None;
        }
        
    }

    fn a_from_b(&self, b: usize) -> Option<usize> {
        if self.price_x < self.b_x * b {
            return None
        }
        let a_part = self.price_x - self.b_x * b;
        if a_part % self.a_x != 0 {
            return None
        } else {
            return Some(a_part / self.a_x);
        }
    }

    fn b_from_a(&self, a: usize) -> Option<usize> {
        if self.price_x < self.a_x * a {
            return None
        }
        let b_part = self.price_x - self.a_x * a;
        if b_part % self.b_x != 0 {
            return None
        } else {
            return Some(b_part / self.b_x);
        }
    }

    fn get_price(&self, a: usize, b: usize) -> usize {
        self.a_price * a + self.b_price * b
    }
}


