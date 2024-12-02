use std::io::{BufReader, BufRead};
use std::fs::File;
use std::ops::Not;

fn main() {

    let file = File::open("input.txt").expect("No such file!");
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.expect("Could not read line!"));

    let mut valid_reports: u64 = 0;
    let mut valid_reports_skip: u64 = 0;

    for report_str in lines
    {

        let mut levels: Vec<u64> = Vec::new();
        for lvl in report_str.split_whitespace()
        {
            levels.push(lvl.parse::<u64>().unwrap());
        }

        if check_validity(&levels)
        {
            valid_reports += 1;
            valid_reports_skip += 1;
        }
        else
        {
            /* Check with level skips. */
            for skip_index in 0..levels.len()
            {
                let level_subset = [&levels[..skip_index], &levels[skip_index+1..]].concat();

                if check_validity(&level_subset)
                {
                    valid_reports_skip += 1;
                    break;
                }
            }   
        }

    }  

    println!("[Part 1] Number of valid reports: {}", valid_reports);
    println!("[Part 2] Number of valid reports: {}", valid_reports_skip);

}


fn check_validity(levels: &Vec<u64>) -> bool
{
    let mut lastlevel: Option<u64> = None;
    let mut valid_asc: bool = true;
    let mut valid_dec: bool = true;

    for currlevel in levels
    {
        /* First run initialize lastlevel. */
        if None == lastlevel
        {
            lastlevel = Some(*currlevel);
            continue;
        }

        /* Check ascending validity. */
        if valid_asc
        {
            if ((lastlevel.unwrap() + 1) <= *currlevel && *currlevel <= (lastlevel.unwrap() + 3)).not()
            {
                valid_asc = false
            }

        }

        /* Check decending validity. */
        if valid_dec
        {
            if ((lastlevel.unwrap()) >= *currlevel + 1 && *currlevel + 3 >= (lastlevel.unwrap())).not()
            {
                valid_dec = false
            }
        }

        lastlevel = Some(*currlevel);
    }
    assert_eq!(false, valid_asc && valid_dec);
    return valid_asc || valid_dec;
}