use std::{io::BufRead, time::Instant};

use anyhow::{anyhow, Result};

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

fn main() -> Result<()> {
    let mut origin = String::new();
    let mut target = String::new();

    let mut handle = std::io::stdin().lock();

    println!("Write the origin word: ");
    handle.read_line(&mut origin)?;

    println!("Write the target word: ");
    handle.read_line(&mut target)?;

    drop(handle);

    origin.pop().unwrap();
    target.pop().unwrap();

    if origin.len() != target.len() {
        return Err(anyhow!("Both words must be of the same length"));
    }

    origin.make_ascii_uppercase();
    target.make_ascii_uppercase();

    let origin: &[u8] = origin.as_bytes();
    let target: &[u8] = target.as_bytes();

    let t0 = Instant::now();

    let words: Vec<&[u8]> = DICTIONARY
        .lines()
        .skip(2)
        .map(str::trim)
        .map(str::as_bytes)
        .filter(|x| x.len() == origin.len())
        .collect();

    let mut seen = Vec::new();
    let mut indexes = Vec::new();
    let mut dist = 1;
    let mut search = vec![(origin, usize::MAX)];
    let mut next_search = vec![];

    loop {
        println!("Searching depth {dist}");

        for &(word, index) in search.iter() {
            let next_index = seen.len();
            seen.push(word);
            indexes.push(index);

            for &next in words.iter() {
                if distance(word, next) == 1
                    && !seen.contains(&next)
                    && search.iter().find(|(w, _)| w == &next).is_none()
                {
                    next_search.push((next, next_index));

                    if next == target {
                        let dur = t0.elapsed();

                        show_solution(dist, next, next_index, seen, indexes);

                        println!("\nElapsed: {dur:?}");

                        return Ok(());
                    }
                }
            }
        }

        search.clear();
        (search, next_search) = (next_search, search);

        dist += 1;
    }
}

fn show_solution(
    dist: usize,
    next: &[u8],
    next_index: usize,
    seen: Vec<&[u8]>,
    indexes: Vec<usize>,
) {
    println!(
        "Found {:?} with distance of {dist}\n",
        String::from_utf8_lossy(next)
    );

    let mut solution = Vec::new();

    solution.push(next);

    let mut i = next_index;

    while i != usize::MAX {
        solution.push(seen[i]);
        i = indexes[i];
    }

    solution.reverse();

    for w in solution {
        println!("{}", String::from_utf8_lossy(w));
    }
}
