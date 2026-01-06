#[tracing::instrument(skip_all)]
pub fn process(_input: String) -> anyhow::Result<impl std::fmt::Debug> {
    unimplemented!("{{day}} - part 1");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        todo!("haven't built test yet");
    }
}