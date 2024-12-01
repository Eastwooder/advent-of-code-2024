use crate::template::Day;
use aoc_client::AocClient;
use std::env::var;

pub fn read_input(day: Day) -> anyhow::Result<String> {
    aoc_client(day)?.get_input().map_err(|err| err.into())
}

pub fn download_puzzle(day: Day) -> anyhow::Result<()> {
    aoc_client(day)?
        .save_puzzle_markdown()
        .map_err(|err| err.into())
}

pub fn submit(day: Day, part: u8, answer: &str) -> anyhow::Result<()> {
    aoc_client(day)?
        .submit_answer_and_show_outcome::<i64, &str>(part.into(), answer)
        .map_err(|err| err.into())
}

fn aoc_client(day: Day) -> anyhow::Result<AocClient> {
    AocClient::builder()
        .session_cookie(var("SESSION_COOKIE")?)?
        .day(day.into_inner().into())?
        .year(get_year().unwrap_or(2024).into())?
        .puzzle_filename(get_puzzle_path(day))
        .build()
        .map_err(|err| err.into())
}

fn get_puzzle_path(day: Day) -> String {
    format!("data/puzzles/{day}.md")
}

fn get_year() -> Option<u16> {
    match var("AOC_YEAR") {
        Ok(x) => x.parse().ok().or(None),
        Err(_) => None,
    }
}
