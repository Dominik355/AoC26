use nom::character::complete::multispace0;
use nom::character::complete::multispace1;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{self},
    multi::separated_list1,
    sequence::{separated_pair, terminated},
};
use std::ops::RangeInclusive;

#[tracing::instrument(skip_all)]
pub fn process(input: String) -> anyhow::Result<impl std::fmt::Debug> {
    let mut ranges = parse_input(&input)
        .map_err(|e| anyhow::anyhow!("failed to parse input: {:?}", e))?
        .1;

    // not using hashset here

    // sort them first
    ranges.sort_by(|a, b| a.start().cmp(b.start()).then(b.end().cmp(a.start())));

    // not merge them
    let mut merged_ranges = Vec::new();
    let mut i = 0;
    let mut total = 0;
    while i < ranges.len() {
        let mut current_r = ranges[i].clone();
        i += 1;
        // merge overlapping ranges
        while i < ranges.len() && current_r.contains(ranges[i].start()) {
            current_r = *current_r.start()..=*current_r.end().max(ranges[i].end());
            i += 1;
        }
        total += current_r.end() - current_r.start() + 1;
        merged_ranges.push(current_r);
    }

    println!("Merged: {:?}", merged_ranges);

    Ok(total)
}

fn parse_input(input: &str) -> IResult<&str, Vec<RangeInclusive<u64>>> {
    let range = |input| {
        separated_pair(complete::u64, tag("-"), complete::u64)
            .map(|(a, b)| a..=b)
            .parse(input)
    };

    terminated(separated_list1(multispace1, range), multispace0).parse(input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_process() -> anyhow::Result<()> {
        todo!("haven't built test yet");
    }
}
