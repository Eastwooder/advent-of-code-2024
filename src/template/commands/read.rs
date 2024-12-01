use std::process;

use crate::template::Day;

pub fn handle(day: Day) {
    if let Err(e) = crate::template::aoc_client::download_puzzle(day) {
        eprintln!("failed to call aoc-client: {e}");
        process::exit(1);
    };
}
