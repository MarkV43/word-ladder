use anyhow::Result;
use rand::{seq::SliceRandom, thread_rng};
use std::time::Instant;

const DICTIONARY: &'static str = include_str!("../res/collins-scrabble-words-2019.txt");

fn distance(w1: &[u8], w2: &[u8]) -> usize {
    let mut count = 0;

    for (a, b) in w1.iter().zip(w2.iter()) {
        if a != b {
            count += 1;
            if count > 1 {
                return count;
            }
        }
    }

    return count;
}

pub fn solve_ladder(origin: &[u8], target: &[u8], randomize: bool) -> Result<()> {
    let t0 = Instant::now();

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

    let mut dist = 1;

    let mut seen_front = vec![origin];
    let mut parents_front = vec![usize::MAX];
    let mut search_front = vec![(origin, 0)];
    let mut next_search_front = vec![];

    let mut seen_back = vec![target];
    let mut parents_back = vec![usize::MAX];
    let mut search_back = vec![(target, 0)];
    let mut next_search_back = vec![];

    loop {
        println!("Searching depth {dist}");

        let sol_front = iterate(
            &mut seen_front,
            &mut parents_front,
            &mut search_front,
            &mut next_search_front,
            &words,
            &seen_back,
        );

        if let Some(index_front) = sol_front {
            let dur = t0.elapsed();

            show_solution(
                index_front,
                true,
                &seen_front,
                &parents_front,
                &seen_back,
                &parents_back,
            );

            println!("\nElapsed: {dur:?}");
            return Ok(());
        }

        let sol_back = iterate(
            &mut seen_back,
            &mut parents_back,
            &mut search_back,
            &mut next_search_back,
            &words,
            &seen_front,
        );

        if let Some(index_back) = sol_back {
            let dur = t0.elapsed();

            show_solution(
                index_back,
                false,
                &seen_front,
                &parents_front,
                &seen_back,
                &parents_back,
            );

            println!("\nElapsed: {dur:?}");
            return Ok(());
        }

        if search_front.is_empty() || search_back.is_empty() {
            println!("\nThere is no solution");
            return Ok(());
        }

        dist += 1;
    }
}

fn iterate<'a>(
    seen: &mut Vec<&'a [u8]>,
    parents: &mut Vec<usize>,
    search: &mut Vec<(&'a [u8], usize)>,
    next_search: &mut Vec<(&'a [u8], usize)>,
    words: &[&'a [u8]],
    seen_other: &[&'a [u8]],
) -> Option<usize> {
    for &(word, parent_index) in search.iter() {
        for &next in words.iter() {
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

    None
}

fn show_solution(
    index: usize,
    front: bool,
    seen_front: &[&[u8]],
    parents_front: &[usize],
    seen_back: &[&[u8]],
    parents_back: &[usize],
) {
    let mut solution_front = Vec::new();
    let mut solution_back = Vec::new();

    if front {
        fill_solution(
            &mut solution_front, //
            index,
            seen_front,
            parents_front,
        );
        fill_solution(
            &mut solution_back, //
            seen_back
                .iter()
                .position(|&w| w == seen_front[index])
                .unwrap(),
            seen_back,
            parents_back,
        );
    } else {
        fill_solution(
            &mut solution_back, //
            index,
            seen_back,
            parents_back,
        );
        fill_solution(
            &mut solution_front, //
            seen_front
                .iter()
                .position(|&w| w == seen_back[index])
                .unwrap(),
            seen_front,
            parents_front,
        );
    }

    println!(
        "\nFound a {}-step solution\n",
        solution_front.len() + solution_back.len() - 2
    );

    for w in solution_front
        .iter()
        .chain(solution_back.iter().rev().skip(1))
    {
        println!("{}", String::from_utf8_lossy(w));
    }
}

fn fill_solution<'a>(
    solution: &mut Vec<&'a [u8]>,
    index: usize,
    seen: &[&'a [u8]],
    parents: &[usize],
) {
    let mut i = index;

    while i != usize::MAX {
        solution.push(seen[i]);
        i = parents[i];
    }

    solution.reverse();
}
