use std::io::{BufReader, BufRead};
use std::fs::File;

const REDMAX: usize = 12;
const GREENMAX: usize = 13;
const BLUEMAX: usize = 14;

#[derive(Debug)]
struct Draw{
    red: usize,
    green: usize,
    blue: usize,
}

#[derive(Debug)]
struct Game{
    id: usize,
    draws: Vec<Draw>,
}

impl Game{
    fn new(id: usize) -> Self {
        Self {id, draws: Vec::new()}
    }

    fn is_valid(&self) -> bool {
        return  self.draws.iter().all(|d| d.is_valid());
    }

    fn get_power(&self) -> usize {
        let red_min = self.draws.iter().map(|d| d.red).max().unwrap_or(0);
        let green_min = self.draws.iter().map(|d| d.green).max().unwrap_or(0);
        let blue_min = self.draws.iter().map(|d| d.blue).max().unwrap_or(0);
        return red_min * green_min * blue_min;
    }
}

impl Draw{
    fn is_valid(&self) -> bool {
        return self.red <= REDMAX && self.green <= GREENMAX && self.blue <= BLUEMAX;
    }
}

fn main() {

    let file = File::open("input.txt").expect("No such file!");
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines()
                    .map(|l| l.expect("Could not read line!"))
                    .collect();

    let games = parse_games(&lines);

    let mut id_sum: usize = 0;
    let mut power_sum: usize = 0;

    for game in &games {
        if game.is_valid() {
            id_sum += game.id;
        }
        power_sum += game.get_power();
    }

    println!("Valid Ids add up to {}", id_sum);
    println!("Game power adds up to {}", power_sum);
}


fn parse_games(input: &Vec<String>) -> Vec<Game> {
    let mut games: Vec<Game> = Vec::new();
    
    for line in input.iter() {
        let id = line.split(":").nth(0).unwrap()[5..].parse::<usize>().unwrap();
        let encoded_draws = line.split(":").nth(1).unwrap_or("");

        let mut game = Game::new(id);
        
        for encoded_draw in encoded_draws.split(";") {
            let mut draw = Draw {
                red: 0,
                green: 0,
                blue: 0
            };
            for color_draw in encoded_draw.split(",") {
                if color_draw.contains("red") {
                    draw.red = extract_count(color_draw);
                }
                if color_draw.contains("green") {
                    draw.green = extract_count(color_draw);
                }
                if color_draw.contains("blue") {
                    draw.blue = extract_count(color_draw);
                }
            }
            game.draws.push(draw);
        }
        games.push(game);
    }

    games
}

fn extract_count(drawinfo: &str) -> usize {
    for possible_num in drawinfo.split(" ") {
        let parsed_num = possible_num.parse::<usize>();
        match parsed_num {
            Ok(x) => return x,
            Err(_) => (),
        };
    }
    0
}
