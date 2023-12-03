use std::io::{BufReader, BufRead};
use std::fs::File;

#[derive(Debug)]
struct Number{
    row: usize,
    start: usize,
    end: usize, // not inclusive
    value: usize,
}

fn main() {

    let file = File::open("input.txt").expect("No such file!");
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines()
                    .map(|l| l.expect("Could not read line!"))
                    .collect();

    let mut engine_table = Vec::<Vec::<char>>::new();
    for line in lines {
        engine_table.push(line.chars().collect());
    }

    let numbers = find_numbers(&engine_table);

    let mut part_number_sum: usize = 0;

    for num in &numbers {
        if check_for_symbol(num.row, num.start, num.end, &engine_table) {
            part_number_sum += num.value;
        }
    }
    println!("Part numbers add up to {}", part_number_sum);

    // PART 2
    let mut gear_ratio_sum: usize = 0;

    for rowidx in 0..engine_table.len() {
        for colidx in 0..engine_table[rowidx].len() {
            if '*' == engine_table[rowidx][colidx] {
                let adjacent_nums = filter_for_adjacents(&numbers, (rowidx, colidx));
                if 2 == adjacent_nums.len() {
                    let gear_ratio = adjacent_nums[0].value * adjacent_nums[1].value;
                    println!("Gear Ratio of {} for gear at {}, {}", gear_ratio, rowidx, colidx);
                    gear_ratio_sum += gear_ratio;
                }
            }
        }
    }

    println!("Gear ratios add up to {}", gear_ratio_sum);


}

fn is_symbol(c: char) -> bool {
    return !c.is_digit(10) && ('.' != c);
}

fn filter_for_adjacents(numbers: &Vec<Number>, position: (usize, usize)) -> Vec<&Number> {
    let mut adjacents = Vec::<&Number>::new();
    let (gearrow, gearcol) = position;

    let gear_row_bound = if 0 == gearrow {0} else {gearrow - 1};

    for num in numbers {
        let num_start_bound = if 0 == num.start {0} else {num.start - 1};
        if (gear_row_bound..gearrow+2).contains(&num.row) {
            if (num_start_bound..num.end+1).contains(&gearcol) {
                adjacents.push(num);
            }
        } 
    }

    return adjacents;
}

fn check_for_symbol(row: usize, start: usize, end: usize, table: &Vec<Vec<char>>) -> bool {
    let left_bound = if 0 == start {start} else {start - 1};
    let right_bound = if table[row].len() == end {end} else {end + 1};
    let upper_bound = if 0 == row {row} else {row - 1}; // up like higher in the table :D
    let lower_bound = if table.len() - 1 == row {row + 1} else {row + 2};

    for ridx in upper_bound..lower_bound {
        for cidx in left_bound..right_bound {
            if is_symbol(table[ridx][cidx]) {
                return true;
            }
        }
    }
    return false;
}

fn find_numbers(table: &Vec<Vec<char>>) -> Vec<Number> {
    let mut number_indices = Vec::<(usize, usize, usize)>::new();
    let mut reading_number: bool = false;
    let mut num_start_idx: usize = 0;

    for (row_idx, table_row ) in table.iter().enumerate() {
        for (col_idx, c) in table_row.iter().enumerate() {
            if c.is_digit(10) {
                if !reading_number {
                    reading_number = true;
                    num_start_idx = col_idx;
                }
            }
            if !c.is_digit(10) && reading_number {
                reading_number = false;
                number_indices.push((row_idx, num_start_idx, col_idx));
            }
        }
        if reading_number {
            reading_number = false;
            number_indices.push((row_idx, num_start_idx, table_row.len()));
        }
    }

    let mut numbers = Vec::<Number>::new();
    for (row, start, end) in number_indices {
        numbers.push(Number{row: row,
                            start: start,
                            end: end,
                            value: String::from_iter(&table[row][start..end]).parse::<usize>().unwrap()
                            });
    }
    return numbers;
}
