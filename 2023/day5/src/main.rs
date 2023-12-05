use std::io::{BufReader, BufRead};
use std::fs::File;

use std::str::FromStr;

#[derive(Debug)]
struct SeedRange {
    range_start: usize,
    range_size: usize,
}

struct MappingRange {
    destination_start: usize,
    source_start: usize,
    range_size: usize,
}

impl MappingRange {
    fn applicable(&self, input: usize) -> bool {
        return self.source_start <= input && input < self.source_start+self.range_size;
    }

    fn applicable_range(&self, input_range: &SeedRange) -> Option<SeedRange> {
        if input_range.range_start < self.source_start + self.range_size &&
            self.source_start < input_range.range_start + input_range.range_size {
            // have overlap
            let overlap_start = self.source_start.max(input_range.range_start);
            let overlap_end = (self.source_start+self.range_size).min(input_range.range_start+input_range.range_size);
            return Some(SeedRange{
                range_start: overlap_start, 
                range_size: overlap_end-overlap_start,
            });
        } else {
            return None;
        }
    }

    fn process(&self, input: usize) -> usize {
        if self.destination_start >= self.source_start {
            return input + (self.destination_start - self.source_start);
        } else {
            return input - (self.source_start - self.destination_start);
        }
    }

    fn process_range(&self, input_range: SeedRange) -> SeedRange {
        if self.destination_start >= self.source_start {
            return SeedRange {
                range_start: input_range.range_start + (self.destination_start - self.source_start),
                range_size: input_range.range_size
            };
        } else {
            return SeedRange {
                range_start: input_range.range_start + (self.source_start - self.destination_start),
                range_size: input_range.range_size
            };
        }
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

    fn process_range(&self, input_range: SeedRange) -> Vec<SeedRange> {
        let mut subranges: Vec<SeedRange> = self.ranges.iter().map(|r| r.applicable_range(&input_range))
            .filter(|o| o.is_some()).map(|s| s.unwrap()).collect();
        
        if subranges.is_empty() {
            // return unmapped input
            return vec![input_range];
        }

        let mut new_ranges = Vec::<SeedRange>::new();
        subranges.sort_by_key(|sr| sr.range_start);

        // check for unmapped subranges
        let mut checked_until: usize = input_range.range_start;
        for sr in &subranges {
            if checked_until < sr.range_start {
                new_ranges.push(SeedRange { 
                    range_start: checked_until,
                    range_size: sr.range_start - checked_until,
                });
            }

            // continue after sr
            checked_until = sr.range_start + sr.range_size;
        }
        // check after last sr
        if checked_until < input_range.range_start + input_range.range_size {
            new_ranges.push(SeedRange { 
                range_start: checked_until,
                range_size: input_range.range_start + input_range.range_size - checked_until,
            });
        }

        // add mapped input subranges, this code is sooo bad
        for sr in subranges {
            let corresponding_mapping_range = self.ranges.iter().filter(|r| r.applicable_range(&sr).is_some()).nth(0).unwrap();
            new_ranges.push(corresponding_mapping_range.process_range(sr));
        }

        return new_ranges;
    }
}


fn main() {
    let file = File::open("input.txt").expect("No such file!");
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines()
                    .map(|l| l.expect("Could not read line!"))
                    .collect();

    // PART 1

    let mut seeds: Vec<usize> = lines[0].split(":").nth(1).unwrap().trim()
                                .split(" ")
                                .map(|s| s.parse::<usize>())
                                .filter(|o| o.is_ok())
                                .map(|o| o.unwrap())
                                .collect();

    let seedcpy = seeds.clone(); // for part 2

    let mut maps = Vec::<Mapping>::new();

    let maplines: Vec<Vec<&String>> = lines[2..].split(|l| l.is_empty())
                .filter(|v| !v.is_empty())
                .map(|la| la.iter().filter(|l| l.chars().all(|c| c.is_numeric() || c.is_ascii_whitespace())).collect())
                .collect();

    for rangelinearray in maplines {
        let mapranges: Vec<MappingRange> = rangelinearray.iter().map(|line| line.parse::<MappingRange>().unwrap()).collect();

        maps.push(Mapping { ranges: mapranges });
    }

    for mapping in &maps {
        seeds = seeds.iter().map(|s| mapping.process(*s)).collect();
    }

    println!("Min location is: {}", seeds.iter().min().unwrap());
    
    // PART 2

    let mut seedranges = Vec::<SeedRange>::new();

    for i in 0..seedcpy.len() / 2 {
        seedranges.push(SeedRange { range_start: seedcpy[i], range_size: seedcpy[i+1] });
    }

    dbg!(&seedranges);
    let mut next_seedranges: Vec<SeedRange>;
    for mapping in &maps {
        next_seedranges = Vec::<SeedRange>::new();
        for sr in seedranges {
            next_seedranges.extend(mapping.process_range(sr));
        }
        seedranges = next_seedranges;
        dbg!(&seedranges);
    }

}
