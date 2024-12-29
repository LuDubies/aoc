use std::io::{BufReader, BufRead};
use std::fs::File;
use std::env;
use myrustlib::PairIterator;

fn main() {

    let args: Vec<String> = env::args().collect();
    let mut filename = "input.txt";
    if args.len() > 1 && args[1] == "testdata"{
        filename = "testinput.txt";
    }
    let file = File::open(filename).expect("No such file!");
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.expect("Could not read line!"));

    let mut antennae: Vec<Antenna> = Vec::new();
    let mut frequencies: Vec<char> = Vec::new();
    let mut antinodes: Vec<(usize, usize)> = Vec::new();
    let mut rows: usize = 0;
    let mut cols: usize = 0;

    for (row, line) in lines.enumerate() {
        for (col, c) in line.chars().enumerate() {
            if '.' != c {
                if !frequencies.contains(&c) {
                    frequencies.push(c);
                }
                antennae.push(Antenna{position: (col, row), frequency: c});
            }
            if col + 1 >= cols {
                cols = col + 1;
            }
        }
        if row + 1 >= rows {
            rows = row + 1;
        }
    }

    for freq in frequencies.iter() {
        let mut pi: PairIterator<Antenna> = PairIterator::from_vec(antennae.iter().filter(|a| a.frequency == *freq).collect());

        while let Some(pair) = pi.next() {
            let diffvec = pos_diff(pair.0.position, pair.1.position);
            if let Some(antinode_pos) = pos_move(pair.0.position, diffvec, (cols, rows)) {
                if !antinodes.contains(&antinode_pos) {
                    antinodes.push(antinode_pos);
                }
            }

            let diffvec = pos_diff(pair.1.position, pair.0.position);
            if let Some(antinode_pos) = pos_move(pair.1.position, diffvec, (cols, rows)) {
                if !antinodes.contains(&antinode_pos) {
                    antinodes.push(antinode_pos);
                }
            }

        }
    }

    println!("[Part 1] Distinct antinodes: {}.", antinodes.len());

    antinodes = Vec::new();

    for freq in frequencies.iter() {
        let mut pi: PairIterator<Antenna> = PairIterator::from_vec(antennae.iter().filter(|a| a.frequency == *freq).collect());

        while let Some(pair) = pi.next() {
            let diffvec = pos_diff(pair.0.position, pair.1.position);
            let mut difffac: i64 = 0;
            while let Some(antinode_pos) = pos_move(pair.0.position, vec_multiply( diffvec, difffac), (cols, rows)) {
                if !antinodes.contains(&antinode_pos) {
                    antinodes.push(antinode_pos);
                }
                difffac += 1;
            }

            difffac = 0;
            let diffvec = pos_diff(pair.1.position, pair.0.position);
            while let Some(antinode_pos) = pos_move(pair.1.position, vec_multiply( diffvec, difffac), (cols, rows)) {
                if !antinodes.contains(&antinode_pos) {
                    antinodes.push(antinode_pos);
                }
                difffac += 1;
            }

        }
    }

    println!("[Part 2] Distinct antinodes: {}.", antinodes.len());    
}

#[derive(Debug)]
struct Antenna {
    position: (usize, usize),
    frequency: char
}

fn pos_move(position: (usize, usize), vector: (i64, i64), constraints: (usize, usize)) -> Option<(usize, usize)> {
    let newpos = (position.0 as i64 + vector.0, position.1 as i64 + vector.1);

    if newpos.0 < 0 || newpos.0 as usize >= constraints.0 || newpos.1 < 0 || newpos.1 as usize >= constraints.1 {
        None
    } else {
        Some((newpos.0 as usize, newpos.1 as usize))
    }

}

fn vec_multiply(vector: (i64, i64), factor: i64) -> (i64, i64) {
    (vector.0 * factor, vector.1 * factor)
}


fn pos_diff(v1: (usize, usize), v2: (usize, usize)) -> (i64, i64) {
    return (v1.0 as i64 - v2.0 as i64, v1.1 as i64 - v2.1 as i64)
} 

