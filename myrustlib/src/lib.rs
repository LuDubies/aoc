
// Returns pairs as tuples of references.
// Pairs are returned once ((A, B) and not (B, A) again).
// No pairs of the same item is returned (no (A, A) pairs).
pub struct PairIterator<'a, T> {
    items: Vec<&'a T>,
    fidx: usize,
    sidx: usize
}

impl<'a, T> PairIterator<'a, T> {
    pub fn from_vec(vector: Vec<&T>) -> PairIterator<T> {
        PairIterator{
            items: vector,
            fidx: 0,
            sidx: 0
        }
    }
}

impl<'a, T> Iterator for PairIterator<'a, T> {
    type Item = (&'a T, &'a T);

    fn next(&mut self) -> Option<Self::Item> {

        if self.fidx >= self.items.len() - 2 {
            return None;
        }

        self.sidx += 1;
        if self.sidx >= self.items.len() {
            self.fidx += 1;
            self.sidx = self.fidx + 1;
        }

        Some((self.items[self.fidx], self.items[self.sidx]))
    }
}


pub struct NeighbourIterator {
    position: (usize, usize),
    constraints: (usize, usize),
    directions: Box<dyn Iterator<Item = Direction>>
}

impl NeighbourIterator {
    pub fn from_pos(position: (usize, usize), constraints: (usize, usize), diagonal: bool) -> NeighbourIterator {
        if diagonal {
            NeighbourIterator {
                position,
                constraints,
                directions: Box::new(Direction::diag_iterator())
            }
        } else {
            NeighbourIterator {
                position,
                constraints,
                directions: Box::new(Direction::iterator())
            }
        }
        
    }
}

impl Iterator for NeighbourIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(direction) = self.directions.next() {
            match direction {
                Direction::North => {
                    if self.position.1 == 0 {
                        continue;
                    } else {
                        return Some((self.position.0, self.position.1 - 1));
                    }
                },
                Direction::East => {
                    if self.position.0 + 1 >= self.constraints.0 {
                        continue;
                    } else {
                        return Some((self.position.0 + 1, self.position.1));
                    }
                },
                Direction::South => {
                    if self.position.1 + 1 >= self.constraints.1 {
                        continue;
                    } else {
                        return Some((self.position.0, self.position.1 + 1));
                    }
                },
                Direction::West => {
                    if self.position.0 == 0 {
                        continue;
                    } else {
                        return Some((self.position.0 - 1, self.position.1));
                    }
                },
                Direction::NorthEast => {
                    if self.position.1 == 0 || self.position.0 + 1 >= self.constraints.0 {
                        continue;
                    } else {
                        return Some((self.position.0 + 1, self.position.1 - 1));
                    }
                },
                Direction::SouthEast => {
                    if self.position.1 + 1 >= self.constraints.1 || self.position.0 + 1 >= self.constraints.0 {
                        continue;
                    } else {
                        return Some((self.position.0 + 1, self.position.1 + 1));
                    }
                },
                Direction::SouthWest => {
                    if self.position.1 + 1 >= self.constraints.1 || self.position.0 == 0 {
                        continue;
                    } else {
                        return Some((self.position.0 - 1, self.position.1 + 1));
                    }
                },
                Direction::NorthWest => {
                    if self.position.0 == 0 || self.position.1 == 0 {
                        continue;
                    } else {
                        return Some((self.position.0 - 1, self.position.1 - 1));
                    }
                }
            }
        }
        None
    }
}


#[derive(PartialEq, Clone, Copy)]
 pub enum Direction {
    North,
    East,
    South,
    West,
    NorthEast,
    SouthEast,
    SouthWest,
    NorthWest
}

impl Direction {
    pub fn is_opposing(&self, other: &Direction) -> bool {
        match self {
            &Direction::North => other == &Direction::South,
            &Direction::East => other == &Direction::West,
            &Direction::South => other == &Direction::North,
            &Direction::West => other == &Direction::East,
            &Direction::NorthEast => other == &Direction::SouthWest,
            &Direction::SouthEast => other == &Direction::NorthWest,
            &Direction::SouthWest => other == &Direction::NorthEast,
            &Direction::NorthWest => other == &Direction::SouthEast
        }
    }

    pub fn iterator() -> impl Iterator<Item = Direction> {
        [Direction::North, Direction::South, Direction::East, Direction::West].iter().copied()
    }

    pub fn diag_iterator() -> impl Iterator<Item = Direction> {
        [Direction::North, Direction::NorthEast, Direction::East, Direction::SouthEast,
        Direction::South, Direction::SouthWest, Direction::West, Direction::NorthWest].iter().copied()
    }
}