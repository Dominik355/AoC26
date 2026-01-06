use anyhow::Context;
use day_04::{get_input_str, parse_day, part1::process};

#[tracing::instrument]
fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let day_str = "day-04";
    let input_data =
        get_input_str(parse_day(day_str)?.1, std::env::args()).context("failed to read input")?;
    let result = process(input_data).context("part 1 failed")?;
    tracing::info!("{:?}", result);
    Ok(())
}
