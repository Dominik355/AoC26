pub mod part1;
pub mod part2;

use std::env::Args;
use nom::{
    IResult, Parser as NomParser, bytes::complete::tag, character::complete, sequence::preceded,
};
use reqwest::blocking::Client;
use reqwest::header::COOKIE;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use tracing::info;

pub fn parse_day(input: &str) -> IResult<&str, u32> {
    preceded(tag("day-"), complete::u32).parse(input)
}

pub fn get_input_str(day: u32, args: Args) -> anyhow::Result<String> {
    Ok(fs::read_to_string(get_input(day, args)?)?)
}

pub fn get_input(day: u32, args: Args) -> anyhow::Result<PathBuf> {
    let dir = std::env::temp_dir().join("aoc25");
    fs::create_dir_all(&dir)?;
    let file = dir.join(format!("day{:02}.txt", day));

    if !file.exists() {
        info!(
            ?day,
            "Input file not found, downloading it now. Path={}",
            file.display()
        );
        let session_id = session_id(args);
        download_input(day, &file, &session_id)?;
    }

    Ok(file)
}

pub fn download_input(day: u32, file_path: &Path, session_id: &str) -> anyhow::Result<()> {
    let url = format!("https://adventofcode.com/2025/day/{day}/input");
    info!("Downloading input from url `{}` with session={}", url, session_id);

    let resp = Client::new()
        .get(url)
        .header(COOKIE, format!("session={session_id}"))
        .send()?;

    if !resp.status().is_success() {
        anyhow::bail!(
            "Request has failed with status '{:?}'. Error: \n{:?}",
            resp.status(),
            resp.text().unwrap_or_default()
        );
    }

    let input_data = resp.text()?;

    let mut file = fs::File::create(file_path).expect("cant create an input file");
    file.write_all(input_data.as_bytes())?;
    info!(
        "downloaded input into file {} with total size of {} bytes",
        file_path.display(),
        file.metadata()?.len()
    );

    Ok(())
}

// first look ito args, if not set - check env vars
fn session_id(mut args: Args) -> String {
    args.nth(1).unwrap_or_else(|| {
        std::env::var("SESSION").expect("Session id env var missing and not provided as arg")
    })
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "session-required")]
    #[test_log::test]
    fn get_input_test() {
        let res = crate::get_input(1);
        assert!(res.is_ok());
        assert!(res.unwrap().metadata().unwrap().len() > 0);
    }
}