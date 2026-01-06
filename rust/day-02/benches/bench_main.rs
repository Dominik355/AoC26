mod part1;
mod part2;

criterion::criterion_main! {
    part1::bench,
    part2::bench,
}
