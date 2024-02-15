use super::{distance, Solver};
use anyhow::{anyhow, Result};
use rand::{seq::SliceRandom, thread_rng};

pub struct BFS2D {
    dictionary: &'static [&'static [u8]],
    exceptions: Vec<&'static [u8]>,
    randomize: bool,
}

impl Solver for BFS2D {
    fn new(dictionary: &'static [&'static [u8]], randomize: bool) -> Self {
        Self {
            dictionary,
            exceptions: Vec::new(),
            randomize,
        }
    }

    fn solve(&self, origin: &[u8], target: &[u8]) -> Result<Vec<String>> {
        assert_eq!(origin.len(), target.len());

        let mut words: Vec<&[u8]> = self
            .dictionary
            .iter()
            .filter(|x| x.len() == origin.len())
            .copied()
            .collect();

        if self.randomize {
            words.shuffle(&mut thread_rng());
        }

        let mut seen_front = vec![origin];
        let mut parents_front = vec![usize::MAX];
        let mut search_front = vec![(origin, 0)];
        let mut next_search_front = vec![];

        let mut seen_back = vec![target];
        let mut parents_back = vec![usize::MAX];
        let mut search_back = vec![(target, 0)];
        let mut next_search_back = vec![];

        loop {
            let sol_front = self.explore(
                &mut seen_front,
                &mut parents_front,
                &mut search_front,
                &mut next_search_front,
                &words,
                &seen_back,
            );

            if let Some(index_front) = sol_front {
                return Ok(self.untangle_solution_2way(
                    index_front,
                    true,
                    &seen_front,
                    &parents_front,
                    &seen_back,
                    &parents_back,
                ));
            }

            let sol_back = self.explore(
                &mut seen_back,
                &mut parents_back,
                &mut search_back,
                &mut next_search_back,
                &words,
                &seen_front,
            );

            if let Some(index_back) = sol_back {
                return Ok(self.untangle_solution_2way(
                    index_back,
                    false,
                    &seen_front,
                    &parents_front,
                    &seen_back,
                    &parents_back,
                ));
            }

            if search_front.is_empty() || search_back.is_empty() {
                return Err(anyhow!("There is no solution"));
            }
        }
    }

    fn find_largest_ladder(&self, length: usize, randomize: bool) -> Vec<String> {
        let mut words: Vec<&[u8]> = self
            .dictionary
            .iter()
            .filter(|x| x.len() == length)
            .copied()
            .collect();

        if randomize {
            words.shuffle(&mut thread_rng());
        }

        let mut seen = vec![words[0]];
        let mut search = vec![words[0]];
        let mut next_search = vec![];

        let mut depth = 1;

        while !search.is_empty() {
            println!("Depth: {depth}  \t{}\t{}", search.len(), seen.len());

            self.explore_without_tracking(&mut seen, &mut search, &mut next_search, &words);

            depth += 1;
        }

        let origin = *seen.last().unwrap();

        seen.clear();
        seen.push(origin);
        let mut parents = vec![usize::MAX];
        let mut search = vec![(origin, 0)];
        let mut next_search = vec![];

        depth = 1;

        while !search.is_empty() {
            println!("Depth: {depth}");

            self.explore(
                &mut seen,
                &mut parents,
                &mut search,
                &mut next_search,
                &words,
                &[],
            );

            depth += 1;
        }

        let target_index = seen.len() - 1;

        self.untangle_solution(target_index, &seen, &parents)
    }

    fn word_exists(&self, word: &[u8]) -> bool {
        self.dictionary.contains(&word)
    }

    fn get_dictionary(&self) -> &'static [&'static [u8]] {
        self.dictionary
    }

    fn set_exceptions(&mut self, exceptions: &[&'static [u8]]) {
        self.exceptions = exceptions.iter().cloned().collect();
    }
}

impl BFS2D {
    fn explore<'a>(
        &self,
        seen: &mut Vec<&'a [u8]>,
        parents: &mut Vec<usize>,
        search: &mut Vec<(&'a [u8], usize)>,
        next_search: &mut Vec<(&'a [u8], usize)>,
        words: &[&'a [u8]],
        seen_other: &[&'a [u8]],
    ) -> Option<usize> {
        for &(word, parent_index) in search.iter() {
            for &next in words {
                if distance(word, next) == 1
                    && !self.exceptions.contains(&next)
                    && !seen.contains(&next)
                {
                    parents.push(parent_index);
                    next_search.push((next, seen.len()));
                    seen.push(next);

                    if seen_other.contains(&next) {
                        return Some(seen.len() - 1);
                    }
                }
            }
        }

        std::mem::swap(search, next_search);
        next_search.clear();

        None
    }

    fn explore_without_tracking<'a>(
        &self,
        seen: &mut Vec<&'a [u8]>,
        search: &mut Vec<&'a [u8]>,
        next_search: &mut Vec<&'a [u8]>,
        words: &[&'a [u8]],
    ) {
        for &word in search.iter() {
            for &next in words {
                if distance(word, next) == 1
                    && !self.exceptions.contains(&next)
                    && !seen.contains(&next)
                {
                    next_search.push(next);
                    seen.push(next);
                }
            }
        }

        std::mem::swap(search, next_search);
        next_search.clear();
    }

    fn untangle_solution_2way<'a>(
        &self,
        index: usize,
        front: bool,
        seen_front: &[&'a [u8]],
        parents_front: &[usize],
        seen_back: &[&'a [u8]],
        parents_back: &[usize],
    ) -> Vec<String> {
        let (solution_front, solution_back) = if front {
            let index_back = seen_back
                .iter()
                .position(|&w| w == seen_front[index])
                .unwrap();
            (
                self.untangle_solution(index, seen_front, parents_front),
                self.untangle_solution(index_back, seen_back, parents_back),
            )
        } else {
            let index_front = seen_front
                .iter()
                .position(|&w| w == seen_back[index])
                .unwrap();
            (
                self.untangle_solution(index_front, seen_front, parents_front),
                self.untangle_solution(index, seen_back, parents_back),
            )
        };

        solution_front
            .into_iter()
            .chain(solution_back.into_iter().rev().skip(1))
            .collect()
    }

    fn untangle_solution<'a>(
        &self,
        index: usize,
        seen: &[&'a [u8]],
        parents: &[usize],
    ) -> Vec<String> {
        let mut solution = Vec::new();
        let mut i = index;

        while i != usize::MAX {
            solution.push(String::from_utf8_lossy(seen[i]).to_string());
            i = parents[i];
        }

        solution.reverse();
        solution
    }
}
