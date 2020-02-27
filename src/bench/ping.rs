use super::prelude::*;

// TODO: this
pub struct PingBenchmarker {}

#[derive(Debug)]
pub struct PingSummary {
    pub avg: u64,
    pub jitter: u64,
    pub stdderivation: f64,
}

impl Benchmarker<PingSummary> for PingBenchmarker {
    fn bench(&self, server: &Server) -> ScoreLogResult<PingSummary> {
        Ok((
            0,
            PingSummary {
                avg: 0,
                jitter: 0,
                stdderivation: 0.0,
            },
        ))
    }
}
