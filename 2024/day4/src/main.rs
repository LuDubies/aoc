use std::io::{BufReader, BufRead};
use std::fs::File;
use std::cmp;
use regex::Regex;


fn main() {

    let file = File::open("input.txt").expect("No such file!");
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
        grid: raw_puzzle,
    }; 

    for horizontal in &wordpuzzle.grid
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
        rising_xmas += xmas_re.captures_iter(&diag.iter().collect::<String>()).count();
        rising_xmas += samx_re.captures_iter(&diag.iter().collect::<String>()).count();
    }

    let fallingit  = DiagonalIterator {
        puzzle: &wordpuzzle,
        index: 0,
        falling: true
    };

    for diag in fallingit
    {
        falling_xmas += xmas_re.captures_iter(&diag.iter().collect::<String>()).count();
        falling_xmas += samx_re.captures_iter(&diag.iter().collect::<String>()).count();
    }

    let total_xmas = horizontal_xmas + vertical_xmas + rising_xmas + falling_xmas;
    println!("[Part 1] Total xmas {}, from horizontal {}, vertical {}, rising {}, falling {}.",
        total_xmas,
        horizontal_xmas,
        vertical_xmas,
        rising_xmas,
        falling_xmas
    );

    let xit = XIterator {
        puzzle: &wordpuzzle,
        index: 0
    };

    let pattern_xmas = xit.map(|xp| xmas_validate(&xp)).filter(|b| *b).count();
    println!("[Part 2] Total xmas patterns {}.", pattern_xmas);

}


struct Wordpuzzle {
    linecount: usize,
    linelength: usize,
    grid: Vec<Vec<char>>
}

// rovides vertical vectors through the puzzle
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
                column.push(self.puzzle.grid[row][self.index]);
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

// Provides all diagonals throughout the puzzle, either falling or rising
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
                let diagonal_length = diag_length(self.index, self.puzzle.linecount, self.puzzle.linelength);
                let mut col_offset: usize = 0;
                if self.index >= self.puzzle.linecount
                {
                    col_offset = self.index - self.puzzle.linecount + 1;
                }
                let mut row_offset: usize = 0;
                if self.index < self.puzzle.linecount
                {
                    row_offset = self.puzzle.linecount - self.index - 1;
                }

                for di in 0..diagonal_length
                {
                    let col = di + col_offset;
                    let row = di + row_offset;
                    diagonal.push(self.puzzle.grid[row][col]);
                }
            }
            else
            {
                // handle rising diagonals    
                let diagonal_length = diag_length(self.index, self.puzzle.linelength, self.puzzle.linecount);
                let mut col_offset: usize = 0;
                if self.index >= self.puzzle.linecount
                {
                    col_offset = self.index - self.puzzle.linecount + 1;
                }

                for di in 0..diagonal_length
                {
                    let col = di + col_offset;
                    let row = cmp::min(self.index, self.puzzle.linecount - 1) - di;
                    diagonal.push(self.puzzle.grid[row][col]);
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

// Provides all 3 by 3 x  patterns in the puzzle, returned as 6 char vector with the falling and rising x diagonals.
struct XIterator<'a>
{
    puzzle: &'a Wordpuzzle,
    index: usize,
}

impl<'a> Iterator for XIterator<'a> {
    type Item = Vec<char>;

    fn next(&mut self) -> Option<Self::Item> {
        let xmax = (self.puzzle.linecount - 2) * (self.puzzle.linelength - 2);
        if self.index < xmax
        {
            let mut x: Vec<char> = Vec::new(); // x is a fitting variable name here ;)
            let xlinelen = self.puzzle.linelength - 2;
            let middlex = self.index % xlinelen + 1;
            let middley = self.index / xlinelen + 1;

            // collect the 6 x chars (idx 1 and 4 are the same)
            x.push(self.puzzle.grid[middley-1][middlex-1]);
            x.push(self.puzzle.grid[middley][middlex]);
            x.push(self.puzzle.grid[middley+1][middlex+1]);
            x.push(self.puzzle.grid[middley+1][middlex-1]);
            x.push(self.puzzle.grid[middley][middlex]);
            x.push(self.puzzle.grid[middley-1][middlex+1]);

            self.index += 1;
            Some(x)
        }
        else
        {
            None    
        }




    }
}


// Validate 5 char array for XMAS pattern
//
//  array indices are shown below (entry 4 is duplicate of 1)
// 
//  0.5
//  .1.
//  3.2
fn xmas_validate(pattern: &Vec<char>) -> bool
{
    if pattern[..3] == ['M', 'A', 'S'] || pattern[..3] == ['S', 'A', 'M']
    {
        if pattern[3..] == ['M', 'A', 'S'] || pattern[3..] == ['S', 'A', 'M']
        {
            return true;
        }
    }
    false
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

    #[test]
    fn pattern_test() {
        assert_eq!(true, xmas_validate(&vec!['M', 'A', 'S', 'M', 'A', 'S']));
        assert_eq!(true, xmas_validate(&vec!['S', 'A', 'M', 'M', 'A', 'S']));
        assert_eq!(false, xmas_validate(&vec!['M', 'X', 'M', 'A', 'S', 'M']));
    }
}

