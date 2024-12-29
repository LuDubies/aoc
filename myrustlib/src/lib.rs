
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