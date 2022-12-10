use std::io::{BufReader, BufRead, Result};
use std::fs::File;

#[derive(Debug)]
struct Command {
    name: String,
    param: Option<i32>,
}

fn main() -> Result<()>{
    let file = File::open("input.txt").expect("No such file!");
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines()
                    .map(|l| l.expect("Could not read line!"))
                    .collect();
    
    let mut commands: Vec<Command> = Vec::new();
    
    for line in lines {
        let split: Vec<&str> = line.split(" ").collect();
        if split.len() == 1{
            commands.push(Command{name: String::from(split[0]), param: None});
        }
        else {
            commands.push(Command{name: String::from(split[0]), param: Some(split[1].parse().unwrap())});
        }
    }

    let mut states: Vec<i32> = Vec::new();
    for c in (20..=220).step_by(40) {
        states.push(c * get_reg_after_cycle(c-1, &commands));
    }

    println!("Sum of cycle states is {}.", states.iter().sum::<i32>());

    // fill string
    let mut to_display = String::new();
    for c in 0..=239 {
        let current_reg_state = get_reg_after_cycle(c, &commands);
        if (current_reg_state - (c % 40)).abs() <= 1 {
            to_display.push_str("#");
        } else {
            to_display.push_str(".");
        }
    }
    assert!(to_display.len() == 240);

    // display string in 40char slices
    for i in (0..=200).step_by(40) {
        println!("{}", &to_display[i..i+40]);
    }
    Ok(())
}

fn get_reg_after_cycle(cycle: i32, commands: &Vec<Command>) -> i32{
    let mut xreg: i32 = 1;
    let mut cycles = 0;


    for command in commands{
        // increase cycle counter
        if command.name == "noop"{
            cycles = cycles + 1;
        } else if command.name == "addx"{
            cycles = cycles + 2;
        }
        // if weve gone to far, return previous register state
        if cycles > cycle{
            return xreg;
        }
        match command.param{
            None => (),
            Some(arg) => xreg = xreg + arg,
        }
        // if weve reached the wanted cycle, return current register content
        if cycles == cycle{
            return xreg;
        }
    }
    xreg
}