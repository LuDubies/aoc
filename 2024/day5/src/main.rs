use std::io::{BufReader, BufRead};
use std::fs::File;
use std::cmp;
use regex::Regex;
use std::env;





fn main() {

    let args: Vec<String> = env::args().collect();
    let mut filename = "input.txt";
    if args.len() > 1 && args[1] == "testdata"{
        filename = "testinput.txt";
    }
    let file = File::open(filename).expect("No such file!");
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.expect("Could not read line!"));

    let rule_re = Regex::new(r"(?<before>\d+)\|(?<after>\d+)").unwrap();
    let update_re = Regex::new(r"(\d+)(?:,(\d+))+").unwrap();

    let mut rules: Vec<Rule> = Vec::new();
    let mut updates: Vec<Update> = Vec::new();

    // parsing
    for line in lines
    {
        match rule_re.captures(&line)
        {
            Some(rule_cap) => {rules.push(Rule {
                before: rule_cap["before"].parse::<usize>().unwrap(),
                after: rule_cap["after"].parse::<usize>().unwrap()
            });},
            None => {}
        }
        match update_re.captures(&line)
        {
            Some(update_cap) => {
                updates.push(Update {
                pages: update_cap.get(0).unwrap().as_str().split(',').map(|d|d.parse::<usize>().unwrap()).collect()
            });},
            None => {}
        }
    }

    // Part 1 & 2
    let mut correct_middle_sum: usize = 0;
    let mut incorrect_middle_sum: usize = 0;
    for upd in updates.iter_mut()
    {
        if upd.violation(&rules).is_none()
        {
            correct_middle_sum += upd.pages[upd.pages.len()/2];
        }
        else {
            upd.correct(&rules);
            incorrect_middle_sum += upd.pages[upd.pages.len()/2];
        }

    }
    println!("[Part 1] Sum of middles of updates not violating a rule: {}", correct_middle_sum);
    println!("[Part 2] Sum of middles of invalid updates after correction: {}", incorrect_middle_sum);

}

#[derive(Debug, Clone)]
struct Rule
{
    before: usize,
    after: usize
}

impl Rule
{
    fn holds(&self, first: usize, second: usize) -> bool
    {
        if second == self.before && first == self.after
        {
            false
        }
        else
        {
            true
        }
    }
}

#[derive(Debug)]
struct Update
{
    pages: Vec<usize>
}

impl Update
{
    // Find the first rule that is violated in the update.
    fn violation(&self, rules: &Vec<Rule>) -> Option<(Rule, usize, usize)>
    {
        let mut pageidx: usize = 0;
        for page in &self.pages
        {
            let mut otheridx: usize = 1;
            for other in &self.pages[pageidx+1..]
            {
                for rule in rules
                {
                    if !rule.holds(*page, *other)
                    {
                        return Some((rule.clone(), pageidx, pageidx+otheridx));
                    }
                }
                otheridx += 1;
            }
            pageidx += 1;
        }

        None
    }

    fn correct(&mut self, rules: &Vec<Rule>) -> ()
    {
        loop {
            match self.violation(rules){
                Some((_, v1, v2)) => {
                    let tmp = self.pages[v1];
                    self.pages[v1] = self.pages[v2];
                    self.pages[v2] = tmp;

                },
                None => { break; }
            }
        }
    }


}
