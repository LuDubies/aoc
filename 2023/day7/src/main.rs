use std::io::{BufReader, BufRead};
use std::fs::File;
use std::str::FromStr;
use std::cmp::Ordering;

const CARDTYPES: [char; 13] = ['J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A'];

fn main() {
    let file = File::open("input.txt").expect("No such file!");
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.expect("Could not read line!"));

    let mut hands: Vec<Hand> = lines.map(|l| l.parse::<Hand>().unwrap()).collect();

    hands.sort_unstable();

    let mut total_winnings: u64 = 0;

    for (i, hand) in hands.iter().enumerate() {
        total_winnings += (i as u64 + 1) * hand.bid;
    }

    println!("The total winnings are: {}", total_winnings);
}


#[derive(Debug, Eq)]
struct Hand {
    cards: [char; 5],
    bid: u64,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseHandError;

impl FromStr for Hand {
    type Err = ParseHandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (c, b) = s
            .split_once(" ")
            .ok_or(ParseHandError)?;

        let parsed_cards: [char; 5] = c.chars().collect::<Vec<char>>().try_into().map_err(|_| ParseHandError)?;
        let parsed_bid = b.parse::<u64>().map_err(|_| ParseHandError)?;

        Ok(Hand { cards: parsed_cards, bid: parsed_bid})
    }
}

impl Hand {
    fn get_type(&self) -> HandType {
        let mut counts = [0; CARDTYPES.len()];

        for _ in self.cards.iter().map(|c| counts[CARDTYPES.iter().position(|x| x == c).unwrap()] += 1) {
            (); // force map to be consumed, this is propably bad code.
        }

        let jokers = counts[CARDTYPES.iter().position(|x| *x == 'J').unwrap()];

        let max_same = counts[1..].iter().fold(0, |a, n| std::cmp::max(a, *n));

        if max_same + jokers >= 5 {
            return HandType::FIVEKIND;
        }

        if max_same + jokers == 4 {
            return HandType::FOURKIND;
        }

        if max_same == 3 {
            // if there were any Jokers and a Tripple, we already returned
            if counts.iter().any(|x| *x == 2) {
                return HandType::FULLHOUSE;
            } else {
                return HandType::THREEKIND;
            }
        }

        if max_same == 2 {
            // can maybe build a full house, better options would already be taken
            let pair_count = counts.iter().filter(|x| **x == 2).count();
            if jokers == 1 {
                if pair_count == 2 {
                    return HandType::FULLHOUSE;
                } else {
                    return HandType::THREEKIND;
                }
            } else {
                if  pair_count == 2 {
                    return HandType::TWOPAIR;
                } else {
                    return HandType::ONEPAIR;
                }
            }    
        }

        if jokers == 2 {
            return HandType::THREEKIND;
        }

        if jokers == 1 {
            return HandType:: ONEPAIR;
        }

        return HandType::HIGHCARD

    }

    fn highcard_compare(&self, other: &Self) -> Ordering {
        for (i, a) in self.cards.iter().enumerate() {
            let b = &other.cards[i];
            let apos  = CARDTYPES.iter().position(|x| x == a).unwrap();
            let bpos = CARDTYPES.iter().position(|x| x == b).unwrap();
            if apos != bpos {
                return apos.cmp(&bpos);
            }
        }
        return Ordering::Equal;
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.get_type() != other.get_type() {
            return self.get_type().cmp(&other.get_type());
        }
        return self.highcard_compare(other);
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}


#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    FIVEKIND = 7, 
    FOURKIND = 6,
    FULLHOUSE = 5,
    THREEKIND = 4,
    TWOPAIR = 3,
    ONEPAIR = 2,
    HIGHCARD = 1,
}