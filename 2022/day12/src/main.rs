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
            Self { height: 'a', neighbours: Vec::new(), cost: INFINITE_COST}
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

fn reset_costs(start: usize, nodes: &mut Vec<Node>) -> () {
    for i in 0..nodes.len() {
        if i == start {
            nodes[i].cost = 0;
        } else {
            nodes[i].cost = INFINITE_COST;
        }
    }
}

fn dijkstra(start: usize, end: usize, nodes: &mut Vec<Node>) -> usize {
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
    nodes[end].cost

}

fn main() -> Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|l| l.expect("Could not read line")).collect();
    
    let mut node_counter = 0;
    let mut nodes: Vec<Node> = Vec::new();
    let mut starts: Vec<usize> = Vec::new();
    let mut end: usize = 0;

    let grid_height: usize = lines.len();
    let grid_width: usize = lines[0].len();

    // parse input into node struct
    for line in lines {
        for c in line.chars() {
            if c == 'S' || c == 'a' { starts.push(node_counter); }
            if c == 'E' { end = node_counter; }
            nodes.push(Node::new(c));
            node_counter = node_counter + 1;
        }
    }

    println!("Parsed {node_counter} nodes.");

    // set neighbours
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

    let mut min_path_len = INFINITE_COST;
    for i in 0..starts.len() {
        reset_costs(starts[i], &mut nodes);
        let cpl = dijkstra(starts[i], end, &mut nodes);
        if cpl < min_path_len { min_path_len = cpl; }
    }
    
    println!("Minimum Path is {min_path_len}.");


    
    Ok(())
}
