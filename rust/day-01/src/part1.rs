#[tracing::instrument]
pub fn process(input: String) -> anyhow::Result<impl std::fmt::Debug> {
    // example data: R10, L33
    let mut total1 = 0;
    let mut total2 = 0;
    let mut pos = 50;
    for l in input.lines() {
        let clicks = l[1..].parse::<i64>()?;
        if l.starts_with("L") {
            total2 += clicks / 100; // full rounds
            // pos == 0 evaluated later
            if pos != 0 && clicks % 100 >= pos {
                total2 += 1;
            }
            pos = (pos - clicks).rem_euclid(100); // otherwise use euclid - same as modulo in python is doing by default for negative
        } else {
            pos += clicks;
            total2 += pos / 100;
            pos %= 100; // modulo is good enough when dealing with positive numbers
        }
        if pos == 0 {
            total1 += 1;
        }
    }

    Ok((total1, total2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        todo!("haven't built test yet");
    }
}
