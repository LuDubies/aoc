use std::io::{BufReader, BufRead};
use std::fs::File;
use std::env;
use regex::Regex;

fn main() {

    let args: Vec<String> = env::args().collect();
    let mut filename = "input.txt";

    let cols: i64;
    let rows: i64;
    if args.len() > 1 && args[1] == "testdata"{
        filename = "testinput.txt";
        cols = 11;
        rows = 7;
    } else {
        cols = 101;
        rows = 103;
    }

    let file = File::open(filename).expect("No such file!");
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.expect("Could not read line!"));

    let robot_re = Regex::new(r"p=(?<r_x>\d+),(?<r_y>\d+) v=(?<s_x>-?\d+),(?<s_y>-?\d+)").unwrap();

    let mut quadrants: (usize, usize, usize, usize) = (0, 0, 0, 0);
    let mut area = BathroomPatrolArea {
        cols: cols, 
        rows: rows,
        robots: Vec::new()
    };

    for line in lines {
        if let Some(robo_cap) = robot_re.captures(&line) {
            let robby = Robot {
                x_start: robo_cap.name("r_x").unwrap().as_str().parse::<i64>().unwrap(),
                y_start: robo_cap.name("r_y").unwrap().as_str().parse::<i64>().unwrap(),
                x_step: robo_cap.name("s_x").unwrap().as_str().parse::<i64>().unwrap(),
                y_step: robo_cap.name("s_y").unwrap().as_str().parse::<i64>().unwrap()
            };

            let robo_pos = robby.after_steps(100, (cols, rows));

            area.robots.push(robby);

            if robo_pos.0 < (cols / 2) {
                // left side
                if robo_pos.1 < (rows / 2) {
                    // first quadrant
                    quadrants.0 += 1;
                }
                if robo_pos.1 > (rows / 2) {
                    // third quadrant
                    quadrants.2 += 1;
                }
            }
            if robo_pos.0 > (cols / 2) {
                // right side
                if robo_pos.1 < (rows / 2) {
                    // second quadrant
                    quadrants.1 += 1;
                }
                if robo_pos.1 > (rows / 2) {
                    // fourth quadrant
                    quadrants.3 += 1;
                }
            }
        }
    }
    println!("Quadrants: {}, {}, {}, {}", quadrants.0, quadrants.1, quadrants.2, quadrants.3);
    println!("[Part 1] Safety factor is {}.", quadrants.0 * quadrants.1 * quadrants.2 * quadrants.3);

    let mut step: usize = 200;
    loop {
        area.print_step(step as i64);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        if input == "quit()" {
            break;
        }
        step += 101; // from observing pattern occurences
    }


}

struct Robot {
    x_start: i64,
    y_start: i64,
    x_step: i64,
    y_step: i64
}

impl Robot {

    fn after_steps(&self, steps: i64, field_dims: (i64, i64)) -> (i64, i64) {

        let x_change: i64 = (self.x_step * steps) % field_dims.0 as i64;
        let y_change: i64 = (self.y_step * steps) % field_dims.1 as i64;

        let x_end: i64 = (field_dims.0 + ((self.x_start + x_change) % field_dims.0)) % field_dims.0;
        let y_end: i64 = (field_dims.1 + ((self.y_start + y_change) % field_dims.1)) % field_dims.1;

        (x_end, y_end)
    }
}

struct BathroomPatrolArea {
    cols: i64,
    rows: i64,
    robots: Vec<Robot>
}

impl BathroomPatrolArea {
    fn print_step(&self, step: i64) -> () {
        println!("");
        println!("Step: {}", step);
        println!("");

        let robo_positions: Vec<(i64, i64)> = self.robots.iter().map(|r| r.after_steps(step, (self.cols, self.rows))).collect();

        for y in 0..self.rows {
            for x in 0..self.cols {
                if robo_positions.contains(&(x,y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("");
        }

    }
}
