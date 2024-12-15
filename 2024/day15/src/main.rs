use std::cmp::Ordering;
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

    let mut warehouse = Warehouse {
        content: Vec::new(),
        robot: (0,0)
    };

    let mut large_warehouse = LargeWarehouse {
        content: Vec::new(),
        robot: (0,0)
    };

    let mut movelist: Vec<Move> = Vec::new();
    let mut map_parsed: bool = false;

    for (row, line) in lines.enumerate() {

        if 0 == line.chars().as_str().len() {
            map_parsed = true;
            continue;
        }

        if !map_parsed {
            for (col, inv) in line.chars().enumerate() {
                if '#' == inv {
                    warehouse.content.push(Inventory {inv_type: InventoryType::WALL, position: (col, row)});
                    large_warehouse.content.push(Inventory {inv_type: InventoryType::WALL, position: (2*col, row)});
                    large_warehouse.content.push(Inventory {inv_type: InventoryType::WALL, position: (2*col+1, row)});
                } else if 'O' == inv {
                    warehouse.content.push(Inventory {inv_type: InventoryType::BARREL, position: (col, row)});
                    large_warehouse.content.push(Inventory {inv_type: InventoryType::BARREL, position: (col*2, row)});
                } else if '@' == inv {
                    warehouse.robot = (col, row);
                    large_warehouse.robot = (col*2, row);
                }
            }
        } else {
            for move_char in line.chars() {
                match move_char {
                    '^' => movelist.push(Move::UP),
                    '>' => movelist.push(Move::RIGHT),
                    'v' => movelist.push(Move::DOWN),
                    '<' => movelist.push(Move::LEFT),
                    _ => ()
                }
            }
        }
    }

    let part1_starttime = Instant::now();

    for m in &movelist {
        warehouse.move_robot(m);
    }
    println!("[Part 1] GPS sum after all moves: {} [Solved in {:?}]", warehouse.gps_sum(), part1_starttime.elapsed());


    let part2_starttime = Instant::now();

    for m in &movelist {
        large_warehouse.move_robot(m);
    }
    
    println!("[Part 2] GPS sum after all moves: {} [Solved in {:?}]", large_warehouse.gps_sum(), part2_starttime.elapsed());
    println!("{}", &large_warehouse);
}

struct Warehouse {
    content: Vec<Inventory>,
    robot: (usize, usize)
}

struct LargeWarehouse {
    content: Vec<Inventory>,
    robot: (usize, usize)
}

#[derive(PartialEq, Debug)]
enum InventoryType {
    WALL,
    BARREL,    
}

#[derive(Debug)]
struct Inventory {
    inv_type: InventoryType,
    position: (usize, usize)
}

#[derive(Debug)]
enum Move {
    UP,
    RIGHT,
    DOWN,
    LEFT
}

impl fmt::Display for Warehouse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (cols, rows) = self.get_dimensions();

        for row in 0..rows {
            for col in 0..cols {
                let mut at_this_pos =self.content.iter().filter(|c| c.position.0 == col && c.position.1 == row);
                if let Some(c) = at_this_pos.next() {
                    match c.inv_type {
                        InventoryType::WALL => write!(f, "#")?,
                        InventoryType::BARREL => write!(f, "O")?
                    }
                } else {
                    if (col, row) == self.robot {
                        write!(f, "@")?;
                    } else {
                        write!(f, ".")?;
                    }
                }
            }
            writeln!(f, "")?;
        }

        Ok(())
    }
}

impl fmt::Display for LargeWarehouse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (cols, rows) = self.get_dimensions();
        let mut after_barrel = false;
        for row in 0..rows {
            for col in 0..cols {
                if after_barrel {
                    write!(f, "]")?;
                    after_barrel = false;
                    continue;
                }
                let mut at_this_pos =self.content.iter().filter(|c| c.position.0 == col && c.position.1 == row);
                if let Some(c) = at_this_pos.next() {
                    match c.inv_type {
                        InventoryType::WALL => write!(f, "#")?,
                        InventoryType::BARREL => {
                            write!(f, "[")?;
                            after_barrel = true;
                        }
                    }
                } else {
                    if (col, row) == self.robot {
                        write!(f, "@")?;
                    } else {
                        write!(f, ".")?;
                    }
                }
            }
            writeln!(f, "")?;
        }

        Ok(())
    }
}

impl LargeWarehouse {

    fn get_dimensions(&self) -> (usize, usize) {
        let mut cols: usize = 0;
        let mut rows: usize = 0;

        for cont in &self.content {
            if cont.position.0 > cols {
                cols = cont.position.0;
            }
            if cont.position.1 > rows {
                rows = cont.position.1
            }
        }
        (cols + 1, rows + 1)
    }

    fn gps_sum(&self) -> usize {
        self.content.iter().filter(|c| c.inv_type == InventoryType::BARREL).map(|b| b.position.1 * 100 + b.position.0).sum()
    }

    fn move_robot(&mut self, direction: &Move) -> () {
        let mut in_the_way_indices: Vec<usize> = Vec::new(); // gather content indices of objects that would be shoved by the robot
        let mut blocked_by_wall = false;
        
        match direction {
            Move::RIGHT => {
                let mut check_pos = self.robot;
                loop {
                    check_pos.0 += 1;
                    let mut at_check_pos = self.content.iter().enumerate().filter(|(_, c)| c.position == check_pos);
                    if let Some((idx,  inv_in_way)) = at_check_pos.next() {
                        if inv_in_way.inv_type == InventoryType::BARREL {
                            // step 2 for Barrels
                            check_pos.0 += 1;
                            in_the_way_indices.push(idx);
                        } else {
                            // break for Walls
                            blocked_by_wall = true;
                            break;        
                        }
                    } else {
                        break;
                    }
                }
            },
            Move::LEFT => {
                let mut check_pos = self.robot;
                loop {
                    check_pos.0 -= 1;
                    let mut wall_at_check_pos = self.content.iter().enumerate().filter(|(_, c)| c.position == check_pos && c.inv_type == InventoryType::WALL);
                    if wall_at_check_pos.next().is_some() {
                        blocked_by_wall = true;
                        // stop at wall
                        break;
                    }
                    check_pos.0 -= 1;
                    let mut barrel_at_check_pos = self.content.iter().enumerate().filter(|(_, c)| c.position == check_pos && c.inv_type == InventoryType::BARREL);
                    if let Some((idx, _)) = barrel_at_check_pos.next() {
                        in_the_way_indices.push(idx);
                    } else {
                        // Nothing in the way
                        break;
                    }
                }
            },
            Move::UP => {
                let mut check_positions: Vec<(usize, usize)> = vec![self.robot];

                loop {
                    check_positions.iter_mut().for_each(|p| p.1 -= 1);
                    let mut new_check_positions: Vec<(usize, usize)> = Vec::new();

                    let mut walls_at_check_positions = self.content.iter().enumerate()
                    .filter(|(_, c)| c.inv_type == InventoryType::WALL && check_positions.contains(&c.position));
                    if walls_at_check_positions.next().is_some() {
                        blocked_by_wall = true;
                        break;
                    }
                    let mut barrels_at_check_positions = self.content.iter().enumerate()
                    .filter(|(_, c)| c.inv_type == InventoryType::BARREL && (check_positions.contains(&c.position) || check_positions.contains(&(c.position.0 + 1, c.position.1))));

                    while let Some((idx, barrel)) = barrels_at_check_positions.next() {
                        in_the_way_indices.push(idx);
                        // add barrel positions as next check_positions
                        new_check_positions.push(barrel.position);
                        new_check_positions.push((barrel.position.0 + 1, barrel.position.1));
                    }
                    if new_check_positions.len() == 0 { break; }
                    check_positions = new_check_positions;
                }
     
            },
            Move::DOWN => {
                let mut check_positions: Vec<(usize, usize)> = vec![self.robot];

                loop {
                    check_positions.iter_mut().for_each(|p| p.1 += 1);
                    let mut new_check_positions: Vec<(usize, usize)> = Vec::new();

                    let mut walls_at_check_positions = self.content.iter().enumerate()
                    .filter(|(_, c)| c.inv_type == InventoryType::WALL && check_positions.contains(&c.position));
                    if walls_at_check_positions.next().is_some() {
                        blocked_by_wall = true;
                        break;
                    }
                    let mut barrels_at_check_positions = self.content.iter().enumerate()
                    .filter(|(_, c)| c.inv_type == InventoryType::BARREL && (check_positions.contains(&c.position) || check_positions.contains(&(c.position.0 + 1, c.position.1))));

                    while let Some((idx, barrel)) = barrels_at_check_positions.next() {
                        in_the_way_indices.push(idx);
                        // add barrel positions as next check_positions
                        new_check_positions.push(barrel.position);
                        new_check_positions.push((barrel.position.0 + 1, barrel.position.1));
                    }
                    if new_check_positions.len() == 0 { break; }
                    check_positions = new_check_positions;
                }

            }
            
        }
        
        if !blocked_by_wall {
            match direction {
                Move::UP => {
                    self.robot.1 -= 1;
                    for idx in in_the_way_indices {
                        self.content[idx].position.1 -= 1;
                    }
                },
                Move::RIGHT => {
                    self.robot.0 += 1;
                    for idx in in_the_way_indices {
                        self.content[idx].position.0 += 1;
                    }
                },
                Move::DOWN => {
                    self.robot.1 += 1;
                    for idx in in_the_way_indices {
                        self.content[idx].position.1 += 1;
                    }
                },
                Move::LEFT => {
                    self.robot.0 -= 1;
                    for idx in in_the_way_indices {
                        self.content[idx].position.0 -= 1;
                    }
                },
            }
        }
    }
}

impl Warehouse {

    fn move_robot(&mut self, direction: &Move) -> () {
        let mut in_the_way: Vec<&mut Inventory>;
        let direct_in_the_way: Vec<&mut Inventory>;
        match direction {
            Move::UP => {
                in_the_way = self.content.iter_mut().filter(|c| c.position.0 == self.robot.0 && c.position.1 < self.robot.1).collect();
                in_the_way.sort_by(|a: &&mut Inventory, b: &&mut Inventory| if a.position.1 > b.position.1 { Ordering::Less } else {Ordering::Greater });
                direct_in_the_way = in_the_way.into_iter().enumerate().filter(|(idx, inv)| inv.position.1 + (idx + 1) == self.robot.1).map(|(_, i)| i).collect();
            },
            Move::RIGHT => {
                in_the_way = self.content.iter_mut().filter(|c| c.position.0 > self.robot.0 && c.position.1 == self.robot.1).collect();
                in_the_way.sort_by(|a: &&mut Inventory, b: &&mut Inventory| if a.position.0 < b.position.0 { Ordering::Less } else {Ordering::Greater });
                direct_in_the_way = in_the_way.into_iter().enumerate().filter(|(idx, inv)| inv.position.0 - (idx + 1)== self.robot.0).map(|(_, i)| i).collect();
            },
            Move::DOWN => {
                in_the_way = self.content.iter_mut().filter(|c| c.position.0 == self.robot.0 && c.position.1 > self.robot.1).collect();
                in_the_way.sort_by(|a: &&mut Inventory, b: &&mut Inventory| if a.position.1 < b.position.1 { Ordering::Less } else {Ordering::Greater });
                direct_in_the_way = in_the_way.into_iter().enumerate().filter(|(idx, inv)| inv.position.1 - (idx + 1) == self.robot.1).map(|(_, i)| i).collect();
            },
            Move::LEFT => {
                in_the_way = self.content.iter_mut().filter(|c| c.position.0 < self.robot.0 && c.position.1 == self.robot.1).collect();
                in_the_way.sort_by(|a: &&mut Inventory, b: &&mut Inventory| if a.position.0 > b.position.0 { Ordering::Less } else {Ordering::Greater });
                direct_in_the_way = in_the_way.into_iter().enumerate().filter(|(idx, inv)| inv.position.0 + (idx + 1)== self.robot.0).map(|(_, i)| i).collect();
            }
        }

        if direct_in_the_way.iter().any(|i| i.inv_type == InventoryType::WALL) {
            // cant move, wall directly ahead
            return ();
        }

        for barrel in direct_in_the_way {
            match direction {
                Move::UP => barrel.position.1 -= 1,
                Move::RIGHT => barrel.position.0 += 1,
                Move::DOWN => barrel.position.1 += 1,
                Move::LEFT => barrel.position.0 -= 1,
            }
        }

        match direction {
            Move::UP => self.robot.1 -= 1,
            Move::RIGHT => self.robot.0 += 1,
            Move::DOWN => self.robot.1 += 1,
            Move::LEFT => self.robot.0 -= 1,
        }
    }

    fn get_dimensions(&self) -> (usize, usize) {
        let mut cols: usize = 0;
        let mut rows: usize = 0;

        for cont in &self.content {
            if cont.position.0 > cols {
                cols = cont.position.0;
            }
            if cont.position.1 > rows {
                rows = cont.position.1
            }
        }
        (cols + 1, rows + 1)
    }

    fn gps_sum(&self) -> usize {
        self.content.iter().filter(|c| c.inv_type == InventoryType::BARREL).map(|b| b.position.1 * 100 + b.position.0).sum()
    }
}