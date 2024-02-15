#![warn(clippy::all, clippy::pedantic)]

mod gui;
mod solver;

use anyhow::{anyhow, Result};
use clap::Parser;
use gui::initialize_egui;
use solver::Solver;
use std::{io::BufRead, time::Instant};

use crate::solver::bfs2d::BFS2D;

const DICTIONARY: &str = include_str!("../res/collins-scrabble-words-2019.txt");

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    word1: Option<String>,
    word2: Option<String>,
    #[arg(short, long)]
    random: bool,
    #[arg(short, long)]
    largest: Option<usize>,
    #[arg(long)]
    gui: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut origin;
    let mut target;

    let dictionary = DICTIONARY
        .lines()
        .map(str::trim)
        .map(str::as_bytes)
        .collect::<Vec<_>>();

    let solver = BFS2D::new(&dictionary, args.random);

    if args.gui {
        initialize_egui(&solver).map_err(|e| anyhow!("Error: {e:?}"))?;
    } else if let Some(length) = args.largest {
        let t0 = Instant::now();

        let result = solver.find_largest_ladder(length, args.random);

        let dur = t0.elapsed();

        println!("{}-step solution found\n", result.len() - 1);

        for word in result {
            println!("{}", word);
        }

        println!("\nElapsed {dur:?}");
    } else {
        if args.word1.is_none() || args.word2.is_none() {
            origin = String::new();
            target = String::new();

            let mut handle = std::io::stdin().lock();

            println!("Write the origin word: ");
            handle.read_line(&mut origin)?;

            println!("Write the target word: ");
            handle.read_line(&mut target)?;

            drop(handle);
        } else {
            origin = args.word1.unwrap();
            target = args.word2.unwrap();
        }

        if origin.len() != target.len() {
            return Err(anyhow!("Both words must be of the same length"));
        }

        origin.make_ascii_uppercase();
        target.make_ascii_uppercase();

        let origin: &[u8] = origin.trim().as_bytes();
        let target: &[u8] = target.trim().as_bytes();

        let t0 = Instant::now();

        let result = solver.solve(origin, target)?;

        let dur = t0.elapsed();

        println!("{}-step solution found\n", result.len() - 1);

        for word in result {
            println!("{}", word);
        }

        println!("\nElapsed {dur:?}");
    }

    Ok(())
}
