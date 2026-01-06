use anyhow::anyhow;
use nom::IResult;
use nom::Parser;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use std::ops::RangeInclusive;
use tracing::info;

#[tracing::instrument(skip_all)]
pub fn process(input: String) -> anyhow::Result<impl std::fmt::Debug> {
    let mut invalid_ids_sum = 0;

    for line in input.lines().map(|l| l.trim()).filter(|l| !l.is_empty()) {
        for id in nom::combinator::all_consuming(ranges)
            .parse(line)
            .map_err(|e| anyhow!("failed to parse AOC input: {e}"))?
            .1
            .into_iter()
            .flatten()
        {
            let id_str = id.to_string();

            let half = id_str.len() / 2;

            for l in 0..half {
                if id_str.len().rem_euclid(l + 1) != 0 {
                    // ignore those where the multiples of repetitions do not equal the length of the number
                    continue;
                }
                if id_str[0..=l]
                    .chars()
                    .cycle()
                    .zip(id_str.chars())
                    .all(|(a, b)| a == b)
                {
                    info!(?id);
                    invalid_ids_sum += id;
                    break;
                }
            }
        }
    }

    Ok(invalid_ids_sum)
}

// lets try to replace stripping with nom crate
fn ranges(line: &str) -> IResult<&str, Vec<RangeInclusive<u64>>> {
    separated_list1(
        tag(","),
        separated_pair(complete::u64, tag("-"), complete::u64).map(|(start, end)| start..=end),
    )
    .parse(line)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        todo!("haven't built test yet");
    }
}
