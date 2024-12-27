use std::io::{BufReader, BufRead};
use std::fs::File;
use std::env;
use std::fmt;
use std::ops::Index;
use std::thread::current;
use std::time::Instant;
use std::collections::VecDeque;

fn main() {

    let args: Vec<String> = env::args().collect();
    let mut filename = "input.txt";
    if args.len() > 1 && args[1] == "testdata"{
        filename = "testinput.txt";
    }
    let file = File::open(filename).expect("No such file!");
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.expect("Could not read line!"));

    let mut nodes: Vec<Node> = Vec::new();

    for (row, line) in lines.enumerate() {
        for (col, tile) in line.chars().into_iter().enumerate() {
            if '.' == tile {
                nodes.push(Node { position: (col, row), score: Score { north: None, east: None, south: None, west: None }, start: false, end: false });
            } else if 'S' == tile {
                nodes.push(Node { position: (col, row), score: Score { north: None, east: None, south: None, west: None }, start: true, end: false });
            } else if 'E' == tile {
                nodes.push(Node { position: (col, row), score: Score { north: None, east: None, south: None, west: None }, start: false, end: true });
            }
        }
    }

    let nodes = directional_dijkstra(nodes);

    println!("{:?}", nodes.iter().filter(|n| n.end).next().unwrap().score);
}

fn directional_dijkstra(mut nodes: Vec<Node>) -> Vec<Node> {

    let mut node_queue: VecDeque<(usize, Direction)> = VecDeque::new();
    let (start_index, start_node) = nodes.iter_mut().enumerate().filter(|n| n.1.start).next().unwrap();
    start_node.score.west = Some(0);
    node_queue.push_back((start_index, Direction::West));  // starting with east orientation, so start was visited from West

    while let Some((node_index, from_direction)) = node_queue.pop_front() {
        let current_score = nodes[node_index].score.score_from(&from_direction).unwrap();

        let current_position = nodes[node_index].position;

        if from_direction != Direction::North {  // check north   
            if let Some(check_id) = check_node(current_position, Direction::North, !from_direction.is_opposing(&Direction::North), current_score, &mut nodes) {
                node_queue.push_back((check_id, Direction::South));
            }
        }
        if from_direction != Direction::East {  // check East
            if let Some(check_id) = check_node(current_position, Direction::East, !from_direction.is_opposing(&Direction::East), current_score, &mut nodes) {
                node_queue.push_back((check_id, Direction::West));
            }
        }
        if from_direction != Direction::South {  // check South
            if let Some(check_id) = check_node(current_position, Direction::South, !from_direction.is_opposing(&Direction::South), current_score, &mut nodes) {
                node_queue.push_back((check_id, Direction::North));
            }
        }
        if from_direction != Direction::West {  // check West
            if let Some(check_id) = check_node(current_position, Direction::West, !from_direction.is_opposing(&Direction::West), current_score, &mut nodes) {
                node_queue.push_back((check_id, Direction::East));
            }
        }

    }
    nodes
}

fn check_node(
    current_position: (usize, usize),
    check_direction: Direction,
    turn: bool,
    score: usize,
    nodes: &mut Vec<Node>
) -> Option<usize> {
    let check_position = match check_direction {
        Direction::North => (current_position.0, current_position.1 - 1),
        Direction::East => (current_position.0 + 1, current_position.1),
        Direction::South => (current_position.0, current_position.1 + 1),
        Direction::West => (current_position.0 - 1, current_position.1)
    };

    if let Some((neighbour_idx, neighbour_node)) = nodes.iter_mut().enumerate()
    .filter(|(_, n)| n.position == check_position).next() {
        let possible_score: usize;
        if turn {
            possible_score = 1001 + score;
        } else {
            possible_score = 1 + score;
        }

        let previous_score = match check_direction {
            Direction::North => neighbour_node.score.south,
            Direction::East => neighbour_node.score.west,
            Direction::South => neighbour_node.score.north,
            Direction::West => neighbour_node.score.east,

        };

        if (previous_score.is_some() && previous_score.unwrap() > possible_score)
        || previous_score.is_none() {
            match check_direction {
                Direction::North => neighbour_node.score.south = Some(possible_score),
                Direction::East => neighbour_node.score.west = Some(possible_score),
                Direction::South => neighbour_node.score.north = Some(possible_score),
                Direction::West => neighbour_node.score.east = Some(possible_score)
            }
            return Some(neighbour_idx);
        }
    }
    None
}

#[derive(Debug)]
struct Node {
    position: (usize, usize),
    score: Score,
    start: bool,
    end: bool
}

#[derive(Debug)]
struct Score {
    north: Option<usize>,
    east: Option<usize>,
    south: Option<usize>,
    west: Option<usize>
}

impl Score {
    fn score_from(&self, direction: &Direction) -> Option<usize> {
        match direction {
            Direction::North => self.north,
            Direction::East => self.east,
            Direction::South => self.south,
            Direction::West => self.west
        }
    }
}

#[derive(PartialEq)]
enum Direction {
    North,
    East,
    South,
    West
}

impl Direction {
    fn is_opposing(&self, other: &Direction) -> bool {
        match self {
            &Direction::North => other == &Direction::South,
            &Direction::East => other == &Direction::West,
            &Direction::South => other == &Direction::North,
            &Direction::West => other == &Direction::East,
        }
    }

    fn get_opposing(&self) -> Direction {
        match self {
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::North => Direction::South,
        }
    }
}


