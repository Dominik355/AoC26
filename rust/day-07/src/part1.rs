use std::collections::HashSet;

#[tracing::instrument(skip_all)]
pub fn process(input: String) -> anyhow::Result<impl std::fmt::Debug> {
    let mut line_iter = input.lines().filter(|line| !line.is_empty());

    let first_line = line_iter.next().unwrap();
    let width = first_line.len();
    let beam_pos = first_line.chars().position(|c| c == 'S').unwrap();

    // pre-allocate max possible size
    // we could basically do this with 2 Vec<bool> and iterate over non-false values,
    let mut previous_indexes: HashSet<usize> = HashSet::with_capacity(width);
    let mut current_indexes: HashSet<usize> = HashSet::with_capacity(width);

    previous_indexes.insert(beam_pos);
    let mut splits = 0;
    for line in line_iter {
        let line_bytes = line.as_bytes();

        for i in previous_indexes.drain() {
            if line_bytes[i] == b'^' {
                current_indexes.insert(i - 1);
                current_indexes.insert(i + 1);
                splits += 1;
            } else {
                current_indexes.insert(i);
            }
        }
        std::mem::swap(&mut previous_indexes, &mut current_indexes);
    }

    Ok(splits)

    ////or we might do it with 'fold'
    // let mut new_positions: HashSet<usize> = HashSet::with_capacity(width);
    // Ok(line_iter
    //     .fold(
    //         (HashSet::from([beam_pos]), 0),
    //         |(mut positions, mut splits), line| {
    //             let line_bytes = line.as_bytes();
    //             for i in positions.drain() {
    //                 if line_bytes[i] == b'^' {
    //                     new_positions.insert(i - 1);
    //                     new_positions.insert(i + 1);
    //                     splits += 1;
    //                 } else {
    //                     new_positions.insert(i);
    //                 }
    //             }
    //             std::mem::swap(&mut positions, &mut new_positions);
    //             (positions, splits)
    //         },
    //     )
    //     .1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        todo!("haven't built test yet");
    }
}
