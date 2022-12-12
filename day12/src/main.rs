use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::collections::VecDeque;


const INFINITE_COST: usize = 99999;
#[derive(Debug)]
struct Node {
    neighbours: Vec<usize>,
    cost: usize,
    height: char,
}

impl Node {
    fn new(height: char) -> Self{
        if height == 'S' {
            Self { height: 'a', neighbours: Vec::new(), cost: 0}
        } else if height == 'E' {
            Self {height: 'z', neighbours: Vec::new(), cost: INFINITE_COST}
        } else {
            Self {height, neighbours: Vec::new(), cost: INFINITE_COST} 
        }
    }

    fn add_neighbour(&mut self, nn: usize) -> () {
        self.neighbours.push(nn);
    }
}

fn can_climb(from: usize, to: usize, nds: &Vec<Node>) -> bool {
    nds[from].height as usize + 1 >= nds[to].height as usize
}

fn main() -> Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|l| l.expect("Could not read line")).collect();
    
    let mut node_counter = 0;
    let mut nodes: Vec<Node> = Vec::new();
    let mut start: usize = 0;
    let mut end: usize = 0;

    let grid_height: usize = lines.len();
    let grid_width: usize = lines[0].len();

    // parse input into node struct
    for line in lines {
        for c in line.chars() {
            if c == 'S' { start = node_counter; }
            if c == 'E' { end = node_counter; }
            nodes.push(Node::new(c));
            node_counter = node_counter + 1;
        }
    }

    println!("Parsed {node_counter} nodes.");


    for id in 0..node_counter {
        if id % grid_width != 0 && can_climb(id, id-1, &nodes) { 
            nodes[id].add_neighbour(id - 1);
         }
        if id % grid_width != grid_width - 1 && can_climb(id, id+1, &nodes) { 
            nodes[id].add_neighbour(id + 1);
         }
        if id + grid_width < node_counter  && can_climb(id, id + grid_width, &nodes) {
             nodes[id].add_neighbour(id + grid_width);
        }
        if id > grid_width && can_climb(id, id - grid_width, &nodes){ 
            nodes[id].add_neighbour(id - grid_width); 
        }
    }


    // dijkstra on the fly
    let mut queue: VecDeque<usize> = VecDeque::new();
    queue.push_back(start);

    loop {
        match queue.pop_front() {
            None => break,
            Some(n) => {
                let current_neighbours: Vec<usize> = Vec::clone(&nodes[n].neighbours);
                for ni in current_neighbours {
                    if nodes[ni].cost == INFINITE_COST {
                        queue.push_back(ni);
                    }
                    if nodes[ni].cost > nodes[n].cost + 1 {
                        nodes[ni].cost = nodes[n].cost + 1;
                    }
                }
            },
        }
    }

    println!("{}", nodes[end].cost);


    
    Ok(())
}
