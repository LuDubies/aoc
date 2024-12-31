use std::collections::HashSet;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::env;
use std::time::Instant;

use myrustlib::NeighbourIterator;

fn main() {

    let args: Vec<String> = env::args().collect();
    let mut filename = "input.txt";
    if args.len() > 1 && args[1] == "testdata"{
        filename = "testinput.txt";
    }
    let file = File::open(filename).expect("No such file!");
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.expect("Could not read line!"));

    let map: Vec<Vec<usize>> = lines.map(|line|line.chars().map(|c| c as usize - 48).collect()).collect();

    let constraints = (map[0].len(), map.len());
    let mut total_score: usize = 0;
    let mut total_rating: usize = 0;

    let starttime = Instant::now();

    for (rowidx, row) in map.iter().enumerate() {
        for (colidx, height) in row.iter().enumerate() {
            if 0 == *height {
                let (trailheads, trailrating) = get_trailheads((colidx, rowidx), constraints, &map, 0);
                total_score += trailheads.len();
                total_rating += trailrating;
            }
        }
    }

    println!("[Part 1] Total score: {}, Total rating: {}, [Solved in {:?}]", total_score, total_rating, starttime.elapsed());

}

fn get_trailheads(position: (usize, usize), constraints: (usize, usize), map: &Vec<Vec<usize>>,  height: usize) ->( HashSet<(usize, usize)>, usize) {
    if 9 == height {
        return (HashSet::from([position]), 1);
    }

    let mut trailheads: HashSet<(usize, usize)> = HashSet::new();
    let mut trailrating: usize = 0;
    let mut neighbours = NeighbourIterator::from_pos(position, constraints, false).filter(|np| map[np.1][np.0] == height + 1);
    
    while let Some(np) = neighbours.next() {
        let (subset, subrating) = get_trailheads(np, constraints, &map, height + 1);
        trailheads.extend(subset);
        trailrating += subrating;
    }

    return (trailheads, trailrating);
}

