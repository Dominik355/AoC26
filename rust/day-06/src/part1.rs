#[tracing::instrument(skip_all)]
pub fn process(input: String) -> anyhow::Result<impl std::fmt::Debug> {
    let mut r = input.lines().rev();
    let symbols = r.next().unwrap().split_ascii_whitespace();
    let nums: Vec<Vec<u64>> = r
        .map(|r| {
            r.split_ascii_whitespace()
                .map(|n| n.parse::<u64>().unwrap())
                .collect()
        })
        .collect();

    let mut total: u64 = 0;
    for (index, symbol) in symbols.enumerate() {
        let current_nums = nums.iter().map(|row| row[index]);

        total += if symbol == "*" {
            current_nums.product::<u64>()
        } else {
            current_nums.sum()
        };
    }

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        todo!("haven't built test yet");
    }
}
