use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::{all_consuming, opt},
    multi::separated_list1,
    sequence::{separated_pair, terminated},
};
use std::ops::RangeInclusive;

#[tracing::instrument(skip_all)]
pub fn process(input: String) -> anyhow::Result<impl std::fmt::Debug> {
    //     let input = r#"3-5
    // 10-14
    // 16-20
    // 12-18
    //
    // 1
    // 5
    // 8
    // 11
    // 17
    // 32"#;

    let (ranges, nums) = all_consuming(parse_input)
        .parse(&input)
        .map_err(|e| anyhow::anyhow!("failed to parse input: {:?}", e))?
        .1;

    let result = nums
        .iter()
        .filter(|n| ranges.iter().any(|r| r.contains(*n)))
        .count();

    Ok(result)
}

fn parse_input(input: &str) -> IResult<&str, (Vec<RangeInclusive<u64>>, Vec<u64>)> {
    let range = |input| {
        separated_pair(complete::u64, tag("-"), complete::u64)
            .map(|(a, b)| a..=b)
            .parse(input)
    };
    let ranges = |input| separated_list1(line_ending, range).parse(input);

    terminated(
        separated_pair(
            ranges,
            line_ending.and(line_ending),
            separated_list1(line_ending, complete::u64),
        ),
        opt(line_ending),
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_process() -> anyhow::Result<()> {
        todo!("haven't built test yet");
    }
}
