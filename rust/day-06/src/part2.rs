// assuming that all the lines have same length
#[tracing::instrument(skip_all)]
pub fn process(input: String) -> anyhow::Result<impl std::fmt::Debug> {
    //     let input = r#"
    // 123 328  51 64
    //  45 64  387 23
    //   6 98  215 314
    // *   +   *   + "#;

    let mut lines = input.lines().filter(|l| !l.is_empty()).rev();
    let symbols: &[u8] = lines.next().unwrap().as_bytes();
    let str_nums: Vec<&[u8]> = lines.map(|r| r.as_bytes()).collect();

    // debug print
    {
        println!("symbols: {:?}", String::from_utf8_lossy(symbols));
        for s in &str_nums {
            println!("row    : {:?}", String::from_utf8_lossy(s));
        }
    }

    // find largest width
    let width = std::cmp::max(
        symbols.len(),
        str_nums.iter().map(|l| l.len()).max().unwrap(),
    );

    let mut total: u64 = 0;
    let mut current_numbers: Vec<u64> = Vec::with_capacity(3); // no more than 3 numbers per column
    for cursor in (0..width).rev() {
        let mut formed_num: u64 = 0;
        let mut multiplier = 1;
        for row in &str_nums {
            if cursor < row.len() && row[cursor].is_ascii_digit() {
                let d = (row[cursor] - b'0') as u64;
                if d != 0 {
                    formed_num += d * multiplier;
                    multiplier *= 10;
                }
            }
        }
        if formed_num > 0 {
            current_numbers.push(formed_num);
        }

        if cursor < symbols.len() && symbols[cursor] != b' ' {
            // end of the column, calculate
            let mut drained = current_numbers.drain(..);
            let column_total = drained
                .next()
                .map(|first| {
                    drained.fold(first, |acc, n| match symbols[cursor] {
                        b'+' => acc + n,
                        b'*' => acc * n,
                        _ => panic!("Invalid symbol"),
                    })
                })
                .unwrap_or(0);

            total += column_total;

            println!("adding column total: {column_total}, grand total: {total}");
        }
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
