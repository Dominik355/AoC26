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

        let (largest_index, largest_val) = numbers[0..(numbers.len() - 1)]
            .iter()
            .enumerate()
            .reduce(|acc, next| if next.1 > acc.1 { next } else { acc })
            .unwrap();

        let second_largest_val = numbers[largest_index + 1..].iter().max().unwrap();

        let num = (largest_val * 10 + second_largest_val) as usize;

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
