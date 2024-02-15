pub mod bfs2d;

use anyhow::Result;

pub trait Solver {
    fn new(dictionary: &'static [&'static [u8]], randomize: bool) -> Self;
    fn solve(&self, origin: &[u8], target: &[u8]) -> Result<Vec<String>>;
    fn find_largest_ladder(&self, length: usize, randomize: bool) -> Vec<String>;
    fn word_exists(&self, word: &[u8]) -> bool;
}

/// Returns the distance between `w1` and `w2`.
/// Since we only care about whether the distance
/// is 1 or not, this method early returns, meaning
/// that a return value of 2 actually a means the
/// distance is of at least 2.
pub(crate) fn distance(w1: &[u8], w2: &[u8]) -> usize {
    let mut count = 0;

    for (a, b) in w1.iter().zip(w2.iter()) {
        if a != b {
            count += 1;
            if count > 1 {
                return count;
            }
        }
    }

    count
}
