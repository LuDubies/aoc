use std::io::{BufReader, BufRead};
use std::fs::File;

#[derive(Debug)]
struct Card {
    id: usize,
    winning: Vec<usize>,
    have: Vec<usize>,
}

impl Card {

    fn matches(&self) -> usize {
        let mut matches: usize = 0;

        for h in self.have.iter() {
            if self.winning.contains(&h) { matches += 1; }
        }
        return matches;
    }
    fn score(&self) -> usize {
        let matches = self.matches().try_into().unwrap_or(0);
        return if 0 == matches {0} else {2_usize.pow(matches - 1)};
    }
}

fn main() {
    let file = File::open("input.txt").expect("No such file!");
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines()
                    .map(|l| l.expect("Could not read line!"))
                    .collect();

    let cards = parse_cards(lines);

    let mut total_points: usize = 0;

    for card in &cards {
        total_points += card.score();
    }

    println!("Total scratch card points: {}", total_points);

    // PART 2

    let mut card_amounts = vec![1_usize; cards.len()];

    for card in &cards {
        let amount = card_amounts[card.id - 1];
        let matches = card.matches();

        for copyid in card.id..std::cmp::min(card.id+matches, card_amounts.len()) {
            card_amounts[copyid] += amount;
        }
    }

    let total_amount: usize = card_amounts.iter().sum();

    println!("Total amount of cards: {}", total_amount);

}


fn parse_cards(lines: Vec<String>) -> Vec<Card> {
    let mut cards = Vec::<Card>::new();

    for line in lines {
        let id = line.split(": ").nth(0).unwrap().split(" ").last().unwrap().parse::<usize>().unwrap();
        let winning_string = line.split(": ").nth(1).unwrap().split(" | ").nth(0).unwrap();
        let have_string = line.split(": ").nth(1).unwrap().split(" | ").nth(1).unwrap();

        let mut card = Card{id: id, winning: vec![], have: vec![] };

        for numstr in winning_string.split(" ") {
            let parse_attempt = numstr.parse::<usize>();
            match parse_attempt {
                Ok(num) => card.winning.push(num),
                Err(_) => (),
            }
        }
        for numstr in have_string.split(" ") {
            let parse_attempt = numstr.parse::<usize>();
            match parse_attempt {
                Ok(num) => card.have.push(num),
                Err(_) => (),
            }
        }

        cards.push(card);
    }

    return cards;
}