#[tracing::instrument(skip_all)]
pub fn process(input: String) -> anyhow::Result<impl std::fmt::Debug> {
    let mut numbers = vec![0; 15];
    let mut total = 0;

    for line in input.lines().map(|l| l.trim()).filter(|l| !l.is_empty()) {
        let bytes = line.as_bytes();
        if bytes.len() != numbers.len() {
            numbers.resize(bytes.len(), 0);
        }
        for &b in bytes {
            numbers.push(b - b'0');
        }

        let mut chosen_batteries = Vec::with_capacity(12);
        let mut current_index = 0;
        for i in 0..12 {
            let (largest_index, largest_val) = numbers[current_index..(numbers.len() - 11 + i)]
                .iter()
                .enumerate()
                .reduce(|acc, next| if next.1 > acc.1 { next } else { acc })
                .unwrap();
            chosen_batteries.push(*largest_val);
            current_index += largest_index + 1;
        }

        let mut num = 0;
        let mut multiplier = 1;
        for n in chosen_batteries.iter().rev() {
            num += *n as usize * multiplier;
            multiplier *= 10;
        }

        total += num;
        tracing::info!("largest in line {} is {}", line, num);
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
