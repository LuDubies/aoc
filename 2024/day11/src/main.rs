use std::io::{BufReader, BufRead};
use std::fs::File;
use std::env;
use std::time::Instant;
use std::collections::HashMap;

fn main() {

    let args: Vec<String> = env::args().collect();
    let mut filename = "input.txt";
    if args.len() > 1 && args[1] == "testdata"{
        filename = "testinput.txt";
    }
    let file = File::open(filename).expect("No such file!");
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|l| l.expect("Could not read line!"));

    // stones can be handled independently

    // create a look up table to speed up previously calculated stone-step combinations
    let mut lut: HashMap<(u64, usize), u64> = HashMap::new();

    let initial_stones: Vec<u64> = lines.next().unwrap().split_whitespace().map(|num_str| num_str.parse::<u64>().unwrap()).collect();

    let mut total_stones: u64 = 0;

    let part1_starttime = Instant::now();
    for initial in &initial_stones {
        let resulting_stones = stones_in_steps(*initial, 25, &mut lut);
        total_stones += resulting_stones;
        println!("Initial stone {} resulted in {} stones after 25 blinks.", initial, resulting_stones);
    }

    println!("[Part 1] Total stones after 25 steps: {} [Solved in {:?}]", &total_stones, part1_starttime.elapsed());
    println!("Created {} LUT entries.", &lut.keys().count());

    total_stones = 0;
    let part2_starttime = Instant::now();
    for initial in initial_stones {
        let resulting_stones = stones_in_steps(initial, 75, &mut lut);
        total_stones += resulting_stones;
        println!("Initial stone {} resulted in {} stones after 75 blinks.", initial, resulting_stones);
    }

    println!("[Part 2] Total stones after 75 steps: {} [Solved in {:?}]", &total_stones, part2_starttime.elapsed());
    println!("Created {} LUT entries.", &lut.keys().count());

}


fn stones_in_steps(stone_value: u64, remaining_steps: usize, lut: &mut HashMap<(u64, usize), u64>) -> u64 {
    if remaining_steps == 0 {
        return 1;
    }
    else if remaining_steps == 1 {
        // disregard the next value(s), just check if stone splits
        if digit_count(stone_value) % 2 == 0 {
            return 2;
        } else {
            return 1;
        }
    } else {
        // check if the result has already been calculated.
        if lut.contains_key(&(stone_value, remaining_steps)) {
            return *lut.get(&(stone_value, remaining_steps)).unwrap();
        }
        else if stone_value == 0 { // first rule
            let result = stones_in_steps(1, remaining_steps - 1, lut);
            lut.insert((stone_value, remaining_steps), result);
            return result;
        }
        else if digit_count(stone_value) % 2 == 0 { // second rule.
            // split and track results of both new stones.
            let divisor = 10u64.pow(digit_count(stone_value) / 2);  // 10 for 2 digits, 100 for 4 digits, etc.

            let first_res = stones_in_steps(stone_value / divisor, remaining_steps - 1, lut);
            let second_res = stones_in_steps(stone_value % divisor, remaining_steps - 1, lut);

            let result = first_res + second_res;
            lut.insert((stone_value, remaining_steps), result);
            return result;
        } else {  // third rule
            let result = stones_in_steps(stone_value * 2024, remaining_steps - 1, lut);
            lut.insert((stone_value, remaining_steps), result);
            return result;
        }
    }
}


fn digit_count(num: u64) -> u32 {
    num.checked_ilog10().unwrap_or(0) + 1
}
