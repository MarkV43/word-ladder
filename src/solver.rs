use anyhow::{anyhow, Result};
use rand::{seq::SliceRandom, thread_rng};

const DICTIONARY: &str = include_str!("../res/collins-scrabble-words-2019.txt");

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

pub fn solve_ladder<'a>(
    origin: &'a [u8],
    target: &'a [u8],
    randomize: bool,
) -> Result<Vec<&'a [u8]>> {
    let mut words: Vec<&[u8]> = DICTIONARY
        .lines()
        .skip(2)
        .map(str::trim)
        .map(str::as_bytes)
        .filter(|x| x.len() == origin.len())
        .collect();

    if randomize {
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
        let sol_front = explore(
            &mut seen_front,
            &mut parents_front,
            &mut search_front,
            &mut next_search_front,
            &words,
            &seen_back,
        );

        if let Some(index_front) = sol_front {
            return Ok(untangle_solution_2way(
                index_front,
                true,
                &seen_front,
                &parents_front,
                &seen_back,
                &parents_back,
            ));
        }

        let sol_back = explore(
            &mut seen_back,
            &mut parents_back,
            &mut search_back,
            &mut next_search_back,
            &words,
            &seen_front,
        );

        if let Some(index_back) = sol_back {
            return Ok(untangle_solution_2way(
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

fn explore<'a>(
    seen: &mut Vec<&'a [u8]>,
    parents: &mut Vec<usize>,
    search: &mut Vec<(&'a [u8], usize)>,
    next_search: &mut Vec<(&'a [u8], usize)>,
    words: &[&'a [u8]],
    seen_other: &[&'a [u8]],
) -> Option<usize> {
    for &(word, parent_index) in search.iter() {
        for &next in words {
            if distance(word, next) == 1 && !seen.contains(&next) {
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
    seen: &mut Vec<&'a [u8]>,
    search: &mut Vec<&'a [u8]>,
    next_search: &mut Vec<&'a [u8]>,
    words: &[&'a [u8]],
) {
    for &word in search.iter() {
        for &next in words {
            if distance(word, next) == 1 && !seen.contains(&next) {
                next_search.push(next);
                seen.push(next);
            }
        }
    }

    std::mem::swap(search, next_search);
    next_search.clear();
}

fn untangle_solution_2way<'a>(
    index: usize,
    front: bool,
    seen_front: &[&'a [u8]],
    parents_front: &[usize],
    seen_back: &[&'a [u8]],
    parents_back: &[usize],
) -> Vec<&'a [u8]> {
    let (solution_front, solution_back) = if front {
        let index_back = seen_back
            .iter()
            .position(|&w| w == seen_front[index])
            .unwrap();
        (
            untangle_solution(index, seen_front, parents_front),
            untangle_solution(index_back, seen_back, parents_back),
        )
    } else {
        let index_front = seen_front
            .iter()
            .position(|&w| w == seen_back[index])
            .unwrap();
        (
            untangle_solution(index_front, seen_front, parents_front),
            untangle_solution(index, seen_back, parents_back),
        )
    };

    solution_front
        .into_iter()
        .chain(solution_back.into_iter().rev().skip(1))
        .collect()
}

fn untangle_solution<'a>(index: usize, seen: &[&'a [u8]], parents: &[usize]) -> Vec<&'a [u8]> {
    let mut solution = Vec::new();
    let mut i = index;

    while i != usize::MAX {
        solution.push(seen[i]);
        i = parents[i];
    }

    solution.reverse();
    solution
}

pub fn find_largest_ladder<'a>(length: usize, randomize: bool) -> Vec<&'a [u8]> {
    let mut words: Vec<&[u8]> = DICTIONARY
        .lines()
        .skip(2)
        .map(str::trim)
        .map(str::as_bytes)
        .filter(|x| x.len() == length)
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

        explore_without_tracking(&mut seen, &mut search, &mut next_search, &words);

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

        explore(
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

    untangle_solution(target_index, &seen, &parents)
}
