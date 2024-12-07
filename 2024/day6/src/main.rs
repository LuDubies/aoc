use std::io::{BufReader, BufRead};
use std::fs::File;
use std::env;
use std::fmt;
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

    let mut laboratory: Laboratory = Laboratory {
        rows: Vec::new(),
        guardx: 9999,
        guardy: 9999,
        startx: 9999,
        starty: 9999,
        direction: Direction::UP
    };


    for (lidx, line) in lines.enumerate() {
        let mut row: Vec<Field> = Vec::new();
        for (fidx, char) in line.chars().enumerate() {
            if char == '.' {
                row.push(Field {visited: vec![false, false, false, false], blocked: false });
            }
            if char == '#' {
                row.push(Field {visited: vec![false, false, false, false], blocked: true });
            }
            if char == '^' {
                row.push(Field {visited: vec![true, false, false, false], blocked: false });
                laboratory.guardx = fidx;
                laboratory.guardy = lidx;
                laboratory.startx = fidx;
                laboratory.starty = lidx;
            }
        }
        laboratory.rows.push(row);
    }

    println!("{}", laboratory);

    let part1_starttime = Instant::now();
    loop {
        match laboratory.step() {
            Ok(_) => {  },
            Err(()) => break
        }
    }
    
    println!("[Part 1] Visited locations: {} [Solved in {:?}]", laboratory.count_visited(), part1_starttime.elapsed());

    // remove starting location
    laboratory.rows[laboratory.starty][laboratory.startx].visited = vec![false, false, false, false];
    // gather candidate locations for part 2
    let mut candidates: Vec<(usize, usize)> = Vec::new();
    for (lidx, row) in laboratory.rows.iter().enumerate() {
        for (fidx, field) in row.iter().enumerate() {
            if field.was_visited() {
                candidates.push((fidx, lidx));
            }
        }
    }

    let candidate_count = candidates.len();
    let mut possible_loop_inducing_locations_count: usize = 0;
    let part2_starttime = Instant::now();

    println!("");
    for (count, (cx, cy)) in candidates.iter().enumerate() {
        laboratory.reset();
        laboratory.rows[*cy][*cx].blocked = true;
        print!("\rTesting candidate {} of {}: ({}, {}). Found {}.", count + 1, candidate_count, cx, cy, possible_loop_inducing_locations_count);

        loop {
            match laboratory.step() {
                Ok(loop_detected) => {
                    if loop_detected {
                        possible_loop_inducing_locations_count += 1;
                        laboratory.rows[*cy][*cx].blocked = false;
                        break;
                    }
                },
                Err(()) => {
                    laboratory.rows[*cy][*cx].blocked = false;
                    break;
                }
            }
        }
    }

    println!("\n[Part 2] Possible locations: {} [Solved in {:?}]", possible_loop_inducing_locations_count, part2_starttime.elapsed());
}

struct Laboratory {
    rows: Vec<Vec<Field>>,
    guardx: usize,
    guardy: usize,
    startx: usize,
    starty: usize,
    direction: Direction
}

impl fmt::Display for Laboratory{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (lidx, row) in self.rows.iter().enumerate() {
            for (fidx, field) in row.iter().enumerate() {
                if self.guardx == fidx && self.guardy == lidx {
                    match self.direction {
                        Direction::UP => {write!(f, "^", )?},
                        Direction::RIGHT => {write!(f, ">", )?},
                        Direction::DOWN => {write!(f, "v", )?},
                        Direction::LEFT => {write!(f, "<", )?}
                    }
                }
                else {
                    write!(f, "{}", field)?
                }
            }
            write!(f, "\n")?
        }
        Ok(())
    }
}

impl Laboratory {
    fn step(&mut self) -> Result<bool, ()> {
        let mut loop_detected = false;
        loop {
            if !self.bounds_check() {
                return Err(());
            }

            if self.in_front().blocked {
                self.turn();
                continue;
            } else {
                break;
            }
        }

        match self.direction {
            Direction::UP => {
                if self.in_front().visited[0] == true {
                    loop_detected = true;
                }
                self.in_front().visited[0] = true;
                self.guardy = self.guardy - 1;
            },
            Direction::RIGHT => {
                if self.in_front().visited[1] == true {
                    loop_detected = true;
                }
                self.in_front().visited[1] = true;
                self.guardx = self.guardx + 1;
            },
            Direction::DOWN => {
                if self.in_front().visited[2] == true {
                    loop_detected = true;
                }
                self.in_front().visited[2] = true;
                self.guardy = self.guardy + 1;
            },
            Direction::LEFT => {
                if self.in_front().visited[3] == true {
                    loop_detected = true;
                }
                self.in_front().visited[3] = true;
                self.guardx = self.guardx - 1;
            }
        }

        Ok(loop_detected)
    }

    fn bounds_check(&self) -> bool {
        match self.direction {
            Direction::UP => self.guardy != 0,
            Direction::RIGHT => self.guardx < self.rows[0].len() - 1,
            Direction::DOWN => self.guardy < self.rows.len() -1,
            Direction::LEFT => self.guardx > 0
        }
    }

    fn in_front(&mut self) -> &mut Field {
        match self.direction {
            Direction::UP => &mut self.rows[self.guardy-1][self.guardx],
            Direction::RIGHT => &mut self.rows[self.guardy][self.guardx+1],
            Direction::DOWN => &mut self.rows[self.guardy+1][self.guardx],
            Direction::LEFT => &mut self.rows[self.guardy][self.guardx-1]
        }
    }

    fn turn(&mut self) -> () {
        self.direction = match self.direction {
            Direction::UP => Direction::RIGHT,
            Direction::RIGHT => Direction::DOWN,
            Direction::DOWN => Direction::LEFT,
            Direction::LEFT => Direction::UP
        }
    }

    fn count_visited(&self) -> usize {
        self.rows.iter().map(|r|
            r.iter().filter(|f|f.was_visited()).count()
        ).sum()
    }

    fn reset(&mut self) -> () {
        // reset all visited status
        for row in self.rows.iter_mut() {
            for field in row {
                field.visited[0] = false;
                field.visited[1] = false;
                field.visited[2] = false;
                field.visited[3] = false;
            }
        }
        // reset guard position
        self.guardx = self.startx;
        self.guardy = self.starty;
        self.direction = Direction::UP;

        self.rows[self.guardy][self.guardx].visited[0] = true;
    }
}


struct Field {
    visited: Vec<bool>,
    blocked: bool
}

impl Field {

}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.visited.iter().any(|v| *v){
            write!(f, "X")
        }
        else if self.blocked {
            write!(f, "#")
        }
        else {
            write!(f, ".")
        }
    }
}

impl Field {
    fn was_visited(&self) -> bool {
        self.visited.iter().any(|v| *v)
    }
}

enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT
}
