use crate::template::Day;
use std::process;

pub fn handle(day: Day) {
    if let Err(e) = crate::template::aoc_client::download_puzzle(day) {
        eprintln!("failed to call download puzzle: {e}");
        process::exit(1);
    };
}
