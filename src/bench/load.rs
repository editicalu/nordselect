use super::{Benchmarker, ParallelBenchmarker};
use crate::Server;

pub struct LoadBenchmarker;

impl Benchmarker for LoadBenchmarker {
    fn bench(server: &Server) -> u32 {
        server.load as u32
    }
}

impl ParallelBenchmarker for LoadBenchmarker {}
