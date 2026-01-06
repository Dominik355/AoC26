#[tracing::instrument(skip_all)]
pub fn process(input: String) -> anyhow::Result<impl std::fmt::Debug> {
    // let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    let mut invalid_ids_sum = 0;

    for line in input.lines().map(|l| l.trim()).filter(|l| !l.is_empty()) {
        for (start, end) in line.split(",").filter_map(|x| {
            let (start, end) = x.split_once("-").unwrap();
            if start.starts_with("0") || end.starts_with("0") {
                None
            } else {
                Some((start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap()))
            }
        }) {
            for num in start..=end {
                let num_str = num.to_string();

                if num_str.len() % 2 != 0 {
                    continue;
                }

                let half = num_str.len() / 2;
                if num_str[..half] == num_str[half..] {
                    invalid_ids_sum += num;
                }
            }
        }
    }

    Ok(invalid_ids_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        todo!("haven't built test yet");
    }
}
