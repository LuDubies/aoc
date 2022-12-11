use std::io::{Result, BufRead, BufReader};
use std::fs::File;

const WORRY_DECREASE: bool = false;

#[derive(Debug)]
struct Monkey<'a>{
    id: usize,
    items: Vec<usize>,
    operation: (&'a str, &'a str),
    test: usize,
    targets: (usize, usize),
    instpection_count: usize,
}

impl <'a> Monkey<'a> {
    fn new(id: usize, items: Vec<usize>, operation: (&'a str, &'a str),
     test: usize, targets: (usize, usize)) -> Self {
        Self {id, items, operation, test, targets, instpection_count: 0}
    }

    fn process_item(&mut self, reducer: Option<usize>) -> Option<(usize, usize)> {
        if self.items.is_empty() {
            return None;
        }
        let mut worry = self.items.remove(0);
        match self.operation.0 {
            "+" => {
                match self.operation.1.parse::<usize>() {
                    Ok(n) => worry = worry + n,
                    Err(_) => worry = worry + worry,
                }
            },
            "*" => {
                match self.operation.1.parse::<usize>() {
                    Ok(n) => worry = worry * n,
                    Err(_) => worry = worry * worry,
                }
            },
            _ => panic!("Invalid operation for monkey {:?}", self.id),
        }
        self.instpection_count = self.instpection_count + 1;  // inc inspection count
        if WORRY_DECREASE{
            worry = worry / 3;  // loose interest
        } else {
            match reducer {
                None => panic!("Need reducer if there is no worry decrease!"),
                Some(r) => worry = worry % r,
            }
        }

        if worry % self.test == 0 {
            println!("Monkey {} threw {} to target {}", self.id, worry, self.targets.0);
            Some((worry, self.targets.0))
        } else {
            println!("Monkey {} threw {} to target {}", self.id, worry, self.targets.1);
            Some((worry, self.targets.1))
        }
    }

    fn catch_item(&mut self, item: usize) -> ()  {
        self.items.push(item);
        println!("Monkey {} caught {item}", self.id);
    }
}


fn main() -> Result<()>{
    let file = File::open("input.txt")?;
    let br = BufReader::new(file);

    let lines: Vec<String> = br.lines()
                    .map(|l| l.expect("Could not read line!"))
                    .collect();
    
    let mut monkeys = parse_monkeys(&lines);

    // testers are all primes, so LCM is just all multiplied
    let mut lcm = 1;
    for m in &monkeys{
        lcm = lcm * m.test;
    }

    println!("{}", lcm);
    
    if WORRY_DECREASE{
        for _ in 0..20 {
            round(&mut monkeys, None);
        }
    } else {
        for _ in 0..10000 {
            round(&mut monkeys, Some(lcm));
        }
    }
    

    let mut inspection_counts: Vec<usize> = monkeys.iter()
                                            .map(|m| m.instpection_count)
                                            .collect();
    inspection_counts.sort();

    let monkey_buisness: usize = inspection_counts[inspection_counts.len()-1] * inspection_counts[inspection_counts.len()-2];
    println!("{:?}", inspection_counts);
    println!("{:?}", monkey_buisness);

    Ok(())
}

fn round(monkeys: &mut Vec<Monkey>, reducer: Option<usize>) -> () {
    for i in 0..monkeys.len(){
        loop {
            match monkeys[i].process_item(reducer) {
                None => break,
                Some(thrown_item) => {
                    let(item, target) = thrown_item;
                    monkeys.iter_mut()
                        .filter(|mon| mon.id == target)
                        .next()
                        .expect("Could not find catching monkey!")
                        .catch_item(item);
                }
            }
        }
        
    }
}

fn parse_monkeys(input: &Vec<String>) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();
    
    for (i, line) in input.iter().enumerate() {
        if line.starts_with("Monkey"){
            let id = parse_id(&input[i]);
            let si = parse_starting_items(&input[i+1]);
            let op = parse_operation(&input[i+2]);
            let tst = parse_test(&input[i+3]);
            let trgt = (parse_target(&input[i+4]), parse_target(&input[i+5]));
            monkeys.push(Monkey::new(id, si, op, tst, trgt));
        }
    }

    monkeys
}

fn parse_id(line: &str) -> usize {
    match line.split(" ").nth(1) {
        None => panic!("Failed parsing line {}!", line),
        Some(rest) => return rest[..rest.len() -1].parse().unwrap(),
    }
}

fn parse_starting_items(line: &str) -> Vec<usize> {
    let mut starting_items: Vec<usize> = Vec::new();
    match line.split(":").nth(1) {
        None => panic!("Failed parsing line {}!", line),
        Some(rest) => {
            for value in rest.split(",") {
                starting_items.push(value.trim().parse().unwrap());
            }
        },
    };
    starting_items
}

fn parse_operation(line: &str) -> (&str, &str) {
    match line.split(":").nth(1) {
        None => panic!("Failed parsing line {}!", line),
        Some(operation) => {
            let parts: Vec<&str> = operation.trim().split(" ").collect();
            return (parts[3], parts[4])
        },
    }
}

fn parse_test(line: &str) -> usize {
    match line.split("by").nth(1) {
        None => panic!("Failed parsing line {}!", line),
        Some(rest) => rest.trim().parse().unwrap()
    }
}

fn parse_target(line: &str) -> usize {
    match line.split("monkey").nth(1) {
        None => panic!("Failed parsing line {}!", line),
        Some(rest) => rest.trim().parse().unwrap()
    }
}
