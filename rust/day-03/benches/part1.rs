use criterion::{BatchSize, Criterion, criterion_group};

fn solution(c: &mut Criterion) {
    let file_path = std::env::temp_dir().join("aoc25").join("day03.txt");
    let input = std::fs::read_to_string(file_path).expect("failed to read file");

    c.bench_function("process_part1", |b| {
        b.iter_batched(
            || input.clone(),
            |i| day_03::part1::process(i).unwrap(),
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(bench, solution,);
