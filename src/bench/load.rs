use super::*;
use crate::Server;

pub struct LoadBenchmarker;

impl Benchmarker for LoadBenchmarker {
    fn bench(&self, server: &Server) -> u32 {
        server.load as u32
    }
}

impl ParallelBenchmarker for LoadBenchmarker {}

impl LoggableBenchmarker<u8> for LoadBenchmarker {
    fn bench_log(&self, server: &Server) -> (u32, u8) {
        (server.load as u32, server.load)
    }
}
