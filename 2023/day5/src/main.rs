use std::io::{BufReader, BufRead};
use std::fs::File;

use std::str::FromStr;


struct MappingRange {
    destination_start: usize,
    source_start: usize,
    range_size: usize,
}

impl MappingRange {
    fn applicable(&self, input: usize) -> bool {
        return self.source_start <= input && input < self.source_start+self.range_size;
    }

    fn process(&self, input: usize) -> usize {
        return (input as isize + (self.destination_start as isize - self.source_start as isize)) as usize;
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseMappingRangeError;

impl FromStr for MappingRange {
    type Err = ParseMappingRangeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers: Vec<Result<usize, ParseMappingRangeError>> = s.split(" ")
        .filter(|ss| !ss.is_empty())
        .map(|ss| ss.parse::<usize>().map_err(|_| ParseMappingRangeError)).collect();

        if numbers.iter().any(|r| r.is_err()){
            return Err(ParseMappingRangeError);
        }

        let pnumbers: Vec<usize> = numbers.iter().map(|n| n.as_ref().unwrap()).copied().collect();
        return Ok(MappingRange { 
            destination_start: pnumbers[0],
            source_start: pnumbers[1],
            range_size: pnumbers[2],
        })
    }
}

struct Mapping {
    ranges: Vec<MappingRange>,
}

impl Mapping {
    fn process(&self, input: usize) -> usize {
        let fitting_range = self.ranges.iter().filter(|r| r.applicable(input)).nth(0);
        match fitting_range {
            Some(r) => return r.process(input),
            None => return input,
        }
    }
}


fn main() {
    let file = File::open("input.txt").expect("No such file!");
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines()
                    .map(|l| l.expect("Could not read line!"))
                    .collect();

    let mut seeds: Vec<usize> = lines[0].split(":").nth(1).unwrap().trim()
                                .split(" ")
                                .map(|s| s.parse::<usize>())
                                .filter(|o| o.is_ok())
                                .map(|o| o.unwrap())
                                .collect();

    let mut maps = Vec::<Mapping>::new();

    let maplines: Vec<Vec<&String>> = lines[2..].split(|l| l.is_empty())
                .filter(|v| !v.is_empty())
                .map(|la| la.iter().filter(|l| l.chars().all(|c| c.is_numeric() || c.is_ascii_whitespace())).collect())
                .collect();

    for rangelinearray in maplines {
        let mapranges: Vec<MappingRange> = rangelinearray.iter().map(|line| line.parse::<MappingRange>().unwrap()).collect();

        maps.push(Mapping { ranges: mapranges });
    }

    for map in maps {
        seeds = seeds.iter().map(|s| map.process(*s)).collect();
    }

    println!("Min location is: {}", seeds.iter().min().unwrap());
    

}
