use std::io::{BufReader, BufRead};
use std::fs::File;
use std::str::FromStr;

fn main() {
    let file = File::open("input.txt").expect("No such file!");
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|l| l.expect("Could not read line!"));

    let instructions: Vec<Instruction> = lines.next().unwrap()
        .chars().map(|c| if 'L' == c {Instruction::LEFT} else {Instruction::RIGHT}).collect();

    let states: Vec<State> = lines.map(|line| line.parse::<State>()).filter(|r| !r.is_err())
        .map(|r| r.unwrap()).collect();

    let start_states: Vec<State> = states.iter().filter(|st| st.is_start()).cloned().collect();
    println!("Found {} start states.", start_states.len());

    let mut machines: Vec<StateMachine> = start_states.iter().map(|ss| StateMachine{
            transition_count: 0,
            current: ss.clone(),
            states: states.clone(),
        })
        .collect();

    let infinite_instructions = instructions.iter().cycle();

    for instr in infinite_instructions {
        let mut stopped = true;
        for machine in machines.iter_mut() {
            if !machine.transition(instr) {
                stopped = false;
            }
        }
        if stopped {
            break;
        }
    }

    println!("Total transitions needed: {}", &machines[0].transition_count);
    
}

#[derive(Debug)]
struct StateMachine {
    transition_count: u64,
    current: State,
    states: Vec<State>,
}

impl StateMachine {
    fn get_state(&self, id: [char; 3]) ->Option<State> {
        return self.states.iter().filter(|st| st.identifier == id).last().copied();
    }

    fn transition(&mut self, instr: &Instruction) -> bool {
        if instr == &Instruction::LEFT {
            self.current = self.get_state(self.current.left).unwrap();
        } else {
            self.current = self.get_state(self.current.right).unwrap();
        }
        self.transition_count += 1;

        return self.current.is_halt();
    }
}

#[derive(Debug, PartialEq)]
enum Instruction {
    LEFT,
    RIGHT,
}

#[derive(Clone, Copy, Debug)]
struct State {
    identifier: [char; 3],
    right: [char; 3],
    left: [char; 3],
}

impl State {
    fn is_start(&self) -> bool {
        return self.identifier[2] == 'A';
    }

    fn is_halt(&self) -> bool {
        return self.identifier[2] == 'Z';
    }
}
#[derive(Debug, PartialEq, Eq)]
struct ParseStateError;
impl FromStr for State {
    type Err = ParseStateError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let identifier: [char; 3] = s.split("=").nth(0).unwrap().trim().chars()
                            .collect::<Vec<char>>().try_into().map_err(|_| ParseStateError)?;

        let (sleft, sright) = s.split("=").nth(1).unwrap().trim()
            .strip_prefix('(')
            .and_then(|s| s.strip_suffix(')'))
            .and_then(|s| s.split_once(','))
            .and_then(|(a, b)| Some((a.trim(), b.trim())))
            .ok_or(ParseStateError)?;

        let left: [char; 3] = sleft.chars().collect::<Vec<char>>().try_into().map_err(|_| ParseStateError)?;
        let right: [char; 3] = sright.chars().collect::<Vec<char>>().try_into().map_err(|_| ParseStateError)?;

        return Ok(State {identifier, left, right});
    }
}