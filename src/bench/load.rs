use super::prelude::*;

pub struct LoadBenchmarker;

impl Benchmarker<u8> for LoadBenchmarker {
    fn bench(&self, server: &Server) -> ScoreLogResult<u8> {
        Ok((server.load as u32, server.load))
    }
}

impl ParallelBenchmarker<u8> for LoadBenchmarker {}
