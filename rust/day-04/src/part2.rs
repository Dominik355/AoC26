use itertools::Itertools;

#[tracing::instrument(skip_all)]
pub fn process(input: String) -> anyhow::Result<impl std::fmt::Debug> {
    let height = input.lines().try_len().unwrap_or(input.lines().count());
    let width = input.lines().next().unwrap().len();
    let mut grid: Vec<bool> = vec![false; width * height];

    for (row_index, line) in input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .enumerate()
    {
        for (column_index, ch) in line.bytes().enumerate() {
            let index = row_index * width + column_index;
            match ch {
                b'@' => grid[index] = true,
                b'.' => grid[index] = false,
                _ => panic!("unexpected char"),
            }
        }
    }

    print_grid(&grid, height);

    let mut total_movable = 0;
    loop {
        let mut movable = 0;
        let mut neighbors = [(false, 0isize); 8];
        let width_isize = width as isize;
        let mut erase_indexes = vec![];
        for (i, &cell) in grid.iter().enumerate() {
            if cell {
                let row = i / width;
                let col = i % width;
                let i_isize = i as isize;

                // has to use isize, because we may go negative on first row/column
                neighbors[0] = (row > 0, i_isize - width_isize);
                neighbors[1] = (row < height - 1, i_isize + width_isize);
                neighbors[2] = (col > 0, i_isize - 1);
                neighbors[3] = (col < width - 1, i_isize + 1);
                neighbors[4] = (row > 0 && col > 0, i_isize - width_isize - 1);
                neighbors[5] = (row > 0 && col < width - 1, i_isize - width_isize + 1);
                neighbors[6] = (row < height - 1 && col > 0, i_isize + width_isize - 1);
                neighbors[7] = (
                    row < height - 1 && col < width - 1,
                    i_isize + width_isize + 1,
                );

                if neighbors
                    .iter()
                    .filter(|(valid, idx)| *valid && grid[*idx as usize])
                    .count()
                    < 4
                {
                    movable += 1;
                    erase_indexes.push(i);
                }
            }
        }

        total_movable += movable;
        if movable == 0 {
            break;
        }

        // remove everything that has been moved this iteration
        for removal in erase_indexes {
            grid[removal] = false;
        }
    }

    Ok(total_movable)
}

fn print_grid(grid: &[bool], width: usize) {
    let mut cursor = 0;

    for cell in grid.iter() {
        if *cell {
            print!("@");
        } else {
            print!(".");
        }

        cursor += 1;
        if cursor % width == 0 {
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        todo!("haven't built test yet");
    }
}
