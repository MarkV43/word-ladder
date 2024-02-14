mod solver;

use crate::solver::solve_ladder;
use anyhow::{anyhow, Result};
use clap::Parser;
use std::io::BufRead;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    word1: Option<String>,
    word2: Option<String>,
    #[arg(short, long)]
    random: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    println!("{args:?}");

    let mut origin;
    let mut target;

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

    solve_ladder(origin, target, args.random)
}
