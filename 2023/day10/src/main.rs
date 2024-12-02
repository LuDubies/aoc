use std::io::{BufReader, BufRead};
use std::fs::File;

const MAX_X: usize = 160;
const MAX_Y: usize = 160;

fn main() {
    let file = File::open("input.txt").expect("No such file!");
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.expect("Could not read line!"));

    
    let mut surface: [[char; MAX_Y]; MAX_X]  = [['.'; MAX_Y]; MAX_X];

    let mut xoffs;
    let mut yoffs = 1;

    let mut currx = 0;
    let mut curry = 0;

    for line in lines {
        xoffs = 1;
        for c in line.chars() {
            if 'S' == c {
                currx = xoffs;
                curry = yoffs;
            }

            surface[xoffs][yoffs] = c;
            xoffs += 1;   
        }
        yoffs += 1;
    }

    let directions = [Direction::N, Direction::E, Direction::S, Direction::W];
    let mut steps = 0;
    let mut currdir = Direction::N;
    let (startx, starty) = (currx, curry);

    for dir in directions {
        match next_direction(tile_in_dir(&surface, (currx, curry), &dir), &target_to_origin(&dir)) {
            None => continue,
            Some(d) => {
                (currx, curry) = step((currx, curry), &dir);
                steps += 1;
                currdir = d;
                break;
            },
        }
    }

    while startx != currx || starty != curry {
        let next_dir = next_direction(tile_in_dir(&surface, (currx, curry), &currdir), &target_to_origin(&currdir)).unwrap();
        (currx, curry) = step((currx, curry), &currdir);
        currdir = next_dir;
        steps += 1;
    }

    let result = steps / 2;
    println!("Total loop steps {steps}, so furthest should be {result}.");

}

#[derive(Debug, Clone)]
enum Direction {
    N = 0,
    E = 1,
    S = 2,
    W = 3,
}

fn tile_in_dir(surface: &[[char; MAX_Y]; MAX_X], pt: (usize, usize), dir: &Direction) -> char {
    let (targetx, targety) = step(pt, dir);
    return surface[targetx][targety];
}

fn step(pt: (usize, usize), dir: &Direction) -> (usize, usize) {
    match dir {
        Direction::N => return (pt.0, pt.1 - 1),
        Direction::E => return (pt.0 + 1, pt.1),
        Direction::S => return (pt.0, pt.1 + 1),
        Direction::W => return (pt.0 -1, pt.1),
    }
}

fn next_direction(tile: char, arrived_from: &Direction) -> Option<Direction> {
    match tile {
        'S' => return Some(arrived_from.clone()),
        '.' => return None,
        '|' => match arrived_from {
            Direction::N => Some(Direction::S),
            Direction::E => None,
            Direction::S => Some(Direction::N),
            Direction::W => None,
        },
        '-' => match arrived_from {
            Direction::N => None,
            Direction::E => Some(Direction::W),
            Direction::S => None,
            Direction::W => Some(Direction::E),
        },
        'L' => match arrived_from {
            Direction::N => Some(Direction::E),
            Direction::E => Some(Direction::N),
            Direction::S => None,
            Direction::W => None,
        },
        'J' => match arrived_from {
            Direction::N => Some(Direction::W),
            Direction::E => None,
            Direction::S => None,
            Direction::W => Some(Direction::N),
        },
        '7' => match arrived_from {
            Direction::N => None,
            Direction::E => None,
            Direction::S => Some(Direction::W),
            Direction::W => Some(Direction::S),
        },
        'F' => match arrived_from {
            Direction::N => None,
            Direction::E => Some(Direction::S),
            Direction::S => Some(Direction::E),
            Direction::W => None,
        },
        _ => return None,       
    }
}

fn target_to_origin(tdir: &Direction) -> Direction {
    match tdir {
        Direction::N => return Direction::S,
        Direction::E => return Direction::W,
        Direction::S => return Direction::N,
        Direction::W => return Direction::E,
    }
}