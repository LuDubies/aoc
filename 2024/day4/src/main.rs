use std::io::{BufReader, BufRead};
use std::fs::File;
use std::cmp;
use regex::Regex;


fn main() {

    let file = File::open("testinput.txt").expect("No such file!");
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.expect("Could not read line!"));

    let xmas_re = Regex::new(r"XMAS").unwrap();
    let samx_re = Regex::new(r"SAMX").unwrap();

    let mut horizontal_xmas: usize = 0;
    let mut vertical_xmas: usize = 0;
    let mut falling_xmas: usize = 0;
    let mut rising_xmas: usize = 0;

    let raw_puzzle: Vec<Vec<char>> = lines.map(|l| l.chars().collect()).collect();

    let wordpuzzle: Wordpuzzle = Wordpuzzle {
        linecount: raw_puzzle.len(),
        linelength: raw_puzzle[0].len(),
        puzzle: raw_puzzle,
    }; 

    for horizontal in &wordpuzzle.puzzle
    {
        horizontal_xmas += xmas_re.captures_iter(&horizontal.iter().collect::<String>()).count();
        horizontal_xmas += samx_re.captures_iter(&horizontal.iter().collect::<String>()).count();
    }

    println!("Horizontal: {}", horizontal_xmas);

    let vertit = VerticalIterator {
        puzzle: &wordpuzzle,
        index: 0,
    };

    for vertical in vertit
    {
        vertical_xmas += xmas_re.captures_iter(&vertical.iter().collect::<String>()).count();
        vertical_xmas += samx_re.captures_iter(&vertical.iter().collect::<String>()).count();
    }

    println!("Vertical: {}", vertical_xmas);

    let risingit = DiagonalIterator {
        puzzle: &wordpuzzle,
        index: 0,
        falling: false
    };

    for diag in risingit
    {
        println!("{:?}", &diag.iter().collect::<String>());
    }

}


struct Wordpuzzle {
    linecount: usize,
    linelength: usize,
    puzzle: Vec<Vec<char>>
}

struct VerticalIterator<'a>
{
    puzzle: &'a Wordpuzzle,
    index: usize,
}

impl<'a> Iterator for VerticalIterator<'a> {
    type Item = Vec<char>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.puzzle.linelength
        {
            let mut column: Vec<char> = Vec::new();
            for row in 0..self.puzzle.linecount
            {
                column.push(self.puzzle.puzzle[self.index][row]);
            }
            self.index += 1;
            Some(column)
        }
        else
        {
            None
        }
    }
}

struct DiagonalIterator<'a>
{
    puzzle: &'a Wordpuzzle,
    index: usize,
    falling: bool,
}

impl<'a> Iterator for DiagonalIterator<'a> {
    type Item = Vec<char>;

    fn next(&mut self) -> Option<Self::Item> {
        let num_diagonals: usize = self.puzzle.linecount + self.puzzle.linelength - 1;
        if self.index < num_diagonals
        {
            let mut diagonal: Vec<char> = Vec::new();
            if self.falling
            {
                // handle falling diagonals
            }
            else
            {
                // handle rising diagonals    
                let diagonal_length = diag_length(self.index, self.puzzle.linelength, self.puzzle.linecount);
                let mut col_offset: usize = 0;
                if self.index > self.puzzle.linecount
                {
                    col_offset = self.index - self.puzzle.linecount + 1;
                }

                for di in 0..diagonal_length
                {
                    let col = di + col_offset;
                    let row = std::cmp::min(self.index, self.puzzle.linecount - 1) - di;
                    diagonal.push(self.puzzle.puzzle[row][col]);
                }
                
            }
            self.index += 1;
            Some(diagonal)
        }
        else
        {
            None
        }
    }
}

// Get the length of the index'th diagonal in a rectangle xsize * ysize.
fn diag_length(index: usize, xsize: usize, ysize: usize) -> usize
{
    let mut dlength = index + 1;
    if index >= xsize
    {
        dlength -= index - xsize + 1;
    }

    if index >= ysize
    {
        dlength -= index - ysize + 1;
    }

    dlength
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diag_test() {
        assert_eq!(2, diag_length(2, 2, 4));
        assert_eq!(1, diag_length(0, 2, 4));
        assert_eq!(1, diag_length(4, 2, 4));

        assert_eq!(1, diag_length(0, 6, 3));
        assert_eq!(2, diag_length(1, 6, 3));
        assert_eq!(3, diag_length(2, 6, 3));
        assert_eq!(3, diag_length(3, 6, 3));
        assert_eq!(1, diag_length(7, 6, 3));
    }
}