use anyhow::{bail, Result};
use std::path::PathBuf;

pub fn get_puzzle_input(year: u64, day: u64) -> Result<String> {
    if let Ok(input) = load_puzzle_input_from_disk(year, day) {
        return Ok(input);
    }

    let input = download_puzzle_input(year, day)?;
    if let Err(e) = write_puzzle_input_to_disk(year, day, &input) {
        eprintln!("could not write puzzle input to disk: {}", e);
    }

    Ok(input)
}

fn write_puzzle_input_to_disk(year: u64, day: u64, input: &str) -> Result<()> {
    let path = PathBuf::from(format!("input/{}/{}.in", year, day));

    std::fs::create_dir_all(format!("input/{}", year))?;

    std::fs::write(path, input)?;

    Ok(())
}

fn load_puzzle_input_from_disk(year: u64, day: u64) -> Result<String> {
    let path = PathBuf::from(format!("input/{}/{}.in", year, day));
    if !path.exists() {
        bail!("file {} does not exist", path.to_string_lossy());
    }

    Ok(std::fs::read_to_string(path)?)
}

fn download_puzzle_input(year: u64, day: u64) -> Result<String> {
    let client = reqwest::blocking::Client::new();

    let resp = client
        .request(
            reqwest::Method::GET,
            format!("https://adventofcode.com/{}/day/{}/input", year, day),
        )
        .header("Cookie", format!("session={}", session_token()?))
        .send()?;

    if resp.status() != 200 {
        bail!(
            "non-200 from adventofcode.com ({}): {}",
            resp.status(),
            resp.text()?
        );
    }

    Ok(resp.text()?)
}

fn session_token() -> Result<String> {
    Ok(std::env::var("ADVENT_SESSION_TOKEN")?)
}
